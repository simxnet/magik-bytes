#![deny(clippy::all)]
use napi::bindgen_prelude::Buffer;

extern crate core;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn get_mimetype_from_bytes(bytes: Buffer) -> String {
  FileType::from_magic_bytes(bytes.as_ref()).to_mimetype()
}

#[derive(Debug, PartialEq)]
pub enum FileType {
  Png,
  Gif,
  Jpeg,
  Webp,
  Webm,
  Mp4,
  Mp3,
  Ogg,
  Wav,
  Unknown
}

impl FileType {
  pub fn to_mimetype(&self) -> String {
    match self {
      FileType::Png => "image/png".to_string(),
      FileType::Gif => "image/gif".to_string(),
      FileType::Jpeg => "image/jpeg".to_string(),
      FileType::Webp => "image/webp".to_string(),
      FileType::Webm => "video/webm".to_string(),
      FileType::Mp4 => "video/mp4".to_string(),
      FileType::Mp3 => "audio/mpeg".to_string(),
      FileType::Ogg => "audio/ogg".to_string(),
      FileType::Wav => "audio/wav".to_string(),
      FileType::Unknown => "unknown".to_string()
    }
  }

  pub fn from_magic_bytes(bytes: &[u8]) -> Self {
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
      FileType::Png
    } else if bytes.starts_with(&[0x47, 0x49, 0x46]) {
      FileType::Gif
    } else if bytes.starts_with(&[0xFF, 0xD8]) {
      FileType::Jpeg
    } else if bytes.starts_with(&[0x52, 0x49, 0x46, 0x46]) {
      FileType::Webp
    } else if bytes.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
      FileType::Webm
    } else if bytes.starts_with(&[0x1A, 0x46, 0x49, 0x4C, 0x45]) {
      FileType::Mp4
    } else if bytes.starts_with(&[0x66, 0x93, 0x21]) {
      FileType::Mp3
    } else if bytes.starts_with(&[0x4F, 0x67, 0x54, 0x4D, 0x56]) {
      FileType::Ogg
    } else if bytes.starts_with(&[0x52, 0x49, 0x46, 0x46]) {
      FileType::Wav
    } else {
      FileType::Unknown
    }
  }
}