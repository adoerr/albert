use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use albert_runtime::{AccountId, GenesisConfig, Signature};

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

type AccountPublic = <Signature as Verify>::Signer;

pub fn pub_key_from_seed<T: Public>(seed: &str) -> <T::Pair as Pair>::Public {
    T::Pair::from_string(&format!("//{}", seed), None)
        .expect("secret string error")
        .public()
}

pub fn account_id_from_seed<T: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<T::Pair as Pair>::Public>,
{
    AccountPublic::from(pub_key_from_seed::<T>(seed)).into_account()
}

pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (
        pub_key_from_seed::<AuraId>(s),
        pub_key_from_seed::<GrandpaId>(s),
    )
}

#[cfg(test)]
mod tests {
    use super::{account_id_from_seed, authority_keys_from_seed, pub_key_from_seed};

    #[test]
    fn test_pub_key_from_seed() {
        let pk = pub_key_from_seed::<sp_core::sr25519::Public>("Alice");
        assert_eq!(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
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
    }

    #[test]
    fn test_authority_keys_from_seed() {
        let ids = authority_keys_from_seed("Alice");
        assert_eq!(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            ids.0.to_string()
        );
        assert_eq!(
            "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu".to_string(),
            ids.1.to_string()
        );
    }
}
