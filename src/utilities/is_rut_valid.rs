use super::get_verification_digit;

/// Check if a given rut is valid
/// ```rust
/// use rutilities::{is_rut_valid};
///
/// fn main() {
///   let valid_rut = "11.111.111-1";
///   assert_eq!(is_rut_valid(&valid_rut), true);
///
///   let invalid_rut = "12.345.678-9";
///   assert_eq!(is_rut_valid(&invalid_rut), false);
/// }
pub fn is_rut_valid(rut: &str) -> bool {
  let mut unformatted_document_number = rut.replace(".", "").replace("-", "");
  let given_verification_digit = match unformatted_document_number.pop() {
    Some(vd) => vd,
    None => return false,
  };

  let expected_verification_digit = match get_verification_digit(&unformatted_document_number) {
    Ok(vd) => vd,
    Err(_) => return false,
  };

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
  fn it_returns_false_for_dirty_rut() {
    assert_eq!(is_rut_valid("muddy87566129"), false);
  }

  #[test]
  fn it_returns_true_for_valid_rut() {
    assert_eq!(is_rut_valid("8756612-9"), true);
  }
}
