#[derive(Clone, Copy, Debug)]
pub struct Mat4D {
    pub content : [[f32; 4]; 4]
}

impl Mat4D {
    pub fn new() -> Mat4D {
        return Mat4D { content : [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]};
    }

    pub fn trans(self, vector : [f32; 3]) -> Mat4D {
        let translate_mat = Mat4D { content : [
            [1.0, 0.0, 0.0, vector[0]],
            [0.0, 1.0, 0.0, vector[1]],
            [0.0, 0.0, 1.0, vector[2]],
            [0.0, 0.0, 0.0, 1.0]
        ]};
        return Mat4D::mult4_d(self, translate_mat);
    }

    fn mult4_d(left : Mat4D, right : Mat4D) -> Mat4D {
        let mut result = Mat4D::new();
        for j in 0..4 {
            for i in 0..4 {
                let mut component = 0.0;
                for k in 0..4 {
                    component += left.content[j][k] * right.content[k][i];
                }
                result.content[j][i] = component;
            }
        }
        return result;
    }
}