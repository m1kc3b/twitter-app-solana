use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Tweet {
    pub author: Pubkey,
    #[max_len(32)]
    pub topic: String,
    #[max_len(500)]
    pub content: String,
    pub likes: u64,
    pub dislikes: u64,
}