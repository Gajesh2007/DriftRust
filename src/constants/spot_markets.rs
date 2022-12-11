use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;
use super::numeric_constants::{QUOTE_PRECISION, LAMPORTS_PER_SOL};
use crate::types::{OracleSource};
use crate::config::*;

pub struct SpotMarketConfig {
    symbol: String,
    market_index: u128,
    oracle: Pubkey,
    mint: Pubkey,
    oracle_source: OracleSource,
    precision: u128,
    serum_market: Option<Pubkey>,
}

pub fn DevnetSpotMarkets() -> Vec<SpotMarketConfig> {
    return vec![
        SpotMarketConfig {
            symbol: "USDC".to_string(),
            market_index: 0,
            oracle: Pubkey::default(),
            oracle_source: OracleSource::QUOTE_ASSET,
            mint: Pubkey::from_str("8zGuJQqwhZafTah7Uc7Z4tXRnguqkn5KLFAP8oV6PHe2").unwrap(),
            precision: QUOTE_PRECISION,
            serum_market: None,
        },
        SpotMarketConfig {
            symbol: "SOL".to_string(),
            market_index: 1,
            oracle: Pubkey::from_str("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix").unwrap(),
            oracle_source: OracleSource::PYTH,
            mint: Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
            precision: LAMPORTS_PER_SOL,
            serum_market: Some(Pubkey::from_str("8N37SsnTu8RYxtjrV9SStjkkwVhmU8aCWhLvwduAPEKW").unwrap()),
        },
        SpotMarketConfig {
            symbol: "BTC".to_string(),
            market_index: 2,
            oracle: Pubkey::from_str("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J").unwrap(),
            oracle_source: OracleSource::PYTH,
            mint: Pubkey::from_str("3BZPwbcqB5kKScF3TEXxwNfx5ipV13kbRVDvfVp5c6fv").unwrap(),
            precision: QUOTE_PRECISION,
            serum_market: Some(Pubkey::from_str("AGsmbVu3MS9u68GEYABWosQQCZwmLcBHu4pWEuBYH7Za").unwrap()),
        },
    ];
}

pub fn MainnetSpotMarkets() -> Vec<SpotMarketConfig> {
    return vec![
        SpotMarketConfig {
            symbol: "USDC".to_string(),
            market_index: 0,
            oracle: Pubkey::default(),
            oracle_source: OracleSource::QUOTE_ASSET,
            mint: Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            precision: QUOTE_PRECISION,
            serum_market: None,
        },
        SpotMarketConfig {
            symbol: "SOL".to_string(),
            market_index: 1,
            oracle: Pubkey::from_str("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG").unwrap(),
            oracle_source: OracleSource::PYTH,
            mint: Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
            precision: LAMPORTS_PER_SOL,
            serum_market: Some(Pubkey::from_str("8BnEgHoWFysVcuFFX7QztDmzuH8r5ZFvyP3sYwn1XTh6").unwrap()),
        },
    ];
}

fn spot_markets(env: &DriftEnv) -> Vec<SpotMarketConfig> {
    match env {
        DriftEnv::Devnet => DevnetSpotMarkets(),
        DriftEnv::MainnetBeta => MainnetSpotMarkets(),
    }
}