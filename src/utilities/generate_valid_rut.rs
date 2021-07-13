use super::get_verification_digit;
use rand::Rng;

pub fn generate_valid_rut() -> String {
  let mut rng = rand::thread_rng();
  let body: u32 = rng.gen_range(1_000_000..100_000_000);
  let mut body = body.to_string();
  let verification_digit = get_verification_digit(&body);

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
