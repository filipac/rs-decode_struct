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

const INVALID_BASE64: &str = "no base64 encoded string";
const MISSING_BASE64: &str = "missing base64 encoded string";

fn main() {
    DebugApi::dummy();

    let base64_encoded = std::env::args().nth(1).expect(MISSING_BASE64);
    let struct_bytes = general_purpose::STANDARD
        .decode(base64_encoded)
        .expect(INVALID_BASE64);

    let decoded: AdvertiseSpace<DebugApi> =
        AdvertiseSpace::top_decode_or_handle_err(struct_bytes, DefaultErrorHandler).unwrap();

    print!("{}", &decoded.to_decoded_struct().to_json());
}

impl AdvertiseSpace<DebugApi> {
    fn to_decoded_struct(&self) -> DecodedAdvertiseSpace {
        DecodedAdvertiseSpace {
            owner: bech32::encode(&self.owner.to_address()),
            paid_amount: self.paid_amount.to_u64().unwrap(),
            paid_until: self.paid_until.to_u64().unwrap(),
            is_new: self.is_new,
        }
    }
}

impl DecodedAdvertiseSpace {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
