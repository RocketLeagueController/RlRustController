use core::fmt::Write;

use heapless::String;

pub struct ControllerState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub start: bool,
    pub back: bool,
    pub left_thumb: bool,
    pub right_thumb: bool,
    pub left_shoulder: bool,
    pub right_shoulder: bool,
    pub guide: bool,
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub left_thumb_x: f32,
    pub left_thumb_y: f32,
    pub right_thumb_x: f32,
    pub right_thumb_y: f32,
    pub left_trigger: f32,
    pub right_trigger: f32,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            up: false,
            down: false,
            left: false,
            right: false,
            start: false,
            back: false,
            left_thumb: false,
            right_thumb: false,
            left_shoulder: false,
            right_shoulder: false,
            guide: false,
            a: false,
            b: false,
            x: false,
            y: false,
            left_thumb_x: 0.0f32,
            left_thumb_y: 0.0f32,
            right_thumb_x: 0.0f32,
            right_thumb_y: 0.0f32,
            left_trigger: 0.0f32,
            right_trigger: 0.0f32,
        }
    }

    pub fn to_string(&self) -> String<300> {
        let mut s: String<300> = String::new();
        Self::add_bool(&mut s, self.up);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.down);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.left);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.right);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.start);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.back);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.left_thumb);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.right_thumb);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.left_shoulder);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.right_shoulder);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.guide);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.a);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.b);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.x);
        Self::add_coma(&mut s);
        Self::add_bool(&mut s, self.y);
        Self::add_coma(&mut s);

        Self::add_number(&mut s, self.left_thumb_x);
        Self::add_coma(&mut s);
        Self::add_number(&mut s, self.left_thumb_y);
        Self::add_coma(&mut s);
        Self::add_number(&mut s, self.right_thumb_x);
        Self::add_coma(&mut s);
        Self::add_number(&mut s, self.right_thumb_y);
        Self::add_coma(&mut s);
        Self::add_number(&mut s, self.left_trigger);
        Self::add_coma(&mut s);
        Self::add_number(&mut s, self.right_trigger);
        _ = s.push_str("\n");

        return s;
    }

    fn add_bool(str: &mut String<300>, value: bool) {
        if value {
            _ = str.push_str("1");
        } else {
            _ = str.push_str("0");
        }
    }

    fn add_coma(str: &mut String<300>) {
        _ = str.push_str(",");
    }

    fn add_number(str: &mut String<300>, value: f32) {
        _ = write!(str, "{value}");
    }
}
