pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use instructions::create_tweet::{CreateTweet, initialize_tweet};
use instructions::add_comment::{AddComment, initialize_comment};
use instructions::remove_comment::{RemoveComment, delete_comment};
use instructions::add_reaction::{AddReaction, initialize_reaction};
use instructions::remove_reaction::{RemoveReaction, delete_reaction};

use state::reaction::ReactionType;

declare_id!("7e957SngurXVSomdZ7Qhw3QeBB3FtZu9GEdpcMcBS5Mu");

#[program]
pub mod twitter {
    use crate::instructions::add_reaction;

    use super::*;

    pub fn create_tweet(ctx: Context<CreateTweet>, topic: String, content: String) -> Result<()> {
        initialize_tweet(ctx, topic, content)?;
        Ok(())
    }

    pub fn add_comment(ctx: Context<AddComment>, content: String) -> Result<()> {
        initialize_comment(ctx, content)?;
        Ok(())
    }

    pub fn remove_comment(ctx: Context<RemoveComment>) -> Result<()> {
        delete_comment(ctx)?;
        Ok(())
    }

    pub fn like(ctx: Context<AddReaction>) -> Result<()> {
        initialize_reaction(ctx, ReactionType::Like)?;
        Ok(())
    }

    pub fn dislike(ctx: Context<AddReaction>) -> Result<()> {
        initialize_reaction(ctx, ReactionType::Dislike)?;
        Ok(())
    }

    pub fn remove_reaction(ctx: Context<RemoveReaction>) -> Result<()> {
        delete_reaction(ctx)?;
        Ok(())
    }
}
