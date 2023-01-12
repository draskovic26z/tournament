use anchor_lang::prelude::*;

#[error_code]
pub enum TRMTError {
    #[msg("Wrong metaplex metadata!")]
    WrongMetaplexMetadata,

    #[msg("Metadata has a wrong collection")]
    WrongCollection,

    #[msg("Player isn't the owner of the token account")]
    WrongOwnerOfTA,

    #[msg("Token account and mint don't match")]
    WrongTokenAccountMint,

    #[msg("Signer doesn't possess the NFT")]
    NoNftInTA,

    WrongWarriorMetadata,
    WarriorMetadataWrongMint,
}
