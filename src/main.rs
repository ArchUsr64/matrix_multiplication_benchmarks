use rand::prelude::*;
use rayon::prelude::*;

use std::time;

type Matrix = Vec<Vec<i32>>;

#[link(name = "main")]
extern "C" {
	fn matrix_multiply_c_ffi(
		m1: *const cty::c_int,
		m2: *const cty::c_int,
		result: *mut cty::c_int,
		size: cty::c_uint,
	) -> cty::c_uint;
}

fn rayon_multiply_steroids(m1: Matrix, m2: Matrix, result: &mut Matrix) {
	let size = m1.len();
	result.par_iter_mut().enumerate().for_each(|(i, row)| {
		row.par_iter_mut()
			.enumerate()
			.for_each(|(j, cell)| (0..size).for_each(|k| *cell += m1[i][k] * m2[k][j]))
	})
}

fn rayon_multiply(m1: Matrix, m2: Matrix, result: &mut Matrix) {
	let size = m1.len();
	result.par_iter_mut().enumerate().for_each(|(i, row)| {
		row.iter_mut()
			.enumerate()
			.for_each(|(j, cell)| (0..size).for_each(|k| *cell += m1[i][k] * m2[k][j]))
	})
}

fn regular_multiply(m1: Matrix, m2: Matrix, result: &mut Matrix) {
	let size = m1.len();
	result.iter_mut().enumerate().for_each(|(i, row)| {
		row.iter_mut()
			.enumerate()
			.for_each(|(j, cell)| (0..size).for_each(|k| *cell += m1[i][k] * m2[k][j]))
	})
}

fn gen_matrix(size: usize) -> Matrix {
	let mut result = vec![vec![0; size]; size];
	(0..size).for_each(|i| (0..size).for_each(|j| result[i][j] = random()));
	result
}

fn benchmark(method: fn(Matrix, Matrix, &mut Matrix), size: usize) -> u128 {
	let m1 = gen_matrix(size);
	let m2 = gen_matrix(size);
	let mut result = vec![vec![0; size]; size];
	let start = time::Instant::now();
	method(m1, m2, &mut result);
	start.elapsed().as_nanos()
}

fn main() {
	let size = 10;
	println!("{}", benchmark(regular_multiply, size));
	println!("{}", benchmark(rayon_multiply, size));
	println!("{}", benchmark(rayon_multiply_steroids, size));
	let mut result = vec![vec![0; size]; size];
	unsafe {
		let x = matrix_multiply_c_ffi(
			gen_matrix(size)[0].as_ptr(),
			gen_matrix(size)[0].as_ptr(),
			result[0].as_mut_ptr(),
			size as u32,
		);
		println!("{result:?}");
		println!("{x:?}");
	}
}
