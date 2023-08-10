use serde_json;
pub trait Jsonable {
    fn to_json(&self) -> String;
}

impl Jsonable for crate::space::decoded_advertise_space::DecodedAdvertiseSpace {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Jsonable for crate::sft::decoded_sft_price::DecodedSftPrice {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
