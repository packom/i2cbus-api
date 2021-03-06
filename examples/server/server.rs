//! Main library entry point for i2cbus_api implementation.

#![allow(unused_imports)]

mod errors {
    error_chain::error_chain!{}
}

pub use self::errors::*;

use chrono;
use futures::{future, Future, Stream};
use hyper::server::conn::Http;
use hyper::service::MakeService as _;
use log::info;
use openssl::ssl::SslAcceptorBuilder;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use swagger;
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;


#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use tokio_openssl::SslAcceptorExt;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use i2cbus_api::models;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub fn create(addr: &str, https: bool) -> Box<dyn Future<Item = (), Error = ()> + Send> {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service_fn = MakeService::new(server);

    let service_fn = MakeAllowAllAuthenticator::new(service_fn, "cosmo");

    let service_fn =
        i2cbus_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service_fn
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set cerificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let service_fn = Arc::new(Mutex::new(service_fn));
            let tls_listener = TcpListener::bind(&addr).unwrap().incoming().for_each(move |tcp| {
                let addr = tcp.peer_addr().expect("Unable to get remote address");

                let service_fn = service_fn.clone();

                hyper::rt::spawn(tls_acceptor.accept_async(tcp).map_err(|_| ()).and_then(move |tls| {
                    let ms = {
                        let mut service_fn = service_fn.lock().unwrap();
                        service_fn.make_service(&addr)
                    };

                    ms.and_then(move |service| {
                        Http::new().serve_connection(tls, service)
                    }).map_err(|_| ())
                }));

                Ok(())
            }).map_err(|_| ());

            Box::new(tls_listener)
        }
    } else {
        // Using HTTP
        Box::new(hyper::server::Server::bind(&addr).serve(service_fn).map_err(|e| panic!("{:?}", e)))
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use i2cbus_api::{
    Api,
    ApiError,
    I2cBusApiResponse,
    I2cBusListResponse,
    I2cBusReadByteResponse,
    I2cBusReadBytesResponse,
    I2cBusReadRegResponse,
    I2cBusWriteByteResponse,
    I2cBusWriteByteRegResponse,
    I2cBusWriteBytesResponse,
    I2cBusWriteBytesRegResponse,
};
use i2cbus_api::server::MakeService;

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{
    fn i2c_bus_api(
        &self,
        context: &C) -> Box<dyn Future<Item=I2cBusApiResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_api() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn i2c_bus_list(
        &self,
        context: &C) -> Box<dyn Future<Item=I2cBusListResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_list() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn i2c_bus_read_byte(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=I2cBusReadByteResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_read_byte({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn i2c_bus_read_bytes(
        &self,
        bus_id: i32,
        addr: i32,
        num_bytes: i32,
        context: &C) -> Box<dyn Future<Item=I2cBusReadBytesResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_read_bytes({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num_bytes, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn i2c_bus_read_reg(
        &self,
        bus_id: i32,
        addr: i32,
        reg: i32,
        num_bytes: i32,
        context: &C) -> Box<dyn Future<Item=I2cBusReadRegResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_read_reg({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, reg, num_bytes, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn i2c_bus_write_byte(
        &self,
        bus_id: i32,
        addr: i32,
        value: i32,
        context: &C) -> Box<dyn Future<Item=I2cBusWriteByteResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_write_byte({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, value, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn i2c_bus_write_byte_reg(
        &self,
        bus_id: i32,
        addr: i32,
        reg: i32,
        value: i32,
        context: &C) -> Box<dyn Future<Item=I2cBusWriteByteRegResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_write_byte_reg({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, reg, value, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn i2c_bus_write_bytes(
        &self,
        bus_id: i32,
        addr: i32,
        values: models::Values,
        context: &C) -> Box<dyn Future<Item=I2cBusWriteBytesResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_write_bytes({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, values, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn i2c_bus_write_bytes_reg(
        &self,
        bus_id: i32,
        addr: i32,
        reg: i32,
        values: models::Values,
        context: &C) -> Box<dyn Future<Item=I2cBusWriteBytesRegResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("i2c_bus_write_bytes_reg({}, {}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, reg, values, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

}
