
#[derive(Debug)]
pub struct AppError{
  pub  message: String
}

impl AppError{
    pub fn new(message: String) -> AppError{
        AppError{message}
    }
}