use anchor_lang::prelude::*;

declare_id!("2E6MStoJPYvmvTBj3ooYYMKcjYxBC1ecdtprmSPJRfmd");

pub ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) => Result<()> {
        msg!("Greetings from {}", context.program_id);
        let user_public_key = context.accounts.user.key();
        msg!("User public key: {:?}", user_public_key);
        msg!("Number: {}", number);
        msg!("Color: {}", color);
        msg!("Hobbies: {:?}", hobbies);

        context.accounts.favorites.set_inner( Favorites {
            number,
            color,
            hobbies,
        });

        Ok(());
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    // We need to specify the length of the string
    // because the string is stored in the account
    // as a fixed size array.
    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,
}

/*
pub mod project_01 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
*/