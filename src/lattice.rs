use crate::spin::Spin;

use rand::{thread_rng, Rng};
use std::fmt;

pub const LATTICE_SIZE: usize = 500;

#[derive(Debug, Clone)]
pub struct Lattice {
	spins: Vec<Spin>,
	j: f64,
	pub h: f64,
}

impl Lattice {
	pub fn new(j: f64) -> Self {
		let mut rng = thread_rng();
		let mut spins: Vec<bool> = vec![false; LATTICE_SIZE * LATTICE_SIZE];
		rng.fill(&mut spins[..]);

		Lattice {
			spins: spins.iter().map(|b| Spin::from(*b)).collect(),
			j,
			h: 0.0, // No magnetic field
		}
	}

	pub fn get_spins(&self) -> &Vec<Spin> {
		&self.spins
	}

	pub fn flip_spin(&mut self, i: usize, j: usize) {
		self.spins[j*LATTICE_SIZE + i].flip();
	}

	pub fn flip_random_spin(&mut self) -> (usize, usize) {
		let mut rng = rand::thread_rng();
		let i = rng.gen_range(0..LATTICE_SIZE);
		let j = rng.gen_range(0..LATTICE_SIZE);

		self.flip_spin(i, j);

		(i, j)
	}

	/* 
		* H = -J Σ(σ_i*σ_j) - h Σ(σ_i)
	*/
	pub fn energy(&self) -> f64 {
		let mut energy = 0.0;

		for y_i in 0..LATTICE_SIZE {
			for x_i in 0..LATTICE_SIZE {
				let spin_i: i32 = self.spins[y_i*LATTICE_SIZE + x_i].into();
				let spin_j_right: i32 = self.spins[y_i * LATTICE_SIZE + (x_i + 1) % LATTICE_SIZE].into();
				let spin_j_up: i32 = self.spins[((y_i + 1) % LATTICE_SIZE) * LATTICE_SIZE + x_i].into();

				energy += self.j * (spin_i * (spin_j_right + spin_j_up)) as f64;

				energy += self.h * spin_i as f64;
			}
		}

		-energy
	}

	pub fn energy_diff(&self, i: usize, j: usize) -> f64 {
		let mut neigbour_spins = 0;

		neigbour_spins += i32::from(self.spins[j * LATTICE_SIZE + (i + 1) % LATTICE_SIZE]);
		neigbour_spins += i32::from(self.spins[((j + 1) % LATTICE_SIZE) * LATTICE_SIZE + i]);

		if i == 0 { // Then i - 1 = LATTICE_SIZE - 1
			neigbour_spins += i32::from(self.spins[j * LATTICE_SIZE + LATTICE_SIZE - 1]);
		} else {
			neigbour_spins += i32::from(self.spins[j * LATTICE_SIZE + i - 1]);
		}

		if j == 0 { // Then j - 1 = LATTICE_SIZE - 1
			neigbour_spins += i32::from(self.spins[(LATTICE_SIZE - 1) * LATTICE_SIZE + i]);
		} else {
			neigbour_spins += i32::from(self.spins[(j - 1) * LATTICE_SIZE + i]);
		}
		
		2.0 * (self.j * (neigbour_spins as f64) + self.h) * i32::from(self.spins[j * LATTICE_SIZE + i]) as f64
	}
}

impl fmt::Display for Lattice {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for i in 0..LATTICE_SIZE {
			for j in 0..LATTICE_SIZE {
				write!(f, "{}", &self.spins[i*LATTICE_SIZE + j])?;
			}
			write!(f, "\n")?;
		}

		Ok(())
	}
}