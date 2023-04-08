mod model;

use glium::{glutin::{self, event::VirtualKeyCode}, Surface, Display, implement_vertex, VertexBuffer, IndexBuffer, uniform};
use model::vertex::Vert3D;

use crate::model::{mvp::Mat4D, scene_loader};

fn main() {
    // Window and context creation
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("Title").with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 576));
    let context = glutin::ContextBuilder::new();
    let display = Display::new(window_builder, context, &event_loop).unwrap();

    implement_vertex!(Vert3D, position, normal);

    // load file
    let input = scene_loader::load_scene();
    let mut buffers = Vec::new();

    // extract data
    for (vertices, indices, model) in input {
        let v_buffer = VertexBuffer::new(&display, &vertices).unwrap();
        let i_buffer = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
        buffers.push((v_buffer, i_buffer, model));
    }

    // create glsl program
    let vs = std::fs::read_to_string("src\\shader\\simple_vertex.glsl").unwrap();
    let fs = std::fs::read_to_string("src\\shader\\normal_fragment.glsl").unwrap();

    let program = glium::Program::from_source(&display, &vs, &fs, None).unwrap();

    // uniforms and constants
    let movement = 0.05f32;
    let mut pos = [0.0, 1.0, 1.0];
    let mut dir = [0.0, -1.0, -1.0];
    let up = [0.0, 1.0, 0.0];

    // projection code from : https://github.com/glium/glium/blob/master/book/tuto-10-perspective.md
    let projection = {
        let (width, height) = display.get_framebuffer_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;
    
        let f = 1.0 / (fov / 2.0).tan();
        [
            [f *  aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    };

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.1, 0.05, 0.1, 1.0);
        target.clear_depth(1.0);

        let view = Mat4D::view_matrix(&pos , &dir, &up);

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
        for (v_buffer, i_buffer, model) in &buffers {
            target.draw(v_buffer, i_buffer, &program, 
            &uniform! {model : model.content, view : view.content, projection : projection, camera_position : pos},
            &glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            }).unwrap();
        }
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::DeviceEvent {event, ..} => match event {
                glium::glutin::event::DeviceEvent::Key(
                    glutin::event::KeyboardInput { scancode: _, state: _, virtual_keycode, modifiers: _ }) => {
                        match virtual_keycode {
                            // camera control 
                            Some(VirtualKeyCode::A) => pos[0] += &movement,
                            Some(VirtualKeyCode::D) => pos[0] -= &movement,
                            Some(VirtualKeyCode::S) => pos[1] -= &movement,
                            Some(VirtualKeyCode::W) => pos[1] += &movement,

                            Some(VirtualKeyCode::F) => pos[2] -= &movement,
                            Some(VirtualKeyCode::C) => pos[2] += &movement,

                            Some(VirtualKeyCode::J) => dir[0] += &movement,
                            Some(VirtualKeyCode::L) => dir[0] -= &movement,
                            Some(VirtualKeyCode::K) => dir[1] -= &movement,
                            Some(VirtualKeyCode::I) => dir[1] += &movement,

                            Some(VirtualKeyCode::Key0) => {
                                pos = [0.0, 1.0, 1.0];
                                dir = [0.0, -1.0, -1.0];
                            },
                            _ => return,
                        }
                }
                _ => return,
            }
            _ => (),
        }
    });

}
