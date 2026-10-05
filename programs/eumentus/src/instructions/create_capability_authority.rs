use anchor_lang::prelude::*;

use crate::{
    constants::AUTHORITY_SEED,
    state::{AuthorityState, CapabilityAuthority},
};

#[derive(Accounts)]
#[instruction(capability: [u8; 32], max_per_action: u64)]
pub struct CreateCapabilityAuthority<'info> {
    #[account(mut)]
    pub principal: Signer<'info>,

    /// CHECK: The agent is an identity referenced by the authority.
    pub agent: UncheckedAccount<'info>,

    /// CHECK: The verifier is an identity authorized to verify capability outcomes.
    pub verifier: UncheckedAccount<'info>,

    #[account(
        init,
        payer = principal,
        space = 8 + CapabilityAuthority::INIT_SPACE,
        seeds = [
            AUTHORITY_SEED,
            agent.key().as_ref(),
            capability.as_ref(),
        ],
        bump
    )]
    pub authority: Account<'info, CapabilityAuthority>,

    pub system_program: Program<'info, System>,
}

pub fn handle_create_capability_authority(
    ctx: Context<CreateCapabilityAuthority>,
    capability: [u8; 32],
    max_per_action: u64,
) -> Result<()> {
    let authority = &mut ctx.accounts.authority;

    authority.agent = ctx.accounts.agent.key();
    authority.verifier = ctx.accounts.verifier.key();
    authority.capability = capability;
    authority.max_per_action = max_per_action;
    authority.state = AuthorityState::Active;

    Ok(())
}
