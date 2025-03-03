use anchor_lang::prelude::*;
use crate::state::reaction::ReactionType;

pub fn initialize_reaction(ctx: Context<AddReaction>, reaction_type: ReactionType) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct AddReaction<'info> {
   
}
