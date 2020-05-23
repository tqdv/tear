use tear::twist;

fn bad_label_syntax() {
	twist! { -label 'a 'b | 1 }
}

fn main () {}