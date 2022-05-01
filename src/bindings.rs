use libc::{c_int, c_short};

#[link(name = "curses")]
extern {
    pub fn initscr() -> *mut u8;
    pub fn addch(_:u32) -> c_int;
    pub fn refresh() -> c_int;
    pub fn getch() -> c_int;
    pub fn endwin() -> c_int;
    pub fn init_color(_:c_short,_:c_short,_:c_short,_:c_short) -> c_int; 
    pub fn init_pair(_:c_short, _:c_short, _:c_short);
    pub fn start_color() -> c_int;
    pub fn attron(_:u32) -> c_int;
    pub fn mvaddch(_:c_int, _:c_int, _:u32) -> c_int;
    pub fn COLOR_PAIR(_:c_int) -> c_int;
    pub fn attroff(_:u32) -> c_int;
}
