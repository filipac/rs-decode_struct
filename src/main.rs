use base64::{engine::general_purpose, Engine as _};
use codec::{DefaultErrorHandler, TopDecode};
use multiversx_chain_vm::{bech32, DebugApi};
use multiversx_sc::{
    api::ManagedTypeApi,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{BigUint, ManagedAddress},
};
use multiversx_sc_codec as codec;
use serde::{Deserialize, Serialize};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Debug)]
struct AdvertiseSpace<M: ManagedTypeApi> {
    owner: ManagedAddress<M>,
    paid_amount: BigUint<M>,
    paid_until: BigUint<M>,
    is_new: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct DecodedAdvertiseSpace {
    owner: String,
    paid_amount: u64,
    paid_until: u64,
    is_new: bool,
}

fn main() {
    // first argument is base64 encoded string
    let base64_encoded = std::env::args().nth(1).expect("no base64 encoded string");
    // transform base64 encoded string to bytes
    let bytes = general_purpose::STANDARD
        .decode(base64_encoded)
        .expect("invalid base64");

    #[allow(unused_variables)]
    let api = DebugApi::dummy();

    // decode bytes to AdvertiseSpace struct
    let decoded: AdvertiseSpace<DebugApi> =
        AdvertiseSpace::top_decode_or_handle_err(bytes, DefaultErrorHandler).unwrap();

    let addr = bech32::encode(&decoded.owner.to_address());

    print!(
        "{}",
        serde_json::to_string(&DecodedAdvertiseSpace {
            owner: addr,
            paid_amount: decoded.paid_amount.to_u64().unwrap(),
            paid_until: decoded.paid_until.to_u64().unwrap(),
            is_new: decoded.is_new,
        })
        .unwrap()
    );
}
