use codec::derive::{NestedDecode, NestedEncode};
use multiversx_chain_vm::DebugApi;
use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::codec;
use multiversx_sc::types::BigUint;
use multiversx_sc::{
    codec::derive::{TopDecode, TopEncode},
    types::EgldOrEsdtTokenIdentifier,
};

use super::decoded_sft_price::DecodedSftPrice;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Debug)]
pub struct SftPrice<M: ManagedTypeApi> {
    pub token: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>,
}

impl SftPrice<DebugApi> {
    pub fn to_decoded_struct(&self) -> DecodedSftPrice {
        let name = &self.token.clone().into_name();
        let token = name.to_boxed_bytes().into_vec();
        let token_name = String::from_utf8(token).unwrap();

        DecodedSftPrice {
            amount: self.amount.to_u64().unwrap(),
            nonce: self.nonce,
            token: token_name,
        }
    }
}
