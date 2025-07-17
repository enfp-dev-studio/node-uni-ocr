#![deny(clippy::all)]

use napi_derive::napi;
use image::DynamicImage;
use napi::bindgen_prelude::*;
use napi::{Error, Result};
use std::time::Duration;
use uni_ocr::{Language, OcrEngine, OcrOptions, OcrProvider};

#[napi(object)]
pub struct UniOcrOptions {
  pub languages: Vec<String>,
  pub confidence_threshold: i32,
  pub timeout: i64,
}

#[napi(object)]
pub struct RecognizeResult {
  pub text: String,
  pub confidence: String,
}

fn from_language_code(code: &str) -> Option<Language> {
  match code.to_lowercase().as_str() {
    "en" => Some(Language::English),
    "zh" => Some(Language::Chinese),
    "de" => Some(Language::German),
    "es" => Some(Language::Spanish),
    "ru" => Some(Language::Russian),
    "ko" => Some(Language::Korean),
    "fr" => Some(Language::French),
    "ja" => Some(Language::Japanese),
    "pt" => Some(Language::Portuguese),
    "tr" => Some(Language::Turkish),
    _ => Some(Language::English),
  }
}

#[napi]
pub async fn recognize(
  image: Either<String, Buffer>,
  options: Option<UniOcrOptions>,
) -> Result<RecognizeResult> {
  // OCR 옵션 구성
  let mut ocr_options = OcrOptions::default();

  if let Some(opts) = options {
    let langs = opts
      .languages
      .iter()
      .filter_map(|code| from_language_code(code))
      .collect::<Vec<_>>();

    ocr_options = ocr_options
      .languages(langs)
      .confidence_threshold(opts.confidence_threshold as f32)
      .timeout(Duration::from_secs(opts.timeout as u64));
  }

  // 이미지 로딩 (DynamicImage)
  let dyn_image: DynamicImage = match image {
    Either::A(path) => image::open(path)
      .map_err(|e| Error::from_reason(format!("Failed to open image file: {:?}", e)))?,
    Either::B(buffer) => image::load_from_memory(&buffer)
      .map_err(|e| Error::from_reason(format!("Failed to decode image buffer: {:?}", e)))?,
  };

  let engine = OcrEngine::new(OcrProvider::Auto)
    .map_err(|e| Error::from_reason(format!("Failed to create OCR engine: {:?}", e)))?
    .with_options(ocr_options);

  // OCR 엔진 초기화
  // let engine = OcrEngine::new(OcrProvider::Auto)?.with_options(ocr_options);
  // `?` couldn't convert the error to `napi::Error`
  // the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
  // the following other types implement trait `From<T>`:
  //   `napi::Error` implements `From<JsUnknown>`
  //   `napi::Error` implements `From<NulError>`
  //   `napi::Error` implements `From<std::io::Error>`rustcClick for full compiler diagnostic
  // lib.rs(66, 16): this can't be annotated with `?` because it has type `Result<_, anyhow::Error>`

  // 인식 실행
  match engine.recognize_image(&dyn_image).await {
    Ok(result) => Ok(RecognizeResult {
      text: result.0,
      confidence: result.1,
    }),
    Err(e) => Err(Error::from_reason(format!("Recognition failed: {:?}", e))),
  }
}