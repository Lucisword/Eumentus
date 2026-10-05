use std::rc::Rc;

use anchor_client::{Client, Cluster};
use anchor_lang::{AccountDeserialize, prelude::Pubkey,};
use eumentus::{
    accounts::CreateCapabilityAuthority,
    instruction::CreateCapabilityAuthority as CreateCapabilityAuthorityIx,
    state::{AuthorityState, CapabilityAuthority},
};
use solana_keypair::{read_keypair_file, Keypair};
use solana_signer::Signer;

#[test]
fn creates_capability_authority() -> Result<(), Box<dyn std::error::Error>> {
    let payer = read_keypair_file(
        std::env::var("HOME")? + "/.config/solana/id.json",
    )?;

    let client = Client::new(Cluster::Localnet, Rc::new(payer));

    let program = client.program(eumentus::ID)?;

    let agent = Keypair::new();
    let verifier = Keypair::new();

    let capability = [7u8; 32];
    let max_per_action = 1_000_000u64;

    let (authority_pda, _) = Pubkey::find_program_address(
        &[
            b"authority",
            agent.pubkey().as_ref(),
            capability.as_ref(),
        ],
        &eumentus::ID,
    );

    program
        .request()
        .accounts(CreateCapabilityAuthority {
            principal: program.payer(),
            agent: agent.pubkey(),
	    verifier: verifier.pubkey(),
            authority: authority_pda,
            system_program: anchor_lang::system_program::ID,
        })
        .args(CreateCapabilityAuthorityIx {
            capability,
            max_per_action,
        })
        .send()?;

    let account = program
        .rpc()
        .get_account(&authority_pda)?;

    let mut data: &[u8] = &account.data;

    let authority = CapabilityAuthority::try_deserialize(&mut data)?;

    assert_eq!(authority.agent, agent.pubkey());
    assert_eq!(authority.capability, capability);
    assert_eq!(authority.max_per_action, max_per_action);
    assert_eq!(authority.state, AuthorityState::Active);

    Ok(())
}
