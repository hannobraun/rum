#[allow(ctypes)]
extern {
	fn print_number(x: int);
}

#[start]
fn main(_: int, _: *const *const u8) -> int {
	unsafe {
		print_number(5);
	}

	0
}
