#![deny(clippy::all)]
use napi::bindgen_prelude::Buffer;

extern crate core;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn get_mimetype_from_bytes(bytes: Buffer) -> String {
  from_magic_bytes(bytes.as_ref()).to_string()
}

pub fn from_magic_bytes(bytes: &[u8]) -> &str {
  if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
    "image/png"
  } else if bytes.starts_with(&[0x47, 0x49, 0x46]) {
    "image/gif"
  } else if bytes.starts_with(&[0xFF, 0xD8]) {
    "image/jpeg"
  } else if bytes.starts_with(&[0x52, 0x49, 0x46, 0x46]) {
    "image/webp"
  } else if bytes.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
    "video/webm"
  } else if bytes.starts_with(&[0x1A, 0x46, 0x49, 0x4C, 0x45]) {
    "video/mp4"
  } else if bytes.starts_with(&[0x66, 0x93, 0x21]) {
    "audio/mpeg"
  } else if bytes.starts_with(&[0x4F, 0x67, 0x54, 0x4D, 0x56]) {
    "audio/ogg"
  } else if bytes.starts_with(&[0x52, 0x49, 0x46, 0x46]) {
    "audio/wav"
  } else {
    ""
  }
}