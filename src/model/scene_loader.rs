use std::iter;

use russimp::scene::Scene;

use super::{vertex::Vert3D, mvp::Mat4D, scene_object::SceneObject};

const FILES: [&str; 2] = ["torus.obj", "plane.obj"];
const TRANSLATES: [[f32; 3]; 2] = [[1.0, 1.0, 1.0], [0.0, 0.0, 0.0]];
const SCALES: [f32; 2] = [0.5, 0.5];

pub fn load_scene() -> Vec<SceneObject> {
    assert!(FILES.len() == TRANSLATES.len());

    let mut scene = Vec::new();
    for i in 0..FILES.len() {
        let input = russimp::scene::Scene::from_file(&("res\\".to_owned() + FILES[i]), vec![]).unwrap();
        let (vertices, indices) = from_scene(input);
        let model = Mat4D::new().trans(TRANSLATES[i]).scale([SCALES[i], SCALES[i], SCALES[i]]);
        let (min, max) = calculate_aabb(&vertices);

        scene.push(SceneObject {
            vertices : vertices,
            indices : indices, 
            model : model,
            bounding_volume : (min, max)
        });
    }
    return scene;
}

fn calculate_aabb(vertices: &Vec<Vert3D>) -> ([f32; 3], [f32; 3]) {
    let mut min = vertices.get(0).unwrap().position;
    let mut max = vertices.get(0).unwrap().position;

    for &v in vertices {
        let [x, y, z] = v.position;

        if x < min[0] { min[0] = x; }
        if x > max[0] { max[0] = x; }      

        if y < min[1] { min[1] = y; }
        if y > max[1] { max[1] = y; }      

        if z < min[2] { min[2] = z; }
        if z > max[2] { max[2] = z; }
    }
    return (min, max);
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