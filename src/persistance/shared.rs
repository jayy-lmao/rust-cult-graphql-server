#[derive(Clone)]
pub enum BatchFnLoadError {
    NotFound,
    DBError(String),
}
