#[derive(Clone, Copy, Debug)]
pub struct Vert3D {
    pub position : [f32; 3],
    pub normal : [f32; 3]
}

impl Vert3D {
    pub fn _from_debug() -> (Vec<Vert3D>, Vec<u32>) {
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
            2, 1, 0,    3, 2, 1,
            6, 5, 4,    5, 6, 7,
            6, 3, 2,    7, 6, 3,

            4, 1, 0,    5, 4, 1,
            5, 3, 1,    7, 5, 3,
            4, 2, 0,    6, 4, 2,
        ];
        return (vertices, indices);
    }
}