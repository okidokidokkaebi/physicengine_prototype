mod model;

use glium::{glutin, Surface, Display, implement_vertex, VertexBuffer, IndexBuffer};
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

    // TODO: load file
    let input = Scene::from_file("res\\cube.obj", vec![]).unwrap();

    // TODO: extract data
    let (vertices, indices): (Vec<Vert3D>, Vec<u32>) = Vert3D::from_scene(input);
    let v_buffer = VertexBuffer::new(&display, &vertices).unwrap();
    let i_buffer = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    // TODO: create glsl program

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.1, 0.05, 0.1, 1.0);

        // TODO: draw
        // target.draw(&v_buffer, &i_buffer, program, uniforms, draw_parameters)

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
            _ => (),
        }
    });

}
