
pub struct Vert
{
	x: f32, y: f32, z: f32,
}

pub struct Mesh
{
	vertices: Vec<Vert>,
	normals: Vec<Vert>,
	indices: Vec<usize>,
	usenormals: bool,
}

impl Mesh
{
	pub fn new(usenormals: bool)->Mesh
	{
		return Mesh{vertices: vec![], normals: vec![], indices: vec![], usenormals: usenormals};
	}

	pub fn save(&self, path: String)
	{
		/*
		let geometry = vec![Geometry
		{
			material_name: None,
			shapes: self.
		}];
		*/
	}
}
