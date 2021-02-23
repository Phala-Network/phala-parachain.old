// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use parachain_primitives::{
	AccountId, Signature, Balance
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
	)
}

pub fn get_chain_spec(id: ParaId) -> ChainSpec {
	let initial_authorities: Vec<(AccountId, AccountId)> = vec![
		authority_keys_from_seed("Alice"),
		authority_keys_from_seed("Bob"),
		authority_keys_from_seed("Charlie"),
		authority_keys_from_seed("Dave"),
		authority_keys_from_seed("Eve"),
		authority_keys_from_seed("Ferdie"),
	];

	let endowed_accounts: Vec<AccountId> = vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
		get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
	];

	let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

	ChainSpec::from_genesis(
		"Phala Local Testnet",
		"phala_local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				root_key.clone(),
				initial_authorities.clone(),
				endowed_accounts.clone(),
				id,
			)
		},
		vec![],
		None,
		None,
		testnet_properties(),
		Extensions {
			relay_chain: "westend-dev".into(),
			para_id: id.into(),
		},
	)
}

pub fn staging_test_net(id: ParaId) -> ChainSpec {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done
	let initial_authorities: Vec<(AccountId, AccountId)> = vec![(
		 // 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
		 hex!["9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"].into(),
		 // 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
		 hex!["781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"].into(),
	 ),(
		 // 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
		 hex!["68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"].into(),
		 // 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
		 hex!["c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"].into(),
	 ),(
		 // 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
		 hex!["547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"].into(),
		 // 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
		 hex!["9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"].into(),
	 ),(
		 // 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
		 hex!["f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"].into(),
		 // 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
		 hex!["66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"].into(),
	 )];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
		"ce4f5981f1b33b5371454c753ac6f3a703e57e1fe20c7b26a1f9bda024008221"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	ChainSpec::from_genesis(
		"Phala Testnet",
		"phala_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				root_key.clone(),
				initial_authorities.clone(),
				endowed_accounts.clone(),
				id,
			)
		},
		Vec::new(),
		None,
		None,
		testnet_properties(),
		Extensions {
			relay_chain: "westend-dev".into(),
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<(
		AccountId,
		AccountId,
	)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_runtime::GenesisConfig {
	// The pubkey of "0x1"
	let dev_ecdsa_pubkey: Vec<u8> = hex!["0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"].to_vec();

	parachain_runtime::GenesisConfig {
		frame_system: Some(parachain_runtime::SystemConfig {
			code: parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(parachain_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		}),
		pallet_sudo: Some(parachain_runtime::SudoConfig { key: root_key }),
		parachain_info: Some(parachain_runtime::ParachainInfoConfig { parachain_id: id }),
		pallet_collective_Instance1: Some(parachain_runtime::CouncilConfig::default()),
		pallet_treasury: Some(Default::default()),
		pallet_phala: Some(parachain_runtime::PhalaModuleConfig {
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), dev_ecdsa_pubkey.clone())
			}).collect(),
			// Now we have 4 contracts but reserve 10 for convenience
			contract_keys: std::iter::repeat(dev_ecdsa_pubkey).take(10).collect(),
		}),
	}
}

fn testnet_properties() -> Option<Properties> {
	let mut p = Properties::new();

	p.insert("tokenSymbol".into(), "PHA".into());
	p.insert("tokenDecimals".into(), 12.into());
	p.insert("ss58Format".into(), 30.into());

	Some(p)
}
