extern crate pct_str;

use pct_str::{PctString, URIReserved};

struct CustomEncoder;

impl pct_str::Encoder for CustomEncoder {
	fn encode(&self, c: char) -> bool {
		URIReserved.encode(c) || c.is_uppercase()
	}
}

fn main() {
	// You can encode any string into a percent-encoded string
	// using the [`PctString::encode`] function.
	// It takes a `char` iterator and a [`Encoder`] instance deciding which
	// characters to encode.
	let pct_string = PctString::encode("Hello World!".chars(), URIReserved);
	// [`URIReserved`] is a predefined encoder for URI-reserved characters.
	assert_eq!(pct_string.as_str(), "Hello World%21");

	// You can create your own encoder by implementing the [`Encoder`] trait.
	let pct_string = PctString::encode("Hello World!".chars(), CustomEncoder);
	println!("{}", pct_string.as_str());
	assert_eq!(pct_string.as_str(), "%48ello %57orld%21");

	// You can also use any function implementing `Fn(char) -> bool`.
	let pct_string = PctString::encode("Hello World!".chars(), char::is_uppercase);
	assert_eq!(pct_string.as_str(), "%48ello %57orld!");
}
