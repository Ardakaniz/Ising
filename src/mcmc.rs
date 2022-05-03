use crate::lattice::{Lattice, LATTICE_SIZE};

use rand::{Rng, rngs::ThreadRng, distributions::{Distribution, Uniform}};


pub const BOHR_MAGNETON: f64 = 1.0;//5.7883817555e-5;
pub const K_B: f64 = 1.0;//8.617333262145e-5;

pub struct MCMC<'a> {
	lattice: &'a mut Lattice,
	
	// Parameters
	pub temperature: f64,

	// Random utils
	rng: ThreadRng,
	spin_id_distrib: Uniform<usize>,
}

impl<'a> MCMC<'a> {
	pub fn new(lattice: &'a mut Lattice) -> Self {
		MCMC {
			lattice,

			temperature: 0.0,

			rng: rand::thread_rng(),
			spin_id_distrib: Uniform::new(0, LATTICE_SIZE)
		}
	}

	pub fn set_external_field(&mut self, external_field: f64) {
		self.lattice.h = BOHR_MAGNETON * external_field;
	}

	pub fn sweep(&mut self) {
		for _ in 0..LATTICE_SIZE*LATTICE_SIZE {
			let i = self.spin_id_distrib.sample(&mut self.rng);
			let j = self.spin_id_distrib.sample(&mut self.rng);
			
			let energy_diff = self.lattice.energy_diff(i, j);

			if energy_diff < 0.0 {
				self.lattice.flip_spin(i, j);
			} else {
				let p = self.rng.gen();
				let factor = (-energy_diff / (K_B * self.temperature)).exp();
				if factor > p {
					self.lattice.flip_spin(i, j);
				}
			}
		}
	}

	pub fn show_lattice(&self) {
		println!("{}", self.lattice);
	}

	pub fn get_lattice(&self) -> &Lattice {
		self.lattice
	}
}