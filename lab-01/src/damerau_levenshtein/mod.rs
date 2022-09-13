//! Поиск расстояния Дамерау-Левенштейна

pub use iterative::Iterative;
pub use memoizing::Memoizing;
pub use recursive::Recursive;

pub trait DamerauLevenshtein {
    /// Расстояние Дамерау-Левенштейна
    fn distance(s: &str, t: &str) -> usize;
}

mod iterative;
mod memoizing;
mod recursive;
