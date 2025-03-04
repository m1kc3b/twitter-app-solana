use anchor_lang::prelude::*;

pub const COMMENT_LENGTH: usize = 500;

#[account]
pub struct Comment {
    pub tweet: Pubkey,
    pub author: Pubkey,
    pub content: [u8; COMMENT_LENGTH],
}

impl Comment {
    pub const LEN: usize = 32 + 32 + COMMENT_LENGTH;
}