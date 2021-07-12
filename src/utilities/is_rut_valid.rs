use super::{clean_rut, get_verification_digit};

pub fn is_rut_valid(rut: &str) -> bool {
  let mut clean_document_number = clean_rut(rut);
  let given_verification_digit = match clean_document_number.pop() {
    Some(vd) => vd,
    None => return false,
  };

  let expected_verification_digit = get_verification_digit(&clean_document_number);

  return given_verification_digit == expected_verification_digit;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_returns_false_for_invalid_rut() {
    assert_eq!(is_rut_valid("1234"), false);
  }

  #[test]
  fn it_returns_false_for_empty_rut() {
    assert_eq!(is_rut_valid(""), false);
  }

  #[test]
  fn it_returns_true_for_valid_rut() {
    assert_eq!(is_rut_valid("8756612-9"), true);
  }
}
