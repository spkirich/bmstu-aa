//! Поиск расстояния Дамерау-Левенштейна

pub trait DamerauLevenshtein {
    /// Расстояние Дамерау-Левенштейна
    fn distance(s: &str, t: &str) -> usize;
}

pub mod iterative;
pub use iterative::Iterative;

pub mod recursive;
pub use recursive::Recursive;

pub mod memoizing;
pub use memoizing::Memoizing;
