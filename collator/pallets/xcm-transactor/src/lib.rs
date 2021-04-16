#![feature(associated_type_defaults)]

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage,
    traits::{Currency, ExistenceRequirement, Get, WithdrawReasons},
};

use sp_runtime::{
    traits::{CheckedConversion, SaturatedConversion},
    RuntimeDebug,
};
use sp_std::{
    convert::{TryInto, TryFrom},
    marker::PhantomData,
    prelude::*,
    result,
};

use codec::{Decode, Encode};

use cumulus_primitives_core::ParaId;
use xcm::v0::{Error, Junction, MultiAsset, MultiLocation, Result};
use xcm_executor::traits::{
    FilterAssetLocation, MatchesFungible, TransactAsset,
};
use xcm_builder::{ParentIsDefault, SiblingParachainConvertsVia, AccountId32Aliases, NativeAsset};

use parachain_utils as utils;

#[derive(Encode, Decode, Eq, PartialEq, Clone, Copy, RuntimeDebug)]
/// Identity of chain.
pub enum ChainId {
    /// The relay chain.
    RelayChain,
    /// A parachain.
    ParaChain(ParaId),
}

pub type AccountId = <T as frame_system::Config>::AccountId;

type BalanceOf<T> =
    <<T as Config>::OwnedCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Configuration trait of this pallet.
pub trait Config: frame_system::Config {
    type AccountId = <T as frame_system::Config>::AccountId;

    /// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
    /// when determining ownership of accounts for asset transacting and when attempting to use XCM
    /// `Transact` in order to determine the dispatch Origin.
    type LocationToAccountId = (
        // The parent (Relay-chain) origin converts to the default `AccountId`.
        ParentIsDefault<AccountId>,
        // Sibling parachain origins convert to AccountId via the `ParaId::into`.
        SiblingParachainConvertsVia<Sibling, AccountId>,
        // Straight up local `AccountId32` origins just alias directly to `AccountId`.
        AccountId32Aliases<RococoNetwork, AccountId>,
    );

    /// Event type used by the runtime.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Matcher: MatchesFungible<BalanceOf<Self>>;
    type AccountIdConverter: LocationConversion<Self::AccountId>;
    type OwnedCurrency: Currency<Self::AccountId>;
    type ParaId: Get<ParaId>;
}

decl_storage! {
    trait Store for Module<T: Config> as PhalaXCMAdapter {}
}

decl_event! (
    pub enum Event<T> where
        <T as frame_system::Config>::AccountId,
        Balance = BalanceOf<T>,
    {
        /// Deposit asset into current chain. [currency_id, account_id, amount, to_tee]
        DepositAsset(Vec<u8>, AccountId, Balance, bool),

        /// Withdraw asset from current chain. [currency_id, account_id, amount, to_tee]
        WithdrawAsset(Vec<u8>, AccountId, Balance, bool),
    }
);

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;
    }
}

impl<T> TransactAsset for Module<T>
where
    T: Config
{
    fn deposit_asset(asset: &MultiAsset, location: &MultiLocation) -> Result {
        log::info!("----------------------- xcm-transactor: trying deposit -------------------------");
        log::info!(
            ">>> asset: {:?}, location: {:?}",
            asset,
            location
        );

		let who = T::AccountIdConverter::from_location(location).ok_or(())?;
		log::info!("who: {:?}", who);
		let currency_id = utils::to_encoded_asset_id(asset).or(Err(Error::FailedToDecode))?;
		log::info!("currency_id: {:?}", currency_id);
		let amount: BalanceOf<T> = T::Matcher::matches_fungible(&asset)
			.ok_or(())?
			.saturated_into();
		log::info!("amount: {:?}", amount);
		let balance_amount = amount.try_into().map_err(|_| ())?;
		log::info!("balance amount: {:?}", balance_amount);

		let mut is_owned_currency = false;
		if let Ok(utils::AssetLocation::Parachain(paraid)) = utils::try_extract_asset_location(asset) {
			if T::ParaId::get() == paraid.into() {
				is_owned_currency = true;
				let _ = T::OwnedCurrency::deposit_creating(&who, balance_amount);
			}
		}

		Self::deposit_event(Event::<T>::DepositAsset(
			currency_id.clone().into(),
			who,
			balance_amount,
			!is_owned_currency,
		));
		log::info!("------------------ xcm-transactor: success deposit ------------------------------");

        Ok(())
    }

    fn withdraw_asset(
		asset: &MultiAsset,
		location: &MultiLocation,
    ) -> result::Result<MultiAsset, Error> {
        log::info!("--------------------- xdm-adapter: trying withdraw ---------------------------");
        log::info!(
            ">>> asset: {:?}, location: {:?}",
            asset,
            location
        );

		let who = T::AccountIdConverter::from_location(location).ok_or(())?;
		log::info!("who: {:?}", who);
		let currency_id = utils::to_encoded_asset_id(asset).or(Err(Error::FailedToDecode))?;
		log::info!("currency_id: {:?}", currency_id);
		let amount: BalanceOf<T> = T::Matcher::matches_fungible(asset)
			.ok_or(())?
			.saturated_into();
		log::info!("amount: {:?}", amount);
		let balance_amount = amount.try_into().map_err(|_| ())?;
		log::info!("balance amount: {:?}", balance_amount);

		let mut is_owned_currency = false;
		if let Ok(utils::AssetLocation::Parachain(paraid)) = utils::try_extract_asset_location(asset) {
			if T::ParaId::get() == paraid.into() {
				is_owned_currency = true;
				let _ = T::OwnedCurrency::withdraw(
					&who,
					balance_amount,
					WithdrawReasons::TRANSFER,
					ExistenceRequirement::AllowDeath,
				)
					.map_err(|_| Error::UnhandledEffect)?;
			}
		}

		Self::deposit_event(Event::<T>::WithdrawAsset(
			currency_id.clone().into(),
			who,
			balance_amount,
			!is_owned_currency,
		));

        log::info!("--------------------- xcm-transactor: success withdraw ---------------------------");
        Ok(asset.clone())
    }
}

pub struct ConcreteMatcher<T>(PhantomData<T>);
impl<T: Get<MultiLocation>, B: TryFrom<u128>> MatchesFungible<B> for ConcreteMatcher<T> {
    fn matches_fungible(a: &MultiAsset) -> Option<B> {
        if let MultiAsset::ConcreteFungible { id, amount } = a {
            if id == &T::get() {
                return CheckedConversion::checked_from(*amount);
            } else if let Some(Junction::GeneralKey(_key)) = id.last() {
                return CheckedConversion::checked_from(*amount);
            } else {
                return None;
            }
        }
        None
    }
}

pub struct AnyLocationFilter;
impl FilterAssetLocation for AnyLocationFilter
{
    fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
        log::info!("filter_asset_location(asset: {:?}, origin: {:?})", asset, origin);
        if NativeAsset::filter_asset_location(asset, origin) {
			log::info!("return true because the asset is native");
            return true;
        }
		log::info!("return true because we accept everything");
        true
    }
}
