use crate::vector::Vector;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

pub struct Martix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub fn mulity<T>(a: &Martix<T>, b: &Martix<T>) -> anyhow::Result<Martix<T>>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + Debug + Default + AddAssign,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("The number of columns of the first matrix must be equal to the number of rows of the second matrix"));
    }
    let mut data = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            // let mut sum = a.data[i * a.cols] * b.data[j];
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
            // data.push(sum);
        }
    }
    Ok(Martix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

pub fn do_product<T>(a: Vector<T>, b: Vector<T>) -> anyhow::Result<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + Default + AddAssign,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("The number of columns of the first matrix must be equal to the number of rows of the second matrix"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

impl<T> Martix<T>
where
    T: Debug,
{
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Self {
        Martix { data, rows, cols }
    }
}

impl<T> Display for Martix<T>
where
    T: Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j != self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> Debug for Martix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Martix(row={},cols={}, {})", self.rows, self.cols, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mulity() -> anyhow::Result<()> {
        let a = Martix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Martix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = mulity(&a, &b)?;
        // println!("{:?}", c);
        assert_eq!(format!("{:?}", c), "Martix(row=2,cols=2, {22 28, 49 64})");
        Ok(())
    }

    #[test]
    fn test_mulity_error() {
        let a = Martix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Martix::new(vec![1, 2, 3, 4, 5, 6], 2, 2);
        assert!(mulity(&a, &b).is_err());
    }
}
