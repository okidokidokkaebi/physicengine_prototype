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
                vertices.push(Vert3D { position: [v.x, v.y, v.z], normal: [n.x, n.y, n.z] })
            }

            for face in mesh.faces {
                indices.push(face.0);
            }
        }

        return (vertices, indices.concat());
    }
}