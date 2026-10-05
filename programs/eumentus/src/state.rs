use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CapabilityAuthority {
	pub agent: Pubkey,
	pub verifier: Pubkey,
	pub capability: [u8; 32],
	pub max_per_action: u64,
	pub state: AuthorityState,
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    Copy,
    InitSpace,
    PartialEq,
    Eq,
    Debug,
)]
pub enum AuthorityState {
	Active,
	Paused,
	Frozen,
	Revoked,
}

 
