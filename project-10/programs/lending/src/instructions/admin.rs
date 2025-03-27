use anchor_lang::prelude::*;

#[derive(Accounts)]

pub struct InitBank<'info> {
  #[account(null)]

  pub signerL Signer<'info>˜
}