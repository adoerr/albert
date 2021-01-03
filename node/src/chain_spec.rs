use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use albert_runtime::{
    AccountId, BalancesConfig, GenesisConfig, Signature, SudoConfig, SystemConfig, WASM_BINARY,
};

/// Generate public key from `seed`.
pub fn pub_key_from_seed<T: Public>(seed: &str) -> <T::Pair as Pair>::Public {
    T::Pair::from_string(&format!("//{}", seed), None)
        .expect("secret string error")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate account id from `seed`.
pub fn account_id_from_seed<T: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<T::Pair as Pair>::Public>,
{
    AccountPublic::from(pub_key_from_seed::<T>(seed)).into_account()
}

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Chain specification configuration for development
pub fn dev_config() -> Result<ChainSpec, String> {
    let wasm =
        WASM_BINARY.ok_or_else(|| "Wasm binary development version not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // Id
        "dev",
        // Chain type
        ChainType::Development,
        // Genesis source
        move || genesis(wasm),
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol Id
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

/// Genesis configuration
fn genesis(wasm: &[u8]) -> GenesisConfig {
    // sudo account
    let sudo_key = account_id_from_seed::<sr25519::Public>("Alice");
    // pre-funded accounts
    let accounts = vec![
        account_id_from_seed::<sr25519::Public>("Alice"),
        account_id_from_seed::<sr25519::Public>("Bob"),
        account_id_from_seed::<sr25519::Public>("Charlie"),
        account_id_from_seed::<sr25519::Public>("Allice/stash"),
        account_id_from_seed::<sr25519::Public>("Bob/stash"),
        account_id_from_seed::<sr25519::Public>("Charlie/stash"),
    ];

    GenesisConfig {
        frame_system: Some(SystemConfig {
            // store wasm runtime
            code: wasm.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            // initial account balances
            balances: accounts.iter().cloned().map(|acc| (acc, 1 << 60)).collect(),
        }),
        pallet_sudo: Some(SudoConfig { key: sudo_key }),
    }
}

#[cfg(test)]
mod tests {
    use super::{account_id_from_seed, pub_key_from_seed};

    #[test]
    fn test_pub_key_from_seed() {
        let pk = pub_key_from_seed::<sp_core::sr25519::Public>("Alice");
        assert_eq!(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            pk.to_string()
        );

        let pk = pub_key_from_seed::<sp_core::sr25519::Public>("Bob");
        assert_eq!(
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
            pk.to_string()
        );

        let pk = pub_key_from_seed::<sp_core::sr25519::Public>("Charlie");
        assert_eq!(
            "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y".to_string(),
            pk.to_string()
        );
    }

    #[test]
    fn test_account_id_from_seed() {
        let id = account_id_from_seed::<sp_core::sr25519::Public>("Alice");
        assert_eq!(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            id.to_string()
        );

        let id = account_id_from_seed::<sp_core::sr25519::Public>("Bob");
        assert_eq!(
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
            id.to_string()
        );

        let id = account_id_from_seed::<sp_core::sr25519::Public>("Charlie");
        assert_eq!(
            "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y".to_string(),
            id.to_string()
        );
    }
}
