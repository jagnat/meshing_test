extern crate image;
extern crate rand;
extern crate obj_exporter;

use image::Rgba;
use image::ImageBuffer;
use obj_exporter::{ObjSet, Object, Geometry, Shape, Primitive, Vertex};

mod perlin;
mod mesh_write;

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

	let vertice = vec![(0., 0., 0.),
						(1., 0., 0.),
						(0., 1., 0.),
						(1., 1., 0.),
						(0., 0., 1.),
						(1., 0., 1.),
						(0., 1., 1.),
						(1., 1., 1.)];

	let shapes = vec![(0,1,2),(1,3,2),
					  (1,5,3),(5,7,3),
					  (5,4,7),(4,6,7),
					  (4,0,6),(0,2,6),
					  (4,5,0),(5,1,0),
					  (2,3,6),(3,7,6)];
	
	let geometry = vec![Geometry{
		material_name: None,
		shapes: shapes.into_iter().map(|(x,y,z)| Shape{
			primitive: Primitive::Triangle(
						   (x, Some(x), Some(0)),
						   (y, Some(y), Some(0)),
						   (z, Some(z), Some(0)),
					),
			groups: vec![],
			smoothing_groups: vec![]}).collect()}];

	let objects: Vec<Object> = vec![Object{
		name: "Cube".to_owned(),
		vertices: vertice.into_iter().map(|(x, y, z)|Vertex{x, y, z}).collect(),
		tex_vertices: vec![],
		normals: vec![],
		geometry: geometry}];

	let oset = ObjSet{material_library:None, objects: objects};

	obj_exporter::export_to_file(&oset, "test_obj.obj").unwrap();
}

