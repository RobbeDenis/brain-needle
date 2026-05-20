
use core::fmt;


#[derive(Debug)]
pub enum BNError
{
    Io(std::io::Error),
    TokenMismatch(String, usize)
}

impl std::fmt::Display for BNError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            BNError::Io(err) => write!(f, "IO error: {}", err),
            BNError::TokenMismatch(err, index) => write!(f, "Token mismatch at index {index}: {err}")
        }
    }
}

impl From<std::io::Error> for BNError{
    fn from(err: std::io::Error) -> Self {
        BNError::Io(err)
    }
}