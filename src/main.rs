extern crate glium;
extern crate image;

use glium::*;

fn main() {
    println!("Program start!");

    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vert {
        position: [f32; 2],
        color: [f32; 3]
    }
    implement_vertex!(Vert, position, color);

    let vertex_buffer = VertexBuffer::new(&display, &vec![
        Vert{ position: [-0.3, 0.7], color: [1.0, 0.0, 0.0] },
        Vert{ position: [0.7, 0.0], color: [0.0, 1.0, 0.0] },
        Vert{ position: [-0.3, -0.7], color: [0.0, 0.0, 1.0] }
    ]).unwrap();
    
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;

        out vec4 col;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            col = vec4(color, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140 

        in vec4 col;
        out vec4 color;

        void main() {
            color = col;
        }
    "#;

    let program = Program::from_source(
        &display, vertex_shader_src, fragment_shader_src, None
    ).unwrap();

    let uniforms = uniforms::EmptyUniforms;
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };
        
    let mut closed = false;
    while !closed {

        // Actual rendering related part below
        {
            


            let mut target = display.draw();

            target.clear_color_and_depth((0.3, 0.3, 0.5, 1.0), 1.0);
            target.draw(
                &vertex_buffer, &indices, &program, &uniforms, &params
            ).unwrap();
            target.finish().unwrap();
        }

        // event handling 
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                glutin::Event::DeviceEvent{event, ..} => match event {
                    glutin::DeviceEvent::Key(k) => {
                        println!("Key {:?} {:?}", 
                            k.virtual_keycode.unwrap(),
                            k.state
                    )
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }

    println!("Program end!")
}
