use std::ops::AddAssign;
use std::ops::Deref;

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: &[T]) -> Self
    where
        T: Copy,
    {
        Vector {
            data: data.to_vec(),
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// pretend this is a heavy operation, CPU-intensive
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
