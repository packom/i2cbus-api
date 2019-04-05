#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

#[macro_use]
extern crate swagger;

#[macro_use]
extern crate url;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.1.2";


#[derive(Debug, PartialEq)]
pub enum I2cBusApiResponse {
    /// OK
    OK ( models::Yaml ) ,
    /// File not found
    FileNotFound ( models::Error ) ,
}

#[derive(Debug, PartialEq)]
pub enum I2cBusListResponse {
    /// OK
    OK ( Vec<models::I2cBusList> ) ,
}

#[derive(Debug, PartialEq)]
pub enum I2cBusReadByteResponse {
    /// OK
    OK ( models::I2cBusRead ) ,
    /// Bad Request
    BadRequest ( models::I2cBusArg ) ,
    /// Transaction Failed
    TransactionFailed ( models::I2cBusError ) ,
}

#[derive(Debug, PartialEq)]
pub enum I2cBusReadBytesResponse {
    /// OK
    OK ( models::I2cBusRead ) ,
    /// Bad Request
    BadRequest ( models::I2cBusArg ) ,
    /// Transaction Failed
    TransactionFailed ( models::I2cBusError ) ,
}

#[derive(Debug, PartialEq)]
pub enum I2cBusReadRegResponse {
    /// OK
    OK ( models::I2cBusRead ) ,
    /// Bad Request
    BadRequest ( models::I2cBusArg ) ,
    /// Transaction Failed
    TransactionFailed ( models::I2cBusError ) ,
}

#[derive(Debug, PartialEq)]
pub enum I2cBusWriteByteResponse {
    /// OK
    OK ( models::I2cBusOk ) ,
    /// Bad Request
    BadRequest ( models::I2cBusArg ) ,
    /// Transaction Failed
    TransactionFailed ( models::I2cBusError ) ,
}

#[derive(Debug, PartialEq)]
pub enum I2cBusWriteByteRegResponse {
    /// OK
    OK ( models::I2cBusOk ) ,
    /// Bad Request
    BadRequest ( models::I2cBusArg ) ,
    /// Transaction Failed
    TransactionFailed ( models::I2cBusError ) ,
}

#[derive(Debug, PartialEq)]
pub enum I2cBusWriteBytesResponse {
    /// OK
    OK ( models::I2cBusOk ) ,
    /// Bad Request
    BadRequest ( models::I2cBusArg ) ,
    /// Transaction Failed
    TransactionFailed ( models::I2cBusError ) ,
}

#[derive(Debug, PartialEq)]
pub enum I2cBusWriteBytesRegResponse {
    /// OK
    OK ( models::I2cBusOk ) ,
    /// Bad Request
    BadRequest ( models::I2cBusArg ) ,
    /// Transaction Failed
    TransactionFailed ( models::I2cBusError ) ,
}

/// API
pub trait Api<C> {


    fn i2c_bus_api(&self, context: &C) -> Box<Future<Item=I2cBusApiResponse, Error=ApiError>>;


    fn i2c_bus_list(&self, context: &C) -> Box<Future<Item=I2cBusListResponse, Error=ApiError>>;


    fn i2c_bus_read_byte(&self, bus_id: models::BusId, addr: models::Addr, context: &C) -> Box<Future<Item=I2cBusReadByteResponse, Error=ApiError>>;


    fn i2c_bus_read_bytes(&self, bus_id: models::BusId, addr: models::Addr, num_bytes: models::NumBytes, context: &C) -> Box<Future<Item=I2cBusReadBytesResponse, Error=ApiError>>;


    fn i2c_bus_read_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, num_bytes: models::NumBytes, context: &C) -> Box<Future<Item=I2cBusReadRegResponse, Error=ApiError>>;


    fn i2c_bus_write_byte(&self, bus_id: models::BusId, addr: models::Addr, value: models::Value, context: &C) -> Box<Future<Item=I2cBusWriteByteResponse, Error=ApiError>>;


    fn i2c_bus_write_byte_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, value: models::Value, context: &C) -> Box<Future<Item=I2cBusWriteByteRegResponse, Error=ApiError>>;


    fn i2c_bus_write_bytes(&self, bus_id: models::BusId, addr: models::Addr, values: models::Values, context: &C) -> Box<Future<Item=I2cBusWriteBytesResponse, Error=ApiError>>;


    fn i2c_bus_write_bytes_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, values: models::Values, context: &C) -> Box<Future<Item=I2cBusWriteBytesRegResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {


    fn i2c_bus_api(&self) -> Box<Future<Item=I2cBusApiResponse, Error=ApiError>>;


    fn i2c_bus_list(&self) -> Box<Future<Item=I2cBusListResponse, Error=ApiError>>;


    fn i2c_bus_read_byte(&self, bus_id: models::BusId, addr: models::Addr) -> Box<Future<Item=I2cBusReadByteResponse, Error=ApiError>>;


    fn i2c_bus_read_bytes(&self, bus_id: models::BusId, addr: models::Addr, num_bytes: models::NumBytes) -> Box<Future<Item=I2cBusReadBytesResponse, Error=ApiError>>;


    fn i2c_bus_read_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, num_bytes: models::NumBytes) -> Box<Future<Item=I2cBusReadRegResponse, Error=ApiError>>;


    fn i2c_bus_write_byte(&self, bus_id: models::BusId, addr: models::Addr, value: models::Value) -> Box<Future<Item=I2cBusWriteByteResponse, Error=ApiError>>;


    fn i2c_bus_write_byte_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, value: models::Value) -> Box<Future<Item=I2cBusWriteByteRegResponse, Error=ApiError>>;


    fn i2c_bus_write_bytes(&self, bus_id: models::BusId, addr: models::Addr, values: models::Values) -> Box<Future<Item=I2cBusWriteBytesResponse, Error=ApiError>>;


    fn i2c_bus_write_bytes_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, values: models::Values) -> Box<Future<Item=I2cBusWriteBytesRegResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {


    fn i2c_bus_api(&self) -> Box<Future<Item=I2cBusApiResponse, Error=ApiError>> {
        self.api().i2c_bus_api(&self.context())
    }


    fn i2c_bus_list(&self) -> Box<Future<Item=I2cBusListResponse, Error=ApiError>> {
        self.api().i2c_bus_list(&self.context())
    }


    fn i2c_bus_read_byte(&self, bus_id: models::BusId, addr: models::Addr) -> Box<Future<Item=I2cBusReadByteResponse, Error=ApiError>> {
        self.api().i2c_bus_read_byte(bus_id, addr, &self.context())
    }


    fn i2c_bus_read_bytes(&self, bus_id: models::BusId, addr: models::Addr, num_bytes: models::NumBytes) -> Box<Future<Item=I2cBusReadBytesResponse, Error=ApiError>> {
        self.api().i2c_bus_read_bytes(bus_id, addr, num_bytes, &self.context())
    }


    fn i2c_bus_read_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, num_bytes: models::NumBytes) -> Box<Future<Item=I2cBusReadRegResponse, Error=ApiError>> {
        self.api().i2c_bus_read_reg(bus_id, addr, reg, num_bytes, &self.context())
    }


    fn i2c_bus_write_byte(&self, bus_id: models::BusId, addr: models::Addr, value: models::Value) -> Box<Future<Item=I2cBusWriteByteResponse, Error=ApiError>> {
        self.api().i2c_bus_write_byte(bus_id, addr, value, &self.context())
    }


    fn i2c_bus_write_byte_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, value: models::Value) -> Box<Future<Item=I2cBusWriteByteRegResponse, Error=ApiError>> {
        self.api().i2c_bus_write_byte_reg(bus_id, addr, reg, value, &self.context())
    }


    fn i2c_bus_write_bytes(&self, bus_id: models::BusId, addr: models::Addr, values: models::Values) -> Box<Future<Item=I2cBusWriteBytesResponse, Error=ApiError>> {
        self.api().i2c_bus_write_bytes(bus_id, addr, values, &self.context())
    }


    fn i2c_bus_write_bytes_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, values: models::Values) -> Box<Future<Item=I2cBusWriteBytesRegResponse, Error=ApiError>> {
        self.api().i2c_bus_write_bytes_reg(bus_id, addr, reg, values, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;

#[derive(Debug)]
pub enum ExtraInfoOk {
    Yaml ( models::Yaml ),
    List ( Vec<models::I2cBusList> ),
    Read ( models::I2cBusRead ),
    OK ( models::I2cBusOk ),
}

#[derive(Debug)]
pub enum ExtraInfoError {
    FileNotFound ( models::Error ),
    Arg ( models::I2cBusArg ),
    Error ( models::I2cBusError ),
}

pub trait OkOrOther {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>;
}

impl OkOrOther for I2cBusApiResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError> {
        match *self {
            I2cBusApiResponse::OK(ref x) => Ok(ExtraInfoOk::Yaml(x.clone())),
            I2cBusApiResponse::FileNotFound(ref x) => Err(ExtraInfoError::FileNotFound(x.clone())),
        }
    }
}

impl OkOrOther for I2cBusListResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>{
        match *self {
            I2cBusListResponse::OK(ref x) => Ok(ExtraInfoOk::List(x.clone())),
        }
    }
}

impl OkOrOther for I2cBusReadByteResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>{
        match *self {
            I2cBusReadByteResponse::OK(ref x) => Ok(ExtraInfoOk::Read(x.clone())),
            I2cBusReadByteResponse::BadRequest(ref x) => Err(ExtraInfoError::Arg(x.clone())),
            I2cBusReadByteResponse::TransactionFailed(ref x) => Err(ExtraInfoError::Error(x.clone())),
        }
    }
}

impl OkOrOther for I2cBusReadBytesResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>{
        match *self {
            I2cBusReadBytesResponse::OK(ref x) => Ok(ExtraInfoOk::Read(x.clone())),
            I2cBusReadBytesResponse::BadRequest(ref x) => Err(ExtraInfoError::Arg(x.clone())),
            I2cBusReadBytesResponse::TransactionFailed(ref x) => Err(ExtraInfoError::Error(x.clone())),
        }
    }
}

impl OkOrOther for I2cBusReadRegResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>{
        match *self {
            I2cBusReadRegResponse::OK(ref x) => Ok(ExtraInfoOk::Read(x.clone())),
            I2cBusReadRegResponse::BadRequest(ref x) => Err(ExtraInfoError::Arg(x.clone())),
            I2cBusReadRegResponse::TransactionFailed(ref x) => Err(ExtraInfoError::Error(x.clone())),
        }
    }
}

impl OkOrOther for I2cBusWriteByteResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>{
        match *self {
            I2cBusWriteByteResponse::OK(ref x) => Ok(ExtraInfoOk::OK(x.clone())),
            I2cBusWriteByteResponse::BadRequest(ref x) => Err(ExtraInfoError::Arg(x.clone())),
            I2cBusWriteByteResponse::TransactionFailed(ref x) => Err(ExtraInfoError::Error(x.clone())),
        }
    }
}

impl OkOrOther for I2cBusWriteByteRegResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>{
        match *self {
            I2cBusWriteByteRegResponse::OK(ref x) => Ok(ExtraInfoOk::OK(x.clone())),
            I2cBusWriteByteRegResponse::BadRequest(ref x) => Err(ExtraInfoError::Arg(x.clone())),
            I2cBusWriteByteRegResponse::TransactionFailed(ref x) => Err(ExtraInfoError::Error(x.clone())),
        }
    }
}

impl OkOrOther for I2cBusWriteBytesResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>{
        match *self {
            I2cBusWriteBytesResponse::OK(ref x) => Ok(ExtraInfoOk::OK(x.clone())),
            I2cBusWriteBytesResponse::BadRequest(ref x) => Err(ExtraInfoError::Arg(x.clone())),
            I2cBusWriteBytesResponse::TransactionFailed(ref x) => Err(ExtraInfoError::Error(x.clone())),
        }
    }
}

impl OkOrOther for I2cBusWriteBytesRegResponse {
    fn ok_or_other(&self) -> Result<ExtraInfoOk, ExtraInfoError>{
        match *self {
            I2cBusWriteBytesRegResponse::OK(ref x) => Ok(ExtraInfoOk::OK(x.clone())),
            I2cBusWriteBytesRegResponse::BadRequest(ref x) => Err(ExtraInfoError::Arg(x.clone())),
            I2cBusWriteBytesRegResponse::TransactionFailed(ref x) => Err(ExtraInfoError::Error(x.clone())),
        }
    }
}

