use anchor_lang::prelude::*;
use crate::state::reaction::{ReactionType, Reaction, TWEET_REACTION_SEED};
use crate::state::tweet::{Tweet, TWEET_SEED};

pub fn initialize_reaction(ctx: Context<AddReaction>, reaction_type: ReactionType) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let author = *ctx.accounts.user.key;
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;

    tweet_reaction.author = author;
    tweet_reaction.tweet = tweet.key();
    tweet_reaction.bump = ctx.bumps.tweet_reaction;

    match reaction_type {
        ReactionType::Like => {
            tweet_reaction.reaction = reaction_type;
            tweet.likes = tweet.likes.checked_add(1).unwrap();
            Ok(())
        },
        ReactionType::Dislike => {
            tweet_reaction.reaction = reaction_type;
            tweet.dislikes = tweet.dislikes.checked_add(1).unwrap();
            Ok(())
        },
    }
}

#[derive(Accounts)]
pub struct AddReaction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Reaction::LEN,
        seeds = [TWEET_REACTION_SEED.as_bytes(), tweet.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [TWEET_SEED.as_bytes(), tweet.topic[..tweet.topic_length as usize].as_ref(), tweet.author.key().as_ref()],
        bump = tweet.bump,
    )]
    pub tweet: Account<'info, Tweet>,
}
