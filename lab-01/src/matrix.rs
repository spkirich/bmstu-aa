//! Работа с матрицами

/**
 * Прямоугольная матрица
 */

pub struct Matrix<T>(Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    /// Заполненная данным элементом матрица
    pub fn new(x: T, m: usize, n: usize) -> Self {
        Self(vec![vec![x; n]; m])
    }

    /// Получить элемент матрицы по индексу
    pub fn get(&self, i: usize, j: usize) -> &T {
        match self {
            Self(rows) => &rows[i][j],
        }
    }

    /// Заменить элемент матрицы по индексу
    pub fn set(&mut self, i: usize, j: usize, x: T) {
        match self {
            Self(rows) => rows[i][j] = x,
        }
    }
}
