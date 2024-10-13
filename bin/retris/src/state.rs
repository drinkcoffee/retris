use std::panic;

pub const MAX_X: u16 = 20;
pub const MAX_Y: u16 = 30;

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

static mut STATE: [[bool; MAX_Y as usize]; MAX_X as usize] =
    [[false; MAX_Y as usize]; MAX_X as usize];

static mut FALLING_TYPE: FourObjType = FourObjType::None;
static mut FALLING_LOCATION_Y: i16 = 0;
static mut FALLING_LOCATION_X: u16 = MAX_X / 2;
static mut FALLING_ORIENTATION: u16 = 0;

pub fn start_new_four() {
    unsafe {
        FALLING_TYPE = FourObjType::Straight;
        FALLING_ORIENTATION = 0;
        FALLING_LOCATION_Y = 0;
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
    if is_invalid_change() {
        // If the change was invalid, reverse it.
        move_right();
    }
}

pub fn move_right() {
    unsafe {
        if FALLING_LOCATION_X < MAX_X - 1 {
            FALLING_LOCATION_X += 1;
        }
    }
    if is_invalid_change() {
        // If the change was invalid, reverse it.
        move_left();
    }
}

pub fn rotate_clockwise() {
    unsafe {
        FALLING_ORIENTATION += 1;
        if FALLING_ORIENTATION == 4 {
            FALLING_ORIENTATION = 0;
        }
    }
    if is_invalid_change() {
        // If the change was invalid, reverse it.
        rotate_anticlockwise();
    }
}

pub fn rotate_anticlockwise() {
    unsafe {
        if FALLING_ORIENTATION == 0 {
            FALLING_ORIENTATION = 4;
        }
        FALLING_ORIENTATION -= 1;
    }
    if is_invalid_change() {
        // If the change was invalid, reverse it.
        rotate_clockwise();
    }
}

// Convert the falling object to part of the background if it has hit the bottom.
pub fn update_if_dropped() {
    let coords = get_falling_coords();
    let x1: u16 = coords.0;
    let y1: u16 = coords.1;
    let x2: u16 = coords.2;
    let y2: u16 = coords.3;
    let x3: u16 = coords.4;
    let y3: u16 = coords.5;
    let x4: u16 = coords.6;
    let y4: u16 = coords.7;

    let mut lock = false;

    println!("x1 {} y1 {}", x1, y1);

    if y1 + 1 == MAX_Y || y2 + 1 == MAX_Y || y3 + 1 == MAX_Y || y4 + 1 == MAX_Y {
        lock = true;
    }
    else if get_locked(x1, y1 + 1)
        || get_locked(x2, y2 + 1)
        || get_locked(x3, y3 + 1)
        || get_locked(x4, y4 + 1)
    {
        lock = true;
    }

    if lock {
        unsafe {
            STATE[x1 as usize][y1 as usize] = true;
            STATE[x2 as usize][y2 as usize] = true;
            STATE[x3 as usize][y3 as usize] = true;
            STATE[x4 as usize][y4 as usize] = true;
        }
        start_new_four();
    }
}

pub fn get_locked(x: u16, y: u16) -> bool {
    if x >= MAX_X {
        panic!("X too big: {}", x)
    }
    if y >= MAX_Y {
        panic!("Y too big: {}", y)
    }

    return unsafe { STATE[x as usize][y as usize] };
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
        FourObjType::None => {
            panic!("Fall type not set");
        }
        FourObjType::Straight => match fall_orientation {
            0 | 2 => {
                x1 = fall_x;
                x2 = fall_x + 1;
                x3 = fall_x + 2;
                x4 = fall_x + 3;
                y1 = fall_y as u16;
                y2 = fall_y as u16;
                y3 = fall_y as u16;
                y4 = fall_y as u16;
            }
            1 | 3 => {
                x1 = fall_x;
                x2 = fall_x;
                x3 = fall_x;
                x4 = fall_x;
                y1 = fall_y as u16;
                y2 = fall_y as u16 + 1;
                y3 = fall_y as u16 + 2;
                y4 = fall_y as u16 + 3;
            }
            _ => {
                panic!("Unknown orientation");
            }
        },
        _ => {
            panic!("Not supported yet");
        }
    }

    return (x1, y1, x2, y2, x3, y3, x4, y4);
}

pub fn check_valid(x: u16, y: u16) -> bool {
    return x < MAX_X && y < MAX_Y;
}

fn is_invalid_change() -> bool {
    let coords = get_falling_coords();
    let x1: u16 = coords.0;
    let y1: u16 = coords.1;
    let x2: u16 = coords.2;
    let y2: u16 = coords.3;
    let x3: u16 = coords.4;
    let y3: u16 = coords.5;
    let x4: u16 = coords.6;
    let y4: u16 = coords.7;

    if !check_valid(x1, y1) || !check_valid(x2, y2) || !check_valid(x3, y3) || !check_valid(x4, y4) {
        return true;
    }

    if get_locked(x1, y1)
        || get_locked(x2, y2)
        || get_locked(x3, y3)
        || get_locked(x4, y4) {
        return true;
    }

    return false;
}