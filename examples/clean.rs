use rutilities::clean_rut;

pub fn main() {
  let rut = "14.780.556-k";
  let clean = clean_rut(rut);
  println!("clean rut is {}", clean);
}
