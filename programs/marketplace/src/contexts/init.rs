use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{state::Marketplace, MarketplaceError};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
      init,
      payer = admin,
      space = Marketplace::INIT_SPACE,
      seeds = [b"marketplace", name.as_bytes()],
      bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
      seeds = [b"treasury", marketplace.key().as_ref()],
      bump
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
      init,
      payer = admin,
      seeds = [b"rewards", marketplace.key().as_ref()],
      bump,
      mint::authority = marketplace,
      mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: InitializeBumps) -> Result<()> {
        require!(
            !name.is_empty() && name.len() < 36,
            MarketplaceError::NameTooLong
        );

        self.marketplace.set_inner(Marketplace {
            name,
            fee,
            admin: self.admin.key(),
            marketplace_bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_mint_bump: bumps.rewards_mint,
        });
        Ok(())
    }
}
