use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ReactionType {
    Like,
    Dislike,
}

#[account]
pub struct Reaction {
    pub tweet: Pubkey,
    pub author: Pubkey,
    pub reaction: ReactionType,
}

impl Reaction {
    pub const LEN: usize = 32 + 32 + 1;
}