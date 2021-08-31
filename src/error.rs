use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Invalid key size.
    #[error("Key_size may not be less than 1!")]
    InvalidKeySize,
}
