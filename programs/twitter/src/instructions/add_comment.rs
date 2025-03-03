use anchor_lang::prelude::*;

pub fn initialize_comment(ctx: Context<AddComment>, content: String) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
pub struct AddComment<'info> {
    
}
