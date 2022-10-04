#[macro_use]
extern crate glium;

mod engine;


use handy_macros::s;
use rand::{thread_rng, Rng};

use crate::engine::Vertex;

const RED: [f32; 3] = [1., 0., 0.];
const GREEN: [f32; 3] = [0., 1., 0.];
const DARK_GREEN: [f32; 3] = [0., 0.6, 0.];
const BLUE: [f32; 3] = [0., 0., 1.];
const WHITE: [f32; 3] = [1., 1., 1.];
#[derive(Debug,Clone, Copy)]
enum CursorChange{
    New, // everything is new 
    WindowChange, // only the window information is new,
    PosChange, // Only the position is new
    ButtonChange, // when the button has been changed
    NoNewInfo,// the obj is empty and should be thrownaway
}


#[derive(Debug,Clone, Copy)]
struct CursorInfo{
    pos: [f64;2],
    in_window: bool,
    button_press: bool,
    info: CursorChange

}
impl CursorInfo{
    pub fn new()->Self{
        Self { pos: [0.,0.],in_window: false, button_press: false, info:CursorChange::NoNewInfo }
    }
    pub fn pos(pos:[f64;2])->Self{
        Self { pos, in_window: false, button_press: false, info: CursorChange::PosChange }
    }
    pub fn window_left(info: bool)->Self{
        Self { pos: [0.,0.], in_window: info, button_press: false,  info: CursorChange::WindowChange }
    }
    pub fn button_press(info: bool)->Self{
        Self { pos: [0.,0.], in_window: false, button_press: info,  info: CursorChange::ButtonChange }

    }
}


enum Fne {
    None,
    Fun(Box<dyn (FnMut(&mut Snake))>),
}

struct Snake {
    //graphics stuff
    data: Vec<Vertex>,
    data_long: Vec<u16>,
    // text stuff
    // posx, posy,size,text
    data_text: Vec<((f32, f32, f32), String)>,
    text_info: (f32, f32, f32),
    // update loop
    updater: Fne,
    // input stuff
    key: u32,
    mouse: CursorInfo,
    //Title stuff
    titbool: bool,
    title: String,
    //button stuff
    //x,y sizex,sizey,sizetext in the f32;5
    // colour in f32;3
    buttons: Vec<([f32;5],[f32;3])>,
    

}
impl Snake {
    pub fn new(d: bool) -> Self {
        let mut snake = Self {
            titbool: false,
            title: s!("Snake"),
            data: Vec::new(),
            data_long: Vec::new(),
            key: 0,
            updater: Fne::None,
            data_text: Vec::new(),
            text_info: (0., 0., 0.),
            buttons: Vec::new(),
            mouse: CursorInfo::new(),
            
        };

        match d {
            true => {
                snake.updater = Self::fun_update();

                return snake;
            }
            false => return snake,
        }
    }

    pub fn keypr(&mut self, k: u32) {
        self.key = k;
    }
    
    pub fn update_text_info(&mut self, info: (f32, f32, f32)) {
        self.text_info = info;
    }
    pub fn index_size(&mut self) -> usize {
        self.data_text.len()
    }
    pub fn text_info_get(&self, index: usize) -> String {
        let (_, text) = self.data_text[index].clone();
        text
    }
    pub fn render_text(&mut self, index: usize) -> [[f32; 4]; 4] {
        let ((x, y, size), _) = self.data_text[index];

        let (w, h, mut text_width) = self.text_info;
        text_width *= size;

        let x = x / 100.;
        let y = y / 100.;

        let matrix_text: [[f32; 4]; 4] = [
            [0.1 / text_width, 0.0, 0.0, 0.0],
            [0.0, 0.1 * (w as f32) / (h as f32) / text_width, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [x, y, 0.0, 0.1f32],
        ];
        matrix_text
    }

    fn text(&mut self, posx: f32, posy: f32, size: f32, text: &str) {
        self.data_text.push(((posx, posy, size), s!(text)))
    }

    

    pub fn render(&mut self) -> (Vec<Vertex>, Vec<u16>) {
        self.data_long = Vec::new();
        self.data = Vec::new();
        self.data_text = Vec::new();

        let mut sn = Snake::new(false);
        sn.key = self.key.clone();
        sn.mouse = self.mouse;
        match &mut self.updater {
            Fne::None => {
                panic!("This cannot happen")
            }
            Fne::Fun(a) => a(&mut sn),
        }
        //(self.updater.un())(&mut sn);
        
        self.data = sn.data;
        self.data_long = sn.data_long;
        self.title = sn.title;
        self.titbool = sn.titbool;
        self.data_text = sn.data_text;
        

        let mut g = Vec::new();
        for x in self.data_long.iter() {
            g.push(*x - 1);
        }

        // println!("v {:#?}",g);

        (self.data.clone(), g)
    }
    pub fn latest_long(&self) -> u16 {
        return match self.data_long.len() {
            0 => 0,
            _ => self.data_long[self.data_long.len() - 1],
        };
    }

    fn rectangle(&mut self, pos: [f32;2], sizex: f32, sizey: f32, color: [f32; 3]) {
        let (x,y) = (pos[0],pos[1]);

        let buff = vec![
            Vertex::new([x + sizex, y + sizey], color),
            Vertex::new([x + sizex, y], color),
            Vertex::new([x, y + sizey], color),
            Vertex::new([x + sizex, y], color),
            Vertex::new([x, y + sizey], color),
            Vertex::new([x, y], color),
        ];
        self.data.extend(buff.iter());
        for _ in 0..6 {
            let val = self.latest_long();

            self.data_long.push(val + 1)
        }
    }
    pub fn _change_title(&mut self, title: &str) {
        self.titbool = true;
        self.title = s!(title);
        println!("a{},,{}", self.titbool, self.title);
    }
    pub fn title(&mut self) -> (bool, &str) {
        let titbool = self.titbool;
        //self.titbool = false;
        (titbool, &self.title)
    }
    pub fn move_mouse(&mut self,info: CursorInfo){
       
        

       // println!("sent here");
        match info.info {
             CursorChange::WindowChange =>{
                
                self.mouse.in_window = info.in_window;

            }
            CursorChange::PosChange =>{
              
                self.mouse.pos = info.pos;
      
            }
            CursorChange::New =>{
                self.mouse = info;
   

            }
            CursorChange::ButtonChange =>{
                self.mouse.button_press = info.button_press;
                
            }
            
            CursorChange::NoNewInfo =>{
                self.mouse.button_press = false;

                drop(info);
        //println!("sent here4");

            }


        }
    }
    pub fn register_button(&mut self, etc: [f32;5],col: [f32;3])->usize{
        self.buttons.push((etc,col));
        self.buttons.len()-1
        
    }
    pub fn button(&mut self, pos: [f32;2],size:[f32;3],fine_tuning: [f32;2],col: [f32; 3],text: &str) -> usize{

        // translate graphics coordinates to text coordinates
        // seems like 1 point in textcord is the same as 0.1 in graphics cords
        // so pos[0] * 10 should be it 
        // leave fine_tuning to [10,10] for standard

        let text_pos = [pos[0] * fine_tuning[0], pos[1] * fine_tuning[1]];
        self.text(text_pos[0], text_pos[1], size[2], text);
        self.rectangle(pos, size[0], size[1], col);
        self.register_button([pos[0],pos[1],size[0],size[1],size[2]], col)


    }

    pub fn button_manager(&mut self){
        let mousepos = translate_mouse_cords(self.mouse.pos);
        let mousex = mousepos[0];
            let mousey = mousepos[1];
        self.rectangle([mousex as f32, mousey as f32], 0.1, 0.1, WHITE);
        
        
        if self.mouse.button_press && self.mouse.in_window{
            

            
            println!("mousebtn prs");
            for (i,o) in self.buttons.iter().enumerate(){
                let o = *o;
                let x = o.0;
                let bx = x[0] as f64;
                let by = x[1] as f64;
                let bw = x[2] as f64;
                let bh = x[3] as f64;


                if 
                    bx < mousex + 0.1 &&
                    bx + bw > mousex &&
                    by < mousey + 0.1 &&
                    bh + by > mousey
                   {

                    panic!("press btn");

                   }
            }


        }else{
            return ()
        }
    }


    fn fun_update() -> Fne {
        let mut x = 0.1;
        let mut y = 0.5;
        let spedx = 0.1;
        let spedy = 0.1;

        let mut ax = spedx * 0.;
        let mut ay = spedy * -1.;

        let mut frame: u64 = 0;

        let mut snakebod: Vec<[f32; 2]> = vec![[x, y]];
        let mut snek_len: usize = 1;

        let mut applex: f32 = 0. + spedx;
        let mut appley: f32 = 0. + spedy;

        let mut grid: Vec<([f32;2],[f32;3],bool)> = Vec::new();
        let mut is_dark_green = false;
        
        
        for y in (-100..100).step_by((spedy * 100.) as usize){
            
            is_dark_green = !is_dark_green;

            for x in (-100..100).step_by((spedy * 100.) as usize){
                is_dark_green = !is_dark_green;
                

                grid.push(([x as f32/100.,y as f32/100.],DARK_GREEN,is_dark_green));
            }
        }



       

        let up = move |s: &mut Self| {
            

            match s.key {
                17 => {
                    // W
                    ay = spedy;
                    ax = 0.;
                }
                30 => {
                    // A
                    ay = 0.;
                    ax = spedx * -1.;
                }
                31 => {
                    // S
                    ay = spedy * -1.;
                    ax = 0.;
                }
                32 => {
                    // D
                    ay = 0.;
                    ax = spedx;
                }
                1 => {
                    println!("{},{} . {},{}", x, y, ax, ay);
                }
                _ => {}
            }
            if frame == 30000{
             //   panic!("fuck you continue coding")
            }


            if frame % 200 == 0 {
                x += ax;
                y += ay;
                // s.change_title("Mashalla");

                for (i, ii) in snakebod.iter().enumerate(){

                  //  println!("{},{},{}",i,&format!("{:.2},{:.2} ",ii[0].abs(),ii[1].abs()),&format!("{:.2},{:.2} ",x.abs(),y.abs()) );


                    if &format!("{:.2},{:.2} ",ii[0],ii[1]) == &format!("{:.2},{:.2} ",x,y) 
                    && i <= snakebod.len()-1{
    
                        panic!("GAME LOST");
    
    
                    }
    
    
                }
                snakebod.push([x, y]);

            }

            //collision
            if format!("{:.2},{:.2} ",applex,appley) == format!("{:.2},{:.2} ",x,y){
                //snek_len+=1;
            /*     let xy = new_apple([spedx,spedy]);
                applex = xy[0];
                appley = xy[1];*/
                
            }
           // s.text(-10., 7., 1., &format!("x: {:.2},{:.2} ",applex,x.abs()));
            s.text(-10., 7., 2., &format!("Score: {} ",snek_len));
            s.text(-15., -5., 0.2, &format!("Mouse: {:#?} ",s.mouse));

            



            // push the new bodypart
            // remove bodyparts that are old
            if snakebod.len() > snek_len {
                //  println!("len {:#?}",snakebod);
                snakebod.remove(0);
            }
            //s.text(-10., 7., 1., "Sample text 123456890");
            //s.text(-5., -7., 3., "Sample text .2..");

            for (pos, colour,is_skip) in &grid{
                if *is_skip{
                    s.rectangle(*pos, spedx, spedy, *colour);
                }
                
            }
            s.button([-0.5,-0.5], [0.6,0.3,2.],[10.,10.],WHITE, "text test");

            s.rectangle([applex, appley], spedx, spedy, RED);
           

            for pos in snakebod.clone() {
                s.rectangle(pos, spedx, spedy, BLUE);
            }
            

            frame += 1;
            
             s.rectangle([0.,0.5], 0.1, 0.1, BLUE);
             s.button_manager();
        };

        Fne::Fun(Box::new(up))

    }

}


// TODO: add fixed intervals use time or something
// TODO: UI
// TODO: Colour change on the text
// TODO:comments
// TODO: Clean up rendering pipeline and speeding up
// TODO: finish game
fn main() {
    engine::run();
}



fn new_apple(rng_rang: [f32;2]) -> [f32; 2] {
    let mut rng = thread_rng();
    let x = rng.gen_range(-1.0..1.0);
    let x = x - (x % rng_rang[0]);

    let y = rng.gen_range(-1.0..1.0);
    let y = y - (y % rng_rang[1]);

    [x, y]
}
// TODO: Only works in the upper right corner rn 
// TODO: have each corner its own translation.
fn translate_mouse_cords(pos: [f64;2])->[f64;2]{
    let mut pos = pos;
    let mut new_pos = [0.,0.];
    
    if pos[0] >= 250.{
        let it1 = (pos[0] / 25.) -10.;
        let posx = it1/ 10.;
      //  println!("posx: {posx} {} +",it1+10.,);
        new_pos[0] = posx;

    }else{
        let it1 = pos[0] / -25.;
        let posx = it1 / 10.;
     //   println!("posx: {posx}   -");
        new_pos[0] = posx;
    }

    
   // pos[1] = 500.- pos[1];
    if pos[1] <= 250.{
   // pos[1] = 500.- pos[1];

    }

    if pos[1] >= 250.{
        let it1 = (pos[1] / -25.);
        let mut posy = it1/ 10.;
        posy +=0.6;
        println!("posx: {posy} {} {} +",it1,pos[1]);
        new_pos[1] = posy + 0.3;

    }else{
        let it1 = (pos[1] / -25.);// - 10. ;
        let mut posy = it1 / 10.;
        posy += 1.;

        println!("posx: {posy} {} {} -",it1,it1*-25.);



        new_pos[1] = posy - 0.1;
        
    }
   // println!("translated pos: {:?}",new_pos);

    new_pos
}