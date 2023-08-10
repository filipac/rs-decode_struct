use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct DecodedSftPrice {
    pub token: String,
    pub nonce: u64,
    pub amount: u64,
}
