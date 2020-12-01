use std::fmt;
use std::ops;

pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    pub fn new(data: Vec<f64>) -> Vector {
        Vector { data: data }
    }

    pub fn size(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// AKA inner scalar or projection product.
    pub fn dot_product(&self, other: &Vector) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .collect::<Vec<(&f64, &f64)>>()
            .iter()
            .map(|(x, y)| **x * **y)
            .sum()
    }

    /// The length of the shadow of other if light shone perpendicularly
    /// onto self.
    pub fn scalar_projection(&self, other: &Vector) -> f64 {
        self.dot_product(other) / self.size()
    }

    /// Scalar projection but in the original direction.
    /// Possibly could be self * scalar_projection, wasn't clear.
    pub fn vector_projection(&self, other: &Vector) -> Vector {
        self * (self.dot_product(other) / self.dot_product(self))
    }

    /// Assumes b1, b2, etc., are orthogonal to each other.
    /// Rotates(?) a vector to a new set of axes.
    pub fn rebase(&self, bs: &Vec<Vector>) -> Vector {
        Vector::new(
            bs.iter()
                .map(|b| self.dot_product(&(b / (b.size() * b.size()))))
                .collect(),
        )
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Self::Output {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .collect::<Vec<(&f64, &f64)>>()
                .iter()
                .map(|(x, y)| **x + **y)
                .collect(),
        }
    }
}

impl ops::Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            data: self.data.iter().map(|x| x * rhs).collect(),
        }
    }
}

impl ops::Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
