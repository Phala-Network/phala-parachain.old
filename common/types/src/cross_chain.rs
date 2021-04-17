use alloc::vec::Vec;
use codec::{Encode, Decode};
use xcm::v0::NetworkId;
use cumulus_primitives_core::ParaId;

use crate::SignedDataType;

#[derive(Encode, Decode)]
pub struct TransferToken<AccountId, Balance> {
    pub token_id: Vec<u8>,
    pub dest: AccountId,
    pub amount: Balance,
    pub sequence: u64,
}

#[derive(Encode, Decode)]
pub struct TransferTokenData<AccountId, Balance> {
    pub data: TransferToken<AccountId, Balance>,
    pub signature: Vec<u8>,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct TransferXToken<AccountId, Balance> {
    pub currency_id: Vec<u8>,
    pub para_id: ParaId,
    pub dest_network: NetworkId,
    pub dest: AccountId,
    pub amount: Balance,
    pub sequence: u64,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct TransferXTokenData<AccountId, Balance> {
    pub data: TransferXToken<AccountId, Balance>,
    pub signature: Vec<u8>,
}

impl<AccountId: Encode, Balance: Encode> SignedDataType<Vec<u8>> for TransferTokenData<AccountId, Balance> {
    fn raw_data(&self) -> Vec<u8> {
        Encode::encode(&self.data)
    }

    fn signature(&self) -> Vec<u8> {
        self.signature.clone()
    }
}

impl<AccountId: Encode, Balance: Encode> SignedDataType<Vec<u8>>
for TransferXTokenData<AccountId, Balance>
{
    fn raw_data(&self) -> Vec<u8> {
        Encode::encode(&self.data)
    }

    fn signature(&self) -> Vec<u8> {
        self.signature.clone()
    }
}
