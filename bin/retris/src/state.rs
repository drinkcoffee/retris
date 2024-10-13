use std::panic;

pub const MAX_X: u16 = 20;
pub const MAX_Y: u16 = 40;

#[derive(Clone, Copy)]
enum FourObjType {
    None,
    Straight,
    Square,
    LeftTrap,
    RightTrap,
    LShape,
    ReverseLShape,
}

static mut STATE: [[u8; MAX_Y as usize]; MAX_X as usize] = [[0u8; MAX_Y as usize]; MAX_X as usize];

static mut FALLING_TYPE: FourObjType = FourObjType::None;
static mut FALLING_LOCATION_Y: i16 = -2;
static mut FALLING_LOCATION_X: u16 = MAX_X / 2;
static mut FALLING_ORIENTATION: u16 = 0;

pub fn start_new_four() {
    unsafe {
        FALLING_TYPE = FourObjType::Straight;
        FALLING_LOCATION_Y = -2;
        FALLING_LOCATION_X = MAX_X / 2;
    }
}

pub fn fall_one() {
    unsafe {
        FALLING_LOCATION_Y += 1;
    }
}

pub fn move_left() {
    unsafe {
        if FALLING_LOCATION_X > 0 {
            FALLING_LOCATION_X -= 1;
        }
    }
}

pub fn move_right() {
    unsafe {
        if FALLING_LOCATION_X < MAX_X - 1 {
            FALLING_LOCATION_X += 1;
        }
    }
}

pub fn rotate_clockwise() {
    unsafe {
        FALLING_ORIENTATION += 1;
        if FALLING_ORIENTATION == 4 {
            FALLING_ORIENTATION = 0;
        }
    }
}

pub fn rotate_anticlockwise() {
    unsafe {
        if FALLING_ORIENTATION == 0 {
            FALLING_ORIENTATION = 4;
        }
        FALLING_ORIENTATION -= 1;
    }
}



pub fn get_locked(x: u16, y: u16) -> u8 {
    if x >= MAX_X {
        panic!("X too big: {}", x)
    }
    if y >= MAX_Y {
        panic!("Y too big: {}", x)
    }
    //println!("X {} Y {}", x,y);

    let colour = unsafe { STATE[x as usize][y as usize] };
    // if colour == 0 {

    //     // unsafe {
    //     //     if FALLING_LOCATION_X == x && FALLING_LOCATION_Y == y as i16 {
    //     //         colour = 1;
    //     //     }
    //     // }
    // }
    return colour;
}

pub fn get_falling_coords() -> (u16, u16, u16, u16, u16, u16, u16, u16) {
    let mut x1: u16 = 0;
    let mut x2: u16 = 0;
    let mut x3: u16 = 0;
    let mut x4: u16 = 0;
    let mut y1: u16 = 0;
    let mut y2: u16 = 0;
    let mut y3: u16 = 0;
    let mut y4: u16 = 0;

    let fall_type;
    let fall_x;
    let fall_y;
    let fall_orientation;

    unsafe {
        fall_type = FALLING_TYPE;
        fall_x = FALLING_LOCATION_X;
        fall_y = FALLING_LOCATION_Y;
        fall_orientation = FALLING_ORIENTATION;
    }

    match fall_type {
        FourObjType::None =>{ panic!("Fall type not set"); },
        FourObjType::Straight=> {
            match fall_orientation {
                0 | 2 => {
                    x1 = fall_x;
                    x2 = fall_x + 1;
                    x3 = fall_x + 2;
                    x4 = fall_x + 3;
                    y1 = fall_y as u16;
                    y2 = fall_y as u16;
                    y3 = fall_y as u16;
                    y4 = fall_y as u16;
                },
                1 | 3 => {
                    x1 = fall_x;
                    x2 = fall_x;
                    x3 = fall_x;
                    x4 = fall_x;
                    y1 = fall_y as u16;
                    y2 = fall_y as u16+ 1;
                    y3 = fall_y as u16 + 2;
                    y4 = fall_y as u16 + 3;
                },
                _ => { panic!("Unknown orientation");}
            }
        },
        _=>{panic!("Not supported yet"); }
    }

    return (x1, y1, x2, y2, x3, y3, x4, y4);

}

pub fn check_valid(x: u16, y: u16) -> bool {
    return x < MAX_X && y < MAX_Y;
}