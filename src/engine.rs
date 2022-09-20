use crate::Snake;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
impl Vertex{
    pub fn new(position: [f32;2], color: [f32;3])->Self{
        Self{
            position,
            color
        }
    }
}

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;


pub fn run() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Snake")
    .with_resizable(false)
    .with_inner_size(glutin::dpi::Size::Logical(glutin::dpi::LogicalSize{width: 500.,height: 500.}));
    


    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    // building the vertex buffer, which contains all the vertices that we will draw
    
        

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
  
    ).unwrap();

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.


    



    let mut snake = Snake::new(true);
    let mut draw = move |x: u32| {
        snake.keypr(x);
        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0., 0.5, 0.0);
        let (pbuf,fbuf) = snake.render();
        let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
            &fbuf).unwrap();


        let vertex_buffer =     glium::VertexBuffer::new(&display,
            &pbuf
        ).unwrap();
        let (new_title, title) = snake.title();

        if new_title{
        println!("b{},{}",new_title,title);

            display.gl_window().window().set_title(title);
        }


        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        
        target.finish().unwrap();
    };

    // Draw the triangle to the screen.
    draw(0);

    // the main loop
    event_loop.run(move |event, _, control_flow| {
       
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    draw(0);
                    glutin::event_loop::ControlFlow::Poll
                },
                glutin::event::WindowEvent::Focused(a) =>{
                    if a {
                        draw(0);
                    }
                    glutin::event_loop::ControlFlow::Poll

                }

          
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            glutin::event::Event::DeviceEvent { device_id: _, event }=>{
                match event {
                    glutin::event::DeviceEvent::Key(a) =>{
                        draw(a.scancode);
                       //s println!("key: {}",a.scancode);
                        

                        glutin::event_loop::ControlFlow::Poll
                    }
                    
                    _=>glutin::event_loop::ControlFlow::Poll
                }
            }
            _ =>{ 
                draw(0);
                glutin::event_loop::ControlFlow::Poll
            },
        };
    });
}