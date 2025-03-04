use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::state::reaction::TWEET_REACTION_SEED;
use crate::state::tweet::{Tweet, TWEET_SEED};
use crate::state::comment::Comment;

pub fn delete_comment(_ctx: Context<RemoveComment>) -> Result<()> {
    msg!("Comment deleted!");
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveComment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(), 
            tweet.key().as_ref(), 
            user.key().as_ref(),
            hash(tweet_comment.content[..tweet_comment.content_length as usize].as_ref()).to_bytes().as_ref(),
        ],
        bump = tweet_comment.bump,
    )]
    pub tweet_comment: Account<'info, Comment>,

    #[account(
        mut,
        seeds = [TWEET_SEED.as_bytes(), tweet.topic[..tweet.topic_length as usize].as_ref(), tweet.author.key().as_ref()],
        bump = tweet.bump,
    )]
    pub tweet: Account<'info, Tweet>,
}
