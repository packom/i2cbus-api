/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for I2cBusApi
    lazy_static! {
        pub static ref I2C_BUS_API_OK: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusApi
    lazy_static! {
        pub static ref I2C_BUS_API_FILE_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusList
    lazy_static! {
        pub static ref I2C_BUS_LIST_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadByte
    lazy_static! {
        pub static ref I2C_BUS_READ_BYTE_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadByte
    lazy_static! {
        pub static ref I2C_BUS_READ_BYTE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadByte
    lazy_static! {
        pub static ref I2C_BUS_READ_BYTE_TRANSACTION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadBytes
    lazy_static! {
        pub static ref I2C_BUS_READ_BYTES_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadBytes
    lazy_static! {
        pub static ref I2C_BUS_READ_BYTES_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadBytes
    lazy_static! {
        pub static ref I2C_BUS_READ_BYTES_TRANSACTION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadReg
    lazy_static! {
        pub static ref I2C_BUS_READ_REG_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadReg
    lazy_static! {
        pub static ref I2C_BUS_READ_REG_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusReadReg
    lazy_static! {
        pub static ref I2C_BUS_READ_REG_TRANSACTION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteByte
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTE_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteByte
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteByte
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTE_TRANSACTION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteByteReg
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTE_REG_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteByteReg
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTE_REG_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteByteReg
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTE_REG_TRANSACTION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteBytes
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTES_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteBytes
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTES_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteBytes
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTES_TRANSACTION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteBytesReg
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTES_REG_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteBytesReg
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTES_REG_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for I2cBusWriteBytesReg
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTES_REG_TRANSACTION_FAILED: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for I2cBusWriteBytes
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTES: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for I2cBusWriteBytesReg
    lazy_static! {
        pub static ref I2C_BUS_WRITE_BYTES_REG: Mime = "application/json".parse().unwrap();
    }

}
