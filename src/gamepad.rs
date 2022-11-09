#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GGSTButton {
    S,
    H,
    P,
    K,
    D
}

#[derive(Copy, Clone, Debug)]
pub struct StickInfo {
    pub x: u8,
    pub y: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Stick {
    pub left_stick: StickInfo,
    pub right_stick: StickInfo,
}

#[derive(Copy, Clone, Debug)]
pub struct Button {
    pub pressed: bool,
    pub id: i32
}

impl Button {
    pub fn get_ggst_button(&self) -> Option<GGSTButton> {
        match self.id {
            11 => Some(GGSTButton::H),
            13 => Some(GGSTButton::K),
            14 => Some(GGSTButton::D),
            15 => Some(GGSTButton::P),
            16 => Some(GGSTButton::S),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GamePad {
    pub up_left_button: Button,
    pub up_right_button: Button,
    pub down_left_button: Button,
    pub down_right_button: Button,
    pub left_button: Button,
    pub right_button: Button,
    pub up_button: Button,
    pub down_button: Button,
    pub l1_button: Button,
    pub l2_button: Button,
    pub r1_button: Button,
    pub r2_button: Button,
    pub x_button: Button,
    pub o_button: Button,
    pub square_button: Button,
    pub triangle_button: Button,
    pub share_button: Button,
    pub option_button: Button,
    pub ps_button: Button,
    pub left_stick_button: Button,
    pub right_stick_button: Button,
    pub stick: Stick,
}

impl GamePad {
    pub fn new() -> Self {
        GamePad {
            up_left_button: Button { pressed: false, id: 1 },
            up_right_button: Button { pressed: false, id: 2 },
            down_left_button: Button { pressed: false, id: 3 },
            down_right_button: Button { pressed: false, id: 4 },
            left_button: Button { pressed: false, id: 5 },
            right_button: Button { pressed: false, id: 6 },
            up_button: Button { pressed: false, id: 7 },
            down_button: Button { pressed: false, id: 8 },
            l1_button: Button { pressed: false, id: 9 },
            l2_button: Button { pressed: false, id: 10 },
            r1_button: Button { pressed: false, id: 11 },
            r2_button: Button { pressed: false, id: 12 },
            x_button: Button { pressed: false, id: 13 },
            o_button: Button { pressed: false, id: 14 },
            square_button: Button { pressed: false, id: 15 },
            triangle_button: Button { pressed: false, id: 16 },
            share_button: Button { pressed: false, id: 17 },
            option_button: Button { pressed: false, id: 18 },
            ps_button: Button { pressed: false, id: 19 },
            left_stick_button: Button { pressed: false, id: 20 },
            right_stick_button: Button { pressed: false, id: 21 },
            stick: Stick {
                left_stick: StickInfo { x: 0, y: 0 },
                right_stick: StickInfo { x: 0, y: 0 },
            },
        }
    }

    pub fn any_attack_pressed(&self) -> bool {
        return self.x_button.pressed || self.o_button.pressed || self.triangle_button.pressed ||
            self.square_button.pressed || self.r1_button.pressed;
    }

    pub fn get_buttons() -> [i32; 21] {
        return [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21];
    }

    pub fn get_button(&self, id: i32) -> Button {
        match id {
            1 => self.up_left_button,
            2 => self.up_right_button,
            3 => self.down_left_button,
            4 => self.down_right_button,
            5 => self.left_button,
            6 => self.right_button,
            7 => self.up_button,
            8 => self.down_button,
            9 => self.l1_button,
            10 => self.l2_button,
            11 => self.r1_button,
            12 => self.r2_button,
            13 => self.x_button,
            14 => self.o_button,
            15 => self.square_button,
            16 => self.triangle_button,
            17 => self.share_button,
            18 => self.option_button,
            20 => self.left_stick_button,
            21 => self.right_stick_button,
            _ => self.ps_button
        }
    }

    pub fn get_state(&self, buf: &[u8]) -> Self {
        GamePad {
            up_left_button: Button { pressed: check_button_pressed((0x05, 0x07, 0xf), buf), id: 1 },
            up_right_button: Button { pressed: check_button_pressed((0x05, 0x01, 0xf), buf), id: 2 },
            down_left_button: Button { pressed: check_button_pressed((0x05, 0x05, 0xf), buf), id: 3 },
            down_right_button: Button { pressed: check_button_pressed((0x05, 0x03, 0xf), buf), id: 4 },
            left_button: Button { pressed: check_button_pressed((0x05, 0x06, 0xf), buf), id: 5 },
            right_button: Button { pressed: check_button_pressed((0x05, 0x02, 0xf), buf), id: 6 },
            up_button: Button { pressed: check_button_pressed((0x05, 0x00, 0xf), buf), id: 7 },
            down_button: Button { pressed: check_button_pressed((0x05, 0x04, 0xf), buf), id: 8 },
            l1_button: Button { pressed: check_button_pressed((0x06, 0x01, 0xff), buf), id: 9 },
            l2_button: Button { pressed: check_button_pressed((0x06, 0x04, 0xff), buf), id: 10 },
            r1_button: Button { pressed: check_button_pressed((0x06, 0x02, 0xff), buf), id: 11 },
            r2_button: Button { pressed: check_button_pressed((0x06, 0x08, 0xff), buf), id: 12 },
            x_button: Button { pressed: check_button_pressed((0x05, 0x20, 0xff), buf), id: 13 },
            o_button: Button { pressed: check_button_pressed((0x05, 0x40, 0xff), buf), id: 14 },
            square_button: Button { pressed: check_button_pressed((0x05, 0x10, 0xff), buf), id: 15 },
            triangle_button: Button { pressed: check_button_pressed((0x05, 0x80, 0xff), buf), id: 16 },
            share_button: Button { pressed: check_button_pressed((0x06, 0x10, 0xff), buf), id: 17 },
            option_button: Button { pressed: check_button_pressed((0x06, 0x20, 0xff), buf), id: 18 },
            ps_button: Button { pressed: check_button_pressed((0x07, 0x01, 0xff), buf), id: 19 },
            left_stick_button: Button { pressed: check_button_pressed((0x06, 0x40, 0xff), buf), id: 20 },
            right_stick_button: Button { pressed: check_button_pressed((0x06, 0x80, 0xff), buf), id: 21 },
            stick: check_stick_rock(buf),
        }
    }
}

pub fn check_button_pressed(conf: (usize, u8, u8), buf: &[u8]) -> bool {
    let rev = buf[conf.0] & conf.2;

    if conf.0 == 0x05 && conf.1 == 0x00 {
        return rev == 0x00;
    }

    let result = rev & conf.1;
    result == conf.1
}

pub fn check_stick_rock(buf: &[u8]) -> Stick {
    let left_stick = StickInfo {
        x: buf[0x01],
        y: buf[0x02],
    };

    let right_stick = StickInfo {
        x: buf[0x03],
        y: buf[0x04],
    };

    Stick {
        left_stick,
        right_stick,
    }
}