mod bindings;
mod flags;
#[macro_use]
mod color_pallet;
use bindings::{COLOR_PAIR, initscr, refresh, getch, endwin, init_color, init_pair, mvaddch, start_color, attron, attroff};
use flags::{Pixel, make_flag};
mod parser;
use parser::parse_text;
use std::fs;
use crate::flags::{correct, shuffle};

fn main() { // Shows flags random pixel with one array with pxel values ranom from this array pop is from array
    //let path = option_env!("FLAG_PATH").unwrap_or(panic!("FLAG_PATH not set set it to the Path of the flag"));
    //let file = fs::read_to_string(path).expect("Error While reading the file");
    //let structure = parse_text(file); 
    
    unsafe {
        initscr();
        start_color();
        
        let structure = parse_text(String::from("COLOR_PINK={0,255,0}
                                 COLOR_BLUE={0,255,0}
                                 [[COLOR_PINK, COLOR_BLUE],[COLOR_PINK, COLOR_BLUE, COLOR_PINK]]
                                "));
        //make_color!(COLOR_PAIR_RED, COLOR_PAIR_BLUE, COLOR_PAIR_PINK, COLOR_PAIR_PURPLE);
        /*
        let mut flag = make_flag(String::from("Hello World"), vec! [vec![COLOR_PAIR_PINK as i32,COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32],
                                                                vec![COLOR_PAIR_PINK as i32,COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32],
                                                                vec![COLOR_PAIR_PURPLE as i32,COLOR_PAIR_PURPLE as i32, COLOR_PAIR_PURPLE as i32, COLOR_PAIR_PURPLE as i32],
                                                                vec![COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32],
                                                                vec![COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32]]);
        */
        let mut flag = make_flag(String::from("Pride"), structure);
        correct(&mut flag);
        shuffle(&mut flag);
        flag.draw(); // REFERESH has to be moved
        getch();
        endwin();
    }
}
