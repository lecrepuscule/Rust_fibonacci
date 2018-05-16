fn main() {
	println!("1st {}", get_fibs(1));
	println!("2nd {}", get_fibs(2));
	println!("3rd {}", get_fibs(3));
	println!("4th {}", get_fibs(4));
	println!("5th {}", get_fibs(5));
}


fn get_fibs(n: u32) -> u32{
	match n {
		1 | 2 => 1,
		_ => get_fibs(n-1) + get_fibs(n-2)
	}
}