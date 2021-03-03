#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use sp_std::convert::TryFrom;
use codec::{Encode, Decode};
use xcm::v0::{Junction, MultiAsset, MultiLocation};

#[derive(Debug)]
pub enum Error {
	UnsupportedAssetId,
	FailedToParseAssetId,
	NotMultiLocationX3,
}

#[derive(Debug)]
pub enum AssetLocation {
	Native,
	RelayChainNative,
	Parachain(u32),
}

impl TryFrom<&MultiLocation> for AssetLocation {
	type Error = Error;

	fn try_from(id: &MultiLocation) -> Result<Self, Self::Error> {
		// id must be: MultiLocation::X3(Parent, Parachain(id), GenericToken(?))
		match id {
			MultiLocation::X3(Junction::Parent, Junction::Parachain { id }, _) =>
				Ok(AssetLocation::Parachain(*id)),
			MultiLocation::X1(Junction::Parent) =>
				Ok(AssetLocation::RelayChainNative),
			_ => Err(Error::UnsupportedAssetId)
		}
	}
}

pub fn try_extract_asset_location(asset: &MultiAsset) -> Result<AssetLocation, Error> {
	match asset {
		MultiAsset::ConcreteFungible { id, .. } => {
			AssetLocation::try_from(id)
		},
		_ => Err(Error::NotMultiLocationX3)
	}
}

pub fn to_encoded_asset_id(asset: &MultiAsset) -> Result<Vec<u8>, Error> {
	match asset {
		MultiAsset::ConcreteFungible { id, .. } => Ok(id.encode()),
		_ => Err(Error::UnsupportedAssetId)
	}
}

pub fn from_encoded_asset_id(id: &Vec<u8>) -> Result<MultiLocation, Error> {
	let path = MultiLocation::decode(&mut &id[..]).map_err(|_| Error::FailedToParseAssetId)?;
	Ok(path)
}
