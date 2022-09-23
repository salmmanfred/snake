



#[macro_use]
extern crate glium;

mod engine; 
use handy_macros::s;

use crate::engine::Vertex;

const RED:[f32;3] = [1.,0.,0.];
const GREEN:[f32;3] = [0.,1.,0.];


enum Fne{
    None,
    Fun(Box<dyn(FnMut(&mut Snake))>)
}




struct Snake{

    data: Vec<Vertex>,

    data_long: Vec<u16>,
    updater: Fne,
    key: u32,
    titbool: bool,
    title: String
}
impl Snake{

    pub fn new(d: bool)->Self{
        let mut snake = Self { titbool: false, title:s!("Snake"), data: Vec::new(), data_long: Vec::new(),key:0, updater: Fne::None};

        match d{
            true =>{
                snake.updater = Self::fun_update();
        
               return snake
            } 
            false =>{
                return snake

            }


        }
        
    }
    pub fn keypr(&mut self, k: u32){
        self.key = k;
    }

    fn fun_update() -> Fne{
        let mut x = 0.5;
        let mut y = 0.5;
        let spedx = 0.05;
        let spedy = 0.05;

        let mut ax = spedx * 1.;
        let mut ay = 0.;

        let mut frame:u64 = 0;

        let mut snakebod: Vec<[f32; 2]> = vec!([x,y]);
        let mut snek_len:usize = 5;
        
        let mut applex:f32 = 0.;
        let mut appley:f32 = 0.;



        let up = move |s: &mut Self| {

            match s.key{
                17 =>{
                    // W
                    ay = spedy;
                    ax = 0.;

                }
                30 =>{
                    // A
                    ay = 0.;
                    ax = spedx * -1.;
                }
                31 =>{
                    // S
                    ay = spedy * -1.;
                    ax = 0.;
                }
                32 =>{
                    // D
                    ay = 0.;
                    ax = spedx;
                }
                1=>{
                    println!("{},{} . {},{}",x,y,ax,ay);
                    
                }
                _=>{

                }
            }

            if frame % 100 == 0{
                x+=ax;
                y+=ay;
                snakebod.push([x, y]);
               // s.change_title("Mashalla");
            }
                
            // push the new bodypart
            // remove bodyparts that are old
            if snakebod.len() > snek_len {
              //  println!("len {:#?}",snakebod);
                snakebod.remove(0);
            }
            s.rectangle(applex,appley, spedx, spedy,RED);

            for x in snakebod.clone(){
                s.rectangle(x[0], x[1], spedx, spedy,GREEN);
            }
            frame +=1;
            
           // s.rectangle(-0.5, 0.5, 0.4, 0.5);
        };



        Fne::Fun(Box::new(up))
    }



    pub fn render(&mut self)->(Vec<Vertex>,Vec<u16>){
        self.data_long = Vec::new();
        self.data = Vec::new();
        
        let mut sn = Snake::new(false);
        sn.key = self.key.clone();
        match &mut self.updater{
            Fne::None =>{

                panic!("This cannot happen")
            }
            Fne::Fun(a)=>{
                 a(&mut sn)
            }
        }
        //(self.updater.un())(&mut sn);

        self.data = sn.data;
        self.data_long = sn.data_long;
        self.title = sn.title;
        self.titbool = sn.titbool;



        let mut g = Vec::new();
        for x in self.data_long.iter() {
            g.push( *x -1);
        }
        
       

       // println!("v {:#?}",g);

        (self.data.clone(),g)


    }
    pub fn latest_long(&self) -> u16{
        return match self.data_long.len(){
            0=>{
                0
            }
            _=>{
                self.data_long[self.data_long.len() -1]
            }
        }
    }


    pub fn rectangle(&mut self,x:f32,y:f32,sizex:f32,sizey:f32,color: [f32;3]){


        let buff = vec!(
            Vertex::new([x+sizex,y+sizey], color),
            Vertex::new([x+sizex,y], color),
            Vertex::new([x,y+sizey], color),
            Vertex::new([x+sizex,y], color),
            Vertex::new([x,y+sizey], color),
            Vertex::new([x,y], color),
        );
        self.data.extend(buff.iter());
        for _ in 0..6{
            let val = self.latest_long();
            
            self.data_long.push(val+1)
        }

        

    }
    pub fn change_title(&mut self, title: &str){
        self.titbool = true;
        self.title = s!(title);
        println!("a{},,{}",self.titbool,self.title);

    }
    pub fn title(&mut self)->(bool,&str){
        let titbool = self.titbool;
        //self.titbool = false;

        (titbool,&self.title)
    }
    

}


fn main() {
    
    engine::run();
    
}
