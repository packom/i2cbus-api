use std::marker::PhantomData;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
use serde_json;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::io;
use url::form_urlencoded;
#[allow(unused_imports)]
use swagger;
use swagger::{ApiError, XSpanIdString, Has, RequestParser};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use swagger::context::ContextualPayload;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

use crate::{Api,
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

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/i2c/api$",
            r"^/i2c/buslist$",
            r"^/i2c/(?P<busId>[^/?#]*)/read/byte/(?P<addr>[^/?#]*)$",
            r"^/i2c/(?P<busId>[^/?#]*)/read/bytes/(?P<addr>[^/?#]*)/(?P<numBytes>[^/?#]*)$",
            r"^/i2c/(?P<busId>[^/?#]*)/read/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)/(?P<numBytes>[^/?#]*)$",
            r"^/i2c/(?P<busId>[^/?#]*)/write/byte/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)/(?P<value>[^/?#]*)$",
            r"^/i2c/(?P<busId>[^/?#]*)/write/byte/(?P<addr>[^/?#]*)/(?P<value>[^/?#]*)$",
            r"^/i2c/(?P<busId>[^/?#]*)/write/bytes/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)$",
            r"^/i2c/(?P<busId>[^/?#]*)/write/bytes/(?P<addr>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_I2C_API: usize = 0;
    pub(crate) static ID_I2C_BUSLIST: usize = 1;
    pub(crate) static ID_I2C_BUSID_READ_BYTE_ADDR: usize = 2;
    lazy_static! {
        pub static ref REGEX_I2C_BUSID_READ_BYTE_ADDR: regex::Regex =
            regex::Regex::new(r"^/i2c/(?P<busId>[^/?#]*)/read/byte/(?P<addr>[^/?#]*)$")
                .expect("Unable to create regex for I2C_BUSID_READ_BYTE_ADDR");
    }
    pub(crate) static ID_I2C_BUSID_READ_BYTES_ADDR_NUMBYTES: usize = 3;
    lazy_static! {
        pub static ref REGEX_I2C_BUSID_READ_BYTES_ADDR_NUMBYTES: regex::Regex =
            regex::Regex::new(r"^/i2c/(?P<busId>[^/?#]*)/read/bytes/(?P<addr>[^/?#]*)/(?P<numBytes>[^/?#]*)$")
                .expect("Unable to create regex for I2C_BUSID_READ_BYTES_ADDR_NUMBYTES");
    }
    pub(crate) static ID_I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES: usize = 4;
    lazy_static! {
        pub static ref REGEX_I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES: regex::Regex =
            regex::Regex::new(r"^/i2c/(?P<busId>[^/?#]*)/read/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)/(?P<numBytes>[^/?#]*)$")
                .expect("Unable to create regex for I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES");
    }
    pub(crate) static ID_I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE: usize = 5;
    lazy_static! {
        pub static ref REGEX_I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE: regex::Regex =
            regex::Regex::new(r"^/i2c/(?P<busId>[^/?#]*)/write/byte/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)/(?P<value>[^/?#]*)$")
                .expect("Unable to create regex for I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE");
    }
    pub(crate) static ID_I2C_BUSID_WRITE_BYTE_ADDR_VALUE: usize = 6;
    lazy_static! {
        pub static ref REGEX_I2C_BUSID_WRITE_BYTE_ADDR_VALUE: regex::Regex =
            regex::Regex::new(r"^/i2c/(?P<busId>[^/?#]*)/write/byte/(?P<addr>[^/?#]*)/(?P<value>[^/?#]*)$")
                .expect("Unable to create regex for I2C_BUSID_WRITE_BYTE_ADDR_VALUE");
    }
    pub(crate) static ID_I2C_BUSID_WRITE_BYTES_REG_ADDR_REG: usize = 7;
    lazy_static! {
        pub static ref REGEX_I2C_BUSID_WRITE_BYTES_REG_ADDR_REG: regex::Regex =
            regex::Regex::new(r"^/i2c/(?P<busId>[^/?#]*)/write/bytes/reg/(?P<addr>[^/?#]*)/(?P<reg>[^/?#]*)$")
                .expect("Unable to create regex for I2C_BUSID_WRITE_BYTES_REG_ADDR_REG");
    }
    pub(crate) static ID_I2C_BUSID_WRITE_BYTES_ADDR: usize = 8;
    lazy_static! {
        pub static ref REGEX_I2C_BUSID_WRITE_BYTES_ADDR: regex::Regex =
            regex::Regex::new(r"^/i2c/(?P<busId>[^/?#]*)/write/bytes/(?P<addr>[^/?#]*)$")
                .expect("Unable to create regex for I2C_BUSID_WRITE_BYTES_ADDR");
    }
}

pub struct MakeService<T, RC> {
    api_impl: T,
    marker: PhantomData<RC>,
}

impl<T, RC> MakeService<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<'a, T, SC, RC> hyper::service::MakeService<&'a SC> for MakeService<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static + Send
{
    type ReqBody = ContextualPayload<Body, RC>;
    type ResBody = Body;
    type Error = Error;
    type Service = Service<T, RC>;
    type Future = future::FutureResult<Self::Service, Self::MakeError>;
    type MakeError = Error;

    fn make_service(&mut self, _ctx: &'a SC) -> Self::Future {
        future::FutureResult::from(Ok(Service::new(
            self.api_impl.clone(),
        )))
    }
}

type ServiceFuture = Box<dyn Future<Item = Response<Body>, Error = Error> + Send>;

fn method_not_allowed() -> ServiceFuture {
    Box::new(future::ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    ))
}

pub struct Service<T, RC> {
    api_impl: T,
    marker: PhantomData<RC>,
}

impl<T, RC> Service<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static {
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> hyper::service::Service for Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + 'static + Send
{
    type ReqBody = ContextualPayload<Body, C>;
    type ResBody = Body;
    type Error = Error;
    type Future = ServiceFuture;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (parts, body) = req.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());
        let mut context = body.context;
        let body = body.inner;

        match &method {

            // I2cBusApi - GET /i2c/api
            &hyper::Method::GET if path.matched(paths::ID_I2C_API) => {
                Box::new({
                        {{
                                Box::new(
                                    api_impl.i2c_bus_api(
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusApiResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/x-yaml")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_API_OK"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusApiResponse::FileNotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_API_FILE_NOT_FOUND"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // I2cBusList - GET /i2c/buslist
            &hyper::Method::GET if path.matched(paths::ID_I2C_BUSLIST) => {
                Box::new({
                        {{
                                Box::new(
                                    api_impl.i2c_bus_list(
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusListResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_LIST_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // I2cBusReadByte - GET /i2c/{busId}/read/byte/{addr}
            &hyper::Method::GET if path.matched(paths::ID_I2C_BUSID_READ_BYTE_ADDR) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUSID_READ_BYTE_ADDR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUSID_READ_BYTE_ADDR in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUSID_READ_BYTE_ADDR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.i2c_bus_read_byte(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusReadByteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_BYTE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusReadByteResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_BYTE_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusReadByteResponse::TransactionFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_BYTE_TRANSACTION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // I2cBusReadBytes - GET /i2c/{busId}/read/bytes/{addr}/{numBytes}
            &hyper::Method::GET if path.matched(paths::ID_I2C_BUSID_READ_BYTES_ADDR_NUMBYTES) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUSID_READ_BYTES_ADDR_NUMBYTES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUSID_READ_BYTES_ADDR_NUMBYTES in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUSID_READ_BYTES_ADDR_NUMBYTES.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_num_bytes = match percent_encoding::percent_decode(path_params["numBytes"].as_bytes()).decode_utf8() {
                    Ok(param_num_bytes) => match param_num_bytes.parse::<i32>() {
                        Ok(param_num_bytes) => param_num_bytes,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter numBytes: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["numBytes"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.i2c_bus_read_bytes(
                                            param_bus_id,
                                            param_addr,
                                            param_num_bytes,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusReadBytesResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_BYTES_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusReadBytesResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_BYTES_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusReadBytesResponse::TransactionFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_BYTES_TRANSACTION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // I2cBusReadReg - GET /i2c/{busId}/read/reg/{addr}/{reg}/{numBytes}
            &hyper::Method::GET if path.matched(paths::ID_I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_reg = match percent_encoding::percent_decode(path_params["reg"].as_bytes()).decode_utf8() {
                    Ok(param_reg) => match param_reg.parse::<i32>() {
                        Ok(param_reg) => param_reg,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter reg: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["reg"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_num_bytes = match percent_encoding::percent_decode(path_params["numBytes"].as_bytes()).decode_utf8() {
                    Ok(param_num_bytes) => match param_num_bytes.parse::<i32>() {
                        Ok(param_num_bytes) => param_num_bytes,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter numBytes: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["numBytes"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.i2c_bus_read_reg(
                                            param_bus_id,
                                            param_addr,
                                            param_reg,
                                            param_num_bytes,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusReadRegResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_REG_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusReadRegResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_REG_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusReadRegResponse::TransactionFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_READ_REG_TRANSACTION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // I2cBusWriteByte - POST /i2c/{busId}/write/byte/{addr}/{value}
            &hyper::Method::POST if path.matched(paths::ID_I2C_BUSID_WRITE_BYTE_ADDR_VALUE) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUSID_WRITE_BYTE_ADDR_VALUE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUSID_WRITE_BYTE_ADDR_VALUE in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUSID_WRITE_BYTE_ADDR_VALUE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_value = match percent_encoding::percent_decode(path_params["value"].as_bytes()).decode_utf8() {
                    Ok(param_value) => match param_value.parse::<i32>() {
                        Ok(param_value) => param_value,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter value: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["value"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.i2c_bus_write_byte(
                                            param_bus_id,
                                            param_addr,
                                            param_value,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusWriteByteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusWriteByteResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTE_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusWriteByteResponse::TransactionFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTE_TRANSACTION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // I2cBusWriteByteReg - POST /i2c/{busId}/write/byte/reg/{addr}/{reg}/{value}
            &hyper::Method::POST if path.matched(paths::ID_I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_reg = match percent_encoding::percent_decode(path_params["reg"].as_bytes()).decode_utf8() {
                    Ok(param_reg) => match param_reg.parse::<i32>() {
                        Ok(param_reg) => param_reg,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter reg: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["reg"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_value = match percent_encoding::percent_decode(path_params["value"].as_bytes()).decode_utf8() {
                    Ok(param_value) => match param_value.parse::<i32>() {
                        Ok(param_value) => param_value,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter value: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["value"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.i2c_bus_write_byte_reg(
                                            param_bus_id,
                                            param_addr,
                                            param_reg,
                                            param_value,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusWriteByteRegResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTE_REG_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusWriteByteRegResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusWriteByteRegResponse::TransactionFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTE_REG_TRANSACTION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // I2cBusWriteBytes - POST /i2c/{busId}/write/bytes/{addr}
            &hyper::Method::POST if path.matched(paths::ID_I2C_BUSID_WRITE_BYTES_ADDR) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUSID_WRITE_BYTES_ADDR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUSID_WRITE_BYTES_ADDR in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUSID_WRITE_BYTES_ADDR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Self::Future {
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
                                        Err(e) => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter Values - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter Values due to schema"))),
                                    }
                                } else {
                                    None
                                };
                                let param_values = match param_values {
                                    Some(param_values) => param_values,
                                    None => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter Values"))
                                                        .expect("Unable to create Bad Request response for missing body parameter Values"))),
                                };

                                Box::new(
                                    api_impl.i2c_bus_write_bytes(
                                            param_bus_id,
                                            param_addr,
                                            param_values,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusWriteBytesResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTES_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusWriteBytesResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTES_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusWriteBytesResponse::TransactionFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTES_TRANSACTION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                            },
                            Err(e) => Box::new(future::ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Values: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Values"))),
                        }
                    })
                ) as Self::Future
            },

            // I2cBusWriteBytesReg - POST /i2c/{busId}/write/bytes/reg/{addr}/{reg}
            &hyper::Method::POST if path.matched(paths::ID_I2C_BUSID_WRITE_BYTES_REG_ADDR_REG) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_I2C_BUSID_WRITE_BYTES_REG_ADDR_REG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE I2C_BUSID_WRITE_BYTES_REG_ADDR_REG in set but failed match against \"{}\"", path, paths::REGEX_I2C_BUSID_WRITE_BYTES_REG_ADDR_REG.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_reg = match percent_encoding::percent_decode(path_params["reg"].as_bytes()).decode_utf8() {
                    Ok(param_reg) => match param_reg.parse::<i32>() {
                        Ok(param_reg) => param_reg,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter reg: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["reg"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Self::Future {
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
                                        Err(e) => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter Values - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter Values due to schema"))),
                                    }
                                } else {
                                    None
                                };
                                let param_values = match param_values {
                                    Some(param_values) => param_values,
                                    None => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter Values"))
                                                        .expect("Unable to create Bad Request response for missing body parameter Values"))),
                                };

                                Box::new(
                                    api_impl.i2c_bus_write_bytes_reg(
                                            param_bus_id,
                                            param_addr,
                                            param_reg,
                                            param_values,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                I2cBusWriteBytesRegResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTES_REG_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusWriteBytesRegResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                I2cBusWriteBytesRegResponse::TransactionFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for I2C_BUS_WRITE_BYTES_REG_TRANSACTION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                            },
                            Err(e) => Box::new(future::ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Values: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Values"))),
                        }
                    })
                ) as Self::Future
            },

            _ if path.matched(paths::ID_I2C_API) => method_not_allowed(),
            _ if path.matched(paths::ID_I2C_BUSLIST) => method_not_allowed(),
            _ if path.matched(paths::ID_I2C_BUSID_READ_BYTE_ADDR) => method_not_allowed(),
            _ if path.matched(paths::ID_I2C_BUSID_READ_BYTES_ADDR_NUMBYTES) => method_not_allowed(),
            _ if path.matched(paths::ID_I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES) => method_not_allowed(),
            _ if path.matched(paths::ID_I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE) => method_not_allowed(),
            _ if path.matched(paths::ID_I2C_BUSID_WRITE_BYTE_ADDR_VALUE) => method_not_allowed(),
            _ if path.matched(paths::ID_I2C_BUSID_WRITE_BYTES_REG_ADDR_REG) => method_not_allowed(),
            _ if path.matched(paths::ID_I2C_BUSID_WRITE_BYTES_ADDR) => method_not_allowed(),
            _ => Box::new(future::ok(
                Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response")
            )) as Self::Future
        }
    }
}

impl<T, C> Clone for Service<T, C> where T: Clone
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
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // I2cBusApi - GET /i2c/api
            &hyper::Method::GET if path.matched(paths::ID_I2C_API) => Ok("I2cBusApi"),
            // I2cBusList - GET /i2c/buslist
            &hyper::Method::GET if path.matched(paths::ID_I2C_BUSLIST) => Ok("I2cBusList"),
            // I2cBusReadByte - GET /i2c/{busId}/read/byte/{addr}
            &hyper::Method::GET if path.matched(paths::ID_I2C_BUSID_READ_BYTE_ADDR) => Ok("I2cBusReadByte"),
            // I2cBusReadBytes - GET /i2c/{busId}/read/bytes/{addr}/{numBytes}
            &hyper::Method::GET if path.matched(paths::ID_I2C_BUSID_READ_BYTES_ADDR_NUMBYTES) => Ok("I2cBusReadBytes"),
            // I2cBusReadReg - GET /i2c/{busId}/read/reg/{addr}/{reg}/{numBytes}
            &hyper::Method::GET if path.matched(paths::ID_I2C_BUSID_READ_REG_ADDR_REG_NUMBYTES) => Ok("I2cBusReadReg"),
            // I2cBusWriteByte - POST /i2c/{busId}/write/byte/{addr}/{value}
            &hyper::Method::POST if path.matched(paths::ID_I2C_BUSID_WRITE_BYTE_ADDR_VALUE) => Ok("I2cBusWriteByte"),
            // I2cBusWriteByteReg - POST /i2c/{busId}/write/byte/reg/{addr}/{reg}/{value}
            &hyper::Method::POST if path.matched(paths::ID_I2C_BUSID_WRITE_BYTE_REG_ADDR_REG_VALUE) => Ok("I2cBusWriteByteReg"),
            // I2cBusWriteBytes - POST /i2c/{busId}/write/bytes/{addr}
            &hyper::Method::POST if path.matched(paths::ID_I2C_BUSID_WRITE_BYTES_ADDR) => Ok("I2cBusWriteBytes"),
            // I2cBusWriteBytesReg - POST /i2c/{busId}/write/bytes/reg/{addr}/{reg}
            &hyper::Method::POST if path.matched(paths::ID_I2C_BUSID_WRITE_BYTES_REG_ADDR_REG) => Ok("I2cBusWriteBytesReg"),
            _ => Err(()),
        }
    }
}
