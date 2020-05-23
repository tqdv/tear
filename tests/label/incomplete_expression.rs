use tear::twist;

fn bad_expression() {
	// It is a compiler message because we can't detect if the expression is incomplete
	twist! { -label 'a | 1 + }
}

fn main () {}