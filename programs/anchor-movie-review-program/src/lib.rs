use anchor_lang::prelude::*;

declare_id!("BmtbaQVxXMSQmsSt8k3JtwBiCvgKkmvXmubpADqKY9CR");

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
