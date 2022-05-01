use crate::bindings::{init_color, init_pair};

#[macro_export]
macro_rules! make_color {
    ($red_pair:ident, $blue_pair:ident, $pink_pair:ident, $purple_pair:ident) => {
       // Red
       let COLOR_RED_FORE: i16 = 11;
       let COLOR_RED_BACK: i16 = 12;
       let $red_pair: i16 = 1;
       // Blue
       let COLOR_BLUE_FORE: i16 = 13;
       let COLOR_BLUE_BACK: i16 = 14;
       let $blue_pair: i16 = 2;
       //Bi Flag
       //Pink
       let COLOR_PINK_FORE: i16 = 15;
       let COLOR_PINK_BACK: i16 = 16;
       let $pink_pair: i16 = 3;
       // Purple
       let COLOR_PURPLE_FORE: i16 = 17;
       let COLOR_PURPLE_BACK: i16 = 18;
       let $purple_pair:i16 = 4; 

       unsafe { // Could use Macro here as well
           init_color(COLOR_RED_FORE, 255,0,0);
           init_color(COLOR_RED_BACK, 255,0,0);
           init_pair($red_pair, COLOR_RED_FORE, COLOR_RED_BACK);
           
           init_color(COLOR_BLUE_FORE, 0,0,255);
           init_color(COLOR_BLUE_BACK, 0,0,255);
           init_pair($blue_pair, COLOR_BLUE_FORE, COLOR_BLUE_BACK);
       
           init_color(COLOR_PINK_FORE, 216,9,126);
           init_color(COLOR_PINK_BACK, 216,9,126);
           init_pair($pink_pair, COLOR_PINK_FORE, COLOR_PINK_BACK);

           init_color(COLOR_PURPLE_FORE, 140,87,156);
           init_color(COLOR_PURPLE_BACK, 140,87,156);
           init_pair($purple_pair, COLOR_PURPLE_FORE, COLOR_PURPLE_BACK);
        } 
    }
}

pub fn make_color_from_pallet(index: i32, rgb: (i32,i32,i32)){ // ERROR WITH colors maybe here
    unsafe { // INIT COLOR NEED 3 VALUES FOREGROUND AND BACKGROUND
        init_color((index+1) as i16, rgb.0 as i16, rgb.1 as i16, rgb.2 as i16);
        init_color((index+2) as i16, rgb.0 as i16, rgb.1 as i16,rgb.2 as i16);
        init_pair(index as i16,(index+1) as i16,(index+2) as i16);
    } // COLOR PALETT INDEXING
}
