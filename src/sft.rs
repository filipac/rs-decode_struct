use crate::jsonable::Jsonable;
use crate::sft::sft_price::SftPrice;
use codec::{DefaultErrorHandler, TopDecode};
use multiversx_chain_vm::DebugApi;
use multiversx_sc_codec as codec;

pub mod decoded_sft_price;
pub mod sft_price;

pub fn decode_sft_price(struct_bytes: Vec<u8>) {
    let decoded: SftPrice<DebugApi> =
        SftPrice::top_decode_or_handle_err(struct_bytes, DefaultErrorHandler).unwrap();

    print!("{}", &decoded.to_decoded_struct().to_json());
}
