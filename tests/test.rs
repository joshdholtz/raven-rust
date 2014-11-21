extern crate raven;

use raven::raven::Raven;

#[test]
fn this_tests_raven() {

	let r = Raven::new( String::from_str("") );
	r.capture();

	//http://doc.rust-lang.org/std/macro.assert_eq!.html
	//panic!("Fail");
}
