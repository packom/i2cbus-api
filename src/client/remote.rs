//!
//! (c) Piers Finlayson 2018
//!
//! This module provides a front-end to the API client implementation which
//! makes it easy to use the client from within another Rust program, by
//! taking care of running its own reactor to drive the asynchronous work
//! implicit is working with a RESTful API.
//!
//! To use, create a new handle then call the methods on this object:
//!
//! ```
//! let handle = Handle::new("http://ip:port");
//! let rsp = handle.write_bytes(...);
//! // Handle response
//! ...
//! ```
//!
//! Calls can of course be chained together
//!
//! See pca9956b/src/http.rs for more comprehensive usage examples
//!

extern crate chrono;
extern crate hyper_tls;
extern crate mime;
extern crate native_tls;
extern crate openssl;
extern crate tokio_core;

use self::tokio_core::reactor;
use super::super::models::*;
use super::Client;
use futures::sync::{mpsc, oneshot};
use futures::{Future, Stream};
use std::sync::mpsc::SendError;
use std::sync::Mutex;
use std::thread;
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

#[allow(unused_imports)]
use {
    Api, ApiError, ApiNoContext, ContextWrapperExt, I2cBusApiResponse, I2cBusListResponse,
    I2cBusReadByteResponse, I2cBusReadBytesResponse, I2cBusReadRegResponse,
    I2cBusWriteByteRegResponse, I2cBusWriteByteResponse, I2cBusWriteBytesRegResponse,
    I2cBusWriteBytesResponse,
};

#[derive(Debug)]
pub enum Response {
    Error(ApiError),
    Api(I2cBusApiResponse),
    List(I2cBusListResponse),
    ReadByte(I2cBusReadByteResponse),
    ReadBytes(I2cBusReadBytesResponse),
    ReadReg(I2cBusReadRegResponse),
    WriteByte(I2cBusWriteByteResponse),
    WriteByteReg(I2cBusWriteByteRegResponse),
    WriteBytes(I2cBusWriteBytesResponse),
    WriteBytesReg(I2cBusWriteBytesRegResponse),
}

#[derive(Clone)]
pub enum RequestType {
    Api,
    List,
    ReadByte {
        bus_id: BusId,
        addr: Addr,
    },
    ReadBytes {
        bus_id: BusId,
        addr: Addr,
        num_bytes: NumBytes,
    },
    ReadReg {
        bus_id: BusId,
        addr: Addr,
        reg: Reg,
        num_bytes: NumBytes,
    },
    WriteByte {
        bus_id: BusId,
        addr: Addr,
        value: Value,
    },
    WriteByteReg {
        bus_id: BusId,
        addr: Addr,
        reg: Reg,
        value: Value,
    },
    WriteBytes {
        bus_id: BusId,
        addr: Addr,
        values: Values,
    },
    WriteBytesReg {
        bus_id: BusId,
        addr: Addr,
        reg: Reg,
        values: Values,
    },
}

struct Request {
    ty: RequestType,
    sender: oneshot::Sender<Response>,
}

#[derive(Clone)]
pub struct Handle {
    sender: mpsc::UnboundedSender<Request>,
}

// fn: function name (should be one of Client::Api::fns)
// fn_ret: fn OK return type (e.g. I2cBusWriteBytesResponse)
// ty: RequestType and Response type (e.g. WriteBytes)
// arg: the args for this function call
macro_rules! make_api_call {
    ( $fn:ident, $fn_ret:ident, $ty:ident, $( $arg:tt, $arg_ty:ident),* ) => {
        pub fn $fn(
            &self,
            $( $arg: $arg_ty, )*
        ) -> Box<Future<Item = $fn_ret, Error = ApiError>> {
            let (rsp_tx, rsp_rx) = oneshot::channel::<Response>();
            match self.sender.unbounded_send(Request {
                ty: RequestType::$ty {
                    $( $arg ),*
                },
                sender: rsp_tx,
            }) {
                Ok(_) => {
                    Box::new(rsp_rx
                        .map(|rsp| match rsp {
                            Response::$ty(x) => x,
                            Response::Error(e) => {
                                $fn_ret::TransactionFailed(I2cBusError {
                                    error: None,
                                    description: Some(format!("Client API Error: {}", e).to_string()), // Would rather turn into ApiError but can't figure out how to do with futures
                                })
                            },
                            _ => panic!("Hit invalid match arm"),
                        })
                        .map_err(|rsp| match rsp {
                            oneshot::Canceled => ApiError("oneshot cancelled error".to_string()), // XXX Replace with error handling
                        })
                    )
                },
                Err(e) => Box::new(futures::future::err(ApiError(
                    format!("unbounded_send failure {:?}", e).to_string(),
                ))),
            }
        }
    }
}

impl Handle {
    pub fn new(url: &'static str) -> Handle {

        let (s, r) = mpsc::unbounded::<Request>();

        thread::spawn(move || {
            let mut core = reactor::Core::new().unwrap();
            let core_handle = core.handle();
            let i2cbus = Client::try_new_http(core_handle.clone(), url)
                .unwrap_or_else(|_| panic!("Failed to connect to I2C bus at {}", url));
            info!("Created connection to I2C bus at {}", url);
            let context: make_context_ty!(
                ContextBuilder,
                EmptyContext,
                Option<AuthData>,
                XSpanIdString
            ) = make_context!(
                ContextBuilder,
                EmptyContext,
                None,
                XSpanIdString(uuid::Uuid::new_v4().to_string())
            );
            let i2cbus_loop = r
                .map_err(|e| warn!("I2C bus API error = {:?}", e))
                .for_each(move |request| {
                    let future = handle_request(request, &i2cbus, &context);
                    core_handle.spawn(future);
                    Ok(())
                });

            core.run(i2cbus_loop)
                .expect("Failed to start i2cbus reactor core loop.");
        });

        Handle { sender: s }
    }

    make_api_call!(
        i2c_bus_read_byte,
        I2cBusReadByteResponse,
        ReadByte,
        bus_id,
        BusId,
        addr,
        Addr
    );

    make_api_call!(
        i2c_bus_read_bytes,
        I2cBusReadBytesResponse,
        ReadBytes,
        bus_id,
        BusId,
        addr,
        Addr,
        num_bytes,
        NumBytes
    );

    make_api_call!(
        i2c_bus_read_reg,
        I2cBusReadRegResponse,
        ReadReg,
        bus_id,
        BusId,
        addr,
        Addr,
        reg,
        Reg,
        num_bytes,
        NumBytes
    );

    make_api_call!(
        i2c_bus_write_byte,
        I2cBusWriteByteResponse,
        WriteByte,
        bus_id,
        BusId,
        addr,
        Addr,
        value,
        Value
    );

    make_api_call!(
        i2c_bus_write_byte_reg,
        I2cBusWriteByteRegResponse,
        WriteByteReg,
        bus_id,
        BusId,
        addr,
        Addr,
        reg,
        Reg,
        value,
        Value
    );

    make_api_call!(
        i2c_bus_write_bytes,
        I2cBusWriteBytesResponse,
        WriteBytes,
        bus_id,
        BusId,
        addr,
        Addr,
        values,
        Values
    );

    make_api_call!(
        i2c_bus_write_bytes_reg,
        I2cBusWriteBytesRegResponse,
        WriteBytesReg,
        bus_id,
        BusId,
        addr,
        Addr,
        reg,
        Reg,
        values,
        Values
    );

}

fn handle_receiver(
    rsp: oneshot::Receiver<Response>,
) -> Box<Future<Item = I2cBusWriteBytesResponse, Error = ApiError>> {
    Box::new(rsp.then(|r| {
        if true {
            return Box::new(futures::future::ok(
                I2cBusWriteBytesResponse::TransactionFailed(I2cBusError {
                    error: None,
                    description: None,
                }),
            ));
        } else {
            return Box::new(futures::future::err(ApiError(
                "unbounded_send failure".to_string(), // Can be API Error
            )));
        }
    }))
}

fn handle_request_api(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(i2cbus.i2c_bus_api(context).then(|result| {
        let response = match result {
            Ok(x) => Response::Api(x),
            Err(e) => Response::Error(e),
        };
        match request.sender.send(response) {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Failed to return Api call {:?}", e);
                Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
            }
        }
    }))
}

fn handle_request_list(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(i2cbus.i2c_bus_list(context).then(|result| {
        let response = match result {
            Ok(x) => Response::List(x),
            Err(e) => Response::Error(e),
        };
        match request.sender.send(response) {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Failed to return List call {:?}", e);
                Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
            }
        }
    }))
}

fn handle_request_read_byte(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
    bus_id: BusId,
    addr: Addr,
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(
        i2cbus
            .i2c_bus_read_byte(bus_id, addr, context)
            .then(|result| {
                let response = match result {
                    Ok(x) => Response::ReadByte(x),
                    Err(e) => Response::Error(e),
                };
                match request.sender.send(response) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to return ReadByte call {:?}", e);
                        Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
                    }
                }
            }),
    )
}

fn handle_request_read_bytes(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
    bus_id: BusId,
    addr: Addr,
    num_bytes: NumBytes,
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(
        i2cbus
            .i2c_bus_read_bytes(bus_id, addr, num_bytes, context)
            .then(|result| {
                let response = match result {
                    Ok(x) => Response::ReadBytes(x),
                    Err(e) => Response::Error(e),
                };
                match request.sender.send(response) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to return ReadBytes call {:?}", e);
                        Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
                    }
                }
            }),
    )
}

fn handle_request_read_reg(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
    bus_id: BusId,
    addr: Addr,
    reg: Reg,
    num_bytes: NumBytes,
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(
        i2cbus
            .i2c_bus_read_reg(bus_id, addr, reg, num_bytes, context)
            .then(|result| {
                let response = match result {
                    Ok(x) => Response::ReadReg(x),
                    Err(e) => Response::Error(e),
                };
                match request.sender.send(response) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to return ReadReg call {:?}", e);
                        Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
                    }
                }
            }),
    )
}

fn handle_request_write_byte(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
    bus_id: BusId,
    addr: Addr,
    value: Value,
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(
        i2cbus
            .i2c_bus_write_byte(bus_id, addr, value, context)
            .then(|result| {
                let response = match result {
                    Ok(x) => Response::WriteByte(x),
                    Err(e) => Response::Error(e),
                };
                match request.sender.send(response) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to return WriteByte call {:?}", e);
                        Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
                    }
                }
            }),
    )
}

fn handle_request_write_byte_reg(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
    bus_id: BusId,
    addr: Addr,
    reg: Reg,
    value: Value,
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(
        i2cbus
            .i2c_bus_write_byte_reg(bus_id, addr, reg, value, context)
            .then(|result| {
                let response = match result {
                    Ok(x) => Response::WriteByteReg(x),
                    Err(e) => Response::Error(e),
                };
                match request.sender.send(response) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to return WriteByteReg call {:?}", e);
                        Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
                    }
                }
            }),
    )
}

fn handle_request_write_bytes(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
    bus_id: BusId,
    addr: Addr,
    values: Values,
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(
        i2cbus
            .i2c_bus_write_bytes(bus_id, addr, values, context)
            .then(|result| {
                let response = match result {
                    Ok(x) => Response::WriteBytes(x),
                    Err(e) => Response::Error(e),
                };
                match request.sender.send(response) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to return WriteBytes call {:?}", e);
                        Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
                    }
                }
            }),
    )
}

fn handle_request_write_bytes_reg(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
    bus_id: BusId,
    addr: Addr,
    reg: Reg,
    values: Values,
) -> Box<dyn Future<Item = (), Error = ()>> {
    Box::new(
        i2cbus
            .i2c_bus_write_bytes_reg(bus_id, addr, reg, values, context)
            .then(|result| {
                let response = match result {
                    Ok(x) => Response::WriteBytesReg(x),
                    Err(e) => Response::Error(e),
                };
                match request.sender.send(response) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to return WriteBytesReg call {:?}", e);
                        Err(()) // Can only return Err(()) to the handle thread, which can't really do anything about it - this is going to lead to a hung request
                    }
                }
            }),
    )
}

fn handle_request(
    request: Request,
    i2cbus: &Client<hyper::client::FutureResponse>,
    context: &make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ),
) -> impl Future<Item = (), Error = ()> {
    let ty = request.ty.clone();
    match ty {
        RequestType::Api => handle_request_api(request, i2cbus, context),
        RequestType::List => handle_request_list(request, i2cbus, context),
        RequestType::ReadByte { bus_id, addr } => {
            handle_request_read_byte(request, i2cbus, context, bus_id, addr)
        }
        RequestType::ReadBytes {
            bus_id,
            addr,
            num_bytes,
        } => handle_request_read_bytes(request, i2cbus, context, bus_id, addr, num_bytes),
        RequestType::ReadReg {
            bus_id,
            addr,
            reg,
            num_bytes,
        } => handle_request_read_reg(request, i2cbus, context, bus_id, addr, reg, num_bytes),
        RequestType::WriteByte {
            bus_id,
            addr,
            value,
        } => handle_request_write_byte(request, i2cbus, context, bus_id, addr, value),
        RequestType::WriteByteReg {
            bus_id,
            addr,
            reg,
            value,
        } => handle_request_write_byte_reg(request, i2cbus, context, bus_id, addr, reg, value),
        RequestType::WriteBytes {
            bus_id,
            addr,
            values,
        } => handle_request_write_bytes(request, i2cbus, context, bus_id, addr, values),
        RequestType::WriteBytesReg {
            bus_id,
            addr,
            reg,
            values,
        } => handle_request_write_bytes_reg(request, i2cbus, context, bus_id, addr, reg, values),
    }
}
