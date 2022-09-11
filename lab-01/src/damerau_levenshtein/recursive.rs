//! Рекурсивная реализация

use crate::DamerauLevenshtein;

pub struct Recursive;

impl DamerauLevenshtein for Recursive {

    /// Расстояние Дамерау-Левенштейна
    ///
    /// Примеры:
    ///
    /// ```
    /// use lab_01::damerau_levenshtein::{DamerauLevenshtein, Recursive};
    ///
    /// // Обе строки пустые
    /// assert_eq!(Recursive::distance("", ""), 0);
    ///
    /// // Только первая строка пустая
    /// assert_eq!(Recursive::distance("", "right"), 5);
    ///
    /// // Только вторая строка пустая
    /// assert_eq!(Recursive::distance("left", ""), 4);
    ///
    /// // Требуется одна вставка
    /// assert_eq!(Recursive::distance("word", "world"), 1);
    ///
    /// // Требуется одно удаление
    /// assert_eq!(Recursive::distance("clock", "lock"), 1);
    ///
    /// // Требуется одна замена
    /// assert_eq!(Recursive::distance("ping", "pong"), 1);
    ///
    /// // Требуется одна перестановка
    /// assert_eq!(Recursive::distance("vse", "sve"), 1);
    ///
    /// // Строки совпадают
    /// assert_eq!(Recursive::distance("zug", "zug"), 0);
    ///
    /// // Строки различаются
    /// assert_eq!(Recursive::distance("heaven", "hell"), 4);
    /// ```

    /// Простая рекурсивная реализация
    fn distance(s: &str, t: &str) -> usize {

        // Длины данных строк
        let (m, n) = (s.len(), t.len());

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

            Self::distance(&s[ .. m - 1], t) + 1,
            Self::distance(s, &t[ .. n - 1]) + 1,

            Self::distance(&s[ .. m - 1], &t[ .. n - 1])
                + if c == d { 0 } else { 1 }
        ];

        if m > 1 && n > 1 {

            // Предпоследние символы данных строк
            let p = s.chars().nth(m - 2).unwrap();
            let q = t.chars().nth(n - 2).unwrap();

            if c == q && p == d {
                cases.push(Self::distance(&s[ .. m - 2], &t[ .. n - 2]) + 1);
            }
        }

        *cases.iter().min().unwrap()
    }
}
