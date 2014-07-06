/*!
  This module provides the syntax extension `iterator!()` that enables mapping, filtering and
  nesting of iterators using a comprehension syntax.
*/

#![feature(macro_rules,phase)]

#![crate_id = "iteratorcomprehensions#0.2.0"]
#![crate_type = "lib"]

/**
  Contains the macros that implement the comprehension syntax.
*/
pub mod macros {
  #![macro_escape]

  /**
    Turns a comma separated list of identifiers into a nested tuple pattern.

    ```notrust
    arglist(i) -> i
    arglist(i, j) -> (j, i)
    arglist(i, j, k) -> (k, (j, i))
    ```
  */
  #[macro_export]
  macro_rules! arglist(
    (
      $var:ident
    ) => (
      $var
    );
    (
      $var:ident $(, $vars:ident)+
    ) => (
      (arglist!($($vars),+), $var)
    );
  )

  /**
    Main implementation of the `iterator!()` extension.
  */
  #[macro_export]
  macro_rules! iterator_tail(
    (
      (),
      (),
      (
        $map:expr
        for $var:ident in $gen:expr
        $(
          for $vars:ident in $gens:expr
          $(if $filters:expr)*
        )*
      )
    ) => (
      iterator_tail!(
        (
          $gen
        ),
        ($var),
        (
          $map
          $(
            for $vars in $gens
            $(if $filters)*
          )*
        )
      )
    );
    (
      (),
      (),
      (
        $map:expr
        for $var:ident in $gen:expr
        if $filter:expr
        $(
          for $vars:ident in $gens:expr
          $(if $filters:expr)*
        )*
      )
    ) => (
      iterator_tail!(
        (
          $gen
          .filter(|&$var| { $filter } )
        ),
        ($var),
        (
          $map
          $(
            for $vars in $gens
            $(if $filters)*
          )*
        )
      )
    );
    (
      ($head:expr),
      ($($envs:ident),+),
      (
        $map:expr
        for $var:ident in $gen:expr
        $(
          for $vars:ident in $gens:expr
          $(if $filters:expr)*
        )*
      )
    ) => (
      iterator_tail!(
        (
          $head
          .flat_map(|arglist!($($envs),+)| {
            ::std::iter::Repeat::new(arglist!($($envs),+)).zip($gen)
          })
        ),
        ($var $(, $envs)*),
        (
          $map
          $(
            for $vars in $gens
            $(if $filters)*
          )*
        )
      )
    );
    (
      ($head:expr),
      ($($envs:ident),+),
      (
        $map:expr
        for $var:ident in $gen:expr
        if $filter:expr
        $(
          for $vars:ident in $gens:expr
          $(if $filters:expr)*
        )*
      )
    ) => (
      iterator_tail!(
        (
          $head
          .flat_map(|arglist!($($envs),+)| {
            ::std::iter::Repeat::new(arglist!($($envs),+)).zip($gen)
          })
          .filter(|&arglist!($var $(, $envs)+)| { $filter })
        ),
        ($var $(, $envs)*),
        (
          $map
          $(
            for $vars in $gens
            $(if $filters)*
          )*
        )
      )
    );
    (
      ($head:expr),
      ($($envs:ident),+),
      (
        $map:expr
      )
    ) => (
      $head
      .map(|arglist!($($envs),+)| { $map })
    );
  )

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
      (i, j) for i in range(0i, 3i) for j in range(0i, i + 1i) if (i + j) % 2 == 0
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
      $(
        for $vars:ident in $gens:expr
        $(if $filters:expr)*
      )+
    ) => (
      iterator_tail!(
        (),
        (),
        (
          $map
          $(
            for $vars in $gens
            $(if $filters)*
          )+
        )
      )
    );
  )
}

#[cfg(test)]
mod tests {
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
      (i, j) for i in range(0i, 3i) for j in range(0i, i + 1i) if (i + j) % 2 == 0
    ).collect();
    assert_eq!(xs, vec!((0, 0), (1, 1), (2, 0), (2, 2)));
  }

  #[test]
  fn iterator3_map_test() {
    let a = vec!(1i, 2i);
    let b = vec!(3i, 5i);
    let c = vec!(7i, 11i);
    let xs: Vec<int> = iterator!(
      *i * *j * *k for i in a.iter() for j in b.iter() for k in c.iter()
    ).collect();
    assert_eq!(xs, vec!(21, 33, 35, 55, 42, 66, 70, 110));
  }

  #[test]
  fn iterator3_filter_map_test() {
    let xs: Vec<(int, int, int)> = iterator!(
      (i, j, k) for i in range(0i, 10i) for j in range(0i, 10i) for k in range(0i, 10i)
      if i == 1 && j == 2 && k == 3
    ).collect();
    assert_eq!(xs, vec!((1, 2, 3)));
  }

  #[test]
  fn iterator3_empty_test() {
    let xs: Vec<int> = iterator!(
      i * j * k for i in range(0i, 2i) for j in range(0i, i) for k in range(0i, 1i)
    ).collect();
    assert_eq!(xs, vec!(0));
  }

  #[test]
  fn iterator6_test() {
    let mut xs = iterator!(
      (i, j, k, l, m, n)
      for i in range(0i, 5i)
      for j in range(0i, 5i)
      for k in range(0i, 5i)
      for l in range(0i, 5i)
      for m in range(0i, 5i)
      for n in range(0i, 5i)
    );
    assert_eq!(xs.next().unwrap(), (0, 0, 0, 0, 0, 0));
    assert_eq!(xs.next().unwrap(), (0, 0, 0, 0, 0, 1));
    assert_eq!(xs.last().unwrap(), (4, 4, 4, 4, 4, 4));
  }
}
