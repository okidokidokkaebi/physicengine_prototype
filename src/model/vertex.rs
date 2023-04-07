use std::iter;

use russimp::scene::Scene;

#[derive(Clone, Copy, Debug)]
pub struct Vert3D {
    pub position : [f32; 3],
    pub normal : [f32; 3]
}

impl Vert3D {
    pub fn from_scene(scene : Scene) -> (Vec<Vert3D>, Vec<u32>) {
        let mut vertices: Vec<Vert3D> = Vec::new();
        let mut indices = Vec::new();

        for mesh in scene.meshes {
            let vertex_list = mesh.vertices;
            let normal_list = mesh.normals;

            assert!(vertex_list.len() == normal_list.len(), "ASSERTION : Amount of vertices must be the same as the amount of vertex normals!");

            for (v, n) in iter::zip(vertex_list, normal_list) {
                vertices.push(Vert3D { position: [v.x, v.y, v.z], normal: [n.x, n.y, n.z] });
                println!("{:?}", vertices.last());
            }

            for face in mesh.faces {
                indices.push(face.0);
                println!("{:?}", indices.last().get_or_insert(&Vec::new()));
            }
        }
        println!("vertices length : {}\tindices length : {}", vertices.len(), indices.len());
        return (vertices, indices.concat());
    }

    pub fn from_debug() -> (Vec<Vert3D>, Vec<u32>) {
    	
        let vertices: Vec<Vert3D> = vec![
            Vert3D { position: [-1.0, -1.0, -1.0], normal: [-1.0, -1.0, -1.0] },
            Vert3D { position: [-1.0, -1.0, 1.0], normal: [-1.0, -1.0, 1.0] },
            Vert3D { position: [-1.0, 1.0, -1.0], normal: [-1.0, 1.0, -1.0] },
            Vert3D { position: [-1.0, 1.0, 1.0], normal: [-1.0, 1.0, 1.0] },
            Vert3D { position: [1.0, -1.0, -1.0], normal: [1.0, -1.0, -1.0] },
            Vert3D { position: [1.0, -1.0, 1.0], normal: [1.0, -1.0, 1.0] },
            Vert3D { position: [1.0, 1.0, -1.0], normal: [1.0, 1.0, -1.0] },
            Vert3D { position: [1.0, 1.0, 1.0], normal: [1.0, 1.0, 1.0] },
        ];

        let indices: Vec<u32> = vec![

            2, 1, 0,
            3, 2, 1,

            6, 5, 4, 

            5, 6, 7,

            6, 3, 2, 
            7, 6, 3,

            4, 1, 0,
            5, 4, 1,

            5, 3, 1,
            7, 5, 3,

            4, 2, 0,
            6, 4, 2,
        ];
        return (vertices, indices);
    }
}