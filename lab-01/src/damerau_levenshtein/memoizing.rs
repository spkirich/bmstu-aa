//! Мемоизирующая реализация

use crate::DamerauLevenshtein;
use crate::Matrix;

/// Кэш найденных расстояний
type Cache = Matrix<Option<usize>>;

pub struct Memoizing;

impl DamerauLevenshtein for Memoizing {
    /// Расстояние Дамерау-Левенштейна
    ///
    /// Примеры:
    ///
    /// ```
    /// use lab_01::damerau_levenshtein::{DamerauLevenshtein, Memoizing};
    ///
    /// // Обе строки пустые
    /// assert_eq!(Memoizing::distance("", ""), 0);
    ///
    /// // Только первая строка пустая
    /// assert_eq!(Memoizing::distance("", "right"), 5);
    ///
    /// // Только вторая строка пустая
    /// assert_eq!(Memoizing::distance("left", ""), 4);
    ///
    /// // Требуется одна вставка
    /// assert_eq!(Memoizing::distance("word", "world"), 1);
    ///
    /// // Требуется одно удаление
    /// assert_eq!(Memoizing::distance("clock", "lock"), 1);
    ///
    /// // Требуется одна замена
    /// assert_eq!(Memoizing::distance("ping", "pong"), 1);
    ///
    /// // Требуется одна перестановка
    /// assert_eq!(Memoizing::distance("vse", "sve"), 1);
    ///
    /// // Строки совпадают
    /// assert_eq!(Memoizing::distance("zug", "zug"), 0);
    ///
    /// // Строки различаются
    /// assert_eq!(Memoizing::distance("heaven", "hell"), 4);
    /// ```

    fn distance(s: &str, t: &str) -> usize {
        // Длины данных строк
        let (m, n) = (s.len(), t.len());

        // Кэш расстояний Дамерау-Левенштейна
        let mut cache = Cache::new(None, m + 1, n + 1);

        helper(s, t, &mut cache)
    }
}

/// Вспомогательная функция
fn helper(s: &str, t: &str, cache: &mut Cache) -> usize {
    // Длины данных строк
    let (m, n) = (s.len(), t.len());

    if let Some(distance) = cache.get(m, n) {
        return *distance;
    }

    if m == 0 {
        return n;
    }

    if n == 0 {
        return m;
    }

    // Последние символы данных строк
    let c = s.chars().nth(m - 1).unwrap();
    let d = t.chars().nth(n - 1).unwrap();

    let mut cases = vec![
        helper(&s[0..m - 1], t, cache) + 1,
        helper(s, &t[0..n - 1], cache) + 1,
        helper(&s[0..m - 1], &t[0..n - 1], cache) + if c == d { 0 } else { 1 },
    ];

    if m > 1 && n > 1 {
        // Предпоследние символы данных строк
        let p = s.chars().nth(m - 2).unwrap();
        let q = t.chars().nth(n - 2).unwrap();

        if c == q && p == d {
            cases.push(helper(&s[0..m - 2], &t[0..n - 2], cache) + 1);
        }
    }

    // Расстояние Дамерау-Левенштейна
    let distance = *cases.iter().min().unwrap();

    // Обновляем кэш
    cache.set(m, n, Some(distance));

    distance
}
