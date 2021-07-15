#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use rutilities::{clean_rut, format_rut, generate_valid_rut, is_rut_valid};

#[wasm_bindgen_test]
fn pass() {
  let rut = generate_valid_rut();

  assert_eq!(is_rut_valid(&rut), true);
  let sample = "11_111_111-1";
  assert_eq!(clean_rut(sample), "111111111");
  assert_eq!(format_rut(sample), "11.111.111-1");
}
