use anchor_lang::prelude::*;

declare_id!("69ftbWSHZ4Bo3GmTZRuuARbjN1y1o5cu3xXBQZt4YJY4");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
