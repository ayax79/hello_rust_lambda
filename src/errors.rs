use rusoto_core::RusotoError;
use std::convert::{From, Into};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use lambda_runtime::error::{HandlerError, LambdaErrorExt};

#[derive(Debug)]
pub enum HelloError {
    DbError(String),
}

impl Display for HelloError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &HelloError::DbError(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for HelloError {}

impl LambdaErrorExt for HelloError {
    fn error_type(&self) -> &str {
        match *self {
            HelloError::DbError(_) => "DatabaseError"
        }
    }
}

impl<E: Error + 'static> From<RusotoError<E>> for HelloError {
    fn from(re: RusotoError<E>) -> Self {
        HelloError::DbError(format!("{}", re))
    }
}

impl Into<HandlerError> for HelloError {

    fn into(self) -> HandlerError {
         HandlerError::new(self)
    }

}
