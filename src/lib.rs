//! a macro to check that a value conforms to a type

/// Checks that an expression `ex` conforms to type `type` at compile time. This should have no runtime cost
/// and can be used with trait types when adding `dyn`.
///
/// ## Short example: check inline that a value implements the `Debug` trait
/// ```rust
///# use is_impl::*;
///# use std::fmt::Debug;
/// let x = is_impl!(dyn Debug, 2);
/// ```
///
/// ## More elaborate example: making sense of a complex future pipeline
/// ```
/// # extern crate futures;
/// # extern crate futures01;
/// extern crate is_impl;
/// # use crate::futures::compat::Future01CompatExt;
/// # use crate::futures::future::Future as Future03;
/// # use crate::futures::FutureExt;
/// # use crate::futures::TryFutureExt;
/// # use crate::futures01::future::Future as Future01;
/// use is_impl::*;
/// # // some futures 0.1 library that spawns a future
/// # fn spawn<E>(_value: impl futures01::Future<Item = E>) {}
/// # // some futures 0.1 library that makes a request
/// # fn request(_test: &str) -> impl futures01::Future<Item = i32, Error = ()> {
/// #     futures01::future::ok(1)
/// # }
/// # // some futures 0.3 library
/// # fn transform(
/// #     x: impl futures::future::Future<Output = i32>,
/// # ) -> impl futures::future::Future<Output = i32> {
/// #     x.map(|value| value * 2)
/// # }
///
/// // assert (inline) the future type
/// let req = is_impl!(dyn Future01<Item=i32, Error=()>, request("something"));
/// // assert (inline) that we are in new futures and have successfully gotten rid of the unit error
/// let new_future = is_impl!(dyn Future03<Output = i32>, req.compat().into_future().map(|x| x.unwrap()));
/// // assert (inline) the transformed type
/// let transformed = is_impl!(dyn Future03<Output = i32>, transform(new_future));
/// // assert (inline) that we are back in futures 0.1 land with an error
/// let res = is_impl!(dyn Future01<Item=i32, Error=()>, transformed.unit_error().compat());
/// // do the thing!
/// spawn(res);
/// ```
///
/// Usually I use `is_impl!` to figure out where the types don't align during development, and then remove the assertions
/// again once I am happy with the code. But since the macro does not have a runtime cost you can also leave them in.
///
#[macro_export]
macro_rules! is_impl {
    ($type:ty, $ex:expr) => {{
        if false {
            let _: Box<$type> = Box::new($ex);
            unreachable!();
        }
        $ex
    }};
}

#[cfg(test)]
mod tests {

    trait A {}

    struct Foo;

    // struct Bar;

    impl A for Foo {}
    #[test]
    fn basic_usage() {
        let foo: Foo = Foo;
        let _foo: Foo = is_impl!(Foo, foo);
        // does not compile
        // let bar: Bar = Bar;
        // let bar: Bar = is_impl!(Foo, bar);
    }

}
