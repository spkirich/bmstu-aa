//! Поиск расстояния Левенштейна

pub trait VanillaLevenshtein {

    /// Расстояние Левенштейна
    fn distance(s: &str, t: &str) -> usize;
}

pub mod iterative;
pub use iterative::Iterative;
