pub use damerau_levenshtein::DamerauLevenshtein;
pub use vanilla_levenshtein::VanillaLevenshtein;

use matrix::Matrix;

pub mod damerau_levenshtein;
pub mod vanilla_levenshtein;

mod matrix;
