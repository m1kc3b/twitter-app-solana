use anchor_lang::prelude::*;

pub const TOPIC_LENGTH: usize = 32;
pub const CONTENT_LENGTH: usize = 500;

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub topic: [u8; TOPIC_LENGTH],
    pub content: [u8; CONTENT_LENGTH],
    pub likes: u64,
    pub dislikes: u64,
}

impl Tweet {
    pub const LEN: usize = 32 + TOPIC_LENGTH + CONTENT_LENGTH + 8 + 8;
}