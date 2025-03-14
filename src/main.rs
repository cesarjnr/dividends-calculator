use std::io::{self, Write};

fn main() {
	let mut quantity_input = String::new();
	let mut total_paid_input = String::new();

	print!("Quantity: ");
	io::stdout().flush().unwrap();
	io::stdin()
		.read_line(&mut quantity_input)
		.expect("Failed to read quantity");

	print!("Total Paid: ");
	io::stdout().flush().unwrap();
	io::stdin()
		.read_line(&mut total_paid_input)
		.expect("Failed to read quantity");

	let quantity: f64 = quantity_input
		.trim()
		.parse()
		.expect("Invalid quantity value");
	let total_paid: f64 = total_paid_input
		.trim()
		.parse()
		.expect("Invalid total paid value");
	let unit_value = total_paid / quantity;

	println!("Unit value: {unit_value}");
}
