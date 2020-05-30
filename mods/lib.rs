
#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;

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
            I2cBusApiResponse::OK(ref x) => Ok(ExtraInfoOk::Yaml(x.clone().into())),
            I2cBusApiResponse::FileNotFound(ref x) => Err(ExtraInfoError::FileNotFound(x.clone().into())),
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
