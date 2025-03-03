use anchor_lang::prelude::*;
use crate::state::reaction::ReactionType;
use crate::state::reaction::Reaction;
use crate::state::tweet::Tweet;

pub fn initialize_reaction(ctx: Context<AddReaction>, reaction_type: ReactionType) -> Result<()> {
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;
    tweet_reaction.author = *ctx.accounts.user.key;

    match reaction_type {
        ReactionType::Like => {
            ctx.accounts.tweet.likes = ctx.accounts.tweet.likes.checked_add(1).unwrap();
            Ok(())
        },
        ReactionType::Dislike => {
            ctx.accounts.tweet.dislikes = ctx.accounts.tweet.dislikes.checked_add(1).unwrap();
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
        space = 8 + Reaction::INIT_SPACE,
        seeds = [tweet.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,

    pub system_program: Program<'info, System>,

    // TODO: impl tweet
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
