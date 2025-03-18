use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub name: String,
    pub admin: Pubkey,
    pub fee: u16,
    // bumps
    pub marketplace_bump: u8,
    pub treasury_bump: u8,
    pub rewards_mint_bump: u8,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 36 + 32 + 2 + 1 + 1 + 1;
}
