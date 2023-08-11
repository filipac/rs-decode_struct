use multiversx_sc::types::{BigUint, EgldOrEsdtTokenIdentifier};
use multiversx_sc::{api::ManagedTypeApi, derive::TypeAbi};
use multiversx_sc_codec as codec;
use multiversx_sc_codec::derive::{TopDecode, TopEncode};

#[derive(TypeAbi, TopEncode, TopDecode, Debug)]
pub struct SftAttributes<M: ManagedTypeApi> {
    pub creation_timestamp: u64,
    pub price: BigUint<M>,
    pub payment_token: EgldOrEsdtTokenIdentifier<M>,
    pub payment_token_nonce: u64,
}
