use client::Name;

pub fn helper() -> Name {
	Name {
		first_name: String::from("hello"),
		last_name: String::from("world"),
	}
}