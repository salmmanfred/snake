use std::fs::File;

use crate::{CursorInfo, Snake};
extern crate glium_text_rusttype as glium_text;
use std::time::Instant;

// how much time between frames.
const TIMEPF: u64 = 17;


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


use glium::index::PrimitiveType;
#[allow(unused_imports)]
use glium::{backend, glutin, Surface};

use glium::Display;
// main
pub fn run() {
    // create the event loop then the window builder
    let event_loop = glutin::event_loop::EventLoop::new();
    // the window has the title Snake and the size 500 by 500 and cant be resized
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Snake")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::Size::Logical(glutin::dpi::LogicalSize {
            width: 500.,
            height: 500.,
        }));
    // creates the context builder then the display 
    let cb = glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();
    // get the scale factor (used by the mouse)
    let scale_factor = display.gl_window().window().scale_factor();

    
    //building the text system 
    let system = glium_text::TextSystem::new(&display);
    

    // creates the font used by text making later
    let font = glium_text::FontTexture::new(
        &display,
        File::open("./src/OpenSans-Medium.ttf").unwrap(),
        55,
        glium_text::FontTexture::ascii_character_list(),
    )
    .unwrap();
    // impl vertex
    implement_vertex!(Vertex, position, color);

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

    
    // here we create the snake 
    let mut snake = Snake::new(true);
    
    let matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    // building the uniforms
    let uniforms = uniform! {
        matrix: matrix.clone()
    };
    // used later by text 
    let (w, h) = display.get_framebuffer_dimensions();
    snake.update_text_info((w as f32, h as f32));

    // draw loop
    let mut draw = move |x: u32, mouse: CursorInfo| {
        // create target and clear screen
        let mut target = display.draw();
        target.clear_color(0.0, 0.3, 0.0, 0.0);
        // changes keypresses and moving the mouse
        snake.keypr(x);
        snake.move_mouse(mouse);

        // getting the fragment buffers and vertex buffers then creates them
        let (pbuf, fbuf) = snake.render();
        let index_buffer =
            glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &fbuf).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(&display, &pbuf).unwrap();
        // checks if there is a new title and gets the title
        let (new_title, title) = snake.title();
        if *new_title {
            display.gl_window().window().set_title(title);
        }
        // draws the grahpics to the screen
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        // checks all the text that is suppose to be written
        for x in 0..snake.index_size() {
            // get the text info
            let text = snake.text_info_get(x);
            // create the text 
            let text = glium_text::TextDisplay::new(&system, &font, text);

            // get the text matrix
            let matrix_text = snake.render_text(x, text.get_width() as f32);
            // draw the text 
            glium_text::draw(
                &text,
                &system,
                &mut target,
                matrix_text,
                (0.0, 0.0, 0.0, 1.0),
            )
            .unwrap();
        }
        //finish
        target.finish().unwrap();
    };

    // Draw the triangle to the screen.
    draw(0, CursorInfo::new());
    // if the game should be ran
    let mut run_game = true;
    // the main loop
    //TODO: make sure mouse movement does not speed up the framerate (KINDA DONE)
    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    draw(0, CursorInfo::new());
                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::Focused(a) => {
                    // if the game is not focused it should noot be running 
                    run_game = a;

                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::CursorEntered { device_id: _ } => {
                    // the mouse entred the window 
                    draw(0, CursorInfo::window_left(true));

                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::CursorLeft { device_id: _ } => {
                    // the mouse left
                    draw(0, CursorInfo::window_left(false));

                    glutin::event_loop::ControlFlow::Poll
                }
                #[allow(deprecated)]
                glutin::event::WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    modifiers: _,
                } => {
                    // logical position of the mouse 
                    let position = position.to_logical(scale_factor);
                    // it then sends the position to the update
                    draw(0, CursorInfo::pos([position.x, position.y]));
                    // wait for the next frame
                    let wait = start_time + std::time::Duration::from_millis(TIMEPF);
                    glutin::event_loop::ControlFlow::WaitUntil(wait)
                }

                _ => glutin::event_loop::ControlFlow::Poll,
            },
            glutin::event::Event::DeviceEvent {
                device_id: _,
                event,
            } => {
                match event {
                    glutin::event::DeviceEvent::Key(a) => {
                        // draw and then send the keypress to the snake obj

                        draw(a.scancode, CursorInfo::new());
                        //s println!("key: {}",a.scancode);
                        // wait for next frame
                        let wait = start_time + std::time::Duration::from_millis(TIMEPF);
                        glutin::event_loop::ControlFlow::WaitUntil(wait)
                    }
                    glutin::event::DeviceEvent::Button { button, state: _ } => {
                        // keyinput but from the mouse
                        if button == 1 {
                            draw(0, CursorInfo::button_press(true))
                        }
                        let wait = start_time + std::time::Duration::from_millis(TIMEPF);
                        glutin::event_loop::ControlFlow::WaitUntil(wait)
                    }

                    _ => glutin::event_loop::ControlFlow::Poll,
                }
            }
            _ => {
                // run game if it should run the game
                if run_game {
                    draw(0, CursorInfo::new());
                }
                // wait for the next frame
                let wait = start_time + std::time::Duration::from_millis(TIMEPF);
                glutin::event_loop::ControlFlow::WaitUntil(wait)
            }
        };
    });
}
