// We test the simple twist!

use tear::twist;
use tear::{next, last, resume};
use tear::Looping;
use tear::Judge;

// All compile fail errors go here
#[cfg(not(any(feature = "experimental", feature = "ignore-ui")))] // Feature flags to ignore test
#[test] fn bad_input () {
	use trybuild;
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/twist/*.rs");
}

#[test] fn simple_break() {
	loop {
		twist! { last!() }
		panic!("Should have broken");
	}
}

#[test] fn simple_continue() {
	let mut i = 0;
	loop {
		i += 1;
		if i > 4 {
			break;
		}

		twist! { next!() }
		panic!("Should be skipped over");
	}
	assert_eq![ i, 5 ]
}

#[test] fn simple_resume() {
	let mut i = 0;
	loop {
		i = twist! { resume!(6) };
		break;
	}
	assert_eq![ i, 6 ];
}

#[test] fn simple_breakval() {
	let x = loop {
		twist! { -val Looping::BreakVal { label: None, value: 5 } }
		panic!("Should have broken");
	};
	assert_eq![ x, 5 ];
}

#[test] fn labeled_break() {
	'a: loop {
		loop {
			twist! { -with 'a | last!() }
			panic!("Should have broken");
		}
	}
}

#[test] fn labeled_continue() {
	let mut i = 0;
	'a: loop {
		i += 1;
		loop {
			if i > 2 {
				break 'a;
			}

			twist! { -with 'a | next!() }
			panic!("Should have broken");
		}
	}
	assert_eq![ i, 3 ];
}

#[test] fn labeled_resume() {
	let mut i = 0;
	'a: loop {
		loop {
			i = twist! { -with 'a | resume!(9) };
			break;
		}
		break;
	}
	assert_eq![ i, 9 ];
}

#[test] fn labeled_breakval() {
	let x = 'a: loop {
		loop {
			twist! { -val -with 'a | Looping::BreakVal { label: None, value: 5 } }
		}
	};
	assert_eq![ x, 5 ];
}

/* I'm too lazy to test all possibilities, so we test 2 of them with the mapping syntax */

#[test] fn map_breakval () {
	let x = loop {
		let _ = twist! { -val Err::<i32, _>("failed") => |_| Looping::BreakVal { label: None, value: 8 } };
		break 3;
	};
	assert_eq![ x, 8 ];
}

#[test] fn map_break_labeled () {
	let mut v = 0;
	'a: loop {
		v = twist! { -with 'a | Some(3) => |_| last!() };
		break;
	}
	assert_eq![ v, 3 ];
}