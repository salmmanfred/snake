



#[macro_use]
extern crate glium;

mod engine; 
use crate::engine::Vertex;

const RED:[f32;3] = [1.,0.,0.];

enum Fne{
    None,
    Fun(Box<dyn(FnMut(&mut Snake))>)
}




struct Snake{

    data: Vec<Vertex>,

    data_long: Vec<u16>,
    updater: Fne,
}
impl Snake{
    pub fn new(d: bool)->Self{
        match d{
            true =>{
                
        
                Self {  data: Vec::new(), data_long: Vec::new(), updater:Self::fun_update()}
            } 
            false =>{
                Self {  data: Vec::new(), data_long: Vec::new(), updater: Fne::None}

            }


        }
        
    }

    fn fun_update() -> Fne{
        let mut x = -1.;



        let up = move |s: &mut Self| {
            x+=0.01;
            s.rectangle(x, 0.5, 0.05, 0.05);
            s.rectangle(-0.5, 0.5, 0.4, 0.5);
        };



        Fne::Fun(Box::new(up))
    }



    pub fn render(&mut self)->(Vec<Vertex>,Vec<u16>){
        self.data_long = Vec::new();
        self.data = Vec::new();
        self.update();
        let mut g = Vec::new();
        for x in self.data_long.iter() {
            g.push( *x -1);
        }
       

        println!("v {:#?}",g);

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
    pub fn rectangle(&mut self,x:f32,y:f32,sizex:f32,sizey:f32){


        let buff = vec!(
            Vertex::new([x+sizex,y+sizey], RED),
            Vertex::new([x+sizex,y], RED),
            Vertex::new([x,y+sizey], RED),
            Vertex::new([x+sizex,y], RED),
            Vertex::new([x,y+sizey], RED),
            Vertex::new([x,y], RED),
        );
        self.data.extend(buff.iter());
        for _ in 0..6{
            let val = self.latest_long();
            
            self.data_long.push(val+1)
        }

        

    }
    pub fn update(&mut self){
        let mut sn = Snake::new(false);
        match &mut self.updater{
            Fne::None =>{

                panic!("Err row 19 this is not suppose to happen")
            }
            Fne::Fun(a)=>{
                 a(&mut sn)
            }
        }
        //(self.updater.un())(&mut sn);

        self.data = sn.data;
        self.data_long = sn.data_long;


        

    }

}


fn main() {
    
        engine::run();
    
}
