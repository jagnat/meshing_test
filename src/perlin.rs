
pub struct Perlin2D
{
	seed: u64,
	xor: u64,
	grad: [u8; 512],
}

impl Perlin2D
{
	pub fn new(seed : u64) -> Perlin2D
	{
		return Perlin2D {seed: seed, xor: seed, grad: [0; 512]};
	}

	pub fn init(&mut self)
	{
		self.xor = if self.seed == 0 { 0xbaadf00d }
		else { self.seed * 12907 };

		for i in 0..256
		{
			self.grad[i] = i as u8;
			self.grad[i + 256] = i as u8;
		}

		for i in 0..512
		{
			let r = (self.xs_64() % 512) as usize;
			let swap : u8 = self.grad[i];
			self.grad[i] = self.grad[r];
			self.grad[r] = swap;
		}
	}

	fn xs_64(&mut self) -> u64
	{
		self.xor ^= self.xor >> 12;
		self.xor ^= self.xor << 25;
		self.xor ^= self.xor >> 27;
		return self.xor.wrapping_mul(2685821657736338717_u64);
	}

	fn pgrad(&self, hash: u32, x: f64, y: f64, z: f64) -> f64
	{
		let h = hash & 15;
		let mut u = if h < 8  { x } else { y };
		let mut v : f64;
		if h < 4 { v = y; }
		else if h == 12 || h == 14 { v = x; }
		else { v = z; }

		if (h & 1) != 0 { u = -u; }
		if (h & 2) != 0 { v = -v; }

		return u + v;
	}

	#[allow(dead_code)]
	fn plerp(a: f64, b: f64, x: f64) -> f64
	{
		return a + (b - a) * x;
	}

	fn pfade(x: f64) -> f64
	{
		return x * x * x * (x * (x * 6. - 15.) + 10.);
	}

	pub fn perl3d(&self, x: f64, y: f64, z: f64) -> f64
	{
		let xi = (x as usize) & 255;
		let yi = (y as usize) & 255;
		let zi = (z as usize) & 255;

		let xf : f64 = x - (x as i32 as f64);
		let yf : f64 = y - (y as i32 as f64);
		let zf : f64 = z - (z as i32 as f64);
		
		let u = Perlin2D::pfade(xf);
		let v = Perlin2D::pfade(yf);
		let w = Perlin2D::pfade(zf);

		let i = self.grad[xi] as usize + yi;
		let ii = self.grad[i] as usize + zi;
		let ij = self.grad[i + 1] as usize + zi;
		let j = self.grad[xi + 1] as usize + yi;
		let ji = self.grad[j] as usize + zi;
		let jj = self.grad[j + 1] as usize + zi;
		
		let p = self.grad;

		let a = self.pgrad(p[ii].into(), xf, yf, zf);
		let b = self.pgrad(p[ji].into(), xf - 1.0, yf, zf);
		let c = self.pgrad(p[ij].into(), xf, yf - 1.0, zf);
		let d = self.pgrad(p[jj].into(), xf - 1.0, yf - 1.0, zf);
		let e = self.pgrad(p[ii + 1].into(), xf, yf, zf - 1.0);
		let f = self.pgrad(p[ji + 1].into(), xf - 1.0, yf, zf - 1.0);
		let g = self.pgrad(p[ij + 1].into(), xf, yf - 1.0, zf - 1.0);
		let h = self.pgrad(p[jj + 1].into(), xf - 1.0, yf - 1.0, zf - 1.0);

		let k0 = a;
		let k1 = b - a;
		let k2 = c - a;
		let k3 = e - a;
		let k4 = a - b - c + d;
		let k5 = a - c - e + g;
		let k6 = a - b - e + f;
		let k7 = -a + b + c - d + e - f - g + h;

		return ((k0 + k1*u + k2*v + k3*w + k4*u*v + k5*v*w + k6*w*u + k7*u*v*w) + 1.0) / 2.0;
	}
}

