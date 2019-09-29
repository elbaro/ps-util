use num_traits::{Num, Zero};
use rand::distributions::Uniform;
use rand::Rng;
use rand::seq::SliceRandom;


pub struct Range<X> {
	pub low: X,
	pub high: X,
}

pub fn generate_tree<X>(n: usize, weight_range: Option<Range<X>>)
where
	X: Num + rand::distributions::uniform::SampleUniform + std::fmt::Display,
{
	let range = Uniform::new(1, n + 1);
	let binary = Uniform::new(0, 2);
	let mut rng = rand::thread_rng();

	let prufer: Vec<usize> = (0..n - 2).map(|_| rng.sample(range)).collect();
	let mut degree = vec![1; n + 1];
	for &v in &prufer {
		degree[v] += 1;
	}

	let mut edges: Vec<(usize, usize)> = Vec::with_capacity(n - 1);

	for &v in &prufer {
		for other in 1..n + 1 {
			if degree[other] == 1 {
				edges.push((v, other));
				degree[v] -= 1;
				degree[other] -= 1;
				break;
			}
		}
	}

	for v in 1..n + 1 {
		if degree[v] == 1 {
			for u in v + 1..n + 1 {
				if degree[u] == 1 {
					edges.push((v, u));
					break;
				}
			}
			break;
		}
	}

	(&mut edges[..]).shuffle(&mut rng);
	for e in &mut edges {
		if rng.sample(&binary) == 0 {
			std::mem::swap(&mut e.0, &mut e.1);
		}
	}

	println!("{}", n);
	match weight_range {
		Some(range) => {
			let dist = Uniform::new(range.low, range.high);
			for e in edges {
				let w = rng.sample(&dist);
				println!("{} {} {}", e.0, e.1, w);
			}
		}
		None => {
			for e in edges {
				println!("{} {}", e.0, e.1);
			}
		}
	}
}

// valtr algo
// http://cglab.ca/~sander/misc/ConvexGeneration/convex.html
pub fn generate_convex<X>(n: usize, coord_range: Range<X>)
where
	X: Num
		+ rand::distributions::uniform::SampleUniform
		+ std::fmt::Display
		+ std::cmp::PartialOrd
		+ num_traits::Signed
		+ std::ops::AddAssign
		+ num_traits::AsPrimitive<i8>,
{
	assert!(n >= 3);

	let coord_dist = Uniform::new(coord_range.low, coord_range.high);
	let mut rng = rand::thread_rng();

	// random points in square -> O(n^n) trials in average
	let generate_chains = &|| -> (X, Vec<X>) {
		let binary = Uniform::new(0, 2);
		let mut rng = rand::thread_rng();
		// return n vectors that sums up to 0
		let mut a: Vec<X> = (0..n).map(|_| rng.sample(&coord_dist)).collect();
		a.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
		let mut chains: Vec<X> = Vec::with_capacity(n);
		let mut last1 = 1;
		let mut last2 = 1;
		for i in 2..n {
			if rng.sample(binary) == 0 {
				chains.push(a[i] - a[last1]);
				last1 = i;
			} else {
				chains.push(a[last2] - a[i]);
				last2 = i;
			}
		}
		chains.push(a[n - 1] - a[last1]);
		chains.push(a[last2] - a[n - 1]);
		(a[1], chains)
	};

	let (minx, mut xs) = generate_chains();
	let (miny, ys) = generate_chains();

	(&mut xs[..]).shuffle(&mut rng);
	// rng.shuffle(&mut xs);
	// rng.shuffle(&mut ys);
	let mut vec: Vec<(X, X)> = (0..n).map(|i| (xs[i], ys[i])).collect();

	// if a vec is (0,0), it should be randomly inserted later

	// sort (xs[i], ys[i]) by angle
	vec.sort_unstable_by(|&(x1, y1), &(x2, y2)| {
		let sx1 = if x1.is_positive() {
			1
		} else if x1.is_negative() {
			3
		} else if y1.is_positive() {
			2
		} else {
			4
		};
		let sx2 = if x2.is_positive() {
			1
		} else if x2.is_negative() {
			3
		} else if y2.is_positive() {
			2
		} else {
			4
		};
		if sx1 != sx2 {
			return sx1.cmp(&sx2);
		}
		if sx1.is_zero() {
			return y2.partial_cmp(&y1).unwrap();
		}
		return (y1 * x2).partial_cmp(&(y2 * x1)).unwrap();
	});

	// how much shift y
	let mut y = X::zero();
	let mut my = X::zero();
	for i in 0..n {
		y += vec[i].1;
		if y < my {
			my = y;
		}
	}

	// shift my -> miny
	// let ret:Vec<(,)> = Vec::with_capacity(n);
	let mut x = minx;
	y = miny - my;
	println!("{}", n);
	for i in 0..n {
		// ret.push((x,y));
		println!("{} {}", x, y);
		// println!("+= {} {}", vec[i].0, vec[i].1);
		x += vec[i].0;
		y += vec[i].1;
	}

	// assert!((x-minx).is_zero(), "sum of vector x = {} {} ", x,minx);
	// assert!((y-(miny-my)).is_zero(), "sum of vector y = {} {} ", y,miny-my);
	// 3.0644979766624543
	// 3.0644979766624516
}
