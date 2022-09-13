//! Поиск простого расстояния Левенштейна

pub use iterative::Iterative;

pub trait VanillaLevenshtein {
    /// Простое расстояние Левенштейна
    fn distance(s: &str, t: &str) -> usize;
}

mod iterative;
