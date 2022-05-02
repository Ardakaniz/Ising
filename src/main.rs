mod lattice;
mod mcmc;
mod spin;

use lattice::Lattice;
use mcmc::MCMC;

fn main() {
	println!("Hello, world!");

	let mut lattice = Lattice::new();
	let mut mcmc = MCMC::new(&mut lattice, 1.0);
	
	mcmc.show_lattice();
	for _ in 0..100 {
		mcmc.sweep();
	}
	mcmc.show_lattice();
}
