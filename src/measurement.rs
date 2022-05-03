use crate::mcmc::MCMC;
use crate::lattice::LATTICE_SIZE;
 
use serde::Deserialize;
use serde_json::Value;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Parameters {
	// Outputs
	spins: bool,
	energy: bool,
	magnetization: bool,

	// Inputs
	field: Vec<f64>,
	temp: Vec<f64>,
}
pub struct Measurement<'a> {
	mcmc: &'a mut MCMC<'a>,

	params: Parameters,
	measurement_count: usize,
	sweep_per_measure: usize,

	spins: Vec<Vec<bool>>,
	energies: Vec<f64>,
	magnetization: Vec<f64>,
}

impl<'a> Measurement<'a> {
	pub fn new(mcmc: &'a mut MCMC<'a>, parameters_filepath: &str) -> Self {
		// Parse parameters
		let mut parameters_file = File::open(parameters_filepath).expect("Could not open parameters file");

		let mut parameters_string = String::new();
		parameters_file.read_to_string(&mut parameters_string).expect("Could not read parameters file");

		let json: Value = serde_json::from_str(&parameters_string).expect("Could not parse parameters file");
		let mut measurement_count = json["measurement_count"].as_u64().unwrap_or(0) as usize; 
		let mut sweep_per_measure = json["sweep_per_measure"].as_u64().unwrap_or(1) as usize;

		let params: Parameters = serde_json::from_value(json).expect("Could not parse parameters file");

		if measurement_count == 0 {
			measurement_count = params.field.len().max(params.temp.len());

			println!("Measurement count: {}", measurement_count);
		}
		if sweep_per_measure == 0 {
			sweep_per_measure = 1;

			println!("Sweep per measure: {}", sweep_per_measure);
		}
		
		Measurement { 
			mcmc,

			params,
			measurement_count,
			sweep_per_measure,

			spins: vec![],
			energies: vec![],
			magnetization: vec![],
		}
	}

	pub fn setup(&mut self) {

		self.mcmc.set_external_field(
			*self.params.field
				.get(0)
				.unwrap_or(&0.0)
		);

		self.mcmc.temperature = 
			*self.params.temp
			.get(0)
			.unwrap_or(&1.0);

		for _ in 0..20 {
			self.mcmc.sweep();
		}
		self.store_data();
	}

	pub fn run(&mut self) {
		let mut last_field_set = false;
		let mut last_temp_set = false;
		
		for i in 1..self.measurement_count {
			if !last_field_set {
				self.mcmc.set_external_field(
						*self.params.field
						.get(i)
						.unwrap_or_else(|| {
							last_field_set = true;
							&0.0
						})
				);
			}

			if !last_temp_set {
				self.mcmc.temperature = 
					*self.params.temp
					.get(i)
					.unwrap_or_else(|| {
						last_temp_set = true; 
						&1.0
					});
				}
			
			for _ in 0..self.sweep_per_measure {
				self.mcmc.sweep();
			}
			self.store_data();
		}
	}

	pub fn save(&self, folder: &str)-> std::io::Result<()> {
		let spin_filepath   = folder.to_owned() + "/spins.dat";
		let energy_filepath = folder.to_owned() + "/energies.dat";
		let mag_filepath    = folder.to_owned() + "/magnetization.dat";

		let mut spins_file = File::create(spin_filepath)?;
		let mut energies_file = File::create(energy_filepath)?;
		let mut mag_file = File::create(mag_filepath)?;
		
		//file.write_all((self.measurement_idx.to_string() + "\n").as_bytes())?;

		for i in 0..self.measurement_count {
			if self.params.spins {
				let bytes = self.spins[i].iter().map(|b| *b as u8).collect::<Vec<_>>();
				spins_file.write_all(&bytes)?;
			}

			if self.params.energy {
				let energy_str = self.energies[i].to_string() + "\n";
				energies_file.write_all(&energy_str.as_bytes())?;
			}

			if self.params.magnetization {
				let mag_str = self.magnetization[i].to_string() + "\n";
				mag_file.write_all(&mag_str.as_bytes())?;
			}
		}

		Ok(())
	}

	fn store_data(&mut self) {
		if self.params.spins {
			self.spins.push(
				self.mcmc.get_lattice().get_spins()
				.iter()
				.map(|x| bool::from(*x))
				.collect()
			);
		}

		if self.params.energy {
			self.energies.push(self.mcmc.get_lattice().energy());
		}

		if self.params.magnetization {
			self.magnetization.push(
				self.mcmc.get_lattice().get_spins().iter().map(|x| i32::from(*x)).sum::<i32>() as f64 / (LATTICE_SIZE * LATTICE_SIZE) as f64
			);
		}
	}
}