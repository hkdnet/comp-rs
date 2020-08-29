pub struct UnionFind(Vec<usize>);

impl UnionFind {
  pub fn new(n: usize) -> Self {
    UnionFind((0..n).collect())
  }
  pub fn root(self: &mut Self, a: usize) -> usize {
    let tmp = self.0[a];
    if tmp == a {
      a
    } else {
      self.0[a] = self.root(tmp);
      self.0[a]
    }
  }
  pub fn unite(self: &mut Self, a: usize, b: usize) {
    let ra = self.root(a);
    let rb = self.root(b);
    if ra != rb {
      self.0[b] = a;
    }
  }
  pub fn same(self: &mut Self, a: usize, b: usize) -> bool {
    return self.root(a) == self.root(b);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_union_find() {
    let mut uf = UnionFind::new(3);
    assert_ne!(uf.root(0), uf.root(1));

    let mut uf = UnionFind::new(3);
    uf.unite(0, 1);
    assert_eq!(uf.root(0), uf.root(1));
    let mut uf = UnionFind::new(3);
    uf.unite(1, 0);
    assert_eq!(uf.root(1), uf.root(0));

    let mut uf = UnionFind::new(3);
    uf.unite(0, 1);
    uf.unite(1, 2);
    assert_eq!(uf.root(0), uf.root(2));

    let mut uf = UnionFind::new(4);
    let v = vec![
      (1, 0, 1),
      (0, 0, 1),
      (0, 2, 3),
      (1, 0, 1),
      (1, 1, 2),
      (0, 0, 2),
      (1, 1, 3),
    ];
    let expected = vec![0, 1, 0, 1];
    let mut actual = vec![];
    for (ty, a, b) in v {
      match ty {
        0 => {
          uf.unite(a, b);
        }
        _ => {
          if uf.same(a, b) {
            actual.push(1);
          } else {
            actual.push(0);
          }
        }
      }
    }
    assert_eq!(expected, actual);
  }
}
