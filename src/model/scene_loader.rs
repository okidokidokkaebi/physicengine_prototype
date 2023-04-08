use std::iter;

use russimp::scene::Scene;

use super::{vertex::Vert3D, mvp::Mat4D};

const FILES: [&str; 2] = ["torus.obj", "plane.obj"];
const TRANSLATES: [[f32; 3]; 2] = [[1.0, 1.0, 1.0], [0.0, 0.0, 0.0]];
const SCALES: [f32; 2] = [0.5, 0.5];

pub fn load_scene() -> Vec<(Vec<Vert3D>, Vec<u32>, Mat4D)> {
    assert!(FILES.len() == TRANSLATES.len());

    let mut scene : Vec<(Vec<Vert3D>, Vec<u32>, Mat4D)> = Vec::new();
    for i in 0..FILES.len() {
        let input = russimp::scene::Scene::from_file(&("res\\".to_owned() + FILES[i]), vec![]).unwrap();
        let (vertices, indices) = from_scene(input);
        let model = Mat4D::new().trans(TRANSLATES[i]).scale([SCALES[i], SCALES[i], SCALES[i]]);
        scene.push((vertices, indices, model));
    }
    return scene;
}

pub fn from_scene(scene : Scene) -> (Vec<Vert3D>, Vec<u32>) {
    let mut vertices: Vec<Vert3D> = Vec::new();
    let mut indices = Vec::new();

    for mesh in scene.meshes {
        let vertex_list = mesh.vertices;
        let normal_list = mesh.normals;

        assert!(vertex_list.len() == normal_list.len(), "ASSERTION : Amount of vertices must be the same as the amount of vertex normals!");

        for (v, n) in iter::zip(vertex_list, normal_list) {
            vertices.push(Vert3D { position: [v.x, v.y, v.z], normal: [n.x, n.y, n.z] });
        }

        for face in mesh.faces {
            indices.push(face.0);
        }
    }
    return (vertices, indices.concat());
}