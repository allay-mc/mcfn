use std::fmt;

#[derive(thiserror::Error, Debug)]
pub struct Error {
    pub location: Location,

    #[source]
    pub kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.location, self.kind)
    }
}

#[derive(Debug)]
pub struct Location {
    pub file: String,
    pub line: u32,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("the macro '{0}' does not exist")]
    UnknownMacro(String),

    #[error("unexpected macro '{0}'")]
    UnexpectedMacro(String),

    #[error("macro '{0}' expects an argument")]
    MissingArgument(String),

    #[error("macro '{0}' does not expect an argument")]
    UnexpectedArgument(String),

    #[error("proc '{0}' does not exist")]
    UnknownProc(String),
}
