fn main() {
	{
		let mut s = String::from("hello");
		println!("{s}");

		s.push_str(", world!"); // push_str() appends a literal to a String

		println!("{s}");
	}
}
