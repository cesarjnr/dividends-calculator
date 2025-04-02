use std::io::{self, Write};

fn main() {
	let mut quantity_input = String::new();
	let mut total_received_input = String::new();
	let mut earning_type_input = String::new();

	print!("Type: ");
	io::stdout().flush().unwrap();
	io::stdin()
		.read_line(&mut earning_type_input)
		.expect("Failed to read type");

	print!("Quantity: ");
	io::stdout().flush().unwrap();
	io::stdin()
		.read_line(&mut quantity_input)
		.expect("Failed to read quantity");

	print!("Total Received: ");
	io::stdout().flush().unwrap();
	io::stdin()
		.read_line(&mut total_received_input)
		.expect("Failed to read quantity");

	let earning_type = earning_type_input.trim();
	let quantity: f64 = quantity_input
		.trim()
		.parse()
		.expect("Invalid quantity value");
	let total_received: f64 = total_received_input
		.trim()
		.parse()
		.expect("Invalid total paid value");
	let total_received_before_taxes: f64 = if earning_type == "JCP" { total_received / (1.0 - 0.15) } else { total_received };
	let taxes: f64 = if earning_type == "JCP" { total_received_before_taxes - total_received } else { 0.0 };
	let unit_value = total_received_before_taxes / quantity;

	println!("============================================================");
	println!("Quantity: {quantity}");
	println!("Total Received (Before Taxes): {total_received_before_taxes}");
	println!("Earnings per Share (Before Taxes): {unit_value}");
	println!("Taxes: {taxes}");
	println!("Total Received (After Taxes): {total_received}");
}
