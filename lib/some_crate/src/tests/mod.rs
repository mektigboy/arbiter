#[allow(missing_docs)]
#[cfg(test)]
use std::str::FromStr;

use anyhow::Result;
use ethers::{prelude::Middleware, types::Address};

use crate::{
    agent::{tests::*, *},
    bindings::arbiter_token::*,
    environment::{tests::*, *},
    manager::{tests::*, *},
    middleware::{tests::*, *},
};

const TEST_ARG_NAME: &str = "ArbiterToken";
const TEST_ARG_SYMBOL: &str = "ARBT";
const TEST_ARG_DECIMALS: u8 = 18;
const TEST_MINT_AMOUNT: u128 = 1;
const TEST_MINT_TO: &str = "0xf7e93cc543d97af6632c9b8864417379dba4bf15";

#[test]
/// Test that the writer contract can echo a string.
/// The writer contract takes in no constructor args.
fn string_write() -> Result<()> {
    Ok(())
}

#[test]
fn token_mint() -> Result<()> {
    Ok(())
}

#[test]
fn auto_deploy() -> Result<()> {
    Ok(())
}

#[test]
fn arbiter_math() -> Result<()> {
    Ok(())
}

#[test]
fn simulation_agent_wallet() {
    let environment = Environment::new(TEST_ENV_LABEL.to_string());
    let agent =
        Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.provider.connection);
    assert_eq!(
        agent.client.default_sender().unwrap(),
        Address::from_str("0xf7e93cc543d97af6632c9b8864417379dba4bf15").unwrap()
    );
}


// TODO: Replace this all with arbitertoken tests and remove the writer contract
async fn deploy() -> Result<ArbiterToken<RevmMiddleware>> {
    let mut environment = Environment::new(TEST_ENV_LABEL.to_string());
    environment.run();
    let agent =
        Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.provider.connection);
    Ok(ArbiterToken::deploy(agent.client, (TEST_ARG_NAME.to_string(), TEST_ARG_SYMBOL.to_string(), TEST_ARG_DECIMALS))?.send().await?)
}

#[tokio::test]
async fn test_deploy() -> Result<()> {
    let arbiter_token = deploy().await?;
    println!("{:?}", arbiter_token);
    assert_eq!(
        arbiter_token.address(),
        Address::from_str("0x6b1d802fba7ec153ece61bb06f1c5580c3025233").unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn call() -> Result<()> {
    let arbiter_token = deploy().await?;
    let admin = arbiter_token.admin();
    let output = admin.call().await?;
    assert_eq!(output, Address::from_str("0xf7e93cc543d97af6632c9b8864417379dba4bf15")?);
    Ok(())
}

#[tokio::test]
async fn transact() -> Result<()> {
    let arbiter_token = deploy().await?;
    let mint = arbiter_token.mint(Address::from_str(TEST_MINT_TO).unwrap(), ethers::types::U256::from(TEST_MINT_AMOUNT));
    let receipt = mint.send().await?.await?.unwrap();
    Ok(())
}

#[tokio::test]
async fn watch() {
    todo!()
}

#[tokio::test]
async fn filter_watcher() {
    todo!()
}