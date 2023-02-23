use anchor_lang::prelude::*;

#[error_code]
pub enum DerugError {
    #[msg("This wallet rugged the collection")]
    RuggerSigner,
}
