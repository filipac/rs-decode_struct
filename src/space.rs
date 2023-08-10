use crate::{jsonable::Jsonable, space::advertise_space::AdvertiseSpace};
use codec::{DefaultErrorHandler, TopDecode};
use multiversx_chain_vm::DebugApi;
use multiversx_sc_codec as codec;

pub mod advertise_space;
pub mod decoded_advertise_space;

pub fn decode_advertise_space(struct_bytes: Vec<u8>) {
    let decoded: AdvertiseSpace<DebugApi> =
        AdvertiseSpace::top_decode_or_handle_err(struct_bytes, DefaultErrorHandler).unwrap();

    print!("{}", &decoded.to_decoded_struct().to_json());
}
