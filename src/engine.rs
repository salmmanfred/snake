use std::fs::File;

use crate::{Snake, CursorInfo};
extern crate glium_text_rusttype as glium_text;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}

use glium::backend::Facade;
use glium::index::PrimitiveType;
#[allow(unused_imports)]
use glium::{backend, glutin, Surface};

use glium::Display;

pub fn run() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Snake")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::Size::Logical(glutin::dpi::LogicalSize {
            width: 500.,
            height: 500.,
        }));
    
    

    let cb = glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();
    // building the vertex buffer, which contains all the vertices that we will draw
    
    let system = glium_text::TextSystem::new(&display);
    display.get_context().get_context();

    // Creating a `FontTexture`, which a regular `Texture` which contains the font.
    // Note that loading the systems fonts is not covered by this library.
    let font = glium_text::FontTexture::new(
        &display,
        File::open("./src/OpenSans-Medium.ttf").unwrap(),
        55,
        glium_text::FontTexture::ascii_character_list(),
    )
    .unwrap();

    implement_vertex!(Vertex, position, color);

    /*   let vertex_buffer =     glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
            ]
        ).unwrap();


    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2,3,4,5]).unwrap();*/

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
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
        }

    )
    .unwrap();

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.

    let mut snake = Snake::new(true);
    let mut draw = move |x: u32, mouse:CursorInfo| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.3, 0.0, 0.0);

        snake.keypr(x);

        // TODO: Do these really have to bere (uniforms aswell)
        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];
        
        snake.move_mouse(mouse);

        

        /*let matrix_text:[[f32; 4]; 4] = [
            [0.2 / text_width, 0.0, 0.0, 0.0,],
            [0.0, 0.2 * (w as f32) / (h as f32) / text_width, 0.0, 0.0,],
            [0.0, 0.0, 0.1, 0.0,],
            [0.01, 0.01, 0.0, 0.1f32,],
        ];*/

        // building the uniforms
        let uniforms = uniform! {
            matrix: matrix.clone()
        };

        // drawing a frame
        let (pbuf, fbuf) = snake.render();
        let index_buffer =
            glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &fbuf).unwrap();

        let vertex_buffer = glium::VertexBuffer::new(&display, &pbuf).unwrap();
        let (new_title, title) = snake.title();

        if new_title {
            println!("b{},{}", new_title, title);

            display.gl_window().window().set_title(title);
        }

        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

            for x in 0..snake.index_size() {
                let text = snake.text_info_get(x);
                let text = glium_text::TextDisplay::new(&system, &font, &text);
    
                let text_width = text.get_width();
    
                let (w, h) = display.get_framebuffer_dimensions();
                snake.update_text_info((w as f32, h as f32, text_width));
                let matrix_text = snake.render_text(x);
    
                glium_text::draw(
                    &text,
                    &system,
                    &mut target,
                    matrix_text,
                    (0.0, 0.0, 0.0, 1.0),
                )
                .unwrap();
            }

        
        target.finish().unwrap();
    };

    // Draw the triangle to the screen.
    draw(0,CursorInfo::new());
    let mut run_game = true;
    // the main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    draw(0,CursorInfo::new());
                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::Focused(a) => {
                    
                        run_game = a;
                    
                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::CursorEntered { device_id: _ } =>{
                    draw(0,CursorInfo::window_left(true));
                   
                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::CursorLeft { device_id: _ } =>{
                    draw(0,CursorInfo::window_left(false));
                    
                    
                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } =>{

                    let position = position.to_logical(1.);
                    draw(0,CursorInfo::pos([position.x,position.y]));
                    
                    
                    glutin::event_loop::ControlFlow::Poll
                    
                }

                _ => glutin::event_loop::ControlFlow::Poll,
            },
            glutin::event::Event::DeviceEvent {
                device_id: _,
                event,
            } => {
                match event {
                    glutin::event::DeviceEvent::Key(a) => {
                        match a.scancode{
                            1 =>{
                                run_game = false;
                                
                            }
                            _=>{}
                        }

                        draw(a.scancode, CursorInfo::new());
                        //s println!("key: {}",a.scancode);

                        glutin::event_loop::ControlFlow::Poll
                    }
                    glutin::event::DeviceEvent::Button { button, state:_ } =>{
                        if button == 1{
                            draw(0,CursorInfo::button_press(true))
                        }
                        glutin::event_loop::ControlFlow::Poll

                    }
                    

                    _ => glutin::event_loop::ControlFlow::Poll,
                }
            }
            _ => {
                if run_game{
                    draw(0,CursorInfo::new());
                }
                glutin::event_loop::ControlFlow::Poll
            }
        };
    });
}
