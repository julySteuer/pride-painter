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
// ERROR IN THIS METHOD
pub fn correct(flag: &Flag) -> Vec<Vec<Pixel>>{
    let mut new_pixels: Vec<Vec<Pixel>> = Vec::new();
    let height = flag.content.len();
    let width = flag.content[0].len();
    let ar = width as f32/(height*5) as f32;
    let size_val = 1.0/ar;
    for y in 0..height {
        let mut inner: Vec<Pixel> = Vec::new();
        for x in 0..width {
        let mut temp_pixel: Vec<Pixel> = Vec::new();
            for i in 0..(size_val as usize) { // try incrementer after size_val
                let mut new_pixel = flag.content[y][x].clone(); //Error Here
                new_pixel.x = (x as i16 * size_val as i16) + i as i16; // LOOK HERE ERROR WITH SCALAR
                new_pixel.y = y as i16;
                temp_pixel.push(new_pixel);
            } // Set to new Pixel Location
            inner.append(&mut temp_pixel);
        }
        new_pixels.push(inner);
    }
    new_pixels
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

pub fn wave_anim(flag: &Flag, period: i16, shift: i16) -> Flag{
    let mut new_flag = Flag{name: flag.name.clone(), content: flag.content.clone()};
    for x in 0..flag.content[0].len() {
        for y in 0..flag.content.len() {
            let sinus_offset = (((shift+x as i16)*5) as f64).to_radians().sin(); // Mess with this function
            new_flag.content[y][x].y = flag.content[y][x].y - sinus_offset as i16;
        }
    }
    new_flag
}

pub fn shuffle(flag: &mut Flag){
    let mut new_pixels: Vec<Vec<Pixel>> = Vec::new();
    let pixels = flag.content.clone();
    for mut elem in pixels.into_iter() {
        elem.shuffle(&mut thread_rng());
        new_pixels.push(elem);
    }
    new_pixels.shuffle(&mut thread_rng());
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
    pub fn new(name: String, content: Vec<Vec<Pixel>>) -> Flag{
        Flag {
            name, content
        }
    }
    pub fn init(&self){
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

    pub fn draw(&self){
        for x in 0..self.content[0].len() {
            for y in 0..self.content.len() {
                self.content[y][x].draw();
            }
        }
        unsafe {
            refresh();
        }
    }
}
