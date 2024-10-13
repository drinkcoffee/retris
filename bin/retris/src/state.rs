use std::panic;


const MAX_X: usize = 20;
const MAX_Y: usize = 40;

static mut STATE: [[u8; MAX_X]; MAX_Y] = [[0u8; MAX_X]; MAX_Y];


pub fn get_colour(x: usize, y: usize) -> u8 {
    if x >= MAX_X {
        panic!("X too big: {}", x)
    }
    if y >= MAX_Y {
        panic!("Y too big: {}", x)
    }

    unsafe{STATE[x][y]}
}