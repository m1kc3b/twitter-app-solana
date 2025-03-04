use anchor_lang::prelude::*;

pub const COMMENT_LENGTH: usize = 500;
pub const COMMENT_SEED: &str = "COMMENT_SEED";

#[account]
pub struct Comment {
    pub tweet: Pubkey,
    pub author: Pubkey,
    pub content: [u8; COMMENT_LENGTH],
    pub content_length: u8,
    pub bump: u8,
}

impl Comment {
    pub const LEN: usize = 32 + 32 + COMMENT_LENGTH;
}