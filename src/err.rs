use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct AppError {
    msg: String
}

impl AppError {
    pub fn new<StrT: Into<String>>(msg: StrT) -> Self {
        Self {
            msg: msg.into()
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "App error: {}", self.msg)
    }
}

impl Error for AppError {}
