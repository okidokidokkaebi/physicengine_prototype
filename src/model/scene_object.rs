use super::{mvp::Mat4D, vertex::Vert3D};

#[derive(Debug)]
pub struct SceneObject {
    pub vertices : Vec<Vert3D>, 
    pub indices : Vec<u32>, 
    pub model : Mat4D, 
    pub bounding_volume : ([f32; 3], [f32; 3])
}

pub fn are_colliding(fst_object : &SceneObject, snd_object : &SceneObject) -> bool {
    let (mut fst_min, mut fst_max) = fst_object.bounding_volume;
    let (mut snd_min, mut snd_max) = snd_object.bounding_volume;
    
    fst_min = Mat4D::mult_mat_vec(fst_object.model, fst_min);
    fst_max = Mat4D::mult_mat_vec(fst_object.model, fst_max);

    snd_min = Mat4D::mult_mat_vec(snd_object.model, snd_min);
    snd_max = Mat4D::mult_mat_vec(snd_object.model, snd_max);

    return  
        fst_min[0] <= snd_max[0] && fst_max[0] >= snd_min[0] &&
        fst_min[1] <= snd_max[1] && fst_max[1] >= snd_min[1] &&
        fst_min[2] <= snd_max[2] && fst_max[2] >= snd_min[2];
}