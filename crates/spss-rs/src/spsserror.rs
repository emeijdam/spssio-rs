use std::fmt;

#[derive(Debug)]
pub enum SpssioErrorCode {
    FileOpenError,
    FileCloseError,
    InvalidHandle,
    InvalidFile,
    NoMemory,
    InvalidVarName,
    VarNotFound,
    DuplicateVar,
    NumericExpected,
    StringExpected,
    InvalidVarType,
    InvalidMeasureLevel,
    InvalidRole,
    InvalidPassword,
    EmptyPassword,
    Other(i32),
}

impl fmt::Display for SpssioErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpssioErrorCode::FileOpenError => write!(f, "File open error"),
            SpssioErrorCode::FileCloseError => write!(f, "File close error"),
            SpssioErrorCode::InvalidHandle => write!(f, "Invalid handle"),
            SpssioErrorCode::InvalidFile => write!(f, "Invalid file"),
            SpssioErrorCode::NoMemory => write!(f, "No memory"),
            SpssioErrorCode::InvalidVarName => write!(f, "Invalid variable name"),
            SpssioErrorCode::VarNotFound => write!(f, "Variable not found"),
            SpssioErrorCode::DuplicateVar => write!(f, "Duplicate variable"),
            SpssioErrorCode::NumericExpected => write!(f, "Numeric value expected"),
            SpssioErrorCode::StringExpected => write!(f, "String value expected"),
            SpssioErrorCode::InvalidVarType => write!(f, "Invalid variable type"),
            SpssioErrorCode::InvalidMeasureLevel => write!(f, "Invalid measure level"),
            SpssioErrorCode::InvalidRole => write!(f, "Invalid role"),
            SpssioErrorCode::InvalidPassword => write!(f, "Invalid password"),
            SpssioErrorCode::EmptyPassword => write!(f, "Empty password"),
            SpssioErrorCode::Other(code) => write!(f, "Other error: {}", code),
        }
    }
}

impl std::error::Error for SpssioErrorCode {}

impl From<i32> for SpssioErrorCode {
    fn from(code: i32) -> Self {
        match code {
            1 => SpssioErrorCode::FileOpenError,
            2 => SpssioErrorCode::FileCloseError,
            5 => SpssioErrorCode::InvalidHandle,
            6 => SpssioErrorCode::InvalidFile,
            7 => SpssioErrorCode::NoMemory,
            10 => SpssioErrorCode::InvalidVarName,
            12 => SpssioErrorCode::VarNotFound,
            13 => SpssioErrorCode::DuplicateVar,
            14 => SpssioErrorCode::NumericExpected,
            15 => SpssioErrorCode::StringExpected,
            17 => SpssioErrorCode::InvalidVarType,
            56 => SpssioErrorCode::InvalidMeasureLevel,
            79 => SpssioErrorCode::InvalidRole,
            80 => SpssioErrorCode::InvalidPassword,
            81 => SpssioErrorCode::EmptyPassword,
            _ => SpssioErrorCode::Other(code),
        }
    }
}