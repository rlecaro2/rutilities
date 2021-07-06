use super::clean_rut;

pub fn format_rut(rut: &str) -> String {
  let clean_rut = clean_rut(rut);
  let verification_digit = match clean_rut.chars().last().clone() {
    Some(vd) => vd,
    None => return "".to_string(),
  };
  let mut reverse_formatted = String::from(format!("{}-", verification_digit));

  let mut counter = 0;
  for digit in clean_rut.chars().rev().skip(1) {
    if counter == 3 {
      reverse_formatted.push('.');
      counter -= 3;
    }

    reverse_formatted.push(digit);
    counter += 1;
  }

  return reverse_formatted.chars().rev().collect();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_formats_an_8_digits_case() {
    assert_eq!(format_rut("11779301k"), "11.779.301-K");
  }

  #[test]
  fn it_formats_a_7_digits_case() {
    assert_eq!(format_rut("1779301k"), "1.779.301-K");
  }

  #[test]
  fn it_formats_a_6_digits_case() {
    assert_eq!(format_rut("7793016"), "779.301-6");
  }

  #[test]
  fn it_formats_a_dirty_case() {
    assert_eq!(format_rut("  1177dirty93 01  k "), "11.779.301-K");
  }

  #[test]
  fn it_returns_empty_string_when_empty() {
    assert_eq!(format_rut(""), "");
  }
}
