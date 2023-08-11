use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;

use crate::sft::sft_price::SftPrice;
mod sft_attributes;
use base64::engine::general_purpose;
use base64::Engine;
use codec::{DefaultErrorHandler, TopDecode};
use multiversx_chain_vm::DebugApi;
use multiversx_sc_codec as codec;
use sft_attributes::SftAttributes;
use space::advertise_space::AdvertiseSpace;
mod jsonable;
mod sft;
mod space;

const INVALID_BASE64: &str = "no base64 encoded string";

#[repr(C)]
pub struct SftPriceRet {
    pub amount: u64,
    pub nonce: u64,
    pub token: *const c_char,
}

#[repr(C)]
pub struct AdvertiseSpaceRet {
    pub owner: *const c_char,
    pub paid_amount: u64,
    pub paid_until: u64,
    pub is_new: bool,
}

#[repr(C)]
pub struct SftResult {
    pub error: bool,
    pub error_message: *const c_char,
    pub item: *const SftPriceRet,
}

#[repr(C)]
pub struct SpaceResult {
    pub error: bool,
    pub error_message: *const c_char,
    pub item: *const AdvertiseSpaceRet,
}

#[repr(C)]
pub struct SftAttributesRet {
    pub creation_timestamp: u64,
    pub price: u64,
    pub payment_token: *const c_char,
    pub payment_token_nonce: u64,
}

#[repr(C)]
pub struct SftAttributesResult {
    pub error: bool,
    pub error_message: *const c_char,
    pub item: *const SftAttributesRet,
}

// function to be called from PHP FFI, return pointer, must be a zero terminated array of C chars.
#[no_mangle]
pub unsafe extern "C" fn decode_sft_price(base64string: *const c_char) -> *mut SftResult {
    DebugApi::dummy();
    let base64_encoded = CStr::from_ptr(base64string).to_str().unwrap();
    let struct_bytes_result = general_purpose::STANDARD.decode(base64_encoded);

    if struct_bytes_result.is_err() {
        // throw php exception
        let item = SftResult {
            error: true,
            error_message: CString::new("invalid base64").unwrap().into_raw(),
            item: std::ptr::null_mut(),
        };
        return Box::into_raw(Box::new(item));
    }

    let struct_bytes = struct_bytes_result.unwrap();

    let decoded = SftPrice::top_decode_or_handle_err(struct_bytes, DefaultErrorHandler);

    if decoded.is_err() {
        let item = SftResult {
            error: true,
            error_message: CString::new("invalid base64").unwrap().into_raw(),
            item: std::ptr::null_mut(),
        };
        return Box::into_raw(Box::new(item));
    }

    let decoded: SftPrice<DebugApi> = decoded.unwrap();

    let decoded_struct = decoded.to_decoded_struct();

    let token = CString::new(decoded_struct.token).unwrap();

    let sft_price_ret = SftPriceRet {
        amount: decoded_struct.amount,
        nonce: decoded_struct.nonce,
        token: token.into_raw(),
    };

    let item = SftResult {
        error: false,
        error_message: std::ptr::null_mut(),
        item: Box::into_raw(Box::new(sft_price_ret)),
    };

    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub unsafe extern "C" fn decode_advertise_space(base64string: *const c_char) -> *mut SpaceResult {
    DebugApi::dummy();
    let base64_encoded = CStr::from_ptr(base64string).to_str().unwrap();
    let struct_bytes_result = general_purpose::STANDARD.decode(base64_encoded);

    if struct_bytes_result.is_err() {
        // throw php exception
        let item = SpaceResult {
            error: true,
            error_message: CString::new("invalid base64").unwrap().into_raw(),
            item: std::ptr::null_mut(),
        };
        return Box::into_raw(Box::new(item));
    }

    let struct_bytes = struct_bytes_result.unwrap();

    let decoded = AdvertiseSpace::top_decode_or_handle_err(struct_bytes, DefaultErrorHandler);

    if decoded.is_err() {
        let item = SpaceResult {
            error: true,
            error_message: CString::new("invalid base64").unwrap().into_raw(),
            item: std::ptr::null_mut(),
        };
        return Box::into_raw(Box::new(item));
    }

    let decoded: AdvertiseSpace<DebugApi> = decoded.unwrap();

    let decoded_struct = decoded.to_decoded_struct();

    let advertise_space_ret = AdvertiseSpaceRet {
        owner: CString::new(decoded_struct.owner).unwrap().into_raw(),
        paid_amount: decoded_struct.paid_amount,
        paid_until: decoded_struct.paid_until,
        is_new: decoded_struct.is_new,
    };

    let item = SpaceResult {
        error: false,
        error_message: std::ptr::null_mut(),
        item: Box::into_raw(Box::new(advertise_space_ret)),
    };

    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub unsafe extern "C" fn decode_sft_attributes(
    base64string: *const c_char,
) -> *mut SftAttributesResult {
    DebugApi::dummy();
    let base64_encoded = CStr::from_ptr(base64string).to_str().unwrap();
    let struct_bytes_result = general_purpose::STANDARD.decode(base64_encoded);

    if struct_bytes_result.is_err() {
        // throw php exception
        let item = SftAttributesResult {
            error: true,
            error_message: CString::new("invalid base64").unwrap().into_raw(),
            item: std::ptr::null_mut(),
        };
        return Box::into_raw(Box::new(item));
    }

    let struct_bytes = struct_bytes_result.unwrap();

    let decoded = SftAttributes::top_decode_or_handle_err(struct_bytes, DefaultErrorHandler);

    if decoded.is_err() {
        let item = SftAttributesResult {
            error: true,
            error_message: CString::new("invalid base64").unwrap().into_raw(),
            item: std::ptr::null_mut(),
        };
        return Box::into_raw(Box::new(item));
    }

    let decoded: SftAttributes<DebugApi> = decoded.unwrap();

    let bb = decoded.payment_token.into_name().to_boxed_bytes();
    let slice = bb.as_slice();

    let ret = SftAttributesRet {
        creation_timestamp: decoded.creation_timestamp,
        price: decoded.price.to_u64().unwrap(),
        payment_token: CString::new(String::from_utf8_lossy(slice).to_string())
            .unwrap()
            .into_raw(),
        payment_token_nonce: decoded.payment_token_nonce,
    };

    let item = SftAttributesResult {
        error: false,
        error_message: std::ptr::null_mut(),
        item: Box::into_raw(Box::new(ret)),
    };

    Box::into_raw(Box::new(item))
}
