extern crate obj_exporter;

use obj_exporter::{ObjSet, Object, Geometry, Shape, Primitive, Vertex};

#[derive(Clone)]
pub struct Vert
{
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

pub struct Triangle
{
	pub verts: [Vert; 3]
}

pub struct Quad
{
	pub verts: [Vert; 4]
}

#[allow(dead_code)]
pub struct Mesh
{
	vertices: Vec<Vert>,
	normals: Vec<Vert>,
	indices: Vec<(usize, usize, usize)>,
	index_count: usize
}

impl Mesh
{
	pub fn new()->Mesh
	{
		return Mesh{vertices: vec![], normals: vec![], indices: vec![], index_count: 0};
	}

	fn add_vertex(&mut self, v: Vert)
	{
		self.vertices.push(v);
		self.index_count += 1;
	}

	pub fn add_tri_indices(&mut self, i: usize, j: usize, k: usize)
	{
		self.indices.push((i, j, k));
	}

	pub fn add_triangle(&mut self, t: Triangle)
	{
		let ci = self.index_count;
		for vert in t.verts.iter().cloned()
		{
			self.add_vertex(vert);
		}
		self.add_tri_indices(ci, ci+1, ci+2);
	}

	pub fn add_quad(&mut self, q: Quad)
	{
		let ci = self.index_count;
		for vert in q.verts.iter().cloned()
		{
			self.add_vertex(vert);
		}
		self.add_tri_indices(ci, ci + 1, ci + 2);
		self.add_tri_indices(ci, ci + 2, ci + 3);
	}

	pub fn add_naive_cube(&mut self, pos: Vert)
	{
		let x = pos.x;
		let y = pos.y;
		let z = pos.z;
		self.add_quad(Quad{verts: [ // Front
			Vert{x:x, y:y, z:z},
			Vert{x:x+1., y:y, z:z},
			Vert{x:x+1., y:y, z:z+1.},
			Vert{x:x, y:y, z:z+1.},]});
		self.add_quad(Quad{verts: [ // Right
			Vert{x:x+1., y:y, z:z},
			Vert{x:x+1., y:y+1., z:z},
			Vert{x:x+1., y:y+1., z:z+1.},
			Vert{x:x+1., y:y, z:z+1.},]});
		self.add_quad(Quad{verts: [ // Back
			Vert{x:x+1., y:y+1., z:z},
			Vert{x:x, y:y+1., z:z},
			Vert{x:x, y:y+1., z:z+1.},
			Vert{x:x+1., y:y+1., z:z+1.},]});
		self.add_quad(Quad{verts: [ // Left
			Vert{x:x, y:y+1., z:z},
			Vert{x:x, y:y, z:z},
			Vert{x:x, y:y, z:z+1.},
			Vert{x:x, y:y+1., z:z+1.},]});
		self.add_quad(Quad{verts: [ // Top
			Vert{x:x, y:y, z:z+1.},
			Vert{x:x+1., y:y, z:z+1.},
			Vert{x:x+1., y:y+1., z:z+1.},
			Vert{x:x, y:y+1., z:z+1.},]});
		self.add_quad(Quad{verts: [ // Bottom
			Vert{x:x, y:y+1., z:z},
			Vert{x:x+1., y:y+1., z:z},
			Vert{x:x+1., y:y, z:z},
			Vert{x:x, y:y, z:z},]});
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
