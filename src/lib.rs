pub mod colors;
mod parse;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not parse colour")]
    InvalidColor,
}
