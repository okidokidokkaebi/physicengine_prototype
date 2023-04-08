use super::{mvp::Mat4D, vertex::Vert3D};

pub struct SceneObject {
    pub vertices : Vec<Vert3D>, 
    pub indices : Vec<u32>, 
    pub model : Mat4D, 
    pub bounding_volume : ([f32; 3], [f32; 3])
}

pub fn are_colliding(fst_object : SceneObject, snd_object : SceneObject) -> f32 {
    
    return 0.0;
}