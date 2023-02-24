use anchor_lang::prelude::*;

#[error_code]
pub enum DerugError {
    #[msg("This wallet rugged the collection")]
    RuggerSigner,

    #[msg("Vote record seeds aren't correct")]
    InvalidVoteRecord,

    #[msg("Token account is not correct for the mint")]
    InvalidTokenAccountMint,

    #[msg("Metadata is not correct for the mint")]
    InvalidMetadata,

    #[msg("Token account doesn't possess the nft")]
    EmptyTokenAccount,

    #[msg("Payer doesn't own the token account")]
    WrongOwner,

    #[msg("User alredy voted with given nft")]
    AlereadyVoted,
}
