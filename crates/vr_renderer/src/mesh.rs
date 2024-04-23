use vulkano::buffer::BufferContents;

use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct VrSurfaceVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
    #[format(R32_UINT)]
    material: u32,
}

impl VrSurfaceVertex {
    pub fn new(x: f32, y: f32, material: u32) -> Self {
        Self {
            position: [x, y],
            material,
        }
    }
}

fn rect(from_x: f32, from_y: f32, to_x: f32, to_y: f32, material: u32) -> Vec<VrSurfaceVertex> {
    let xmin = from_x.min(to_x);
    let xmax = from_x.max(to_x);
    let ymin = from_y.min(to_y);
    let ymax = from_y.max(to_y);

    vec![VrSurfaceVertex::new(xmin, ymin, material),
         VrSurfaceVertex::new(xmin, ymax, material),
         VrSurfaceVertex::new(xmax, ymax, material),
         VrSurfaceVertex::new(xmin, ymin, material),
         VrSurfaceVertex::new(xmax, ymax, material),
         VrSurfaceVertex::new(xmax, ymin, material),
    ]
}

fn all(vecs: Vec<Vec<VrSurfaceVertex>>) -> Vec<VrSurfaceVertex> {
    let mut new = vec![];

    for vec in vecs {
        for elem in vec {
            new.push(elem)
        }
    }

    new
}


pub fn get_model_vertices() -> Vec<VrSurfaceVertex> {
    all(vec![
        rect(-1.0, -1.0, 0.0, 1.0, 0),
        rect(0.0, -1.0, 1.0, 1.0, 1),
    ])
}