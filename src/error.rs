use std::error::Error;
use std::sync::PoisonError;
use tray_item::TIError;

pub type InnerResult<T> = Result<T, InnerError>;

#[derive(Debug)]
pub struct InnerError {
    msg: String,
}

impl InnerError {
    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

impl From<Box<dyn std::error::Error>> for InnerError {
    fn from(e: Box<dyn Error>) -> Self {
        Self { msg: e.to_string() }
    }
}

impl From<TIError> for InnerError {
    fn from(e: TIError) -> Self {
        Self { msg: e.to_string() }
    }
}

impl From<String> for InnerError {
    fn from(e: String) -> Self {
        Self { msg: e.to_string() }
    }
}

impl<Guard> From<PoisonError<Guard>> for InnerError {
    fn from(e: PoisonError<Guard>) -> Self {
        Self { msg: e.to_string() }
    }
}
