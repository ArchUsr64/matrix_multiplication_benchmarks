use rand::prelude::*;
use rayon::prelude::*;

use std::time;

type Matrix = Vec<Vec<usize>>;

fn rayon_multiply_steroids(m1: Matrix, m2: Matrix, result: &mut Matrix) {
	let size = m1.len();
	result.par_iter_mut().enumerate().for_each(|(i, row)| {
		row.par_iter_mut()
			.enumerate()
			.for_each(|(j, cell)| (0..size).for_each(|k| *cell += m1[i][k] * m2[k][i]))
	})
}

fn rayon_multiply(m1: Matrix, m2: Matrix, result: &mut Matrix) {
	let size = m1.len();
	result.par_iter_mut().enumerate().for_each(|(i, row)| {
		row.iter_mut()
			.enumerate()
			.for_each(|(j, cell)| (0..size).for_each(|k| *cell += m1[i][k] * m2[k][i]))
	})
}

fn regular_multiply(m1: Matrix, m2: Matrix, result: &mut Matrix) {
	let size = m1.len();
	result.iter_mut().enumerate().for_each(|(i, row)| {
		row.iter_mut()
			.enumerate()
			.for_each(|(j, cell)| (0..size).for_each(|k| *cell += m1[i][k] * m2[k][i]))
	})
}

fn gen_matrix(size: usize) -> Vec<Vec<usize>> {
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
	let size = 1000;
	println!("{}", benchmark(regular_multiply, size));
	println!("{}", benchmark(rayon_multiply, size));
	println!("{}", benchmark(rayon_multiply_steroids, size));
}
