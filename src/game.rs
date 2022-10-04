use rand::Rng;
use rand::thread_rng;

use crate::BLUE;
use crate::DARK_GREEN;
use crate::Fne;
use crate::RED;
use crate::Snake;
use crate::WHITE;


const SPEDX: f32 = 0.1;
const SPEDY: f32 = 0.1;



pub trait GameLogic{
    fn menue() -> Fne;
    fn game_loop() -> Fne;
    fn end_screen() -> Fne;
}
impl GameLogic for Snake{
    fn menue() -> Fne {
       
        let mut grid: Vec<([f32;2],[f32;3],bool)> = Vec::new();
        let mut is_dark_green = false;
        
        
        for y in (-100..100).step_by((SPEDY * 100.) as usize){
            
            is_dark_green = !is_dark_green;

            for x in (-100..100).step_by((SPEDY * 100.) as usize){
                is_dark_green = !is_dark_green;
                

                grid.push(([x as f32/100.,y as f32/100.],DARK_GREEN,is_dark_green));
            }
        }

        let up = move |s: &mut Self| {

            for (pos, colour,is_skip) in &grid{
                if *is_skip{
                    s.rectangle(*pos, SPEDX, SPEDY, *colour);
                }
                
            }
            s._change_title("Snake Menue");



            s.text(-5.,7., 1., "Snake");

            let btn_id = s.button([-0.3,-0.1], [0.6,0.3,4.],[5.,3.],WHITE, "Play");

            
           

            
            
            let btn_prs = s.button_manager();
            if btn_prs == btn_id{
                s.interface = 1;
                s._change_title("Snake");
                
            }


        };

        Fne::Fun(Box::new(up))
    }

    fn game_loop() -> Fne {
        let mut x = 0.1;
        let mut y = 0.5;
        let spedx = SPEDX;
        let spedy = SPEDY;

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
              // panic!("fuck you continue coding")
            }


            if frame % 100 == 0 {
                x += ax;
                y += ay;
                // s.change_title("Mashalla");

                for (i, ii) in snakebod.iter().enumerate(){

                  //  println!("{},{},{}",i,&format!("{:.2},{:.2} ",ii[0].abs(),ii[1].abs()),&format!("{:.2},{:.2} ",x.abs(),y.abs()) );


                    if &format!("{:.2},{:.2} ",ii[0]+0.00001,ii[1]+0.00001) == &format!("{:.2},{:.2} ",x+0.00001,y+0.00001) 
                    && i <= snakebod.len()-1{
    
                        //panic!("GAME LOST");
                        s.interface = 2;
    
                    }
    
    
                }
                snakebod.push([x, y]);

            }

            //collision
            if format!("{:.2},{:.2} ",applex,appley) == format!("{:.2},{:.2} ",x,y){
                snek_len+=1;
              let xy = new_apple([spedx,spedy]);
                
                applex = xy[0]+0.000001;
                appley = xy[1]+0.000001;
                s.score = snek_len;
                
            }
           
            s.text(-10., 7., 2., &format!("Score: {} ",snek_len));
            

            



            
            if snakebod.len() > snek_len {
                
                snakebod.remove(0);
            }
           

            for (pos, colour,is_skip) in &grid{
                if *is_skip{
                    s.rectangle(*pos, spedx, spedy, *colour);
                }
                
            }
            

            s.rectangle([applex, appley], spedx, spedy, RED);
           

            for pos in snakebod.clone() {
                s.rectangle(pos, spedx, spedy, BLUE);
            }
            

            frame += 1;
            
             


        };

        Fne::Fun(Box::new(up))
    }

    fn end_screen() -> Fne {


        let mut grid: Vec<([f32;2],[f32;3],bool)> = Vec::new();
        let mut is_dark_green = false;
        
        
        for y in (-100..100).step_by((SPEDY * 100.) as usize){
            
            is_dark_green = !is_dark_green;

            for x in (-100..100).step_by((SPEDY * 100.) as usize){
                is_dark_green = !is_dark_green;
                

                grid.push(([x as f32/100.,y as f32/100.],DARK_GREEN,is_dark_green));
            }
        }

        let up = move |s: &mut Self| {

            for (pos, colour,is_skip) in &grid{
                if *is_skip{
                    s.rectangle(*pos, SPEDX, SPEDY, *colour);
                }
                
            }


            s.text(-5.,7., 1., &format!("Score: {}",s.score));
            let btn_id = s.button([-0.3,-0.1], [0.6,0.3,2.],[9.,4.],WHITE, "Play Again");
            let btn_id_2 = s.button([-0.3,-0.5], [0.6,0.3,4.],[5.,8.],WHITE, "Exit");


            
            let btn_prs = s.button_manager();
            if btn_prs == btn_id{
                s.interface = 1;
                
            }else if btn_prs == btn_id_2{
                panic!("");
            }


        };

        Fne::Fun(Box::new(up))
    }
}


fn new_apple(rng_rang: [f32;2]) -> [f32; 2] {
    let mut rng = thread_rng();
    let x = rng.gen_range(-1.0..1.0);
    let x = x - (x % rng_rang[0]);

    let y = rng.gen_range(-1.0..1.0);
    let y = y - (y % rng_rang[1]);



    [x, y]
}