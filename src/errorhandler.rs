use std::fmt;
use rusqlite::Error as RusqliteError;
use calamine::Error as CalamineError;
use calamine::XlsxError as ExcelError;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum AppError {
    Sqlite(RusqliteError),
    OpenExcelError(CalamineError),
    ExcelError(ExcelError),
    GenericError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::Sqlite(ref err) => write!(f, "SQLite error: {}", err),
            AppError::OpenExcelError(ref err) => write!(f, "Calamine error: {}", err),
            AppError::ExcelError(ref err) => write!(f, "Excel error: {}", err),
            AppError::GenericError(ref err) => write!(f, "Generic error: {}", err),
        }
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            AppError::Sqlite(ref err) => Some(err),
            AppError::OpenExcelError(ref err) => Some(err),
            AppError::ExcelError(ref err) => Some(err),
            AppError::GenericError(_) => None
        }
    }
}

impl From<RusqliteError> for AppError {
    fn from(err: RusqliteError) -> AppError {
        AppError::Sqlite(err)
    }
}

impl From<CalamineError> for AppError {
    fn from(err: CalamineError) -> AppError {
        AppError::OpenExcelError(err)
    }
}

impl From<ExcelError> for AppError {
    fn from(err: ExcelError) -> AppError {
        AppError::ExcelError(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::GenericError(err)
    }
}
