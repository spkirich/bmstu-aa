pub mod vanilla_levenshtein;
pub use vanilla_levenshtein::VanillaLevenshtein;

pub mod damerau_levenshtein;
pub use damerau_levenshtein::DamerauLevenshtein;

mod matrix;
use matrix::Matrix;
