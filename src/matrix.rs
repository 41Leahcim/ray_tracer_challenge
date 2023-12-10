use std::ops::{Index, IndexMut, Mul};

use crate::tuple::Tuple;

pub struct Matrix {
    rows: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn populate(rows: Vec<Vec<f64>>) -> Self {
        Self { rows }
    }

    pub fn identity() -> Self {
        Self::populate(
            (0usize..4)
                .map(|row| {
                    (0..4)
                        .map(|column| if row == column { 1.0 } else { 0.0 })
                        .collect()
                })
                .collect(),
        )
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        let mut transform = Matrix::identity();
        transform[0][3] = x;
        transform[1][3] = y;
        transform[2][3] = z;
        &transform * self
    }

    pub fn rotate_z(&self, radians: f64) -> Self {
        let mut transform = Matrix::identity();
        transform[0][0] = radians.cos();
        transform[0][1] = -radians.sin();
        transform[1][0] = radians.sin();
        transform[1][1] = radians.cos();
        &transform * self
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self::Output {
        Matrix::populate(
            self.rows
                .iter()
                .map(|row| {
                    (0..other.rows[0].len())
                        .map(|col| {
                            row.iter()
                                .zip(other.rows.iter().map(|row| row[col]))
                                .map(|(left, right)| *left * right)
                                .sum()
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Self::Output {
        let mut values = [0.0; 4];
        for (result, values) in values.iter_mut().zip(self.rows.iter()) {
            *result = values[0] * tuple.x
                + values[1] * tuple.y
                + values[2] * tuple.z
                + values[3] * tuple.w;
        }
        let mut result = Tuple::point(values[0], values[1], values[2]);
        result.w = values[3];
        result
    }
}
