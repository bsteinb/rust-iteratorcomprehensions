#![feature(phase)]

#[cfg(not(test))]
#[phase(plugin,link)]
extern crate iteratorcomprehensions;

#[cfg(not(test))]
use std::fmt::{Show};

#[cfg(not(test))]
fn dump<T: Show, I: Iterator<T>>(it: I) {
  let mut it = it;
  it.next().map(|x| { print!("{}", x); });
  for x in it {
    print!(", {}", x);
  }
  println!("");
}

#[cfg(not(test))]
fn main() {
  dump(iterator!(i for i in range(0i, 10i)));

  dump(iterator!(i * i for i in range(0i, 10i)));

  let a = 10;
  dump(iterator!(a * i for i in range(0i, 10i)));

  dump(iterator!(-x for x in range(-10i, 10i) if x <= 0));

  dump(iterator!(i for i in range(0i, 5i)));

  dump(iterator!(i * j for i in range(1i, 4i) for j in range(1i, 4i)));

  dump(iterator!(i + j for i in range(0i, 1000i) for j in range(0i, 1000i) if (i, j) == (999, 999)));

  let a = vec!( 1i, 3i, 3i, 7i  );
  let b = vec!( 4i, 2i );
  dump(iterator!( (i, j) for i in b.iter() for j in a.iter() ));

  dump(iterator!( (i, j) for i in range(0i, 0i) for j in range(0i, 1i) ));

  dump(iterator!( (i, j) for i in range(1i, 4i) for j in range(1i, i + 1i) ));

  dump(iterator!( (i, j, k) for i in range(0i, 3i) for j in range(0i, 3i) for k in range(0i, 3i) ));
  dump(iterator!( (i, j, k) for i in range(0i, 3i) for j in range(0i, 3i) for k in range(0i, 0i) ));
}
