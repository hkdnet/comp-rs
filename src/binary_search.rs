pub fn binary_search<T, F>(l: T, r: T, f: F) -> T
where
  T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + PartialEq + From<u8> + Copy,
  F: Fn(T) -> bool,
{
  let mut l = l;
  let mut r = r;

  loop {
    let idx = (l + r) / T::from(2u8);
    // idx == r is required when l > r.
    if idx == l || idx == r {
      break;
    }
    if f(idx) {
      l = idx;
    } else {
      r = idx;
    }
  }

  l
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_binary_search() {
    let arr = &vec![1, 2, 3, 4];
    assert_eq!(binary_search(0, arr.len(), |i| arr[i] < 3), 1);
    assert_eq!(binary_search(0, arr.len(), |i| arr[i] < 100), 3);
    assert_eq!(binary_search(0, arr.len(), |i| arr[i] < -1), 0);
    assert_eq!(binary_search(1, 3, |i| arr[i] < 3), 1);
    assert_eq!(binary_search(1, 3, |i| arr[i] < 100), 2);
    assert_eq!(binary_search(1, 3, |i| arr[i] < -1), 1);
    assert_eq!(binary_search(4, -1, |i| arr[i as usize] < 3), 0);
  }
}
