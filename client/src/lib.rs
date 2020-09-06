#[derive(Eq, PartialEq, Debug)]
pub struct Name {
	pub first_name: String,
	pub last_name: String,
}

pub fn full_name(first: String, last: String) -> Name {
	Name {
		first_name: first,
		last_name: last,
	}
}

mod test {
	use super::*;

	#[test]
	fn full_name_works() {
		assert_eq!(
			full_name(String::from("hello"), String::from("world")), 
			test_utils::helper()
		)
	}
}