use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use crate::types::{OracleSource};
use crate::config::*;

pub struct PerpMarketConfig {
	full_name: Option<String>,
	category: Option<[String; 2]>,
	symbol: String,
	base_asset_symbol: String,
	market_index: u128,
	launch_ts: u128,
	oracle: Pubkey,
	oracle_source: OracleSource,
}

pub fn devnet_perp_markets() -> Vec<PerpMarketConfig> { 
    return vec![PerpMarketConfig {
        full_name: Some(String::from("Solana")),
        category: Some(["L1".to_string(), "Infra".to_string()]),
        symbol: "SOL-PERP".to_string(),
        base_asset_symbol: "SOL".to_string(),
        market_index: 0,
        oracle: Pubkey::from_str("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J").unwrap(),
        launch_ts: 1655751353000,
        oracle_source: OracleSource::PYTH,
    },
    PerpMarketConfig {
        full_name: Some("Bitcoin".to_string()),
        category: Some(["L1".to_string(), "Payment".to_string()]),
        symbol: "BTC-PERP".to_string(),
        base_asset_symbol: "BTC".to_string(),
        market_index: 1,
        oracle: Pubkey::from_str("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J").unwrap(),
        launch_ts: 1655751353000,
        oracle_source: OracleSource::PYTH,
    },
    PerpMarketConfig {
        full_name: Some("Ethereum".to_string()),
        category: Some(["L1".to_string(), "Infra".to_string()]),
        symbol: "ETH-PERP".to_string(),
        base_asset_symbol: "ETH".to_string(),
        market_index: 2,
        oracle: Pubkey::from_str("EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw").unwrap(),
        launch_ts: 1637691133472,
        oracle_source: OracleSource::PYTH,
    },
];
}

pub fn mainnet_perp_markets() -> Vec<PerpMarketConfig> {
    return vec![
        PerpMarketConfig {
            full_name: Some("Solana".to_string()),
            category: Some(["L1".to_string(), "Infra".to_string()]),
            symbol: "SOL-PERP".to_string(),
            base_asset_symbol: "SOL".to_string(),
            market_index: 0,
            oracle: Pubkey::from_str("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG").unwrap(),
            launch_ts: 1667560505000,
            oracle_source: OracleSource::PYTH,
        },
        PerpMarketConfig {
            full_name: Some("Bitcoin".to_string()),
            category: Some(["L1".to_string(), "Payment".to_string()]),
            symbol: "BTC-PERP".to_string(),
            base_asset_symbol: "BTC".to_string(),
            market_index: 1,
            oracle: Pubkey::from_str("GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU").unwrap(),
            launch_ts: 1670347281000,
            oracle_source: OracleSource::PYTH,
        },
        PerpMarketConfig {
            full_name: Some("Ethereum".to_string()),
            category: Some(["L1".to_string(), "Infra".to_string()]),
            symbol: "ETH-PERP".to_string(),
            base_asset_symbol: "ETH".to_string(),
            market_index: 2,
            oracle: Pubkey::from_str("JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWd").unwrap(),
            launch_ts: 1670347281000,
            oracle_source: OracleSource::PYTH,
        },
    ];
}

fn perp_markets(env: &DriftEnv) -> Vec<PerpMarketConfig> {
    match env {
        DriftEnv::Devnet => devnet_perp_markets(),
        DriftEnv::MainnetBeta => mainnet_perp_markets(),
    }
}