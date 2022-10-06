#[macro_use]
extern crate glium;

mod engine;
mod game;
use crate::game::GameLogic;

use handy_macros::s;

use crate::engine::Vertex;
/*
These are the colours I will be using 

*/
const RED: [f32; 3] = [1., 0., 0.];
#[allow(dead_code)]
const GREEN: [f32; 3] = [0., 1., 0.];
const DARK_GREEN: [f32; 3] = [0., 0.6, 0.];
const BLUE: [f32; 3] = [0., 0.09, 1.];
const WHITE: [f32; 3] = [1., 1., 1.];


/*
This is an enum that sends the different commands on how to change cursor. 
*/
#[derive(Debug, Clone, Copy)]
enum CursorChange {          // everything is new
    WindowChange, // only the window information is new,
    PosChange,    // Only the position is new
    ButtonChange, // when the button has been changed
    NoNewInfo,    // the obj is empty and should be thrownaway
}
/*
This struct holds all cursor related information 
*/
#[derive(Debug, Clone, Copy)]
pub struct CursorInfo {
    pos: [f64; 2], // the current position of the cursor (in cursor position space)
    in_window: bool, // if the cursor is in the window 
    button_press: bool, // if there has been a button pressed
    info: CursorChange, // what has been changed in the Cursor 
}
impl CursorInfo {
    pub fn new() -> Self {
        Self {
            pos: [0., 0.],
            in_window: false,
            button_press: false,
            info: CursorChange::NoNewInfo,
        }
    }
    pub fn pos(pos: [f64; 2]) -> Self {
        Self {
            pos,
            in_window: false,
            button_press: false,
            info: CursorChange::PosChange,
        }
    }
    pub fn window_left(info: bool) -> Self {
        Self {
            pos: [0., 0.],
            in_window: info,
            button_press: false,
            info: CursorChange::WindowChange,
        }
    }
    pub fn button_press(info: bool) -> Self {
        Self {
            pos: [0., 0.],
            in_window: false,
            button_press: info,
            info: CursorChange::ButtonChange,
        }
    }
}

// Enum for holding function loops
pub enum Fne {
    None,
    Fun(Box<dyn (FnMut(&mut Snake))>),
}
/*  the main snake Struct 
This holds all the infromation needed to run the game
*/
pub struct Snake {
    //graphics stuff
    data: Vec<Vertex>,
    data_long: Vec<u16>,
    // text stuff
    // posx, posy,size,text
    data_text: Vec<((f32, f32, f32), String)>,
    // holds the width and height of the screen
    text_info: (f32, f32), 
    
    // update loop
    rooms: Vec<Fne>,
    //current interface
    interface: usize,
    //score
    score: usize,
    lost: bool,
    // input stuff
    key: u32,
    mouse: CursorInfo,
    //Title stuff
    titbool: bool,
    title: String,
    //button stuff
    //x,y sizex,sizey,sizetext in the f32;5
    // colour in f32;3
    buttons: Vec<([f32; 5], [f32; 3])>,
}
impl Snake {
    // create a new Snake struct 
    // the d bool is if it needs to add the game loops or not 
    pub fn new(d: bool) -> Self {
        let mut snake = Self {
            titbool: false,
            title: s!("Snake"),
            data: Vec::new(),
            data_long: Vec::new(),
            key: 0,
            rooms: Vec::new(),
            interface: 0,
            score: 0,
            lost: false,
            data_text: Vec::new(),
            text_info: (0., 0.),
            buttons: Vec::new(),
            mouse: CursorInfo::new(),
        };

        match d {
            true => {
                snake.rooms = Snake::rooms();

                return snake;
            }
            false => return snake,
        }
    }
    // register the new key press
    pub fn keypr(&mut self, k: u32) {
        self.key = k;
    }
    // update general text information of width and height
    pub fn update_text_info(&mut self, info: (f32, f32)) {
        self.text_info = info;
    }
    
    pub fn index_size(&mut self) -> usize {
        self.data_text.len()
    }
    // function to get the text data at index
    pub fn text_info_get(&self, index: usize) -> &String {
        &self.data_text[index].1
    }
    //function to get all the data needed for the text 
    pub fn render_text(&mut self, index: usize, width: f32) -> [[f32; 4]; 4] {
        // get the data of the text except the string.
        let ((x, y, size), _) = self.data_text[index];
        // get the width and height of the window.
        let (w, h) = self.text_info;
        //get the "True width"
        let text_width = width * size;
        //x and y needs to be in the 0.01 space of the coordinate system
        let x = x / 100.;
        let y = y / 100.;
        // assembles all the data to get the matrix that is gonna be applied to the text 
        let matrix_text: [[f32; 4]; 4] = [
            [0.1 / text_width, 0.0, 0.0, 0.0],
            [0.0, 0.1 * (w as f32) / (h as f32) / text_width, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            //x and y positions apparently (I have no idea how these matrixes work)
            [x, y, 0.0, 0.1f32],
        ];
        matrix_text
    }
    // nice frontend to create a text 
    fn text(&mut self, posx: f32, posy: f32, size: f32, text: &str) {
        self.data_text.push(((posx, posy, size), s!(text)))
    }
    // function called to get all the neccesary data to run the game
    pub fn render(&mut self) -> (&Vec<Vertex>,&Vec<u16>) {
      
        //create a snake obj that is going to be sent to the loop
        let mut sn = Snake::new(false);
        // copies over neccesary information
        sn.key = self.key;
        sn.mouse = self.mouse;
        sn.interface = self.interface;
        sn.score = self.score;

        match &mut self.rooms[self.interface] {
            Fne::Fun(a) => a(&mut sn),
            _=>{}
        }
        //(self.updater.un())(&mut sn);
        // gets the new neccesary infromation for next time
        // im not doing self = sn because not all information needs to be ported over 
        self.data = sn.data;
        self.interface = sn.interface;
        self.data_long = sn.data_long;
        self.title = sn.title;
        self.titbool = sn.titbool;
        self.data_text = sn.data_text;
        self.score = sn.score;
        // if sn.lost is true it will clean all rooms and reset them to zero
        if sn.lost {
            self.rooms = Self::rooms();
        } 

        (&self.data, &self.data_long)
    }
    
    // creates a rectangle
    fn rectangle(&mut self, pos: [f32; 2], sizex: f32, sizey: f32, color: [f32; 3]) {
        // translate into x and y 
        let (x, y) = (pos[0], pos[1]);
        // create the vertext buffer for a rectangle
        let buff = vec![
            Vertex::new([x + sizex, y + sizey], color),
            Vertex::new([x + sizex, y], color),
            Vertex::new([x, y + sizey], color),
            Vertex::new([x + sizex, y], color),
            Vertex::new([x, y + sizey], color),
            Vertex::new([x, y], color),
        ];
        // send the buffer into data
        self.data.extend(buff.iter());
        // creates the fragment buffer
        for x in self.data_long.len()..self.data_long.len()+6 {
            self.data_long.push(x as u16)
        }
    }
    // change the title of the screen
    
    pub fn change_title(&mut self, title: &str) {
        self.titbool = true;
        self.title = s!(title);
        //println!("a{},,{}", self.titbool, self.title);
    }
    // returns the title and if it has been changed
    pub fn title(&mut self) -> (&bool, &str) {
        (&self.titbool, &self.title)
    }
    // gets new information on the mouse
    pub fn move_mouse(&mut self, info: CursorInfo) {
        // println!("sent here");
        match info.info {
            // if it has gone in or out of the screen
            CursorChange::WindowChange => {
                self.mouse.in_window = info.in_window;
            }
            // changes in the mouse position
            CursorChange::PosChange => {
                self.mouse.pos = info.pos;
            }
           // if there has been a button press
            CursorChange::ButtonChange => {
                self.mouse.button_press = info.button_press;
            }
            // if nothing has been changed
            CursorChange::NoNewInfo => {
                self.mouse.button_press = false;

                drop(info);
                
            }
        }
    }
    // register a new button and sends the "Id" of the button
    pub fn register_button(&mut self, etc: [f32; 5], col: [f32; 3]) -> usize {
        self.buttons.push((etc, col));
        self.buttons.len()
    }
    // create a button element
    pub fn button(
        &mut self,
        pos: [f32; 2],
        size: [f32; 3],
        fine_tuning: [f32; 2],
        col: [f32; 3],
        text: &str,
    ) -> usize {
        // translate graphics coordinates to text coordinates
        // seems like 1 point in textcord is the same as 0.1 in graphics cords
        // so pos[0] * 10 should be it
        // leave fine_tuning to [10,10] for standard

        let text_pos = [pos[0] * fine_tuning[0], pos[1] * fine_tuning[1]];
        self.text(text_pos[0], text_pos[1], size[2], text);
        self.rectangle(pos, size[0], size[1], col);
        self.register_button([pos[0], pos[1], size[0], size[1], size[2]], col)
    }
    // manages all the buttons 
    pub fn button_manager(&mut self) -> usize {
        /*let mousepos = translate_mouse_cords(self.mouse.pos);
            let mousex = mousepos[0];
            let mousey = mousepos[1];
        self.rectangle([mousex as f32,mousey as f32], 0.1, 0.1, WHITE);*/


        //if there has been a button press and the mouse is in the window 
        // then it will run the program
        if self.mouse.button_press && self.mouse.in_window {
            // get the mouse position in the graphics coordinates 
            let mousepos = translate_mouse_cords(self.mouse.pos);
            let mousex = mousepos[0];
            let mousey = mousepos[1];
            // checks all the buttons 
            for (i, o) in self.buttons.iter().enumerate() {
                let o = *o;
                let x = o.0;
                let bx = x[0] as f64;
                let by = x[1] as f64;
                let bw = x[2] as f64;
                let bh = x[3] as f64;
                // if it is overlapping it will return the button id
                if bx < mousex + 0.05 && bx + bw > mousex && by < mousey + 0.05 && bh + by > mousey
                {
                    return i + 1;
                }
            }
            return 0;
        } else {
            return 0;
        }
    }
}

// TODO: add fixed intervals use time or something (Done)
// TODO: UI (Done)
// TODO: Make a menue (Done)
// TODO: Text overhaul 
// TODO: Comments (Done)
// TODO: Clean up rendering pipeline and speeding up (WIP)
// TODO: Finish game
fn main() {
    engine::run();
}

//make the mouse cords get into graphics cords
fn translate_mouse_cords(pos: [f64; 2]) -> [f64; 2] {
    // you might be asking how i got these numbers and if I got them using some sort of
    //mathematical formula but no this is by trial and error
    [((pos[0] / 250.) - 1.), ((pos[1] / -250.) + 0.9)]
}
