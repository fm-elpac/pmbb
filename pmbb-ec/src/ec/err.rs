//! 自定义错误类型
use std::error::Error;
use std::fmt::{Display, Formatter};

/// pmbb-ec 自定义错误类型
#[derive(Debug, Clone)]
pub struct E(pub String);

impl E {
    pub fn new(m: String) -> Self {
        Self(m)
    }
}

impl Error for E {}

impl Display for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<String> for E {
    fn from(v: String) -> Self {
        Self::new(v)
    }
}

impl From<&str> for E {
    fn from(v: &str) -> Self {
        Self::new(v.into())
    }
}
