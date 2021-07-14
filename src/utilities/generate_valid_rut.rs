use super::get_verification_digit;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

/// Generate a valid, clean rut.
/// Useful for tests.
/// ```rust
/// use rutilities::{generate_valid_rut, is_rut_valid, clean_rut};
///
/// fn main() {
///   let generated = generate_valid_rut();
///
///   assert_eq!(is_rut_valid(&generated), true);
///   assert_eq!(clean_rut(&generated), generated);
/// }
#[wasm_bindgen(js_name = generateValidRut)]
pub fn generate_valid_rut() -> String {
  let mut rng = rand::thread_rng();
  let body: u32 = rng.gen_range(1_000_000..100_000_000);
  let mut body = body.to_string();
  let verification_digit = get_verification_digit(&body).expect("Generated invalid rut body");

  body.push(verification_digit);
  return body;
}

#[cfg(test)]
mod tests {
  use super::super::{clean_rut, is_rut_valid};
  use super::*;

  #[test]
  fn it_returns_a_clean_valid_rut() {
    let generated = generate_valid_rut();

    assert_eq!(is_rut_valid(&generated), true);
    assert_eq!(clean_rut(&generated), generated);
  }
}
