extern crate glium;

use glium::{glutin, Surface, Vertex};

#[derive(Copy, Clone)]
struct Vertex2d {
    position: [f32; 2],
}

glium::implement_vertex!(Vertex2d, position);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("PP Engine")
        .with_inner_size(glutin::dpi::LogicalSize::new(640.0, 400.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.5, 0.5, 1.0);
        }
    "#;

    let shape = [
        Vertex2d { position: [-0.25, -0.25] },
        Vertex2d { position: [-0.25, 0.25] },
        Vertex2d { position: [0.25, -0.25] },
        Vertex2d { position: [0.25, 0.25] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color(0.53, 0.81, 0.92, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniform! {}, &Default::default()).unwrap();
        target.finish().unwrap();
    });
}

