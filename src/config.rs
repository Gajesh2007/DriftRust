use solana_sdk::pubkey::Pubkey;
use crate::types::{OracleSource};

struct SpotMarketConfig {
    symbol: String,
    market_index: u128,
    oracle: Pubkey,
    mint: Pubkey,
    oracle_source: OracleSource,
    precision: u128,
    serum_market: Option<Pubkey>,
}
struct PerpMarketConfig {
	full_name: Option<String>,
	category: Option<Vec<String>>,
	symbol: String,
	base_asset_symbol: String,
	market_index: u128,
	launch_ts: u128,
	oracle: Pubkey,
	oracle_source: OracleSource,
}

pub struct DriftConfig {
    ENV: DriftEnv,
    PYTH_ORACLE_MAPPING_ADDRESS: String,
    DRIFT_PROGRAM_ID: String,
    USDC_MINT_ADDRESS: String,
    SERUM_V3: String,
    V2_ALPHA_TICKET_MINT_ADDRESS: String,
    PERP_MARKETS: Vec<PerpMarketConfig>,
    SPOT_MARKETS: Vec<SpotMarketConfig>,
}

pub enum DriftEnv {
    Devnet,
    MainnetBeta,
}
