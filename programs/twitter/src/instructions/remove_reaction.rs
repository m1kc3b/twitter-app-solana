use anchor_lang::prelude::*;

use crate::state::reaction::{ReactionType, Reaction, TWEET_REACTION_SEED};
use crate::state::tweet::{Tweet, TWEET_SEED};

pub fn delete_reaction(ctx: Context<RemoveReaction>) -> Result<()> {
  let tweet = &mut ctx.accounts.tweet;
  let tweet_reaction = &ctx.accounts.tweet_reaction;

  match tweet_reaction.reaction {
      ReactionType::Like => {
        tweet.likes = tweet.likes.checked_sub(1).unwrap(); // TODO: impl error when likes = 0
      },
      ReactionType::Dislike => {
        tweet.dislikes = tweet.dislikes.checked_sub(1).unwrap(); // // TODO: impl error when dislikes = 0
      }
  }
  Ok(())
}

#[derive(Accounts)]
pub struct RemoveReaction<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
    mut,
    close = user,
    seeds = [TWEET_REACTION_SEED.as_bytes(), tweet.key().as_ref(), user.key().as_ref()],
    bump = tweet_reaction.bump,
  )]
  pub tweet_reaction: Account<'info, Reaction>,

  #[account(
      mut,
      seeds = [TWEET_SEED.as_bytes(), tweet.topic[..tweet.topic_length as usize].as_ref(), tweet.author.key().as_ref()],
      bump = tweet.bump,
  )]
  pub tweet: Account<'info, Tweet>,
}