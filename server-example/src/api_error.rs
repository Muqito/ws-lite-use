#[derive(Debug)]
pub enum APIError {
    Unknown,
}
impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown error")
    }
}
impl From<std::io::Error> for APIError {
    fn from(_: std::io::Error) -> APIError {
        APIError::Unknown
    }
}
