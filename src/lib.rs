mod core;

use candid::{CandidType, Deserialize};
use std::include_bytes;

const IMAGE_SIZE_IN_PIXELS: usize = 1024;
const LOGO_TRANSPARENT: &[u8] = include_bytes!("assets/logo_transparent.png");
const LOGO_WHITE: &[u8] = include_bytes!("assets/logo_white.png");

#[derive(CandidType, Deserialize)]
struct QROptions {
    pub add_logo: bool,
    pub add_gradient: bool,
    pub add_transparency: Option<bool>,
}

#[derive(CandidType, Deserialize)]
struct QrError {
    message: String,
}

#[derive(CandidType, Deserialize)]
enum QrResult {
    Image(Vec<u8>),
    Err(QrError),
}

fn qrcode_impl(input: String, options: QROptions) -> QrResult {
    let logo = if options.add_transparency == Some(true) {
        LOGO_TRANSPARENT
    } else {
        LOGO_WHITE
    };
    let res = match core::generate(input, options, logo, IMAGE_SIZE_IN_PIXELS) {
        Ok(blob) => QrResult::Image(blob),
        Err(err) => QrResult::Err(QrError {
            message: err.to_string()
        }),
    };
    ic_cdk::println!(
        "Executed instructions:\n{:?}", ic_cdk::api::performance_counter(0)
    );
    res
}

#[ic_cdk::update]
fn qrcode_update(input: String, options: QROptions) -> QrResult {
    qrcode_impl(input, options)
}

#[ic_cdk::query]
fn qrcode_query(input: String, qroptions: QROptions) -> QrResult {
    qrcode_impl(input, qroptions)
}