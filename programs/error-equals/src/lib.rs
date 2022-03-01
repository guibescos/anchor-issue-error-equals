use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
    #[msg("My error")]
    MyError,
}

#[cfg(test)]
pub mod tests {
    use crate::ErrorCode;
    use anchor_lang::prelude::{error, Pubkey, Result};

    #[test]
    fn test_error() {
        fn returns_error() -> Result<()> {
            return Err(error!(ErrorCode::MyError));
        }

        assert!(returns_error().is_err())
    }

    #[test]
    fn test_error_2() {
        fn returns_error() -> Result<()> {
            return Err(error!(ErrorCode::MyError));
        }

        assert_eq!(returns_error(), Err(error!(ErrorCode::MyError)))
    }
}
