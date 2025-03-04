use anchor_lang::prelude::*;

pub const TWEET_REACTION_SEED: &str = "TWEET_REACTION_SEED";

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
    pub bump: u8,
}

impl Reaction {
    pub const LEN: usize = 32 + 32 + 1;
}