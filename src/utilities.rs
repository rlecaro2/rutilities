mod get_verification_digit;
use get_verification_digit::get_verification_digit;

mod clean;
pub use clean::clean_rut;

mod format;
pub use format::format_rut;

mod is_rut_valid;
pub use is_rut_valid::is_rut_valid;

mod generate_valid_rut;
pub use generate_valid_rut::generate_valid_rut;
