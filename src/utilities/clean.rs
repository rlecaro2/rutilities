use regex::Regex;

pub fn clean_rut(doc: &str) -> String {
  let mut rut = String::from(doc);
  rut = rut.replace("k", "K");

  let leading_zeroes = Regex::new(r"^0+").expect("invalid leading_zeroes regex");
  rut = leading_zeroes.replace(&rut, "").to_owned().to_string();

  let non_k_letters = Regex::new(r"[^\dK]").expect("invalid non_k_letters regex");
  rut = non_k_letters.replace_all(&rut, "").to_owned().to_string();

  return rut;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_cleans_a_real_case() {
    assert_eq!(clean_rut("11.779.301-k"), "11779301K");
  }

  #[test]
  fn it_removes_periods_and_slashes() {
    assert_eq!(clean_rut("12.345.678-9"), "123456789");
  }

  #[test]
  fn it_ignores_unwanted_characters_order() {
    assert_eq!(clean_rut("..1.-2-3-.4.5."), "12345");
  }

  #[test]
  fn it_removes_leading_zeroes() {
    assert_eq!(clean_rut("007"), "7");
  }

  #[test]
  fn it_uppercases_k() {
    assert_eq!(clean_rut("k"), "K");
  }

  #[test]
  fn it_does_not_panic_with_an_empty_string() {
    assert_eq!(clean_rut(""), "");
  }
}
