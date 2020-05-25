// We test the twist! -label syntax

use tear::twist;
use tear::{next, last, resume};
use tear::anybox;
use tear::Looping;

type L = Looping<i32, ()>;

const JUST_BREAK :L = Looping::Break { label: None };
const BREAK_0 :L = Looping::Break { label: Some(0) };

// All compile fail errors go here
#[cfg(not(any(feature = "experimental", feature = "ignore-ui")))] // Feature flags to ignore test
#[test] fn bad_input () {
	use trybuild;
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/label/*.rs");
}

#[test] fn just_break () {
	let mut x = 0;
	'a: loop {
		loop {
			twist! { -label 'a | JUST_BREAK }
			panic!("Should break before this");
		}
		x = 1;
		break;
	}
	assert_eq![ x, 1, "Only broke the innermost loop" ];
}

#[test] fn break_label () {
	'a: loop {
		loop {
			twist! { -label 'a | BREAK_0 }
			panic!("Should break before this");
		}
		panic!("Didn't break the label")
	}
}

#[test] fn resume () {
	let mut x :i32 = 5;
	'a: loop {
		// This can't infer B type, so we use resume!() instead
		// x = twist! { -label 'a | Looping::Resume(1) };
		x = twist! { -label 'a | resume!(1) };
		break;
	}
	assert_eq![ x, 1 ];
}

#[test] fn continue_loop () {	
	let mut x :i32 = 0;
	'a: loop {
		x += 1;

		// This can't infer B type, so we use next!() instead
		twist! { -label 'a |
			if x < 4 { next!() }
			else { last!() }
		}
	}
	assert_eq![ x, 4 ];
}

#[test] fn continue_label () {
	let mut x :i32 = 0;
	'a: loop {
		x += 1;
		// This can't infer B type, so we use next!() instead
		twist! { -label 'a |
			if x < 4 { next!(0) }
			else { last!() }
		}
		x -= 1;
	}
	assert_eq![ x, 4 ];
}

#[test] fn break_label_two () {
	'a: loop {
		'b: loop {
			twist! { -label 'a, 'b | last!(0) }
			panic!("Should break before this");
		}
		panic!("Didn't break the label")
	}
}

#[test] fn breakval () {
	let x = 'a: loop {
		'b: loop {
			twist! { -label 'a :i32, 'b | Looping::BreakVal { label: Some(0), value: 8 } }
			panic!("Should break before this");
		}
		panic!("Didn't break the label")
	};
	assert_eq![ x, 8 ];
}

#[test] fn breakval_multiple () {
	let mut y = 0;
	let x = 'a: loop {
		let z :i32 = 'b: loop {
			loop {
				y += 1;
				twist! { -label 'a :i32, 'b :i32 |
					if y > 5 { Looping::BreakVal { label: Some(0), value: 8 } }
					else { Looping::BreakVal { label: Some(1), value: 3 } }
				}
				y -= 1;
			}
		};
		assert_eq![ z, 3 ];
	};
	assert_eq![ y, 6 ];
	assert_eq![ x, 8 ];
}

#[test] fn break_and_breakval () {
	let mut y = 0;
	let mut a = 0;
	let x = 'a: loop {
		'c: loop {
			let z = 'b: loop {
				'd: loop {
					let v = twist! { -label 'a :i32, 'c, 'b :i32, 'd |
						if y < 5 { Looping::Resume (6) }
						else if a < 8 { Looping::Break { label: Some(3) } }
						else if y == 5 { y += 1; Looping::BreakVal { label: Some(2), value: 3 } }
						else { Looping::BreakVal { label: Some(0), value: 4 } }
					};
					assert_eq![ v, 6 ]; println!("1/5");
					y += 1;
				}
				a += 1;
			};
			assert_eq![ z, 3 ]; println!("2/5");
		}
	};
	assert_eq![ y, 6 ]; println!("3/5");
	assert_eq![ x, 4 ]; println!("4/5");
	assert_eq![ a, 8 ]; println!("5/5");
}

#[test] fn innermost_breakval () {
	let mut c = 0;
	let v = 'v: loop {
		'a: loop {
			let x = loop {
				twist! { -val i32, -label 'a, 'v :i32 |
					if c < 3 { Looping::BreakVal { label: None, value: 0 } }
					else if c == 3 { c += 1; Looping::Break { label: Some(0) } }
					else { Looping::BreakVal { label: Some(1), value: 7 } }
				}
			};
			assert_eq![ x, 0 ]; println!("1/3");
			c += 1;
		}
	};
	assert_eq![ v, 7 ]; println!("2/3");
	assert_eq![ c, 4 ]; println!("3/3");
}

#[test] fn anybox () {
	struct S { d :i32 }
	
	let x = anybox!(S { d: 5 });
	let s = match x.downcast::<S>() {
		Ok(v) => *v,
		_ => panic!("Failed to get our S back"),
	};
	
	assert_eq![ s.d, 5 ];
}

#[test] fn box_breakval () {
	let mut i = 0;
	let mut f = || {
		let ii = i;
		i += 1;
		if ii == 0 { Looping::BreakVal { label: Some(1), value: anybox!(2) } }
		else if ii == 1 { Looping::BreakVal { label: Some(2), value: anybox!("yeah".to_string()) } }
		else { Looping::Break { label: Some(0) } }
	};
	
	'a: loop {
		let b = 'b: loop {
			let c = 'c: loop {
				loop {
					twist! { -box -label 'a, 'b :i32, 'c :String | f() }
					break;
				}
			};
			assert_eq![ c, "yeah".to_string() ]; println!("1/2");
		};
		assert_eq![ b, 2 ]; println!("2/2");
	}
}

#[test] fn box_breakval_innermost () {
	use std::any::Any;
	fn create_closure () -> impl FnMut() -> Looping<(), Box<dyn Any>> {
		let mut i = 0;
		
		move || {
			let v = match i {
				x if x == 0 => Looping::BreakVal { label: None, value: anybox!(0) },
				x if x == 1 => Looping::Break { label: Some(0) },
				_ => unreachable!(),
			};
			i += 1;
			v
		}
	}
	
	let mut f = create_closure();
	
	'a: loop {
		let v = loop {
			twist! { -box -val i32, -label 'a | f() }
		};
		assert_eq![ v, 0 ]; println!("1/1");
	}
}

/* Too lazy to test more than one example for map syntax */

#[test] fn breakval_multiple_map () {
	use tear::Judge;

	let v :i32 = 'a: loop {
		'b: loop {
			let x = twist! { -label 'a :i32, 'b | Some(4) => |_| Looping::BreakVal { label: Some(0), value: 0 } };
			break 'a (x * 2);
		}
		break 3;
	};
	assert_eq![ v, 8 ];
}