use crate::lattice::{Lattice, LATTICE_SIZE};

use rand::{Rng, rngs::ThreadRng};

pub struct MCMC<'a> {
	lattice: &'a mut Lattice,
	temperature: f64,
	rng: ThreadRng,
}

impl<'a> MCMC<'a> {
	pub fn new(lattice: &'a mut Lattice, temperature: f64) -> Self {
		MCMC {
			lattice,
			temperature,
			rng: rand::thread_rng(),
		}
	}

	pub fn sweep(&mut self) {
		for _ in 0..LATTICE_SIZE*LATTICE_SIZE {
			let (i, j) = self.lattice.flip_random_spin();
			let energy_diff = self.lattice.energy_diff(i, j);

			if energy_diff < 0.0 {
				self.lattice.flip_spin(i, j);
			} else {
				let p = self.rng.gen();

				if (-energy_diff / self.temperature).exp() > p {
					self.lattice.flip_spin(i, j);
				}
			}
		}
	}

	pub fn show_lattice(&self) {
		println!("{}", self.lattice);
	}
}