use base64::{engine::general_purpose, Engine as _};
use codec::{DefaultErrorHandler, TopDecode};
use multiversx_chain_vm::DebugApi;
use multiversx_sc_codec as codec;
use serde_json::json;

use crate::sft_attributes::SftAttributes;

pub mod jsonable;
pub mod sft;
pub mod sft_attributes;
pub mod space;

const INVALID_BASE64: &str = "no base64 encoded string";
const MISSING_BASE64: &str = "missing base64 encoded string";

const SUPPORTED_DECODE_TYPES: [&str; 3] = ["advertise_space", "sft_price", "sft_attributes"];

fn main() {
    DebugApi::dummy();

    let decode_type = std::env::args().nth(1).expect("missing decode type");

    if !SUPPORTED_DECODE_TYPES.contains(&&*decode_type) {
        panic!("unsupported decode type");
    }

    let base64_encoded = std::env::args().nth(2).expect(MISSING_BASE64);
    let struct_bytes = general_purpose::STANDARD
        .decode(base64_encoded)
        .expect(INVALID_BASE64);

    if decode_type == "advertise_space" {
        crate::space::decode_advertise_space(struct_bytes);
    } else if decode_type == "sft_price" {
        crate::sft::decode_sft_price(struct_bytes);
    } else if decode_type == "sft_attributes" {
        let decoded: SftAttributes<DebugApi> =
            SftAttributes::top_decode_or_handle_err(struct_bytes, DefaultErrorHandler).unwrap();

        let bb = decoded.payment_token.into_name().to_boxed_bytes();
        let slice = bb.as_slice();

        let json = json!({
            "creation_timestamp": decoded.creation_timestamp,
            "price": decoded.price.to_u64().unwrap(),
            "payment_token": String::from_utf8_lossy(slice),
            "payment_token_nonce": decoded.payment_token_nonce,
        });
        print!("{}", &json.to_string());
    }
}
