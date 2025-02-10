use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, // the user who's the maker of the escrow and the owner of the ATA 
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
      mut, 
      associated_token::mint = mint_a, 
      associated_token::authority = maker
    )]
    pub maker_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      init,
      payer = maker,
      space = 8 + Escrow::INIT_SPACE,
      seeds = [b"escrow", maker.key().as_ref(), seeds.to_le_bytes().as_ref()],
      // seeds = [b"escrow", maker.key().as_ref()],
      // seeds = [b"escrow"],
      bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
      init, 
      payer = maker,
      associated_token::mint = mint_a,
      associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info>{
  pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()>{
    self.escrow.set_inner(Escrow {
      seed,
      maker: self.maker.key(),
      mint_a: self.mint_a.key(),
      mint_b: self.mint_b.key(),
      receive,
      bump: bumps.escrow,
    });

    Ok(())
  }

  pub fn deposit(&mut self, amount: u64) -> Result<()>{
    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
      from: self.maker_a_ata.to_account_info(), // what does this account mean/represent?
      to: self.vault.to_account_info(), // why transfer from maker to vault and not to escrow directly?
      authority: self.maker.to_account_info(),
      mint: self.mint_a.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;

    Ok(())
  }
}