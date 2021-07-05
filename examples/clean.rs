use rutilities::clean;

pub fn main() {
  let rut = "14.780.556-k";
  let clean = clean(rut);
  println!("clean rut is {}", clean);
}
