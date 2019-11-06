# About

This is a tiny macro that just checks that an expression conforms to a trait type.

# Motivation

When writing complex futures and streams code at the time when I had to use both futures 0.1 and futures 0.3 libraries,
I often got lot with all the `.compat()` calls. One technique I typically use in other typed languages is to add types
everywhere until I can precisely locate the problem.

The problem is that in rust the types are either sometimes several pages long, or not nameable at all. So ideally what you
want to do is to check that a type conforms to a trait. Naively I tried something like this:

```notrust
let f: impl Future<Output=i32> = something();
```

But that is not valid rust syntax. impl traits can not be used in these positions. Now with this trait you could write the
above assertion as

```rust
let f = is_impl!(dyn Future<Output=i32>, something());
```

A larger example can be found in the [examples](examples/future_compat.rs) and in the documentation.
