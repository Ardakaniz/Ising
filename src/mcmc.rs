use crate::lattice::{Lattice, LATTICE_SIZE};

use rand::{Rng, rngs::ThreadRng, distributions::{Distribution, Uniform}};

pub struct MCMC<'a> {
	lattice: &'a mut Lattice,
	temperature: f64,
	rng: ThreadRng,
	spin_id_distrib: Uniform<usize>,
}

impl<'a> MCMC<'a> {
	pub fn new(lattice: &'a mut Lattice, temperature: f64) -> Self {
		MCMC {
			lattice,
			temperature,
			rng: rand::thread_rng(),
			spin_id_distrib: Uniform::new(0, LATTICE_SIZE*LATTICE_SIZE)
		}
	}

	pub fn sweep(&mut self) {
		for _ in 0..LATTICE_SIZE*LATTICE_SIZE {
			let i = self.spin_id_distrib.sample(&mut self.rng);
			let j = self.spin_id_distrib.sample(&mut self.rng);
			
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