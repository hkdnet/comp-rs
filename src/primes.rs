use std::collections::BTreeSet;

pub fn primes<T>(n: T) -> BTreeSet<T>
where
  T: PartialEq
    + From<u8>
    + Copy
    + std::cmp::Ord
    + std::ops::Mul<Output = T>
    + std::ops::Add<Output = T>
    + std::ops::Div<Output = T>
    + std::ops::Rem<Output = T>,
{
  let mut m = n;
  let mut p = T::from(2u8);
  let mut ret = BTreeSet::new();
  while p * p <= n {
    while (m % p) == T::from(0u8) {
      ret.insert(p);
      m = m / p;
    }
    p = p + T::from(1u8);
  }
  if m != T::from(1u8) {
    ret.insert(m);
  }

  ret
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_primes() {
    use std::iter::FromIterator;
    let examples = vec![
      (1, vec![]),
      (2, vec![2]),
      (4, vec![2]),
      (6, vec![2, 3]),
      (11, vec![11]),
    ];
    for (n, v) in examples {
      let actual = primes(n);
      let expected = BTreeSet::from_iter(v);
      assert_eq!(actual, expected);
    }
    assert_eq!(primes(1 as usize), BTreeSet::new());
    assert_eq!(primes(1 as i32), BTreeSet::new());
    assert_eq!(primes(1 as u32), BTreeSet::new());
  }
}
