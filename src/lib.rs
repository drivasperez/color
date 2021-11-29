pub mod colors;
mod parse;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not parse colour")]
    InvalidColor,
    #[error("Invalid color type `{0}` valid colors are: `hex`, `rgb`, `rgba`, `hsla`")]
    InvalidColorType(String),
}
