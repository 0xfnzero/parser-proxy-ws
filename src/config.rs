use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub grpc: GrpcConfig,
    pub protocols: ProtocolsConfig,
    pub events: EventsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcConfig {
    pub endpoint: String,
    pub token: Option<String>,
    pub enable_metrics: bool,
    pub enable_tls: bool,
    pub connection_timeout_ms: u64,
    pub request_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolsConfig {
    pub pumpfun: bool,
    pub pumpswap: bool,
    pub bonk: bool,
    pub raydium_amm_v4: bool,
    pub raydium_clmm: bool,
    pub raydium_cpmm: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventsConfig {
    pub block_meta: bool,
    pub bonk_trade: bool,
    pub bonk_pool_create: bool,
    pub bonk_migrate_amm: bool,
    pub pumpfun_trade: bool,
    pub pumpfun_create: bool,
    pub pumpfun_complete: bool,
    pub pumpfun_migrate: bool,
    pub pumpswap_buy: bool,
    pub pumpswap_sell: bool,
    pub pumpswap_create_pool: bool,
    pub pumpswap_pool_created: bool,
    pub pumpswap_trade: bool,
    pub pumpswap_liquidity_added: bool,
    pub pumpswap_liquidity_removed: bool,
    pub pumpswap_pool_updated: bool,
    pub pumpswap_fees_claimed: bool,
    pub raydium_cpmm_swap: bool,
    pub raydium_cpmm_deposit: bool,
    pub raydium_cpmm_withdraw: bool,
    pub raydium_cpmm_initialize: bool,
    pub raydium_clmm_swap: bool,
    pub raydium_clmm_create_pool: bool,
    pub raydium_clmm_open_position: bool,
    pub raydium_clmm_close_position: bool,
    pub raydium_clmm_increase_liquidity: bool,
    pub raydium_clmm_decrease_liquidity: bool,
    pub raydium_clmm_open_position_with_token_ext_nft: bool,
    pub raydium_clmm_collect_fee: bool,
    pub raydium_amm_v4_swap: bool,
    pub raydium_amm_v4_deposit: bool,
    pub raydium_amm_v4_withdraw: bool,
    pub raydium_amm_v4_initialize2: bool,
    pub raydium_amm_v4_withdraw_pnl: bool,
    pub orca_whirlpool_swap: bool,
    pub orca_whirlpool_liquidity_increased: bool,
    pub orca_whirlpool_liquidity_decreased: bool,
    pub orca_whirlpool_pool_initialized: bool,
    pub meteora_pools_swap: bool,
    pub meteora_pools_add_liquidity: bool,
    pub meteora_pools_remove_liquidity: bool,
    pub meteora_pools_bootstrap_liquidity: bool,
    pub meteora_pools_pool_created: bool,
    pub meteora_pools_set_pool_fees: bool,
    pub meteora_damm_v2_swap: bool,
    pub meteora_damm_v2_add_liquidity: bool,
    pub meteora_damm_v2_remove_liquidity: bool,
    pub meteora_damm_v2_initialize_pool: bool,
    pub meteora_damm_v2_create_position: bool,
    pub meteora_damm_v2_close_position: bool,
    pub meteora_damm_v2_claim_position_fee: bool,
    pub meteora_damm_v2_initialize_reward: bool,
    pub meteora_damm_v2_fund_reward: bool,
    pub meteora_damm_v2_claim_reward: bool,
    pub token_account: bool,
    pub nonce_account: bool,
    pub token_info: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 9001,
            },
            grpc: GrpcConfig {
                endpoint: "https://solana-yellowstone-grpc.publicnode.com:443".to_string(),
                token: None,
                enable_metrics: true,
                enable_tls: true,
                connection_timeout_ms: 10000,
                request_timeout_ms: 30000,
            },
            protocols: ProtocolsConfig {
                pumpfun: true,
                pumpswap: false,
                bonk: false,
                raydium_amm_v4: false,
                raydium_clmm: false,
                raydium_cpmm: false,
            },
            events: EventsConfig {
                block_meta: false,
                bonk_trade: false,
                bonk_pool_create: false,
                bonk_migrate_amm: false,
                pumpfun_trade: true,
                pumpfun_create: true,
                pumpfun_complete: false,
                pumpfun_migrate: false,
                pumpswap_buy: false,
                pumpswap_sell: false,
                pumpswap_create_pool: false,
                pumpswap_pool_created: false,
                pumpswap_trade: false,
                pumpswap_liquidity_added: false,
                pumpswap_liquidity_removed: false,
                pumpswap_pool_updated: false,
                pumpswap_fees_claimed: false,
                raydium_cpmm_swap: false,
                raydium_cpmm_deposit: false,
                raydium_cpmm_withdraw: false,
                raydium_cpmm_initialize: false,
                raydium_clmm_swap: false,
                raydium_clmm_create_pool: false,
                raydium_clmm_open_position: false,
                raydium_clmm_close_position: false,
                raydium_clmm_increase_liquidity: false,
                raydium_clmm_decrease_liquidity: false,
                raydium_clmm_open_position_with_token_ext_nft: false,
                raydium_clmm_collect_fee: false,
                raydium_amm_v4_swap: false,
                raydium_amm_v4_deposit: false,
                raydium_amm_v4_withdraw: false,
                raydium_amm_v4_initialize2: false,
                raydium_amm_v4_withdraw_pnl: false,
                orca_whirlpool_swap: false,
                orca_whirlpool_liquidity_increased: false,
                orca_whirlpool_liquidity_decreased: false,
                orca_whirlpool_pool_initialized: false,
                meteora_pools_swap: false,
                meteora_pools_add_liquidity: false,
                meteora_pools_remove_liquidity: false,
                meteora_pools_bootstrap_liquidity: false,
                meteora_pools_pool_created: false,
                meteora_pools_set_pool_fees: false,
                meteora_damm_v2_swap: false,
                meteora_damm_v2_add_liquidity: false,
                meteora_damm_v2_remove_liquidity: false,
                meteora_damm_v2_initialize_pool: false,
                meteora_damm_v2_create_position: false,
                meteora_damm_v2_close_position: false,
                meteora_damm_v2_claim_position_fee: false,
                meteora_damm_v2_initialize_reward: false,
                meteora_damm_v2_fund_reward: false,
                meteora_damm_v2_claim_reward: false,
                token_account: false,
                nonce_account: false,
                token_info: false,
            },
        }
    }
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", path.as_ref()))?;

        Ok(config)
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        match Self::load_from_file(&path) {
            Ok(config) => {
                tracing::info!("✅ Loaded config from: {:?}", path.as_ref());
                config
            }
            Err(e) => {
                tracing::warn!("⚠️  Failed to load config: {}, using default", e);
                Self::default()
            }
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {:?}", path.as_ref()))?;

        Ok(())
    }

    pub fn get_enabled_protocols(&self) -> Vec<sol_parser_sdk::grpc::Protocol> {
        use sol_parser_sdk::grpc::Protocol;
        let mut protocols = Vec::new();

        if self.protocols.pumpfun {
            protocols.push(Protocol::PumpFun);
        }
        if self.protocols.pumpswap {
            protocols.push(Protocol::PumpSwap);
        }
        if self.protocols.bonk {
            protocols.push(Protocol::Bonk);
        }
        if self.protocols.raydium_amm_v4 {
            protocols.push(Protocol::RaydiumAmmV4);
        }
        if self.protocols.raydium_clmm {
            protocols.push(Protocol::RaydiumClmm);
        }
        if self.protocols.raydium_cpmm {
            protocols.push(Protocol::RaydiumCpmm);
        }

        protocols
    }

    pub fn get_enabled_event_types(&self) -> Vec<sol_parser_sdk::grpc::EventType> {
        use sol_parser_sdk::grpc::EventType;
        let mut event_types = Vec::new();

        if self.events.block_meta { event_types.push(EventType::BlockMeta); }
        if self.events.bonk_trade { event_types.push(EventType::BonkTrade); }
        if self.events.bonk_pool_create { event_types.push(EventType::BonkPoolCreate); }
        if self.events.bonk_migrate_amm { event_types.push(EventType::BonkMigrateAmm); }
        if self.events.pumpfun_trade { event_types.push(EventType::PumpFunTrade); }
        if self.events.pumpfun_create { event_types.push(EventType::PumpFunCreate); }
        if self.events.pumpfun_complete { event_types.push(EventType::PumpFunComplete); }
        if self.events.pumpfun_migrate { event_types.push(EventType::PumpFunMigrate); }
        if self.events.pumpswap_buy { event_types.push(EventType::PumpSwapBuy); }
        if self.events.pumpswap_sell { event_types.push(EventType::PumpSwapSell); }
        if self.events.pumpswap_create_pool { event_types.push(EventType::PumpSwapCreatePool); }
        if self.events.pumpswap_pool_created { event_types.push(EventType::PumpSwapPoolCreated); }
        if self.events.pumpswap_trade { event_types.push(EventType::PumpSwapTrade); }
        if self.events.pumpswap_liquidity_added { event_types.push(EventType::PumpSwapLiquidityAdded); }
        if self.events.pumpswap_liquidity_removed { event_types.push(EventType::PumpSwapLiquidityRemoved); }
        if self.events.pumpswap_pool_updated { event_types.push(EventType::PumpSwapPoolUpdated); }
        if self.events.pumpswap_fees_claimed { event_types.push(EventType::PumpSwapFeesClaimed); }
        if self.events.raydium_cpmm_swap { event_types.push(EventType::RaydiumCpmmSwap); }
        if self.events.raydium_cpmm_deposit { event_types.push(EventType::RaydiumCpmmDeposit); }
        if self.events.raydium_cpmm_withdraw { event_types.push(EventType::RaydiumCpmmWithdraw); }
        if self.events.raydium_cpmm_initialize { event_types.push(EventType::RaydiumCpmmInitialize); }
        if self.events.raydium_clmm_swap { event_types.push(EventType::RaydiumClmmSwap); }
        if self.events.raydium_clmm_create_pool { event_types.push(EventType::RaydiumClmmCreatePool); }
        if self.events.raydium_clmm_open_position { event_types.push(EventType::RaydiumClmmOpenPosition); }
        if self.events.raydium_clmm_close_position { event_types.push(EventType::RaydiumClmmClosePosition); }
        if self.events.raydium_clmm_increase_liquidity { event_types.push(EventType::RaydiumClmmIncreaseLiquidity); }
        if self.events.raydium_clmm_decrease_liquidity { event_types.push(EventType::RaydiumClmmDecreaseLiquidity); }
        if self.events.raydium_clmm_open_position_with_token_ext_nft { event_types.push(EventType::RaydiumClmmOpenPositionWithTokenExtNft); }
        if self.events.raydium_clmm_collect_fee { event_types.push(EventType::RaydiumClmmCollectFee); }
        if self.events.raydium_amm_v4_swap { event_types.push(EventType::RaydiumAmmV4Swap); }
        if self.events.raydium_amm_v4_deposit { event_types.push(EventType::RaydiumAmmV4Deposit); }
        if self.events.raydium_amm_v4_withdraw { event_types.push(EventType::RaydiumAmmV4Withdraw); }
        if self.events.raydium_amm_v4_initialize2 { event_types.push(EventType::RaydiumAmmV4Initialize2); }
        if self.events.raydium_amm_v4_withdraw_pnl { event_types.push(EventType::RaydiumAmmV4WithdrawPnl); }
        if self.events.orca_whirlpool_swap { event_types.push(EventType::OrcaWhirlpoolSwap); }
        if self.events.orca_whirlpool_liquidity_increased { event_types.push(EventType::OrcaWhirlpoolLiquidityIncreased); }
        if self.events.orca_whirlpool_liquidity_decreased { event_types.push(EventType::OrcaWhirlpoolLiquidityDecreased); }
        if self.events.orca_whirlpool_pool_initialized { event_types.push(EventType::OrcaWhirlpoolPoolInitialized); }
        if self.events.meteora_pools_swap { event_types.push(EventType::MeteoraPoolsSwap); }
        if self.events.meteora_pools_add_liquidity { event_types.push(EventType::MeteoraPoolsAddLiquidity); }
        if self.events.meteora_pools_remove_liquidity { event_types.push(EventType::MeteoraPoolsRemoveLiquidity); }
        if self.events.meteora_pools_bootstrap_liquidity { event_types.push(EventType::MeteoraPoolsBootstrapLiquidity); }
        if self.events.meteora_pools_pool_created { event_types.push(EventType::MeteoraPoolsPoolCreated); }
        if self.events.meteora_pools_set_pool_fees { event_types.push(EventType::MeteoraPoolsSetPoolFees); }
        if self.events.meteora_damm_v2_swap { event_types.push(EventType::MeteoraDammV2Swap); }
        if self.events.meteora_damm_v2_add_liquidity { event_types.push(EventType::MeteoraDammV2AddLiquidity); }
        if self.events.meteora_damm_v2_remove_liquidity { event_types.push(EventType::MeteoraDammV2RemoveLiquidity); }
        if self.events.meteora_damm_v2_initialize_pool { event_types.push(EventType::MeteoraDammV2InitializePool); }
        if self.events.meteora_damm_v2_create_position { event_types.push(EventType::MeteoraDammV2CreatePosition); }
        if self.events.meteora_damm_v2_close_position { event_types.push(EventType::MeteoraDammV2ClosePosition); }
        if self.events.meteora_damm_v2_claim_position_fee { event_types.push(EventType::MeteoraDammV2ClaimPositionFee); }
        if self.events.meteora_damm_v2_initialize_reward { event_types.push(EventType::MeteoraDammV2InitializeReward); }
        if self.events.meteora_damm_v2_fund_reward { event_types.push(EventType::MeteoraDammV2FundReward); }
        if self.events.meteora_damm_v2_claim_reward { event_types.push(EventType::MeteoraDammV2ClaimReward); }
        if self.events.token_account { event_types.push(EventType::TokenAccount); }
        if self.events.nonce_account { event_types.push(EventType::NonceAccount); }
        if self.events.token_info { event_types.push(EventType::TokenInfo); }

        event_types
    }
}