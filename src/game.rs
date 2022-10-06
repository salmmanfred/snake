use handy_macros::s;
use rand::thread_rng;
use rand::Rng;
use std::time::Instant;
use crate::Fne;
use crate::Snake;
use crate::BLUE;
use crate::DARK_GREEN;
use crate::RED;
use crate::WHITE;

const SPEDX: f32 = 0.1;
const SPEDY: f32 = 0.1;

// logic trait 
pub trait GameLogic {
    fn rooms() -> Vec<Fne>;
}
// implement it with the all the windows
impl GameLogic for Snake {
    fn rooms() -> Vec<Fne> { 
        vec![menu(), game_loop(), end_screen_and_pause(s!("Play again")), end_screen_and_pause(s!("Continue"))]
    }
}
// main menu
fn menu() -> Fne {
    // create the schekred backgroudn grid
    let grid = create_grid();


    let up = move |s: &mut Snake| {
        // render the grid
        for (pos, colour, is_skip) in &grid {
            if *is_skip {
                s.rectangle(*pos, SPEDX, SPEDY, *colour);
            }
        }
        
        s.change_title("Snake menu");
        // big text with the Snake name
        s.text(-5., 7., 1., "Snake");
        // create the play button
        let btn_id = s.button([-0.3, -0.1], [0.6, 0.3, 4.], [5., 3.], WHITE, "Play");
        // check if the play button has yet been pressed
        let btn_prs = s.button_manager();
        if btn_prs == btn_id {
            // change to the game if it has been pressed
            s.interface = 1;
            s.change_title("Snake");
        }
    };

    Fne::Fun(Box::new(up))
}

fn game_loop() -> Fne {
    // the main game
    let mut x = 0.1;
    let mut y = 0.5;
    // the speed in x and y axis;
    let spedx = SPEDX;
    let spedy = SPEDY;
    // acceleration ax and ay (Actually just speed)
    let mut ax = spedx * 0.;
    let mut ay = spedy * -1.;
    // current frame
    let mut frame: u64 = 0;
    // snake body with all the body parts and then the lenght of the snake
    let mut snakebod: Vec<[f32; 2]> = vec![[x, y]];
    let mut snek_len: usize = 1;

    // apples cordinates
    let mut applex: f32 = 0. + spedx;
    let mut appley: f32 = 0. + spedy;

    let grid = create_grid();
    
    let now = Instant::now();
    let up = move |s: &mut Snake| {
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
                // change to the pause screen.
                s.interface = 3;
                println!("{},{} . {},{}", x, y, ax, ay);
            }
            _ => {}
        }
       
        // only update the snake positions every 20th frame
        if frame % 20 == 0 {
            x += ax;
            y += ay;
            // s.change_title("Mashalla");

            // check if the snake has hit itself
            for (i, ii) in snakebod.iter().enumerate() {
                
                // the cordinates roundedup to 0.01 instead of 0.0001 or 0.00001
                // adds the 0.0001 to make sure its it does not make a stupid -0 instead of 0
                if (((ii[0]*100.).round() / 100.)+0.0001,((ii[1]*100.).round() / 100.)+0.0001) == 
                (((x*100.).round() / 100.)+0.0001,((y*100.).round() / 100.)+0.0001)
                    && i <= snakebod.len() - 1
                {
                   // change to loss screen
                    s.interface = 2;
                    // enable lost so it resets everything.
                    s.lost = true;
                    
                    let elapsed_time = now.elapsed();
                    println!("time: {:?}, frames {frame}",elapsed_time)
                }
            }
            // checks collision with the apple 
            if (((applex*100.).round() / 100.)+0.0001,((appley*100.).round() / 100.)+0.0001) == 
            (((x*100.).round() / 100.)+0.0001,((y*100.).round() / 100.)+0.0001) {
                // add lenght 
                snek_len += 1;
                // new apple position
                let xy = new_apple([spedx, spedy]);
                // stop -0 from happening 
                applex = xy[0] + 0.000001;
                appley = xy[1] + 0.000001;
                
                s.score = snek_len;
            }
            snakebod.push([x, y]);
        }

        //collision
        
        // score text 
        s.text(-10., 7., 2., &format!("Score: {} ", snek_len));
        // if the snake body is too big remove the oldest part in the vector
        if snakebod.len() > snek_len {
            snakebod.remove(0);
        }
        // render grid
        for (pos, colour, is_skip) in &grid {
            if *is_skip {
                s.rectangle(*pos, spedx, spedy, *colour);
            }
        }
        // render apple
        s.rectangle([applex, appley], spedx, spedy, RED);
        // render the snake
        for pos in &snakebod {
            s.rectangle(*pos, spedx, spedy, BLUE);
        }

        frame += 1;
    };

    Fne::Fun(Box::new(up))
}

fn end_screen_and_pause(txt: String) -> Fne {
    // what should be displayed

    let txt = txt.clone();
    let grid = create_grid();
    

    let up = move |s: &mut Snake| {
        for (pos, colour, is_skip) in &grid {
            if *is_skip {
                s.rectangle(*pos, SPEDX, SPEDY, *colour);
            }
        }
        // make the text saying what score you are currently on
        s.text(-5., 7., 1., &format!("Score: {}", s.score));
        // buttons to play again or continue or just exit 
        let btn_id = s.button([-0.3, -0.1], [0.6, 0.3, 2.], [9., 4.], WHITE, txt.as_str());
        let btn_id_2 = s.button([-0.3, -0.5], [0.6, 0.3, 4.], [5., 8.], WHITE, "Exit");
        // check the buttons
        let btn_prs = s.button_manager();
        if btn_prs == btn_id {
            s.interface = 1;
        } else if btn_prs == btn_id_2 {
            panic!(""); // Todo: Better exit system.
        }
    };

    Fne::Fun(Box::new(up))
}

// new position for the apple
fn new_apple(rng_rang: [f32; 2]) -> [f32; 2] {
    let mut rng = thread_rng();
    let x = rng.gen_range(-1.0..1.0);
    // make sure its on the grid
    let x = x - (x % rng_rang[0]);

    let y = rng.gen_range(-1.0..1.0);
    let y = y - (y % rng_rang[1]);

    [x, y]
}
// creates a shekerd type grids
fn create_grid()->Vec<([f32; 2], [f32; 3], bool)>{
    let mut grid: Vec<([f32; 2], [f32; 3], bool)> = Vec::new();
    let mut is_dark_green = false;

    for y in (-100..100).step_by((SPEDY * 100.) as usize) {
        is_dark_green = !is_dark_green;

        for x in (-100..100).step_by((SPEDY * 100.) as usize) {
            is_dark_green = !is_dark_green;

            grid.push((
                [x as f32 / 100., y as f32 / 100.],
                DARK_GREEN,
                is_dark_green,
            ));
        }
    }
    return grid;
}
