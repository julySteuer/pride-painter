use crate::bindings::{attron, attroff, COLOR_PAIR, mvaddch, refresh};
use rand::{thread_rng, Rng, prelude::SliceRandom};
use std::{thread, time};

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub x: i16,
    pub y: i16,
    color: i32
}

#[derive(Debug)]
pub struct Flag {
    pub name: String, // Stringify macro stringify!()
    pub content: Vec<Vec<Pixel>>
}

pub fn correct(flag: &mut Flag){
    let mut new_pixels: Vec<Vec<Pixel>> = Vec::new();
    let height = flag.content.len();
    let width = flag.content[0].len();
    let ar = width as f32/(height*7) as f32;
    let size_val = 1.0/ar;
    for x in 0..width {
        for y in 0..height {
            let mut temp_pixel: Vec<Pixel> = Vec::new();
            for i in 0..(size_val as usize) {
                let mut new_pixel = flag.content[y][x].clone(); //Error Here
                new_pixel.x = x as i16+ i as i16; // LOOK HERE ERROR WITH SCALAR
                new_pixel.y = y as i16;
                temp_pixel.push(new_pixel);
            } // Set to new Pixel Location
            new_pixels.push(temp_pixel);
        }
    }
    
    flag.content = new_pixels;
}

pub fn make_flag(name: String, structure: Vec<Vec<i32>>) -> Flag { // 4:3 ratio
       // Make flag here (Generate Pixels from $structure (Structure are RGB Values in i16, 2d array) -> put Into Flag) 
       let mut pixels: Vec<Vec<Pixel>> = Vec::new();
       for y in 0..structure.len() {
            let mut temp_pixel: Vec<Pixel> = Vec::new();
            for x in 0..structure[0].len() {
                let pixel = Pixel{x:x as i16,y:y as i16,color: structure[y][x]};// Here
                temp_pixel.push(pixel);
            }
            pixels.push(temp_pixel);
       }
       Flag{name, content:pixels}
}

pub fn shuffle(flag: &mut Flag){
    let mut new_pixels: Vec<Vec<Pixel>> = Vec::new();
    let pixels = flag.content.clone();
    for mut elem in pixels.into_iter() {
        elem.shuffle(&mut thread_rng());
        new_pixels.push(elem);
    }
    flag.content = new_pixels;
}

fn zoom(pixels:Vec<Vec<Pixel>>, scalar: i16) -> Vec<Vec<Pixel>> {
    todo!();
}

impl Pixel {
    pub fn new(x: i16, y: i16, color: i32) -> Pixel{
        Pixel{x,y,color}
    }

    pub fn draw(&self){
        unsafe { 
            attron(COLOR_PAIR(self.color) as u32);
            mvaddch(self.y as i32,self.x as i32,219u8 as u32);
            attroff(COLOR_PAIR(self.color) as u32);
        }
    }
}

impl Flag {
    pub fn draw(&self){
        for x in 0..self.content[0].len(){ // Fixed it a bit
         for y in 0..self.content.len() {
            self.content[y][x].draw();       
            let one_sec = time::Duration::from_millis(100);
            thread::sleep(one_sec);
            unsafe {
                refresh();
            }
        }
       } 
    }
}