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

    pub fn scale(self, scale : [f32; 3]) -> Mat4D {
        let scale_mat = Mat4D { content : [
            [scale[0], 0.0, 0.0, 0.0],
            [0.0, scale[1], 0.0, 0.0],
            [0.0, 0.0, scale[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]};
        return Mat4D::mult4_d(self, scale_mat);
    }

    // code from: https://github.com/glium/glium/blob/master/book/tuto-12-camera.md
    pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> Mat4D{
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];
    
        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        Mat4D {content : [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]}
    }

    pub fn trans(self, vector : [f32; 3]) -> Mat4D {
        let translate_mat = Mat4D { content : [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ -vector[0], vector[1], -vector[2], 1.0]
        ]};
        return Mat4D::mult4_d(self, translate_mat);
    }

    fn mult4_d(left : Mat4D, right : Mat4D) -> Mat4D {
        let mut result = Mat4D::new();
        for i in 0..4 {
            for j in 0..4 {
                let mut component = 0.0;
                for k in 0..4 {
                    component += left.content[i][k] * right.content[k][j];
                }
                result.content[i][j] = component;
            }
        }
        return result;
    }

    pub fn _test() {
        let a = Mat4D {content : [
            [1.0,	 2.0,	 3.0,	 4.0],
            [5.0,	 6.0,	 7.0,	 8.0],
            [9.0,	10.0,	11.0,	12.0],
           [13.0,	14.0,	15.0,	16.0]
        ]};

        let b = Mat4D {content : [
            [2.0,	2.0,	2.0,	2.0],
            [3.0,	3.0,	3.0,	3.0],
            [4.0,	4.0,	4.0,	4.0],
            [6.0,	5.0,	4.0,	3.0]
        ]};

        println!("{:?}", Mat4D::mult4_d(a, b).content);
    } 

}