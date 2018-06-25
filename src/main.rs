extern crate image;
extern crate rand;

use image::Rgba;
use image::ImageBuffer;

mod perlin;

fn main()
{
	let p2d = perlin::Perlin2D::new(490295);
	let mut img : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(512, 512);
	for x in 0..512
	{
		for y in 0..512
		{
			img.put_pixel(x, y, Rgba([0, 0, 59, 255]));
		}
	}
	img.save("test.png").unwrap();
}

