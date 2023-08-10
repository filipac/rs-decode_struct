use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct DecodedAdvertiseSpace {
    pub owner: String,
    pub paid_amount: u64,
    pub paid_until: u64,
    pub is_new: bool,
}
