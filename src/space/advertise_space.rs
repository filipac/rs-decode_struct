use multiversx_chain_vm::{bech32, DebugApi};
use multiversx_sc::{
    api::ManagedTypeApi,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{BigUint, ManagedAddress},
};
use multiversx_sc_codec as codec;

use super::decoded_advertise_space::DecodedAdvertiseSpace;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Debug)]
pub struct AdvertiseSpace<M: ManagedTypeApi> {
    pub owner: ManagedAddress<M>,
    pub paid_amount: BigUint<M>,
    pub paid_until: BigUint<M>,
    pub is_new: bool,
}

impl AdvertiseSpace<DebugApi> {
    pub fn to_decoded_struct(&self) -> DecodedAdvertiseSpace {
        DecodedAdvertiseSpace {
            owner: bech32::encode(&self.owner.to_address()),
            paid_amount: self.paid_amount.to_u64().unwrap(),
            paid_until: self.paid_until.to_u64().unwrap(),
            is_new: self.is_new,
        }
    }
}
