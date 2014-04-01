#[feature(phase)];

#[crate_id = "examples"];

#[phase(syntax,link)]
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
  dump(iterator!(i for i in range(0, 10)));

  dump(iterator!(i * i for i in range(0, 10)));

  let a = 10;
  dump(iterator!(a * i for i in range(0, 10)));

  dump(iterator!(-x for x in range(-10, 10) if x <= 0));

  dump(iterator!(i for i in range(0, 5)));

  dump(iterator!(i * j for i in range(1, 4) for j in range(1, 4)));

  dump(iterator!(i + j for i in range(0, 1000) for j in range(0, 1000) if (i, j) == (999, 999)));

  let a = ~[ 1, 3, 3, 7 ];
  let b = ~[ 4, 2 ];
  dump(iterator!( (i, j) for i in b.iter() for j in a.iter() ));

  dump(range(0, 0));
  dump(iterator!( (i, j) for i in range(0, 0) for j in range(0, 1) ));

  dump(iterator!( (i, j) for i in range(1, 4) for j in range(1, i + 1) ));

  dump(iterator!( (i, j, k) for i in range(0, 3) for j in range(0, 3) for k in range(0, 3) ));
  dump(iterator!( (i, j, k) for i in range(0, 3) for j in range(0, 3) for k in range(0, 0) ));
}
