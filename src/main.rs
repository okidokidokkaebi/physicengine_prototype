mod model;

use glium::{glutin::{self, event::VirtualKeyCode}, Surface, Display, implement_vertex, VertexBuffer, IndexBuffer, uniform};
use model::vertex::Vert3D;

use crate::model::{mvp::Mat4D, scene_loader, scene_object::{SceneObject, are_colliding}};

fn main() {
    // Window and context creation
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("Title").with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 576));
    let context = glutin::ContextBuilder::new();
    let display = Display::new(window_builder, context, &event_loop).unwrap();

    implement_vertex!(Vert3D, position, normal);

    // load file
    let scene_objects: Vec<SceneObject> = scene_loader::load_scene();

    // extract data
    let mut buffers: Vec<(VertexBuffer<Vert3D>, IndexBuffer<u32>, Mat4D)> = Vec::new();
    for object in &scene_objects {
        let v_buffer = VertexBuffer::new(&display, &object.vertices).unwrap();
        let i_buffer = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &object.indices).unwrap();
        buffers.push((v_buffer, i_buffer, object.model));
    }

    // load control object
    let player_scene = russimp::scene::Scene::from_file(&("res\\cube.obj"), vec![]).unwrap();
    let (player_vertices, player_indices) = scene_loader::from_scene(player_scene);
    let player_model = Mat4D::new().scale([0.5, 0.5, 0.5]);
    let mut player_object = SceneObject {
        vertices : player_vertices.clone(),
        indices : player_indices, 
        model : player_model,
        bounding_volume : scene_loader::calculate_aabb(&player_vertices)
    };
    
    let player_v_buffer = VertexBuffer::new(&display, &player_object.vertices).unwrap();
    let player_i_buffer = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &player_object.indices).unwrap();

    // create glsl program
    let vs = std::fs::read_to_string("src\\shader\\simple_vertex.glsl").unwrap();
    let fs = std::fs::read_to_string("src\\shader\\normal_fragment.glsl").unwrap();
    let p_fs = std::fs::read_to_string("src\\shader\\simple_fragment.glsl").unwrap();
    let program = glium::Program::from_source(&display, &vs, &fs, None).unwrap();
    let player_program = glium::Program::from_source(&display, &vs, &p_fs, None).unwrap();

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
        // Draw Scene
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
        
        // Draw Player
        target.draw(&player_v_buffer, &player_i_buffer, &player_program,
            &uniform! {model : player_object.model.content, view : view.content, projection : projection, camera_position : pos},
            &glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            }).unwrap();
        
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
        
        for scene_object in &scene_objects {
            are_colliding(&scene_object, &player_object);
        }

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

                            Some(VirtualKeyCode::Left)  => player_object.model = player_object.model.trans([movement, 0.0, 0.0]),
                            Some(VirtualKeyCode::Right) => player_object.model = player_object.model.trans([-movement, 0.0, 0.0]),
                            Some(VirtualKeyCode::Up)    => player_object.model = player_object.model.trans([0.0, 0.0, -movement]),
                            Some(VirtualKeyCode::Down)  => player_object.model = player_object.model.trans([0.0, 0.0, movement]),

                            Some(VirtualKeyCode::Key0) => {
                                pos = [0.0, 1.0, 1.0];
                                dir = [0.0, -1.0, -1.0];
                                player_object.model = Mat4D::new();
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
