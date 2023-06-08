#[derive(Debug)]
pub struct ParseStateError(pub String);

impl std::fmt::Display for ParseStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing state: {}", self.0)
    }
}

impl std::error::Error for ParseStateError {}
