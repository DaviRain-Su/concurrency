use crate::vector::do_product;
use crate::vector::Vector;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
use std::ops::Mul;
use std::sync::mpsc;
use std::thread;

const NUM_THREADS: usize = 4;

pub struct Martix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    rows: Vector<T>,
    cols: Vector<T>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, rows: Vector<T>, cols: Vector<T>) -> Self {
        MsgInput { idx, rows, cols }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

impl<T> MsgOutput<T> {
    pub fn new(idx: usize, value: T) -> Self {
        MsgOutput { idx, value }
    }
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Msg { input, sender }
    }
}

pub fn mulity<T>(a: &Martix<T>, b: &Martix<T>) -> anyhow::Result<Martix<T>>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + Debug
        + Default
        + AddAssign
        + Send
        + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("The number of columns of the first matrix must be equal to the number of rows of the second matrix"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = do_product(msg.input.rows, msg.input.cols)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Error: {}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // generate 4 threads which receive msg and do dot product

    let mut data = vec![T::default(); a.rows * b.cols];
    let mut receivers = Vec::with_capacity(a.rows * b.cols);

    // map/reduce: map phase
    for i in 0..a.rows {
        for j in 0..b.cols {
            // TODO: Need to optimize
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(&col_data);
            let idx = i * b.cols + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Error: {}", e);
            }
            receivers.push(rx);
        }
    }

    // map/reduce: reduce phase
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }

    Ok(Martix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
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

impl<T> Mul for Martix<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + Debug
        + Default
        + AddAssign
        + Send
        + 'static,
{
    type Output = Martix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        mulity(&self, &rhs).expect("Error in mulity")
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
        assert_eq!(format!("{:?}", c), "Martix(row=2,cols=2, {22 28, 49 64})");
        Ok(())
    }

    #[test]
    fn test_mulity_error() {
        let a = Martix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Martix::new(vec![1, 2, 3, 4], 2, 2);
        assert!(mulity(&a, &b).is_err());
    }

    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Martix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Martix::new(vec![1, 2, 3, 4, 5, 6], 2, 2);
        let _c = a * b;
        // assert_eq!(format!("{:?}", c), "Martix(row=2,cols=2, {22 28, 49 64})");
    }
}
