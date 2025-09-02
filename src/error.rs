use thiserror::Error;

pub type ResultX<T> = std::result::Result<T, ErrorX>;
pub type ResultGrapes<T> = std::result::Result<T, GrapesError>;

#[derive(Error, Debug)]
pub enum ErrorX {
    #[error("Could not load x11_dl: {0} - {1}")]
    CouldNotLoadX11(String, String),
    #[error("Display: {0} - {1}")]
    Display(String, String),
    #[error("Window: {0} - {1}")]
    Window(String, String),
    #[error("{0} to {1}")]
    TypeConversion(String, String),
    #[error("{0}")]
    Generic(String),
}

pub type ResultG<T> = std::result::Result<T, GrapesError>;
#[derive(Error, Debug)]
pub enum GrapesError {
    #[error("Error loading resource {0}")]
    LoadingResource(String),
    #[error("Parsing resource {0}")]
    ParsingResource(String),
    #[error("Unsupported: {0}")]
    Unsupported(String),
    #[error("Illegal Conversion: {0}")]
    IllegalConversion(String),
    #[error("x error")]
    XError(#[from] ErrorX)
}
