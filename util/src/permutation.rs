use std::error;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TryFromError;
impl fmt::Display for TryFromError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ill-formed slice to permutation conversion attempted",)
    }
}
impl error::Error for TryFromError {}

/// Returns true if a vector is a permutation.
///
/// That is, all the elements in `0..len` occur exactly once in the vector.
fn is_permutation(v: &[usize]) -> bool {
    let n = v.len();
    let mut seen = (0..n).map(|_| false).collect::<Vec<_>>();
    for &e in v {
        if (0..n).contains(&e) {
            seen[e] = true;
        }
    }
    seen.into_iter().all(|b| b)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Permutation {
    map: Vec<usize>,
}
impl Permutation {
    /// Returns the identity permutation of n elements.
    pub fn identity(n: usize) -> Permutation {
        Permutation {
            map: (0..n).collect::<Vec<_>>(),
        }
    }
    /// Returns the permutation of n elements which rotates r steps to the left.
    pub fn rotate_left(n: usize, r: usize) -> Permutation {
        Permutation {
            map: (0..n).map(|i| (i + r) % n).collect::<Vec<_>>(),
        }
    }
    /// Returns the permutation of n elements which rotates r steps to the right.
    pub fn rotate_right(n: usize, r: usize) -> Permutation {
        Permutation {
            map: (0..n).map(|i| (i + n - r % n) % n).collect::<Vec<_>>(),
        }
    }
    /// Returns the permutation which exchanges the elements at i and j.
    pub fn transpose(n: usize, i: usize, j: usize) -> Permutation {
        assert!(i < n && j < n);
        Permutation {
            map: (0..n)
                .map(|k| {
                    if k == i {
                        j
                    } else if k == j {
                        i
                    } else {
                        k
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
    /// Applies the permutation to an element.
    pub fn apply(&self, i: usize) -> usize {
        self.map[i]
    }
    /// Returns a vector permuted by this permutation.
    pub fn permute<T: Clone>(&self, v: &[T]) -> Vec<T> {
        assert_eq!(self.map.len(), v.len());
        (0..self.map.len())
            .map(|i| v[self.apply(i)].clone())
            .collect::<Vec<_>>()
    }
    /// Returns the composition of the permutation with itself.
    pub fn square(&self) -> Permutation {
        self * self
    }
    /// Returns the composition of the permutation with itself `exp` number of times.
    pub fn pow(&self, exp: u32) -> Permutation {
        if exp == 0 {
            Permutation::identity(self.map.len())
        } else if exp == 1 {
            self.clone()
        } else if exp % 2 == 0 {
            self.square().pow(exp / 2)
        } else {
            self * self.pow(exp - 1)
        }
    }
    /// Returns the inverse permutation.
    pub fn inv(&self) -> Permutation {
        let len = self.map.len();
        let mut map = vec![0; len];
        for i in 0..len {
            let j = self.map[i];
            map[j] = i;
        }
        Permutation { map }
    }
}
impl ops::Mul<Permutation> for Permutation {
    type Output = Permutation;
    fn mul(self, other: Permutation) -> Self::Output {
        assert_eq!(self.map.len(), other.map.len());
        Permutation {
            map: (0..self.map.len())
                .map(|i| other.apply(self.apply(i)))
                .collect::<Vec<_>>(),
        }
    }
}
impl ops::Mul<Permutation> for &Permutation {
    type Output = Permutation;
    fn mul(self, other: Permutation) -> Self::Output {
        assert_eq!(self.map.len(), other.map.len());
        Permutation {
            map: (0..self.map.len())
                .map(|i| other.apply(self.apply(i)))
                .collect::<Vec<_>>(),
        }
    }
}
impl ops::Mul<&Permutation> for Permutation {
    type Output = Permutation;
    fn mul(self, other: &Permutation) -> Self::Output {
        assert_eq!(self.map.len(), other.map.len());
        Permutation {
            map: (0..self.map.len())
                .map(|i| other.apply(self.apply(i)))
                .collect::<Vec<_>>(),
        }
    }
}
impl ops::Mul<&Permutation> for &Permutation {
    type Output = Permutation;
    fn mul(self, other: &Permutation) -> Self::Output {
        assert_eq!(self.map.len(), other.map.len());
        Permutation {
            map: (0..self.map.len())
                .map(|i| other.apply(self.apply(i)))
                .collect::<Vec<_>>(),
        }
    }
}
impl TryFrom<Vec<usize>> for Permutation {
    type Error = TryFromError;
    fn try_from(v: Vec<usize>) -> Result<Permutation, TryFromError> {
        if is_permutation(&v) {
            Ok(Permutation { map: v })
        } else {
            Err(TryFromError)
        }
    }
}
impl TryFrom<&[usize]> for Permutation {
    type Error = TryFromError;
    fn try_from(a: &[usize]) -> Result<Permutation, TryFromError> {
        if is_permutation(a) {
            Ok(Permutation { map: a.to_vec() })
        } else {
            Err(TryFromError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_permutation;
    use super::Permutation;
    use super::TryFromError;

    #[test]
    fn test_is_permutation_true() {
        assert_eq!(true, is_permutation(&vec![2, 1, 0]));
    }
    #[test]
    fn test_is_permutation_false_missing_element() {
        assert_eq!(false, is_permutation(&vec![0, 1, 1]));
    }
    #[test]
    fn test_is_permutation_false_out_of_range() {
        assert_eq!(false, is_permutation(&vec![2, 7, 1]));
    }

    #[test]
    fn test_identity() {
        let id = Permutation::identity(3);
        assert_eq!(Permutation { map: vec![0, 1, 2] }, id);
    }

    #[test]
    fn test_rotate_left() {
        let p = Permutation::rotate_left(3, 1);
        assert_eq!(Permutation { map: vec![1, 2, 0] }, p);
    }

    #[test]
    fn test_rotate_right() {
        let p = Permutation::rotate_right(3, 1);
        assert_eq!(Permutation { map: vec![2, 0, 1] }, p);
    }

    #[test]
    fn test_transpose() {
        let p = Permutation::transpose(3, 1, 2);
        assert_eq!(Permutation { map: vec![0, 2, 1] }, p);
    }

    #[test]
    fn test_apply() {
        let p = Permutation { map: vec![0, 2, 1] };
        assert_eq!(2, p.apply(1));
    }

    #[test]
    fn test_permute() {
        let p = Permutation { map: vec![0, 2, 1] };
        assert_eq!(vec!['a', 'c', 'b'], p.permute(&vec!['a', 'b', 'c']));
    }

    #[test]
    fn test_square() {
        let p = Permutation::rotate_left(3, 1);
        assert_eq!(Permutation { map: vec![2, 0, 1] }, p.square());
    }

    #[test]
    fn test_pow() {
        let p = Permutation::rotate_left(3, 1);
        assert_eq!(Permutation::identity(3), p.pow(3));
    }

    #[test]
    fn test_inv() {
        let p = Permutation::rotate_left(3, 1);
        assert_eq!(Permutation::rotate_right(3, 1), p.inv());
    }
    #[test]
    fn test_inv_identity() {
        let id = Permutation::identity(3);
        assert_eq!(id, id.inv());
    }

    #[test]
    fn test_mul_mm() {
        let p0 = Permutation::rotate_left(3, 1);
        let p1 = Permutation::rotate_right(3, 1);
        assert_eq!(Permutation::identity(3), p0 * p1);
    }
    #[test]
    fn test_mul_mr() {
        let p0 = Permutation::rotate_left(3, 1);
        let p1 = Permutation::rotate_right(3, 1);
        assert_eq!(Permutation::identity(3), p0 * &p1);
    }
    #[test]
    fn test_mul_rm() {
        let p0 = Permutation::rotate_left(3, 1);
        let p1 = Permutation::rotate_right(3, 1);
        assert_eq!(Permutation::identity(3), &p0 * p1);
    }
    #[test]
    fn test_mul_rr() {
        let p0 = Permutation::rotate_left(3, 1);
        let p1 = Permutation::rotate_right(3, 1);
        assert_eq!(Permutation::identity(3), &p0 * &p1);
    }

    #[test]
    fn test_try_from_ok() {
        let v = vec![2, 1, 0];
        let result = Ok(Permutation { map: v.clone() });
        assert_eq!(result, Permutation::try_from(v));
    }
    #[test]
    fn test_try_from_err_missing_element() {
        assert_eq!(Err(TryFromError), Permutation::try_from(vec![0, 1, 1]));
    }
    #[test]
    fn test_try_from_err_out_of_range() {
        assert_eq!(Err(TryFromError), Permutation::try_from(vec![2, 7, 1]));
    }
}
