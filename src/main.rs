extern crate image;
extern crate rand;

use image::Rgba;
use image::ImageBuffer;

mod perlin;

fn main()
{
	let mut p2d = perlin::Perlin2D::new(490295);
	p2d.init();
	let mut img : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(512, 512);
	for x in 0..512
	{
		for y in 0..512
		{
			let p = p2d.perl3d(x as f64 / 64., 0.3289424, y as f64 / 64.);
			let b : u8 = (p * 255.) as u8;
			img.put_pixel(x, y, Rgba([b, b, b, 255]));
		}
	}
	img.save("test.png").unwrap();
}

