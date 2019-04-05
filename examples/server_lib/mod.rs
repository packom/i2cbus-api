//! Main library entry point for i2cbus_api implementation.

mod server;

mod errors {
    error_chain!{}
}

pub use self::errors::*;
use std::io;
use std::clone::Clone;
use std::marker::PhantomData;
use hyper;
use i2cbus_api;
use swagger::{Has, XSpanIdString};
//use swagger::auth::Authorization;

pub struct NewService<C>{
    marker: PhantomData<C>
}

impl<C> NewService<C>{
    pub fn new() -> Self {
        NewService{marker:PhantomData}
    }
}

impl<C> hyper::server::NewService for NewService<C> where C: Has<XSpanIdString>  + Clone + 'static {
    type Request = (hyper::Request, C);
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Instance = i2cbus_api::server::Service<server::Server<C>, C>;

    /// Instantiate a new server.
    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(i2cbus_api::server::Service::new(server::Server::new()))
    }
}
