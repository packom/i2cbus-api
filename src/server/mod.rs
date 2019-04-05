#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate uuid;
extern crate chrono;
extern crate percent_encoding;
extern crate url;


use std::sync::Arc;
use std::marker::PhantomData;
use std::fs;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use hyper::header::{Headers, ContentType};
use self::url::form_urlencoded;
use mimetypes;

use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::{ApiError, XSpanId, XSpanIdString, Has, RequestParser};
use swagger::auth::Scopes;

use {Api,
     I2cBusApiResponse,
     I2cBusListResponse,
     I2cBusReadByteResponse,
     I2cBusReadBytesResponse,
     I2cBusReadRegResponse,
     I2cBusWriteByteResponse,
     I2cBusWriteByteRegResponse,
     I2cBusWriteBytesResponse,
     I2cBusWriteBytesRegResponse
     };
#[allow(unused_imports)]
use models;

pub mod context;

header! { (Warning, "Warning") => [String] }

mod paths {
    extern crate regex;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(&[
            r"^/i2c/api$",
            r"^/i2c/buslist$",
            r"^/i2c/bus/(?P<busId>[^/?#]*)/read/byte/(?P<addr>[^/?#]*)$",
            r"^/i2c/bus/(?P<busId>[^/?#]*)/read/bytes/(?P<addr>[^/?#]*)/(?P<numBytes>[^/?#]*)$",
            r"^/i2c/bus/(?P<busId>[^/?#]*)/read/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)/(?P<numBytes>[^/?#]*)$",
            r"^/i2c/bus/(?P<busId>[^/?#]*)/write/byte/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)/(?P<value>[^/?#]*)$",
            r"^/i2c/bus/(?P<busId>[^/?#]*)/write/byte/(?P<addr>[^/?#]*)/(?P<value>[^/?#]*)$",
            r"^/i2c/bus/(?P<busId>[^/?#]*)/write/bytes/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)$",
            r"^/i2c/bus/(?P<busId>[^/?#]*)/write/bytes/(?P<addr>[^/?#]*)$"
        ]).unwrap();
    }
    pub static ID_I2C_BUS_API: usize = 0;
    pub static ID_I2C_BUS_LIST: usize = 1;
    pub static ID_I2C_BUS_BUSID_READ_BYTE_ADDR: usize = 2;
    lazy_static! {
        pub static ref REGEX_I2C_BUS_BUSID_READ_BYTE_ADDR: regex::Regex = regex::Regex::new(r"^/i2c/bus/(?P<busId>[^/?#]*)/read/byte/(?P<addr>[^/?#]*)$").unwrap();
    }
    pub static ID_I2C_BUS_BUSID_READ_BYTES_ADDR_NUMBYTES: usize = 3;
    lazy_static! {
        pub static ref REGEX_I2C_BUS_BUSID_READ_BYTES_ADDR_NUMBYTES: regex::Regex = regex::Regex::new(r"^/i2c/bus/(?P<busId>[^/?#]*)/read/bytes/(?P<addr>[^/?#]*)/(?P<numBytes>[^/?#]*)$").unwrap();
    }
    pub static ID_I2C_BUS_BUSID_READ_REG_ADDR_REG_NUMBYTES: usize = 4;
    lazy_static! {
        pub static ref REGEX_I2C_BUS_BUSID_READ_REG_ADDR_REG_NUMBYTES: regex::Regex = regex::Regex::new(r"^/i2c/bus/(?P<busId>[^/?#]*)/read/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)/(?P<numBytes>[^/?#]*)$").unwrap();
    }
    pub static ID_I2C_BUS_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE: usize = 5;
    lazy_static! {
        pub static ref REGEX_I2C_BUS_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE: regex::Regex = regex::Regex::new(r"^/i2c/bus/(?P<busId>[^/?#]*)/write/byte/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)/(?P<value>[^/?#]*)$").unwrap();
    }
    pub static ID_I2C_BUS_BUSID_WRITE_BYTE_ADDR_VALUE: usize = 6;
    lazy_static! {
        pub static ref REGEX_I2C_BUS_BUSID_WRITE_BYTE_ADDR_VALUE: regex::Regex = regex::Regex::new(r"^/i2c/bus/(?P<busId>[^/?#]*)/write/byte/(?P<addr>[^/?#]*)/(?P<value>[^/?#]*)$").unwrap();
    }
    pub static ID_I2C_BUS_BUSID_WRITE_BYTES_REG_ADDR_REG: usize = 7;
    lazy_static! {
        pub static ref REGEX_I2C_BUS_BUSID_WRITE_BYTES_REG_ADDR_REG: regex::Regex = regex::Regex::new(r"^/i2c/bus/(?P<busId>[^/?#]*)/write/bytes/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)$").unwrap();
    }
    pub static ID_I2C_BUS_BUSID_WRITE_BYTES_ADDR: usize = 8;
    lazy_static! {
        pub static ref REGEX_I2C_BUS_BUSID_WRITE_BYTES_ADDR: regex::Regex = regex::Regex::new(r"^/i2c/bus/(?P<busId>[^/?#]*)/write/bytes/(?P<addr>[^/?#]*)$").unwrap();
    }
}

pub struct NewService<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> NewService<T, C> {
        NewService{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::NewService for NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Instance = Service<T, C>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(Service::new(self.api_impl.clone()))
    }
}

pub struct Service<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> Service<T, C> {
        Service{api_impl: api_impl.into(), marker: PhantomData}
    }
}

macro_rules! parse_error {
    ($ctx:expr, $arg: expr, $mt:ident, $e: expr) => {
        {
            let mut response = Response::new();
            response.headers_mut().set(XSpanId((&$ctx as &Has<XSpanIdString>).get().0.to_string()));
            response.set_status(StatusCode::BadRequest);
            response.headers_mut().set(ContentType(mimetypes::responses::$mt.clone()));
            let http_err = models::I2cBusArg {
                arg: Some($arg.to_string()),
                description: Some($e),
            };
            let body = serde_json::to_string(&http_err).expect("impossible to fail to serialize");
            response.set_body(body);
            return Box::new(future::ok(response))
        }
    };
}

impl<T, C> hyper::server::Service for Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Response, Error=Error>>;

    fn call(&self, (req, mut context): Self::Request) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (method, uri, _, headers, body) = req.deconstruct();
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        // This match statement is duplicated below in `parse_operation_id()`.
        // Please update both places if changing how this code is autogenerated.
        match &method {

            // I2cBusApi - GET /i2c/bus/api
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_API) => {







                Box::new({
                        {{

                                Box::new(api_impl.i2c_bus_api(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusApiResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_API_OK.clone()));


                                                    response.set_body(String::from(body));
                                                },
                                                I2cBusApiResponse::FileNotFound

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_API_FILE_NOT_FOUND.clone()));


                                                    response.set_body(String::from(body));
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // I2cBusList - GET /i2c/bus/list
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_LIST) => {







                Box::new({
                        {{

                                Box::new(api_impl.i2c_bus_list(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusListResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_LIST_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // I2cBusReadByte - GET /i2c/bus/{busId}/read/byte/{addr}
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_BUSID_READ_BYTE_ADDR) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUS_BUSID_READ_BYTE_ADDR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUS_BUSID_READ_BYTE_ADDR in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUS_BUSID_READ_BYTE_ADDR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<models::BusId>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => parse_error!(
                            context,
                            "busId",
                            I2C_BUS_READ_BYTE_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "busId",
                        I2C_BUS_READ_BYTE_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<models::Addr>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => parse_error!(
                            context,
                            "addr",
                            I2C_BUS_READ_BYTE_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "addr",
                        I2C_BUS_READ_BYTE_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };





                Box::new({
                        {{

                                Box::new(api_impl.i2c_bus_read_byte(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusReadByteResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_BYTE_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusReadByteResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_BYTE_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusReadByteResponse::TransactionFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_BYTE_TRANSACTION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // I2cBusReadBytes - GET /i2c/bus/{busId}/read/bytes/{addr}/{numBytes}
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_BUSID_READ_BYTES_ADDR_NUMBYTES) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUS_BUSID_READ_BYTES_ADDR_NUMBYTES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUS_BUSID_READ_BYTES_ADDR_NUMBYTES in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUS_BUSID_READ_BYTES_ADDR_NUMBYTES.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<models::BusId>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => parse_error!(
                            context,
                            "busId",
                            I2C_BUS_READ_BYTES_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "busId",
                        I2C_BUS_READ_BYTES_BAD_REQUEST,
                        format!("Couldn't parse path parameter, {}", e)
                    ),
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<models::Addr>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => parse_error!(
                            context,
                            "addr",
                            I2C_BUS_READ_BYTES_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "addr",
                        I2C_BUS_READ_BYTES_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_num_bytes = match percent_encoding::percent_decode(path_params["numBytes"].as_bytes()).decode_utf8() {
                    Ok(param_num_bytes) => match param_num_bytes.parse::<models::NumBytes>() {
                        Ok(param_num_bytes) => param_num_bytes,
                        Err(e) => parse_error!(
                            context,
                            "numBytes",
                            I2C_BUS_READ_BYTES_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "numBytes",
                        I2C_BUS_READ_BYTES_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };





                Box::new({
                        {{

                                Box::new(api_impl.i2c_bus_read_bytes(param_bus_id, param_addr, param_num_bytes, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusReadBytesResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_BYTES_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusReadBytesResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_BYTES_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusReadBytesResponse::TransactionFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_BYTES_TRANSACTION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // I2cBusReadReg - GET /i2c/bus/{busId}/read/reg/{addr}/{reg}/{numBytes}
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_BUSID_READ_REG_ADDR_REG_NUMBYTES) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUS_BUSID_READ_REG_ADDR_REG_NUMBYTES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUS_BUSID_READ_REG_ADDR_REG_NUMBYTES in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUS_BUSID_READ_REG_ADDR_REG_NUMBYTES.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<models::BusId>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => parse_error!(
                            context,
                            "busId",
                            I2C_BUS_READ_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "busId",
                        I2C_BUS_READ_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<models::Addr>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => parse_error!(
                            context,
                            "addr",
                            I2C_BUS_READ_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "addr",
                        I2C_BUS_READ_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_reg = match percent_encoding::percent_decode(path_params["reg"].as_bytes()).decode_utf8() {
                    Ok(param_reg) => match param_reg.parse::<models::Reg>() {
                        Ok(param_reg) => param_reg,
                        Err(e) => parse_error!(
                            context,
                            "reg",
                            I2C_BUS_READ_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "reg",
                        I2C_BUS_READ_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_num_bytes = match percent_encoding::percent_decode(path_params["numBytes"].as_bytes()).decode_utf8() {
                    Ok(param_num_bytes) => match param_num_bytes.parse::<models::NumBytes>() {
                        Ok(param_num_bytes) => param_num_bytes,
                        Err(e) => parse_error!(
                            context,
                            "numBytes",
                            I2C_BUS_READ_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "numBytes",
                        I2C_BUS_READ_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };





                Box::new({
                        {{

                                Box::new(api_impl.i2c_bus_read_reg(param_bus_id, param_addr, param_reg, param_num_bytes, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusReadRegResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_REG_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusReadRegResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_REG_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusReadRegResponse::TransactionFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_READ_REG_TRANSACTION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // I2cBusWriteByte - POST /i2c/bus/{busId}/write/byte/{addr}/{value}
            &hyper::Method::Post if path.matched(paths::ID_I2C_BUS_BUSID_WRITE_BYTE_ADDR_VALUE) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUS_BUSID_WRITE_BYTE_ADDR_VALUE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUS_BUSID_WRITE_BYTE_ADDR_VALUE in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUS_BUSID_WRITE_BYTE_ADDR_VALUE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<models::BusId>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => parse_error!(
                            context,
                            "busId",
                            I2C_BUS_WRITE_BYTE_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "busId",
                        I2C_BUS_WRITE_BYTE_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<models::Addr>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => parse_error!(
                            context,
                            "addr",
                            I2C_BUS_WRITE_BYTE_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "addr",
                        I2C_BUS_WRITE_BYTE_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_value = match percent_encoding::percent_decode(path_params["value"].as_bytes()).decode_utf8() {
                    Ok(param_value) => match param_value.parse::<models::Value>() {
                        Ok(param_value) => param_value,
                        Err(e) => parse_error!(
                            context,
                            "value",
                            I2C_BUS_WRITE_BYTE_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "value",
                        I2C_BUS_WRITE_BYTE_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };





                Box::new({
                        {{

                                Box::new(api_impl.i2c_bus_write_byte(param_bus_id, param_addr, param_value, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusWriteByteResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTE_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusWriteByteResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTE_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusWriteByteResponse::TransactionFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTE_TRANSACTION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // I2cBusWriteByteReg - POST /i2c/bus/{busId}/write/byte/reg/{addr}/{reg}/{value}
            &hyper::Method::Post if path.matched(paths::ID_I2C_BUS_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUS_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUS_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUS_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<models::BusId>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => parse_error!(
                            context,
                            "busId",
                            I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "busId",
                        I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<models::Addr>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => parse_error!(
                            context,
                            "addr",
                            I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "addr",
                        I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_reg = match percent_encoding::percent_decode(path_params["reg"].as_bytes()).decode_utf8() {
                    Ok(param_reg) => match param_reg.parse::<models::Reg>() {
                        Ok(param_reg) => param_reg,
                        Err(e) => parse_error!(
                            context,
                            "reg",
                            I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "reg",
                        I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_value = match percent_encoding::percent_decode(path_params["value"].as_bytes()).decode_utf8() {
                    Ok(param_value) => match param_value.parse::<models::Value>() {
                        Ok(param_value) => param_value,
                        Err(e) => parse_error!(
                            context,
                            "value",
                            I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "value",
                        I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };





                Box::new({
                        {{

                                Box::new(api_impl.i2c_bus_write_byte_reg(param_bus_id, param_addr, param_reg, param_value, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusWriteByteRegResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTE_REG_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusWriteByteRegResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusWriteByteRegResponse::TransactionFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTE_REG_TRANSACTION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // I2cBusWriteBytes - POST /i2c/bus/{busId}/write/bytes/{addr}
            &hyper::Method::Post if path.matched(paths::ID_I2C_BUS_BUSID_WRITE_BYTES_ADDR) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUS_BUSID_WRITE_BYTES_ADDR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUS_BUSID_WRITE_BYTES_ADDR in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUS_BUSID_WRITE_BYTES_ADDR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<models::BusId>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => parse_error!(
                            context,
                            "busId",
                            I2C_BUS_WRITE_BYTES_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "busId",
                        I2C_BUS_WRITE_BYTES_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<models::Addr>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => parse_error!(
                            context,
                            "addr",
                            I2C_BUS_WRITE_BYTES_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "addr",
                        I2C_BUS_WRITE_BYTES_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };




                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_values: Option<models::Values> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_values) => param_values,
                                        Err(e) => parse_error!(
                                            context,
                                            "Values",
                                            I2C_BUS_WRITE_BYTES_BAD_REQUEST,
                                            format!("Couldn't parse body parameter, {}", e)
                                        ),
                                    }

                                } else {
                                    None
                                };
                                let param_values = match param_values {
                                    Some(param_values) => param_values,
                                    None => parse_error!(
                                        context,
                                        "",
                                        I2C_BUS_WRITE_BYTES_BAD_REQUEST,
                                        format!("Missing body parameter")
                                    ),
                                };


                                Box::new(api_impl.i2c_bus_write_bytes(param_bus_id, param_addr, param_values, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusWriteBytesResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTES_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusWriteBytesResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTES_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusWriteBytesResponse::TransactionFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTES_TRANSACTION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))


                            },
                            Err(e) => parse_error!(
                                context,
                                "Values",
                                I2C_BUS_WRITE_BYTES_BAD_REQUEST,
                                format!("Couldn't parse body parameter, {}", e)
                            ),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // I2cBusWriteBytesReg - POST /i2c/bus/{busId}/write/bytes/reg/{addr}/{reg}
            &hyper::Method::Post if path.matched(paths::ID_I2C_BUS_BUSID_WRITE_BYTES_REG_ADDR_REG) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUS_BUSID_WRITE_BYTES_REG_ADDR_REG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUS_BUSID_WRITE_BYTES_REG_ADDR_REG in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUS_BUSID_WRITE_BYTES_REG_ADDR_REG.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<models::BusId>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => parse_error!(
                            context,
                            "busId",
                            I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "busId",
                        I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<models::Addr>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => parse_error!(
                            context,
                            "addr",
                            I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "addr",
                        I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };
                let param_reg = match percent_encoding::percent_decode(path_params["reg"].as_bytes()).decode_utf8() {
                    Ok(param_reg) => match param_reg.parse::<models::Reg>() {
                        Ok(param_reg) => param_reg,
                        Err(e) => parse_error!(
                            context,
                            "reg",
                            I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST,
                            format!("Couldn't parse path parameter, {}", e)
                        ),
                    },
                    Err(e) => parse_error!(
                        context,
                        "reg",
                        I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST,
                        format!("Couldn't percent-decode path parameter as UTF-8: {}", e)
                    ),
                };




                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_values: Option<models::Values> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_values) => param_values,
                                        Err(e) => parse_error!(
                                            context,
                                            "Values",
                                            I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST,
                                            format!("Couldn't parse body parameter, {}", e)
                                        ),
                                    }

                                } else {
                                    None
                                };
                                let param_values = match param_values {
                                    Some(param_values) => param_values,
                                    None => parse_error!(
                                        context,
                                        "",
                                        I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST,
                                        format!("Missing body parameter")
                                    ),
                                };


                                Box::new(api_impl.i2c_bus_write_bytes_reg(param_bus_id, param_addr, param_reg, param_values, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusWriteBytesRegResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTES_REG_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusWriteBytesRegResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                I2cBusWriteBytesRegResponse::TransactionFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::I2C_BUS_WRITE_BYTES_REG_TRANSACTION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))


                            },
                            Err(e) => parse_error!(
                                context,
                                "Values",
                                I2C_BUS_WRITE_BYTES_BAD_REQUEST,
                                format!("Couldn't parse body parameter, {}", e)
                            ),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            _ => Box::new(future::ok(Response::new().with_status(StatusCode::NotFound))) as Box<Future<Item=Response, Error=Error>>,
        }
    }
}

impl<T, C> Clone for Service<T, C>
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl RequestParser for ApiRequestParser {
    fn parse_operation_id(request: &Request) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {

            // I2cBusApi - GET /i2c/bus/api
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_API) => Ok("I2cBusApi"),

            // I2cBusList - GET /i2c/bus/list
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_LIST) => Ok("I2cBusList"),

            // I2cBusReadByte - GET /i2c/bus/{busId}/read/byte/{addr}
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_BUSID_READ_BYTE_ADDR) => Ok("I2cBusReadByte"),

            // I2cBusReadBytes - GET /i2c/bus/{busId}/read/bytes/{addr}/{numBytes}
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_BUSID_READ_BYTES_ADDR_NUMBYTES) => Ok("I2cBusReadBytes"),

            // I2cBusReadReg - GET /i2c/bus/{busId}/read/reg/{addr}/{reg}/{numBytes}
            &hyper::Method::Get if path.matched(paths::ID_I2C_BUS_BUSID_READ_REG_ADDR_REG_NUMBYTES) => Ok("I2cBusReadReg"),

            // I2cBusWriteByte - POST /i2c/bus/{busId}/write/byte/{addr}/{value}
            &hyper::Method::Post if path.matched(paths::ID_I2C_BUS_BUSID_WRITE_BYTE_ADDR_VALUE) => Ok("I2cBusWriteByte"),

            // I2cBusWriteByteReg - POST /i2c/bus/{busId}/write/byte/reg/{addr}/{reg}/{value}
            &hyper::Method::Post if path.matched(paths::ID_I2C_BUS_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE) => Ok("I2cBusWriteByteReg"),

            // I2cBusWriteBytes - POST /i2c/bus/{busId}/write/bytes/{addr}
            &hyper::Method::Post if path.matched(paths::ID_I2C_BUS_BUSID_WRITE_BYTES_ADDR) => Ok("I2cBusWriteBytes"),

            // I2cBusWriteBytesReg - POST /i2c/bus/{busId}/write/bytes/reg/{addr}/{reg}
            &hyper::Method::Post if path.matched(paths::ID_I2C_BUS_BUSID_WRITE_BYTES_REG_ADDR_REG) => Ok("I2cBusWriteBytesReg"),
            _ => Err(()),
        }
    }
}
