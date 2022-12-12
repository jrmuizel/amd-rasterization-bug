#[macro_use]
extern crate glium;

//mod support;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;
use glium::Rect;



fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();


    dbg!(display.get_opengl_renderer_string());

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);
        let mut data = Vec::new();
        /*for v in result.iter() {
            data.push(Vertex{ position: [v.x, v.y], color: [0.0, v.coverage, 0.0]})
        }*/

        /*for i in 0..2000 {
            let i = i as f32;
            data.push(Vertex{ position: [0., i + 0.5], color: [0., 1.0, 0.] });
            data.push(Vertex{ position: [i, i + 0.5 + 0.5], color: [0., 1.0, 0.] });
            data.push(Vertex{ position: [i, i + 0.5 - 0.5], color: [0., 1.0, 0.] });

            data.push(Vertex{ position: [0., i - 0.5], color: [1., 0., 0.] });
            data.push(Vertex{ position: [0., i + 0.5], color: [1., 0., 0.] });
            data.push(Vertex{ position: [i, i ], color: [1., 0., 0.] });
        }*/

        for i in 0..2000 {
            let i = i as f32;
            let width = 2100.;
            data.push(Vertex{ position: [0., i + 0.5], color: [0., 1.0, 0.] });
            data.push(Vertex{ position: [width, i + 0.5 + 0.5], color: [0., 1.0, 0.] });
            data.push(Vertex{ position: [width, i + 0.5 - 0.5], color: [0., 1.0, 0.] });

            data.push(Vertex{ position: [0., i - 0.5], color: [1., 0., 0.] });
            data.push(Vertex{ position: [0., i + 0.5], color: [1., 0., 0.] });
            data.push(Vertex{ position: [width, i ], color: [1., 0., 0.] });
        }
        glium::VertexBuffer::new(&display,&data).unwrap()

        /*glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-10.5, -10.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  10.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 10.5, -10.5], color: [1.0, 0.0, 0.0] },
            ]
        ).unwrap()*/
    };



    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 matrix;
                uniform vec2 viewport;
                uniform vec2 iviewport;

                in vec2 position;
                in vec3 color;

                out vec3 vColor;

                void main() {
                    vec2 pos;
                    pos  = position;
                    pos -= viewport/2.0;

                    //gl_Position = vec4(2*pos*iviewport + vec2(0., -iviewport.y/16384), 0.0, 1.0);
                    gl_Position = vec4(2*pos*iviewport + vec2(0., 0.), 0.0, 1.0);

                    //gl_Position = vec4(2*pos*iviewport + vec2(0., -2*1.175494351e-38), 0.0, 1.0);

                    //1.175494351e-38
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;

                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

    ).unwrap();

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let draw = move || {
        let size = display.gl_window().window().inner_size();
        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [2.0/size.width as f32, 0.0, 0.0, 0.0],
                [0.0, 2.0/size.height as f32, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            viewport: [size.width as f32, size.height as f32],
            iviewport: [1./(size.width as f32), 1./size.height as f32]

        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);

        // draw parameters
        let params = glium::DrawParameters {
            //viewport: Some(Rect{left: 0, bottom: 0, width: 400, height: 400}),
            depth: glium::Depth {

                .. Default::default()
            },
            .. Default::default()
        };
        
        target.draw(&vertex_buffer, glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &program, &uniforms, &params).unwrap();
        target.finish().unwrap();
    };

    // Draw the triangle to the screen.
    draw();

    // the main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(size) => {
                    draw();
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}
