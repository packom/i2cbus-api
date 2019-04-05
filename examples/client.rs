#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate openapi_client;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
#[macro_use]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;
extern crate clap;
extern crate tokio_core;

use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Has, Push, AuthData};

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
use tokio_core::reactor;
#[allow(unused_imports)]
use openapi_client::{ApiNoContext, ContextWrapperExt,
                      ApiError,
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
use clap::{App, Arg};

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
    "I2cBusApi",
    "I2cBusList",
])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("example.com")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("80")
            .help("Port to contact"))
        .get_matches();

    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        openapi_client::Client::try_new_https(core.handle(), &base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        openapi_client::Client::try_new_http(core.handle(), &base_url)
            .expect("Failed to create HTTP client")
    };

    let context: make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
        make_context!(ContextBuilder, EmptyContext, None, XSpanIdString(self::uuid::Uuid::new_v4().to_string()));
    let client = client.with_context(context);

    match matches.value_of("operation") {

        Some("I2cBusApi") => {
            let result = core.run(client.i2c_bus_api());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("I2cBusList") => {
            let result = core.run(client.i2c_bus_list());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("I2cBusReadByte") => {
        //     let result = core.run(client.i2c_bus_read_byte(???, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("I2cBusReadBytes") => {
        //     let result = core.run(client.i2c_bus_read_bytes(???, ???, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("I2cBusReadReg") => {
        //     let result = core.run(client.i2c_bus_read_reg(???, ???, ???, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("I2cBusWriteByte") => {
        //     let result = core.run(client.i2c_bus_write_byte(???, ???, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("I2cBusWriteByteReg") => {
        //     let result = core.run(client.i2c_bus_write_byte_reg(???, ???, ???, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("I2cBusWriteBytes") => {
        //     let result = core.run(client.i2c_bus_write_bytes(???, ???, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("I2cBusWriteBytesReg") => {
        //     let result = core.run(client.i2c_bus_write_bytes_reg(???, ???, ???, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

