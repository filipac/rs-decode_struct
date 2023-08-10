use base64::{engine::general_purpose, Engine as _};
use multiversx_chain_vm::DebugApi;

pub mod jsonable;
pub mod sft;
pub mod space;

const INVALID_BASE64: &str = "no base64 encoded string";
const MISSING_BASE64: &str = "missing base64 encoded string";

const SUPPORTED_DECODE_TYPES: [&str; 2] = ["advertise_space", "sft_price"];

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
    }
}
