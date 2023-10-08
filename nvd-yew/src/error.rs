use thiserror::Error as ThisError;
#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum Error {
    //http request error (unable to send request, eg api server is down)
    #[error("Http Request Error")]
    RequestError,
    //401 (eg not logged in)
    #[error("Unauthorized")]
    Unauthorized,
    //403 (logged in but not allowed)
    #[error("Forbidden")]
    Forbidden,
    //404
    #[error("Not Found")]
    NotFound,
    //422
    #[error("Unprocessable Entity")]
    UnprocessableEntity,
    //500
    #[error("Internal Server Error")]
    InternalServerError,
    //502
    #[error("Bad Gateway")]
    BadGateway,
    //503
    #[error("Service Unavailable")]
    ServiceUnavailable,
    //504
    #[error("Gateway Timeout")]
    GatewayTimeout,
    //Other
    #[error("Other Error")]
    Other,
    //deserialize error (unable to parse recieved data)
    #[error("Deserialize Error")]
    DeserializeError,
}
