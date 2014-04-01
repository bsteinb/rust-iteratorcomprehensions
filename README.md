# rust-iteratorcomprehensions

A comprehension syntax for rust's Iterators.

```
iterator!( i * j for i in range(0,3) for j in range(0, i + 1) )
```