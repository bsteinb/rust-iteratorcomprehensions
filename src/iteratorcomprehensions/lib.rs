/*!
  This module provides the syntax extension `iterator!()` that enables mapping, filtering and
  nesting of iterators using a comprehension syntax as well as the associated types `Prepend1`…
  `Prependn` which return the elements of an iterator in the last position of a 1…n element tuple,
  the preceding positions being constant.
*/

#![feature(macro_rules,phase)]

#![crate_id = "iteratorcomprehensions#0.1.0"]
#![crate_type = "lib"]

pub use prepend::{prepend1,Prepend1,prepend2,Prepend2};

/**
  Contains the macros that implement the comprehension syntax.
*/
pub mod macros {
  #![macro_escape]
  /**
    The `iterator!()` macro implements the following comprehension syntax:

    ```notrust
    iterator!(
      map_expr for var_1 in gen_expr_1 [if filter_expr_1]
      [… for var_n in gen_expr_n [if filter_expr]]
    )
    ```

    * `var_1`… `var_n` identify the iteration variables associated with each of the nested
      iterators.
    * `gen_expr_1`… `gen_expr_n` are expressions that evaluate to an `Iterator`. `gen_expr_i` can
      refer to all "outer" iteration variables `var_1`… `var_(i-1)`.
    * `map_expr` is an expression that constructs the elements of the iterator comprehension from
      the iteration variables.
    * `filter_expr_1`… `filter_expr_n` are expressions that evaluate to a boolean which filters
      the iterator elements based on the "outer" iteration variables.

    `iterator!()` evaluates to an expression which itself implements the `Iterator` trait.

    The expression

    ```notrust
    iterator!(
      (i, j) for i in range(0,3) for j in range(0, i + 1) if (i + j) % 2 == 0
    )
    ```
    evaluates to an iterator that contains the elements

    ```notrust
    (0,0), (1,1), (2,0), (2,2)
    ```
  */
  #[macro_export]
  macro_rules! iterator(
    (
      $map:expr
      for $var:ident in $gen:expr
      $(if $filter:expr)*
    ) => (
      $gen
      $(.filter(|&$var| { $filter }))*
      .map(|$var| { $map })
    );
    (
      $map:expr
      for $var1:ident in $gen1:expr
      $(if $filter1:expr)*
      for $var2:ident in $gen2:expr
      $(if $filter2:expr)*
    ) => (
      $gen1
      $(.filter(|&$var1| { $filter1 }))*
      .flat_map(|$var1| { iteratorcomprehensions::prepend1($var1, $gen2) })
      $(.filter(|&($var1, $var2)| { $filter2 }))*
      .map(|($var1, $var2)| { $map })
    );
    (
      $map:expr
      for $var1:ident in $gen1:expr
      $(if $filter1:expr)*
      for $var2:ident in $gen2:expr
      $(if $filter2:expr)*
      for $var3:ident in $gen3:expr
      $(if $filter3:expr)*
    ) => (
      $gen1
      $(.filter(|&$var1| { $filter1 }))*
      .flat_map(|$var1| { iteratorcomprehensions::prepend1($var1, $gen2) })
      $(.filter(|&($var1, $var2)| { $filter2 }))*
      .flat_map(|($var1, $var2)| { iteratorcomprehensions::prepend2($var1, $var2, $gen3) })
      $(.filter(|&($var1, $var2, $var3)| { $filter3 }))*
      .map(|($var1, $var2, $var3)| { $map })
    );
  )
}

/**
  Contains the types `Prepend1`… `Prependn` as well as the convenience functions `prepend1`… `prependn`.
*/
pub mod prepend {
  /**
    Maps each element `i` in `iter` to the tuple `(val1, i)`.
  */
  pub struct Prepend1<T1, T, It> {
    iter: It,
    val1: T1
  }

  impl<T1: Clone, T, It: Iterator<T>> Prepend1<T1, T, It> {
    #[inline]
    fn new(v1: T1, it: It) -> Prepend1<T1, T, It> {
      Prepend1 { iter: it, val1: v1 }
    }
  }

  impl<T1: Clone, T, It: Iterator<T>> Iterator<(T1, T)> for Prepend1<T1, T, It> {
    #[inline]
    fn next(&mut self) -> Option<(T1, T)> {
      self.iter.next().map(|val| (self.val1.clone(), val))
    }
  }

  /**
    Constructs a new `Prepend1`.
  */
  #[inline]
  pub fn prepend1<T1: Clone, T, It: Iterator<T>>(v1: T1, it: It) -> Prepend1<T1, T, It> {
    Prepend1::new(v1, it)
  }

  /**
    Maps each element `i` in `iter` to the tuple `(val1, val2, i)`.
  */
  pub struct Prepend2<T1, T2, T, It> {
    iter: It,
    val1: T1,
    val2: T2
  }

  impl<T1: Clone, T2: Clone, T, It: Iterator<T>> Prepend2<T1, T2, T, It> {
    #[inline]
    fn new(v1: T1, v2: T2, it: It) -> Prepend2<T1, T2, T, It> {
      Prepend2 { iter: it, val1: v1, val2: v2 }
    }
  }

  impl<T1: Clone, T2: Clone, T, It: Iterator<T>> Iterator<(T1, T2, T)> for Prepend2<T1, T2, T, It> {
    #[inline]
    fn next(&mut self) -> Option<(T1, T2, T)> {
      self.iter.next().map(|val| (self.val1.clone(), self.val2.clone(), val))
    }
  }

  /**
    Constructs a new `Prepend2`.
  */
  #[inline]
  pub fn prepend2<T1: Clone, T2: Clone, T, It: Iterator<T>>(v1: T1, v2: T2, it: It)
  -> Prepend2<T1, T2, T, It> {
    Prepend2::new(v1, v2, it)
  }
}

#[cfg(test)]
mod tests {
  use iteratorcomprehensions = prepend; // Yikes!

  #[test]
  fn iterator1_test() {
    let xs: Vec<int> = iterator!( i for i in range(0i, 3i) ).collect();
    assert_eq!(xs, vec!(0, 1, 2));
  }

  #[test]
  fn iterator1_map_test() {
    let xs: Vec<int> = iterator!( i + 1 for i in range(0i, 3i) ).collect();
    assert_eq!(xs, vec!(1, 2, 3));
  }

  #[test]
  fn iterator1_filter_test() {
    let xs: Vec<int> = iterator!( i for i in range(0i, 3i) if i % 2 == 1 ).collect();
    assert_eq!(xs, vec!(1));
  }

  #[test]
  fn iterator1_filter_map_test() {
    let xs: Vec<int> = iterator!( i * 2 for i in range(0i, 3i) if i % 2 == 1 ).collect();
    assert_eq!(xs, vec!(2));
  }

  #[test]
  fn iterator2_map_test() {
    let xs: Vec<int> = iterator!( i * j for i in range(1i, 3i) for j in range(2i, 4i) ).collect();
    assert_eq!(xs, vec!(2, 3, 4, 6));
  }

  #[test]
  fn iterator2_filter_map_test() {
    let xs: Vec<int> = iterator!(
      i / j for i in range(6i, 9i) for j in range(1i, 4i) if i % j == 0
    ).collect();
    assert_eq!(xs, vec!(6, 3, 2, 7, 8, 4));
  }

  #[test]
  fn iterator2_example_test() {
    let xs: Vec<(int,int)> = iterator!(
      (i,j) for i in range(0i, 3i) for j in range(0i, i + 1i) if (i + j) % 2 == 0
    ).collect();
    assert_eq!(xs, vec!((0, 0), (1, 1), (2, 0), (2, 2)));
  }

  #[test]
  fn iterator3_map_test() {
    let xs: Vec<int> = iterator!(
      i * j * k for i in range(1i, 3i) for j in range(2i, 4i) for k in range(3i, 5i)
    ).collect();
    assert_eq!(xs, vec!(6, 8, 9, 12, 12, 16, 18, 24));
  }

  #[test]
  fn iterator3_filter_map_test() {
    let xs: Vec<int> = iterator!(
      i + j + k for i in range(0i, 10i) for j in range(0i, 10i) for k in range(0i, 10i)
      if i == 1 && j == 1 && k == 1
    ).collect();
    assert_eq!(xs, vec!(3));
  }

  #[test]
  fn iterator3_empty_test() {
    let xs: Vec<int> = iterator!(
      i * j * k for i in range(0i, 2i) for j in range(0i, i) for k in range(0i, 1i)
    ).collect();
    assert_eq!(xs, vec!(0));
  }
}
