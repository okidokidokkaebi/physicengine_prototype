mod model;

use glium::{glutin::{self, event::VirtualKeyCode}, Surface, Display, implement_vertex, VertexBuffer, IndexBuffer, uniform};
use model::vertex::Vert3D;
use russimp::scene::Scene;

fn main() {
    // Window and context creation
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("Title").with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 576));
    let context = glutin::ContextBuilder::new();
    let display = Display::new(window_builder, context, &event_loop).unwrap();

    implement_vertex!(Vert3D, position, normal);

    // load file
    let input = Scene::from_file("res\\torus.obj", vec![]).unwrap();

    // extract data
    let (vertices, indices): (Vec<Vert3D>, Vec<u32>) = Vert3D::from_scene(input);
    let v_buffer = VertexBuffer::new(&display, &vertices).unwrap();
    let i_buffer = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    // create glsl program
    let vs = std::fs::read_to_string("src\\shader\\simple_vertex.glsl").unwrap();
    let fs = std::fs::read_to_string("src\\shader\\simple_fragment.glsl").unwrap();

    let program = glium::Program::from_source(&display, &vs, &fs, None).unwrap();


    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.1, 0.05, 0.1, 1.0);

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

        target.draw(&v_buffer, &i_buffer, &program, &uniform! {}, &Default::default()).unwrap();
        
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

        target.finish().unwrap();

        // TODO: add camera control
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
                            Some(VirtualKeyCode::A) => (),
                            Some(VirtualKeyCode::D) => (),
                            Some(VirtualKeyCode::S) => (),
                            Some(VirtualKeyCode::W) => (),

                            Some(VirtualKeyCode::J) => (),
                            Some(VirtualKeyCode::L) => (),
                            Some(VirtualKeyCode::K) => (),
                            Some(VirtualKeyCode::I) => (),

                            Some(VirtualKeyCode::Key0) => (),
                            _ => return,
                        }
                }
                _ => return,
            }
            _ => (),
        }
    });

}
