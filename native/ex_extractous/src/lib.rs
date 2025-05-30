use extractous::{Extractor, TesseractOcrConfig, PdfParserConfig, PdfOcrStrategy};
use rustler::NifResult;

#[rustler::nif(schedule = "DirtyCpu")]
pub fn extract(file_path: &str, as_xml: bool) -> NifResult<(String, String)> {
  let extractor = Extractor::new()
    .set_xml_output(as_xml);

  extract_from_file(extractor, file_path)
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn extract_with_ocr(file_path: &str, as_xml: bool) -> NifResult<(String, String)> {
  let extractor = Extractor::new()
    .set_xml_output(as_xml)
    .set_ocr_config(TesseractOcrConfig::new())
    .set_pdf_config(PdfParserConfig::new().set_ocr_strategy(PdfOcrStrategy::OCR_AND_TEXT_EXTRACTION));

  extract_from_file(extractor, file_path)
}

fn extract_from_file(extractor: Extractor, file_path: &str) -> NifResult<(String, String)> {
  match extractor.extract_file_to_string(&file_path) {
    Ok((content, metadata)) => Ok((content, format!("{:?}", metadata))),
    Err(e) => Err(rustler::Error::Term(Box::new(format!("Extraction failed: {}", e))))
  }
}


rustler::init!("Elixir.ExExtractous");
