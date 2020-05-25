#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
#[allow(unused_imports)]
use i2cbus_api::{Api, ApiNoContext, Client, ContextWrapperExt, models,
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

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Has, Push, AuthData};

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "I2cBusApi",
                "I2cBusList",
                "I2cBusReadByte",
                "I2cBusReadBytes",
                "I2cBusReadReg",
                "I2cBusWriteByte",
                "I2cBusWriteByteReg",
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client")
    };

    let context: swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let client = client.with_context(context);

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("I2cBusApi") => {
            let result = rt.block_on(client.i2c_bus_api(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("I2cBusList") => {
            let result = rt.block_on(client.i2c_bus_list(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("I2cBusReadByte") => {
            let result = rt.block_on(client.i2c_bus_read_byte(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("I2cBusReadBytes") => {
            let result = rt.block_on(client.i2c_bus_read_bytes(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("I2cBusReadReg") => {
            let result = rt.block_on(client.i2c_bus_read_reg(
                  56,
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("I2cBusWriteByte") => {
            let result = rt.block_on(client.i2c_bus_write_byte(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("I2cBusWriteByteReg") => {
            let result = rt.block_on(client.i2c_bus_write_byte_reg(
                  56,
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("I2cBusWriteBytes") => {
            let result = rt.block_on(client.i2c_bus_write_bytes(
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("I2cBusWriteBytesReg") => {
            let result = rt.block_on(client.i2c_bus_write_bytes_reg(
                  56,
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
