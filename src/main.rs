extern crate image;

use image::Rgba;
use image::ImageBuffer;

fn main()
{
	let mut img : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(512, 512);
	for x in 0..512
	{
		for y in 0..512
		{
			img.put_pixel(x, y, Rgba([0, 0, 59, 255]));
		}
	}
	let img_write_res = img.save("test.png").unwrap();
}

