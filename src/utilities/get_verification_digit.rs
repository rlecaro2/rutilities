/**
 * The formula for the verification is:
 *   reverse the number
 *   calculate the dot product of:
 *      that number and
 *      the vector [2,3,4,5,6,7] (loop over if you need more numbers)
 *   get the modulus by 11
 *   subtract to 11 the result (11 - x)
 *   that's it.
 *   Special cases: 10 -> K, 11 -> 0
 */

/// Returns the verification (or check) digit for the given rut.
/// Useful for verification or generating a random rut.
/// @see https://es.wikipedia.org/wiki/Anexo:Implementaciones_para_algoritmo_de_rut#Javascript
/// # Arguments
///
/// * `rut` - A string slice that holds the document nubmer withour verification digit
pub fn get_verification_digit(rut: &str) -> char {
  let rut_numbers_reversed: Vec<u32> = String::from(rut)
    .chars()
    .map(|character| {
      character
        .to_digit(10)
        .unwrap_or_else(|| panic!("'{}' cant be coerced to a number", character))
    })
    .rev()
    .collect();

  let mut summed = 0;
  let mut multiplier = 2;
  for digit in rut_numbers_reversed.iter() {
    summed += digit * multiplier;
    multiplier = match multiplier {
      7 => 2,
      m => m + 1,
    };
  }

  let raw_verification_digit = 11 - summed.rem_euclid(11);
  return match raw_verification_digit {
    10 => 'K',
    11 => '0',
    r => r
      .to_string()
      .pop()
      .expect("There is something really wrong."),
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_gets_a_real_verification_number() {
    assert_eq!(get_verification_digit("10956996"), '8');
  }

  #[test]
  fn it_gets_a_real_verification_number_2() {
    assert_eq!(get_verification_digit("17697567"), '9');
  }

  #[test]
  fn it_returns_k_when_rest_is_greater_than_0() {
    assert_eq!(get_verification_digit("13754444"), 'K');
  }

  #[test]
  #[should_panic(expected = "'r' cant be coerced to a number")]
  fn it_panics_when_rut_has_letters() {
    get_verification_digit("not_a_number");
  }
}
