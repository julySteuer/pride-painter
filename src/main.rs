mod bindings;
mod flags;
#[macro_use]
mod color_pallet;
use bindings::{COLOR_PAIR, initscr, refresh, getch, endwin, init_color, init_pair, mvaddch, start_color, attron, attroff};
use flags::{Pixel, make_flag};
//mod parser;
//use parser::parse_text;

use crate::flags::{correct, shuffle};

fn main() { // Shows flags random pixel with one array with pxel values ranom from this array pop is from array
    unsafe {
        initscr();
        start_color();
        make_color!(COLOR_PAIR_RED, COLOR_PAIR_BLUE, COLOR_PAIR_PINK, COLOR_PAIR_PURPLE);
        let mut flag = make_flag(String::from("Hello World"), vec! [vec![COLOR_PAIR_PINK as i32,COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32],
                                                                vec![COLOR_PAIR_PINK as i32,COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32, COLOR_PAIR_PINK as i32],
                                                                vec![COLOR_PAIR_PURPLE as i32,COLOR_PAIR_PURPLE as i32, COLOR_PAIR_PURPLE as i32, COLOR_PAIR_PURPLE as i32],
                                                                vec![COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32],
                                                                vec![COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32, COLOR_PAIR_BLUE as i32]]);
        correct(&mut flag);
        shuffle(&mut flag);
        flag.draw(); // REFERESH has to be moved
        getch();
        endwin();
    }

   // parse_text(String::from("COLOR_PINK={10,20,30}
   //                          COLOR_BLUE={20,10,30}"))
}
