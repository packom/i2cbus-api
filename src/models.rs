#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


/// A valid I2C slave address
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Addr(i32);

impl ::std::convert::From<i32> for Addr {
    fn from(x: i32) -> Self {
        Addr(x)
    }
}

impl ::std::convert::From<Addr> for i32 {
    fn from(x: Addr) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Addr {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Addr {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// A valid bus ID as returned by i2c_bus_list
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct BusId(i32);

impl ::std::convert::From<i32> for BusId {
    fn from(x: i32) -> Self {
        BusId(x)
    }
}

impl ::std::convert::From<BusId> for i32 {
    fn from(x: BusId) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for BusId {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for BusId {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Some error text
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Error(String);

impl ::std::convert::From<String> for Error {
    fn from(x: String) -> Self {
        Error(x)
    }
}

impl ::std::convert::From<Error> for String {
    fn from(x: Error) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Error {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for Error {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Indicates a malformed request, likely a badly formatted or invalid argument)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct I2cBusArg {
    #[serde(rename = "arg")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arg: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}

impl I2cBusArg {
    pub fn new() -> I2cBusArg {
        I2cBusArg {
            arg: None,
            description: None,
        }
    }
}

/// An error response from the I2C transaction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct I2cBusError {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<i32>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}

impl I2cBusError {
    pub fn new() -> I2cBusError {
        I2cBusError {
            error: None,
            description: None,
        }
    }
}

/// A list of available I2C buses
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct I2cBusList {
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

}

impl I2cBusList {
    pub fn new() -> I2cBusList {
        I2cBusList {
            path: None,
            id: None,
        }
    }
}

/// A successful response from the I2C transaction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct I2cBusOk {
    #[serde(rename = "ok")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ok: Option<i32>,

}

impl I2cBusOk {
    pub fn new() -> I2cBusOk {
        I2cBusOk {
            ok: None,
        }
    }
}

/// A successful read response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct I2cBusRead {
    #[serde(rename = "ok")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ok: Option<i32>,

    #[serde(rename = "values")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<models::I2cByte>>,

}

impl I2cBusRead {
    pub fn new() -> I2cBusRead {
        I2cBusRead {
            ok: None,
            values: None,
        }
    }
}

/// A byte read from the I2C bus
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct I2cByte(i32);

impl ::std::convert::From<i32> for I2cByte {
    fn from(x: i32) -> Self {
        I2cByte(x)
    }
}

impl ::std::convert::From<I2cByte> for i32 {
    fn from(x: I2cByte) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for I2cByte {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for I2cByte {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Number of bytes to write or read
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct NumBytes(i32);

impl ::std::convert::From<i32> for NumBytes {
    fn from(x: i32) -> Self {
        NumBytes(x)
    }
}

impl ::std::convert::From<NumBytes> for i32 {
    fn from(x: NumBytes) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for NumBytes {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for NumBytes {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// An I2C slave device register
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Reg(i32);

impl ::std::convert::From<i32> for Reg {
    fn from(x: i32) -> Self {
        Reg(x)
    }
}

impl ::std::convert::From<Reg> for i32 {
    fn from(x: Reg) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Reg {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Reg {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// A value to read from or write to the I2C bus
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Value(i32);

impl ::std::convert::From<i32> for Value {
    fn from(x: i32) -> Self {
        Value(x)
    }
}

impl ::std::convert::From<Value> for i32 {
    fn from(x: Value) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Value {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Bytes to write to the bus
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Values {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<models::I2cByte>>,

}

impl Values {
    pub fn new() -> Values {
        Values {
            values: None,
        }
    }
}

/// A YAML file
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Yaml(String);

impl ::std::convert::From<String> for Yaml {
    fn from(x: String) -> Self {
        Yaml(x)
    }
}

impl ::std::convert::From<Yaml> for String {
    fn from(x: Yaml) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Yaml {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for Yaml {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

// Manually added

use std::str::FromStr;
use std::num::ParseIntError;

impl FromStr for Addr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(Addr(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for BusId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(BusId(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for Reg {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(Reg(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for Value {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(Value(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for NumBytes {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(NumBytes(val)),
            Err(e) => Err(e)
        }
    }
}

impl<'a> ::std::convert::From<&'a u8> for I2cByte {
    fn from(x: &u8) -> Self {
        I2cByte(<i32>::from(*x))
    }
}
