// all about formatted print syntax
fn main() {
	// in general the {} will be auto replaced with any arguments. These will be stringified.
	println!("{} days", 31);	// number is i32 by default
	println!("{} million using undy separators", 31_000_000i64);	// number becomes type i64
	println!("{} degrees Centigrade", 39.1f64);	// floating point number now
    // using positional arguments/parameters
	println!("{0}, this is {1}, {1}, meet {0}", "Alice", "Bob");
	// using named arguments
	println!("{subject} {verb} {object}", object="the lazy dog",subject="the quick brown fox", verb="jumps over" );
	
	// special formatting can be specified after the colon symbol :
	// in this case b is binary number
	println!("{} of {:b} people know binary, the half doesn't", 1, 85);

	// you can right-align text with specified width. this will output spaces or depending on the specified width
	println!("space padding {number:>width$}", number=1, width=6);
	println!("zero padding {number:>0width$}", number=1, width=6);
	println!("My name is {0}, {1} {0}", "Bond", "James");

	// create a struct which contains an i32
	#[derive(Debug)]	// needed to print the struct
	struct Structure(i32);

	// try to print the struct
	// println!("This struct `{}` won't print...", Structure(3));
	println!("This special formatter can now print {:?}!", Structure(3));

}
