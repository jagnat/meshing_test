extern crate image;
extern crate rand;
extern crate obj_exporter;

use image::Rgba;
use image::ImageBuffer;

mod perlin;
mod mesh_write;

use mesh_write::{Mesh, Vert, Triangle, Quad};

fn main()
{
	let mut p2d = perlin::Perlin2D::new(490295);
	p2d.init();
	//let mut img : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(512, 512);
	/*
	for x in 0..512
	{
		let p = p2d.perl3d(x as f64 / 128., 0.3289424, 0.9428985);
		let b = (p * 512.) as u32;
		for y in 0..512
		{
			if y < b.into()
			{
				img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
			}
			else
			{
				img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
			}
		}
	}
	*/
	
	//img.save("test.png").unwrap();

	let mut mesh = Mesh::new();
	for x in 0..32
	{
		for y in 0..32
		{
			let b = (p2d.perl3d(x as f64 / 16., y as f64 / 16., 53.45) * 32.) as usize;
			for z in 0..b
			{
				mesh.add_naive_cube(Vert{x:x as f64, y:y as f64, z: z as f64});
			}
		}
	}
	mesh.save("test.obj".into());
}

