use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, Result,
};
use futures_util::future::LocalBoxFuture;
use log::{error, warn};
use std::future::{ready, Ready};
use std::time::Duration;
use tokio::time::timeout;

use crate::models::ApiResponse;

pub struct PanicRecovery;

impl<S, B> Transform<S, ServiceRequest> for PanicRecovery
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PanicRecoveryMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PanicRecoveryMiddleware { service }))
    }
}

pub struct PanicRecoveryMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for PanicRecoveryMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(response) => Ok(response),
                Err(err) => {
                    error!("Service error: {:?}", err);
                    Err(err)
                }
            }
        })
    }
}

pub struct RequestTimeout {
    timeout: Duration,
}

impl RequestTimeout {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            timeout: Duration::from_secs(timeout_secs),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestTimeout
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestTimeoutMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestTimeoutMiddleware {
            service,
            timeout: self.timeout,
        }))
    }
}

pub struct RequestTimeoutMiddleware<S> {
    service: S,
    timeout: Duration,
}

impl<S, B> Service<ServiceRequest> for RequestTimeoutMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        let timeout_duration = self.timeout;

        Box::pin(async move {
            match timeout(timeout_duration, fut).await {
                Ok(response) => response,
                Err(_) => {
                    warn!("Request timed out after {:?}", timeout_duration);
                    Err(actix_web::error::ErrorRequestTimeout("Request timeout"))
                }
            }
        })
    }
}