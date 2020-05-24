#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;


/// A valid I2C slave address
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Addr(i32);

impl std::convert::From<i32> for Addr {
    fn from(x: i32) -> Self {
        Addr(x)
    }
}


impl std::convert::From<Addr> for i32 {
    fn from(x: Addr) -> Self {
        x.0
    }
}

impl std::ops::Deref for Addr {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Addr {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// A valid bus ID as returned by i2c_bus_list
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BusId(i32);

impl std::convert::From<i32> for BusId {
    fn from(x: i32) -> Self {
        BusId(x)
    }
}


impl std::convert::From<BusId> for i32 {
    fn from(x: BusId) -> Self {
        x.0
    }
}

impl std::ops::Deref for BusId {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for BusId {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// Some error text
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Error(String);

impl std::convert::From<String> for Error {
    fn from(x: String) -> Self {
        Error(x)
    }
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Error {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Error(x.to_string()))
    }
}

impl std::convert::From<Error> for String {
    fn from(x: Error) -> Self {
        x.0
    }
}

impl std::ops::Deref for Error {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Error {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// Indicates a malformed request, likely a badly formatted or invalid argument)
// Methods for converting between header::IntoHeaderValue<I2cBusArg> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<I2cBusArg>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<I2cBusArg>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for I2cBusArg - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<I2cBusArg> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <I2cBusArg as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into I2cBusArg - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the I2cBusArg value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for I2cBusArg {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref arg) = self.arg {
            params.push("arg".to_string());
            params.push(arg.to_string());
        }


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a I2cBusArg value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for I2cBusArg {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub arg: Vec<String>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing I2cBusArg".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "arg" => intermediate_rep.arg.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing I2cBusArg".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(I2cBusArg {
            arg: intermediate_rep.arg.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
        })
    }
}



/// An error response from the I2C transaction
// Methods for converting between header::IntoHeaderValue<I2cBusError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<I2cBusError>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<I2cBusError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for I2cBusError - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<I2cBusError> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <I2cBusError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into I2cBusError - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the I2cBusError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for I2cBusError {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref error) = self.error {
            params.push("error".to_string());
            params.push(error.to_string());
        }


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a I2cBusError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for I2cBusError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub error: Vec<i32>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing I2cBusError".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "error" => intermediate_rep.error.push(i32::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing I2cBusError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(I2cBusError {
            error: intermediate_rep.error.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
        })
    }
}



/// A list of available I2C buses
// Methods for converting between header::IntoHeaderValue<I2cBusList> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<I2cBusList>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<I2cBusList>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for I2cBusList - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<I2cBusList> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <I2cBusList as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into I2cBusList - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the I2cBusList value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for I2cBusList {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref path) = self.path {
            params.push("path".to_string());
            params.push(path.to_string());
        }


        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a I2cBusList value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for I2cBusList {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub path: Vec<String>,
            pub id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing I2cBusList".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "path" => intermediate_rep.path.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "id" => intermediate_rep.id.push(i32::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing I2cBusList".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(I2cBusList {
            path: intermediate_rep.path.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}



/// A successful response from the I2C transaction
// Methods for converting between header::IntoHeaderValue<I2cBusOk> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<I2cBusOk>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<I2cBusOk>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for I2cBusOk - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<I2cBusOk> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <I2cBusOk as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into I2cBusOk - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the I2cBusOk value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for I2cBusOk {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref ok) = self.ok {
            params.push("ok".to_string());
            params.push(ok.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a I2cBusOk value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for I2cBusOk {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub ok: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing I2cBusOk".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "ok" => intermediate_rep.ok.push(i32::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing I2cBusOk".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(I2cBusOk {
            ok: intermediate_rep.ok.into_iter().next(),
        })
    }
}



/// A successful read response
// Methods for converting between header::IntoHeaderValue<I2cBusRead> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<I2cBusRead>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<I2cBusRead>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for I2cBusRead - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<I2cBusRead> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <I2cBusRead as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into I2cBusRead - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the I2cBusRead value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for I2cBusRead {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref ok) = self.ok {
            params.push("ok".to_string());
            params.push(ok.to_string());
        }


        if let Some(ref values) = self.values {
            params.push("values".to_string());
            params.push(values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a I2cBusRead value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for I2cBusRead {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub ok: Vec<i32>,
            pub values: Vec<Vec<models::I2cByte>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing I2cBusRead".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "ok" => intermediate_rep.ok.push(i32::from_str(val).map_err(|x| format!("{}", x))?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in I2cBusRead".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing I2cBusRead".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(I2cBusRead {
            ok: intermediate_rep.ok.into_iter().next(),
            values: intermediate_rep.values.into_iter().next(),
        })
    }
}



/// A byte read from the I2C bus
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct I2cByte(i32);

impl std::convert::From<i32> for I2cByte {
    fn from(x: i32) -> Self {
        I2cByte(x)
    }
}


impl std::convert::From<I2cByte> for i32 {
    fn from(x: I2cByte) -> Self {
        x.0
    }
}

impl std::ops::Deref for I2cByte {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for I2cByte {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// Number of bytes to write or read
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NumBytes(i32);

impl std::convert::From<i32> for NumBytes {
    fn from(x: i32) -> Self {
        NumBytes(x)
    }
}


impl std::convert::From<NumBytes> for i32 {
    fn from(x: NumBytes) -> Self {
        x.0
    }
}

impl std::ops::Deref for NumBytes {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for NumBytes {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// An I2C slave device register
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Reg(i32);

impl std::convert::From<i32> for Reg {
    fn from(x: i32) -> Self {
        Reg(x)
    }
}


impl std::convert::From<Reg> for i32 {
    fn from(x: Reg) -> Self {
        x.0
    }
}

impl std::ops::Deref for Reg {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Reg {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// A value to read from or write to the I2C bus
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Value(i32);

impl std::convert::From<i32> for Value {
    fn from(x: i32) -> Self {
        Value(x)
    }
}


impl std::convert::From<Value> for i32 {
    fn from(x: Value) -> Self {
        x.0
    }
}

impl std::ops::Deref for Value {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// Bytes to write to the bus
// Methods for converting between header::IntoHeaderValue<Values> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Values>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Values>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Values - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Values> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Values as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Values - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the Values value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Values {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref values) = self.values {
            params.push("values".to_string());
            params.push(values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Values value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Values {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub values: Vec<Vec<models::I2cByte>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Values".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in Values".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Values".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Values {
            values: intermediate_rep.values.into_iter().next(),
        })
    }
}



/// A YAML file
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Yaml(String);

impl std::convert::From<String> for Yaml {
    fn from(x: String) -> Self {
        Yaml(x)
    }
}

impl std::string::ToString for Yaml {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Yaml {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Yaml(x.to_string()))
    }
}

impl std::convert::From<Yaml> for String {
    fn from(x: Yaml) -> Self {
        x.0
    }
}

impl std::ops::Deref for Yaml {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Yaml {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


