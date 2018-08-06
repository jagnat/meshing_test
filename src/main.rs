extern crate image;
extern crate rand;
extern crate obj_exporter;

use image::Rgba;
use image::ImageBuffer;

mod perlin;
mod mesh_write;

use mesh_write::Mesh;

fn main()
{
	let mut p2d = perlin::Perlin2D::new(490295);
	p2d.init();
	let mut img : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(512, 512);
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
	
	img.save("test.png").unwrap();


	let mut mesh = Mesh::new();
	mesh.add_vertex(0., 0., 0.);
	mesh.add_vertex(1., 0., 0.);
	mesh.add_vertex(0., 1., 0.);
	mesh.add_vertex(1., 1., 0.);
	mesh.add_vertex(0., 0., 1.);
	mesh.add_vertex(1., 0., 1.);
	mesh.add_vertex(0., 1., 1.);
	mesh.add_vertex(1., 1., 1.);

	mesh.add_tri(0, 1, 2);
	mesh.add_tri(1, 3, 2);

	mesh.add_tri(1, 5, 3);
	mesh.add_tri(5, 7, 3);

	mesh.add_tri(5, 4, 7);
	mesh.add_tri(4, 6, 7);

	mesh.add_tri(4, 0, 6);
	mesh.add_tri(0, 2, 6);

	mesh.add_tri(4, 5, 0);
	mesh.add_tri(5, 1, 0);

	mesh.add_tri(2, 3, 6);
	mesh.add_tri(3, 7, 6);

	mesh.save("test.obj".into());
}

