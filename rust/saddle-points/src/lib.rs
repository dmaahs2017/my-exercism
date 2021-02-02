pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let m = Matrix::new(input);
    match m.saddle_points() {
        Ok(p) => p,
        Err(e) => match e {
            Error::EmptyMatrix => Vec::new(),
            Error::IndexOutOfBounds => {
                panic!("Index Out Of Bounds, Likely non-rectangular matrix")
            }
        },
    }
}

struct Matrix<'a, T> {
    matrix: &'a [Vec<T>],
}

type MatrixResult<T = Vec<(usize, usize)>, E = Error> = Result<T, E>;

#[derive(Debug)]
enum Error {
    EmptyMatrix,
    IndexOutOfBounds,
}

impl<'a, T> Matrix<'a, T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Copy + std::fmt::Debug,
{
    fn saddle_points(&self) -> MatrixResult {
        let mut points = Vec::new();
        for row in 0..self.rows() {
            let max_row = self.iter_row(row)?.max().ok_or(Error::EmptyMatrix)?;
            for col in 0..self.cols() {
                let min_col = self.iter_col(col).min().ok_or(Error::EmptyMatrix)?;
                let n = self.get(row, col)?;
                if n == max_row && n == min_col {
                    points.push((row, col))
                }
            }
        }
        Ok(points)
    }

    fn new(matrix: &'a [Vec<T>]) -> Self {
        Self { matrix }
    }

    fn rows(&self) -> usize {
        self.matrix.len()
    }

    fn cols(&self) -> usize {
        match self.matrix.get(0) {
            Some(c) => c.len(),
            None => 0,
        }
    }

    fn get(&self, row: usize, col: usize) -> MatrixResult<&T> {
        self.matrix
            .get(row)
            .ok_or(Error::IndexOutOfBounds)?
            .get(col)
            .ok_or(Error::IndexOutOfBounds)
    }

    fn iter_row(&self, n: usize) -> MatrixResult<impl Iterator<Item = &T>> {
        self.matrix
            .get(n)
            .ok_or(Error::IndexOutOfBounds)
            .map(|row| row.iter())
    }

    fn iter_col(&self, n: usize) -> impl Iterator<Item = &T> {
        self.matrix
            .iter()
            .flat_map(move |r| r.iter().skip(n).take(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_works() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let m: Matrix<u64> = Matrix::new(&v);
        assert_eq!(m.get(0, 0).unwrap().clone(), 1);
        assert_eq!(m.get(1, 0).unwrap().clone(), 4);
        assert_eq!(m.get(0, 1).unwrap().clone(), 2);
    }

    #[test]
    fn row_works() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let m: Matrix<u64> = Matrix::new(&v);

        assert_eq!(
            m.iter_row(0).unwrap().cloned().collect::<Vec<u64>>(),
            vec![1, 2, 3]
        );
        assert_eq!(
            m.iter_row(1).unwrap().cloned().collect::<Vec<u64>>(),
            vec![4, 5, 6]
        );
        assert_eq!(
            m.iter_row(1)
                .unwrap()
                .cloned()
                .enumerate()
                .collect::<Vec<(usize, u64)>>(),
            vec![(0, 4), (1, 5), (2, 6)]
        );
    }

    #[test]
    fn col_works() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let m: Matrix<u64> = Matrix::new(&v);

        assert_eq!(m.iter_col(0).cloned().collect::<Vec<u64>>(), vec![1, 4]);
        assert_eq!(m.iter_col(1).cloned().collect::<Vec<u64>>(), vec![2, 5]);
        assert_eq!(m.iter_col(2).cloned().collect::<Vec<u64>>(), vec![3, 6]);
        assert_eq!(
            m.iter_col(2)
                .cloned()
                .enumerate()
                .collect::<Vec<(usize, u64)>>(),
            vec![(0, 3), (1, 6)]
        );
    }
}
