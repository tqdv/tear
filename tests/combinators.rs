// Testing the "combinators" feature
#![cfg(feature = "combinators")]

use tear::prelude::*;
use tear::Judge;
use either::Either::*;

#[test] fn side_works () {
	let v :ValRet<_, ()> = Val(2);

	let v = v.side().map_left(|_| 46).map_right(|x| x * 2);
	assert_eq![ v, Right(4) ];
}
