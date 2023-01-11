
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    FileError,
    ParseError,
}
