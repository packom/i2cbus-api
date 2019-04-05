//! Server implementation of i2cbus_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use i2cbus_api::{Api, ApiError,
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
use i2cbus_api::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{


    fn i2c_bus_api(&self, context: &C) -> Box<Future<Item=I2cBusApiResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_api() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn i2c_bus_list(&self, context: &C) -> Box<Future<Item=I2cBusListResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_list() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn i2c_bus_read_byte(&self, bus_id: models::BusId, addr: models::Addr, context: &C) -> Box<Future<Item=I2cBusReadByteResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_read_byte({:?}, {:?}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn i2c_bus_read_bytes(&self, bus_id: models::BusId, addr: models::Addr, num_bytes: models::NumBytes, context: &C) -> Box<Future<Item=I2cBusReadBytesResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_read_bytes({:?}, {:?}, {:?}) - X-Span-ID: {:?}", bus_id, addr, num_bytes, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn i2c_bus_read_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, num_bytes: models::NumBytes, context: &C) -> Box<Future<Item=I2cBusReadRegResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_read_reg({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", bus_id, addr, reg, num_bytes, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn i2c_bus_write_byte(&self, bus_id: models::BusId, addr: models::Addr, value: models::Value, context: &C) -> Box<Future<Item=I2cBusWriteByteResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_write_byte({:?}, {:?}, {:?}) - X-Span-ID: {:?}", bus_id, addr, value, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn i2c_bus_write_byte_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, value: models::Value, context: &C) -> Box<Future<Item=I2cBusWriteByteRegResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_write_byte_reg({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", bus_id, addr, reg, value, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn i2c_bus_write_bytes(&self, bus_id: models::BusId, addr: models::Addr, values: models::Values, context: &C) -> Box<Future<Item=I2cBusWriteBytesResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_write_bytes({:?}, {:?}, {:?}) - X-Span-ID: {:?}", bus_id, addr, values, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn i2c_bus_write_bytes_reg(&self, bus_id: models::BusId, addr: models::Addr, reg: models::Reg, values: models::Values, context: &C) -> Box<Future<Item=I2cBusWriteBytesRegResponse, Error=ApiError>> {
        let context = context.clone();
        println!("i2c_bus_write_bytes_reg({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", bus_id, addr, reg, values, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
