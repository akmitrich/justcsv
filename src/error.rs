use derive_more::From;

/// Crate `Result` type alias
pub type Result<T> = core::result::Result<T, Error>;

/// Crate error type
#[derive(Debug, From)]
pub enum Error {
    /// Any error with description
    #[from]
    Custom(String),
    // -- Externals
    /// Some IO fail
    #[from]
    Io(std::io::Error),
    /// `nom` parsing fail
    #[from]
    Nom(nom::error::Error<&'static str>),
    // -- Internals
    /// Basically is a normal completion
    StreamComplete,
    /// Stream ended but no correct CSV parsed
    UnexpectedEof,
    /// User calls for headers to write but there were some records written into stream
    WriteHeadersAfterRecords,
    /// Owned variant of a `nom` error
    NomFailed(String),
}

impl Error {
    /// Create crate `Error` from anything implementing `Display`
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
