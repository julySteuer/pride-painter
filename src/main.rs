mod bindings;
mod flags;
#[macro_use]
mod color_pallet;
use bindings::{COLOR_PAIR, initscr, refresh, getch, endwin, init_color, init_pair, mvaddch, start_color, attron, attroff};
use color_pallet::make_color_from_pallet;
use flags::{Pixel, make_flag, Flag};
mod parser;
use parser::parse_text;
use std::fs;
use std::{thread, time};
use crate::{flags::{correct, shuffle, wave_anim}, bindings::clear};
//TODO: Waving animation with sin curve x axis is the x val calc offset  
// Make File with.f of .flag ending
fn main() { // Shows flags random pixel with one array with pxel values ranom from this array pop is from array
    //let path = option_env!("FLAG_PATH").unwrap_or(panic!("FLAG_PATH not set set it to the Path of the flag"));
    //let file = fs::read_to_string(path).expect("Error While reading the file");
    //let structure = parse_text(file); 
    
    unsafe {
        initscr();
        start_color();
        /*
        let structure = parse_text(String::from("COLOR_PINK={255,0,0}
                                 COLOR_BLUE={0,0,255}
                                 [[COLOR_PINK, COLOR_PINK], [COLOR_BLUE,COLOR_BLUE]]
                                "));
        */
        make_color!(COLOR_PAIR_RED, COLOR_PAIR_BLUE, COLOR_PAIR_PINK, COLOR_PAIR_PURPLE);
        //let COLOR_PAIR_PINK = 3;
        //let COLOR_PAIR_PURPLE = 6;
        //let COLOR_PAIR_BLUE = 8;
        //make_color_from_pallet(COLOR_PAIR_BLUE, (0,0,255));
        //make_color_from_pallet(COLOR_PAIR_PURPLE, (50,0,50));
        //make_color_from_pallet(COLOR_PAIR_PINK, (90,0,50));
        
        let mut flag = make_flag(String::from("Hello World"), vec! [vec![COLOR_PAIR_PINK as i32,COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32],
                                                                vec![COLOR_PAIR_PINK as i32,COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32],
                                                                vec![COLOR_PAIR_PURPLE as i32,COLOR_PAIR_PURPLE as i32, COLOR_PAIR_PURPLE as i32, COLOR_PAIR_PURPLE as i32],
                                                                vec![COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32],
                                                                vec![COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32]]);
        
        
        //let mut flag = make_flag(String::from("Pride"), structure);
        flag.content = correct(&flag);
        //shuffle(&mut flag);
        let time_to_sleep = time::Duration::from_millis(200);
        let mut i = 0;
        loop {
            let new_flag = wave_anim(&flag, 7, i);
            new_flag.draw(); // REFERESH has to be moved
            i += 1;
            clear(); 
            thread::sleep(time_to_sleep);
        }
        //getch();
        //endwin();
    }
}
