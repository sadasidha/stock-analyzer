pub struct AppError {
    pub message: String,
}

impl AppError {
    pub fn new(message: &str, file_name: &str, line_no: u32) -> Self {
        let msg = format!("{}[{}]: {}", file_name, line_no, message);
        AppError {
            message: msg,
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::new(&format!("Database error: {}", err), file!(), line!())
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        AppError::new(&format!("CSV parsing error: {}", err), file!(), line!())
    }
}
