

use rand;

pub struct Perlin2D
{
	seed: u32,
	grad: [u8; 512],
}

impl Perlin2D
{
	pub fn new(seed : u32) -> Perlin2D
	{
		return Perlin2D {seed: seed, grad: [0; 512]};
	}

	pub fn init(&mut self)
	{
		let sd: &[_] = &[self.seed];
		let mut rng: rand::StdRng = rand::SeedableRng::from_seed(sd);

		for i in 0..512
		{
			let ind1: u8 = rand::random();
			let ind2: u8 = rand::random();
		}
	}
}

