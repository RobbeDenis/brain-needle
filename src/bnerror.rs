
use core::fmt;


#[derive(Debug)]
pub enum BNError
{
    Io(std::io::Error),
    LoopTokenMismatch,
    EndLoopTokenMismatch
}

impl std::fmt::Display for BNError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            BNError::Io(err) => write!(f, "IO error: {}", err),
            BNError::LoopTokenMismatch => write!(f, "Loop token mismatch: found loop start '[' without matching end ']'"),
            BNError::EndLoopTokenMismatch => write!(f, "Loop token mismatch: found loop end ']' without matching start '['")
        }
    }
}

impl PartialEq for BNError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LoopTokenMismatch, Self::LoopTokenMismatch) => true,
            (Self::EndLoopTokenMismatch, Self::EndLoopTokenMismatch) => true,
            (Self::Io(a), Self::Io(b)) => a.kind() == b.kind(),
            _ => false,
        }
    }
}

impl From<std::io::Error> for BNError{
    fn from(err: std::io::Error) -> Self {
        BNError::Io(err)
    }
}
