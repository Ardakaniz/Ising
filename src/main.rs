mod lattice;
mod spin;
mod mcmc;
mod measurement;

use lattice::Lattice;
use mcmc::MCMC;
use measurement::Measurement;

fn main() -> std::io::Result<()> {
	let mut lattice = Lattice::new(mcmc::K_B);
	let mut mcmc = MCMC::new(&mut lattice);
	let mut measurement = Measurement::new(&mut mcmc, "parameters.json");

	use std::time::Instant;
	let start = Instant::now();
	
	measurement.setup();
	measurement.run();

	let elapsed = start.elapsed();
	println!("Total time exceeded: {:?}", elapsed);

	measurement.save("out")?;

	Ok(())
}
