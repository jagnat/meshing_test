extern crate obj_exporter;

use obj_exporter::{ObjSet, Object, Geometry, Shape, Primitive, Vertex};

pub struct Vert
{
	x: f64, y: f64, z: f64,
}

#[allow(dead_code)]
pub struct Mesh
{
	vertices: Vec<Vert>,
	normals: Vec<Vert>,
	indices: Vec<(usize, usize, usize)>,
}

impl Mesh
{
	pub fn new()->Mesh
	{
		return Mesh{vertices: vec![], normals: vec![], indices: vec![]};
	}

	pub fn add_vertex(&mut self, x: f64, y: f64, z: f64)
	{
		self.vertices.push(Vert{x:x, y:y, z:z});
	}

	pub fn add_tri(&mut self, i: usize, j: usize, k: usize)
	{
		self.indices.push((i, j, k));
	}

	pub fn save(&self, path: String)
	{
		let geometry = vec![Geometry
		{
			material_name: None,
			shapes: self.indices.iter().map(|(x, y, z)| Shape
			{
				primitive: Primitive::Triangle(
					(*x, Some(*x), Some(0)),
					(*y, Some(*y), Some(0)),
					(*z, Some(*z), Some(0))),
				groups: vec![],
				smoothing_groups: vec![]
			}).collect()
		}];

		
		let objects = vec![Object
		{
			name: "Mesh".into(),
			vertices: self.vertices.iter().map(|v|Vertex{x:v.x, y:v.y, z:v.z}).collect(),
			tex_vertices: vec![],
			normals: vec![],
			geometry: geometry,
		}];

		let oset = ObjSet{material_library:None, objects: objects};
		obj_exporter::export_to_file(&oset, path).unwrap();
	}
}
