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
pub fn get_verification_digit(rut: &str) -> Result<char, &'static str> {
  let rut_numbers_reversed: Vec<Option<u32>> = String::from(rut)
    .chars()
    .map(|character| character.to_digit(10))
    .rev()
    .collect();

  if rut_numbers_reversed
    .iter()
    .any(|maybe_num| maybe_num.is_none())
  {
    return Err("Rut is not a number");
  }

  let rut_numbers_reversed: Vec<u32> = rut_numbers_reversed.iter().map(|d| d.unwrap()).collect();

  let mut sum = 0;
  let mut multiplier = 2;
  for digit in rut_numbers_reversed.iter() {
    sum += digit * multiplier;
    multiplier = match multiplier {
      7 => 2,
      m => m + 1,
    };
  }

  let raw_verification_digit = 11 - sum.rem_euclid(11);
  return match raw_verification_digit {
    10 => Ok('K'),
    11 => Ok('0'),
    r => match r.to_string().pop() {
      Some(vd) => Ok(vd),
      None => Err("Something is really wrong."),
    },
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_gets_a_real_verification_number() {
    assert_eq!(get_verification_digit("10956996"), Ok('8'));
  }

  #[test]
  fn it_gets_a_real_verification_number_2() {
    assert_eq!(get_verification_digit("17697567"), Ok('9'));
  }

  #[test]
  fn it_returns_k_when_rest_is_greater_than_0() {
    assert_eq!(get_verification_digit("13754444"), Ok('K'));
  }

  #[test]
  fn it_errs_when_rut_has_letters() {
    assert_eq!(
      get_verification_digit("not_a_number"),
      Err("Rut is not a number")
    );
  }
}
