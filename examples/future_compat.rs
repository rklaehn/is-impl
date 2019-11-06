extern crate futures;
extern crate futures01;
extern crate is_impl;

use crate::futures::compat::Future01CompatExt;
use crate::futures::future::Future as Future03;
use crate::futures::FutureExt;
use crate::futures::TryFutureExt;
use crate::futures01::future::Future as Future01;
use is_impl::*;

// some futures 0.1 library that spawns a future
fn spawn<E>(_value: impl futures01::Future<Item = E>) {}

// some futures 0.1 library that makes a request
fn request(_test: &str) -> impl futures01::Future<Item = i32, Error = ()> {
    futures01::future::ok(1)
}

// some futures 0.3 library
fn transform(
    x: impl futures::future::Future<Output = i32>,
) -> impl futures::future::Future<Output = i32> {
    x.map(|value| value * 2)
}

fn main() {
    // we get a 0.1 future from some lib
    let req = request("something");
    // we go into futures 0.3 land, get rid of the unit error,
    // do something with the future, then add an unit error again
    // and go back into futures 0.1 land to run the result
    //
    // looks easy enough, but if you somehow mess up the types you get a
    // wall of errors!
    let res = transform(req.compat().into_future().map(|x| x.unwrap()))
        .unit_error()
        .compat();
    spawn(res);

    // assert (inline) the future type
    let req = is_impl!(dyn Future01<Item=i32, Error=()>, request("something"));
    // assert (inline) that we are in new futures and have successfully gotten rid of the unit error
    let new_future = is_impl!(
        dyn Future03<Output = i32>,
        req.compat().into_future().map(|x| x.unwrap())
    );
    // assert (inline) the transformed type
    let transformed = is_impl!(dyn Future03<Output = i32>, transform(new_future));
    // assert (inline) that we are back in futures 0.1 land with an error
    let res = is_impl!(dyn Future01<Item=i32, Error=()>, transformed
        .unit_error()
        .compat());
    // do the thing!
    spawn(res);
}
