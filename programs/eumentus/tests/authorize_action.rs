use std::rc::Rc;

use anchor_client::{Client, Cluster};
use anchor_lang::prelude::Pubkey;
use eumentus::{
    accounts::CreateCapabilityAuthority,
    instruction::{
        AuthorizeAction as AuthorizeActionIx,
        CreateCapabilityAuthority as CreateCapabilityAuthorityIx,
    },
};
use solana_keypair::{read_keypair_file, Keypair};
use solana_signer::Signer;

fn create_program() -> anchor_client::Program<Rc<Keypair>> {
    let payer = read_keypair_file(
        std::env::var("HOME").unwrap() + "/.config/solana/id.json",
    )
    .unwrap();

    let client = Client::new(Cluster::Localnet, Rc::new(payer));
    client.program(eumentus::ID).unwrap()
}

fn setup() -> (
    anchor_client::Program<Rc<Keypair>>,
    Keypair,
    [u8; 32],
    Pubkey,
) {
    let program = create_program();

    let agent = Keypair::new();
    let verifier=Keypair::new();
    let capability = [7u8; 32];

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
            max_per_action: 1_000_000,
        })
        .send()
        .unwrap();

    (program, agent, capability, authority_pda)
}

#[test]
fn authorizes_action_within_limit() -> Result<(), Box<dyn std::error::Error>> {
    let (program, agent, capability, authority_pda) = setup();

    program
        .request()
        .signer(&agent)
        .accounts(eumentus::accounts::AuthorizeAction {
            agent: agent.pubkey(),
            authority: authority_pda,
        })
        .args(AuthorizeActionIx {
            capability,
            amount: 100,
        })
        .send()?;

    Ok(())
}

#[test]
fn rejects_action_above_limit() -> Result<(), Box<dyn std::error::Error>> {
    let (program, agent, capability, authority_pda) = setup();

    let result = program
        .request()
        .signer(&agent)
        .accounts(eumentus::accounts::AuthorizeAction {
            agent: agent.pubkey(),
            authority: authority_pda,
        })
        .args(AuthorizeActionIx {
            capability,
            amount: 1_000_001,
        })
        .send();

    assert!(result.is_err());

    Ok(())
}


#[test]
fn rejects_wrong_agent() -> Result<(), Box<dyn std::error::Error>> {
    let (program, agent, capability, authority_pda) = setup();

    let attacker = Keypair::new();

    let result = program
        .request()
        .signer(&attacker)
        .accounts(eumentus::accounts::AuthorizeAction {
            agent: attacker.pubkey(),
            authority: authority_pda,
        })
        .args(AuthorizeActionIx {
            capability,
            amount: 100,
        })
        .send();

    assert!(result.is_err());

    // Keep `agent` live so the original authority remains meaningful
    // for the duration of this test.
    let _ = agent;

    Ok(())
}
