use super::DamerauLevenshtein;
use crate::Matrix;

/// Итеративно
pub struct Iterative;

impl DamerauLevenshtein for Iterative {
    /// Расстояние Дамерау-Левенштейна
    ///
    /// ```
    /// use lab_01::{damerau_levenshtein, DamerauLevenshtein};
    ///
    /// // Обе строки пустые
    /// assert_eq!(damerau_levenshtein::Iterative::distance("", ""), 0);
    ///
    /// // Только первая строка пустая
    /// assert_eq!(damerau_levenshtein::Iterative::distance("", "right"), 5);
    ///
    /// // Только вторая строка пустая
    /// assert_eq!(damerau_levenshtein::Iterative::distance("left", ""), 4);
    ///
    /// // Требуется одна вставка
    /// assert_eq!(damerau_levenshtein::Iterative::distance("word", "world"), 1);
    ///
    /// // Требуется одно удаление
    /// assert_eq!(damerau_levenshtein::Iterative::distance("clock", "lock"), 1);
    ///
    /// // Требуется одна замена
    /// assert_eq!(damerau_levenshtein::Iterative::distance("ping", "pong"), 1);
    ///
    /// // Требуется одна перестановка
    /// assert_eq!(damerau_levenshtein::Iterative::distance("vse", "sve"), 1);
    ///
    /// // Строки совпадают
    /// assert_eq!(damerau_levenshtein::Iterative::distance("zug", "zug"), 0);
    ///
    /// // Строки различаются
    /// assert_eq!(damerau_levenshtein::Iterative::distance("heaven", "hell"), 4);
    /// ```

    fn distance(s: &str, t: &str) -> usize {
        // Длины данных строк
        let (m, n) = (s.len(), t.len());

        // Матрица расстояний Дамерау-Левенштейна
        let mut matrix = Matrix::new(0, m + 1, n + 1);

        for i in 0..m + 1 {
            for j in 0..n + 1 {
                if i == 0 {
                    matrix.set(0, j, j);
                    continue;
                }

                if j == 0 {
                    matrix.set(i, 0, i);
                    continue;
                }

                // Текущие символы данных строк
                let c = s.chars().nth(i - 1).unwrap();
                let d = t.chars().nth(j - 1).unwrap();

                let mut cases = vec![matrix.get(i - 1, j - 1) + if c == d { 0 } else { 1 }];

                cases.push(matrix.get(i - 1, j) + 1);
                cases.push(matrix.get(i, j - 1) + 1);

                if i > 1 && j > 1 {
                    // Предыдущие символы данных строк
                    let p = s.chars().nth(i - 2).unwrap();
                    let q = t.chars().nth(j - 2).unwrap();

                    if c == q && p == d {
                        cases.push(matrix.get(i - 2, j - 2) + 1);
                    }
                }

                matrix.set(i, j, *cases.iter().min().unwrap());
            }
        }

        *matrix.get(m, n)
    }
}
