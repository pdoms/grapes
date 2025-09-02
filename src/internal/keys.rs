use std::{
    ops::{Index, IndexMut},
    time::{Duration, Instant},
};

use crate::events::keyboard::K;

pub struct Keys {
    state: [bool; K::Count as usize],
    prev: Instant,
    delta: Duration,
    prev_state: [bool; K::Count as usize],
    keys_down_dur: [f32; K::Count as usize],
    repeat_delay: f32,
    repeat_rate: f32,
}

impl Index<K> for Keys {
    type Output = bool;

    fn index(&self, index: K) -> &Self::Output {
        &self.state[index as usize]
    }
}

impl IndexMut<K> for Keys {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.state[index as usize]
    }
}

impl Keys {
    pub fn new() -> Self {
        Self {
            state: [false; K::Count as usize],
            prev: Instant::now(),
            delta: Duration::from_secs(0),
            prev_state: [false; K::Count as usize],
            keys_down_dur: [-1.0; K::Count as usize],
            repeat_delay: 0.250,
            repeat_rate: 0.050,
        }
    }

    pub fn set(&mut self, k: K) {
        self.state[k as usize] = true;
    }
    pub fn clear(&mut self, k: K) {
        self.state[k as usize] = false;
    }

    pub fn update(&mut self) {
        self.delta = self.prev.elapsed();
        self.prev = Instant::now();
        let delta = self.delta.as_secs_f32();

        for idx in 0..self.state.len() {
            if self.state[idx] {
                if self.keys_down_dur[idx] < 0.0 {
                    self.keys_down_dur[idx] = 0.0;
                } else {
                    self.keys_down_dur[idx] += delta;
                }
            } else {
                self.keys_down_dur[idx] = -1.0;
            }
            self.prev_state[idx] = self.state[idx];
        }
    }

    pub fn is_down(&self, k: K) -> bool {
        self.state[k as usize]
    }

    fn is_k_pressed(&self, k: K, repeat: bool) -> bool {
        let t = self.keys_down_dur[k as usize];
        if t == 0.0 {
            return true;
        }

        if repeat && t > self.repeat_delay {
            let delta = self.delta.as_secs_f32();
            let delay = self.repeat_delay;
            let rate = self.repeat_rate;

            if (((t - delay) % rate) > rate * 0.5) != (((t - delay - delta) % rate) > rate * 0.5) {
                return true;
            }
        }
        false
    }

    pub fn is_released(&self, k: K) -> bool {
        self.prev_state[k as usize] && !self.state[k as usize]
    }

    pub fn is_pressed(&self, k: K) -> bool {
        self.is_k_pressed(k, true)
    }
}

impl From<usize> for K {
    fn from(value: usize) -> Self {
        K::from(value as u32)
    }
}

impl From<u64> for K {
    fn from(value: u64) -> Self {
        K::from(value as u32)
    }
}

impl From<u32> for K {
    fn from(value: u32) -> Self {
        use x11_dl::keysym::*;
        match value {
            XK_0 => K::K0,                                     //  K0
            XK_1 => K::K1,                                     //  K1
            XK_2 => K::K2,                                     //  K2
            XK_3 => K::K3,                                     //  K3
            XK_4 => K::K4,                                     //  K4
            XK_5 => K::K5,                                     //  K5
            XK_6 => K::K6,                                     //  K6
            XK_7 => K::K7,                                     //  K7
            XK_8 => K::K8,                                     //  K8
            XK_9 => K::K9,                                     //  K9
            XK_F1 => K::F1,                                    //  F1
            XK_F2 => K::F2,                                    //  F2
            XK_F3 => K::F3,                                    //  F3
            XK_F4 => K::F4,                                    //  F4
            XK_F5 => K::F5,                                    //  F5
            XK_F6 => K::F6,                                    //  F6
            XK_F7 => K::F7,                                    //  F7
            XK_F8 => K::F8,                                    //  F8
            XK_F9 => K::F9,                                    //  F9
            XK_F10 => K::F10,                                  //  F10
            XK_F11 => K::F11,                                  //  F11
            XK_F12 => K::F12,                                  //  F12
            x11_dl::keysym::XK_a => K::A,                      //  A
            x11_dl::keysym::XK_b => K::B,                      //  B
            x11_dl::keysym::XK_c => K::C,                      //  C
            x11_dl::keysym::XK_d => K::D,                      //  D
            x11_dl::keysym::XK_e => K::E,                      //  E
            x11_dl::keysym::XK_f => K::F,                      //  F
            x11_dl::keysym::XK_g => K::G,                      //  G
            x11_dl::keysym::XK_h => K::H,                      //  H
            x11_dl::keysym::XK_i => K::I,                      //  I
            x11_dl::keysym::XK_j => K::J,                      //  J
            x11_dl::keysym::XK_k => K::K,                      //  K
            x11_dl::keysym::XK_l => K::L,                      //  L
            x11_dl::keysym::XK_m => K::M,                      //  M
            x11_dl::keysym::XK_n => K::N,                      //  N
            x11_dl::keysym::XK_o => K::O,                      //  O
            x11_dl::keysym::XK_p => K::P,                      //  P
            x11_dl::keysym::XK_q => K::Q,                      //  Q
            x11_dl::keysym::XK_r => K::R,                      //  R
            x11_dl::keysym::XK_s => K::S,                      //  S
            x11_dl::keysym::XK_t => K::T,                      //  T
            x11_dl::keysym::XK_u => K::U,                      //  U
            x11_dl::keysym::XK_v => K::V,                      //  V
            x11_dl::keysym::XK_w => K::W,                      //  W
            x11_dl::keysym::XK_x => K::X,                      //  X
            x11_dl::keysym::XK_y => K::Y,                      //  Y
            x11_dl::keysym::XK_z => K::Z,                      //  Z
            x11_dl::keysym::XK_apostrophe => K::Quote,         //  Quote
            x11_dl::keysym::XK_comma => K::Comma,              //  Comma
            x11_dl::keysym::XK_minus => K::Dash,               //  Dash
            x11_dl::keysym::XK_period => K::Period,            //  Period
            x11_dl::keysym::XK_slash => K::ForwardSlash,       //  ForwardSlash
            x11_dl::keysym::XK_semicolon => K::SemiColon,      //  SemiColon
            x11_dl::keysym::XK_less => K::LessThan,            //  LessThan
            x11_dl::keysym::XK_equal => K::Equal,              //  Equal
            x11_dl::keysym::XK_bracketleft => K::SquareOpen,   //  SquareOpen
            x11_dl::keysym::XK_backslash => K::BackSlash,      //  BackSlash
            x11_dl::keysym::XK_bracketright => K::SquareClose, //  SquareClose  ,
            x11_dl::keysym::XK_grave => K::BackTick,           //  BackTick
            x11_dl::keysym::XK_Left => K::ArrowLeft,           //  ArrowLeft
            x11_dl::keysym::XK_Up => K::ArrowUp,               //  ArrowUp
            x11_dl::keysym::XK_Down => K::ArrowDown,           //  ArrowDown
            x11_dl::keysym::XK_Right => K::ArrowRight,         //  ArrowRight
            x11_dl::keysym::XK_space => K::Space,              //  Space
            x11_dl::keysym::XK_BackSpace => K::BackSpace,      //  BackSpace
            x11_dl::keysym::XK_Tab => K::Tab,                  //  Tab
            x11_dl::keysym::XK_Return => K::Enter,             //  Enter
            x11_dl::keysym::XK_Delete => K::Delete,            //  Delete
            x11_dl::keysym::XK_Caps_Lock => K::CapsLock,       //  CapsLock
            x11_dl::keysym::XK_Shift_L => K::LeftShift,        //  LeftShift
            x11_dl::keysym::XK_Shift_R => K::RightShift,       //  RightShift
            x11_dl::keysym::XK_Control_L => K::LeftCtrl,       //  LeftCtrl
            x11_dl::keysym::XK_Control_R => K::RightCtrl,      //  RightCtrl
            x11_dl::keysym::XK_Super_L => K::Mod,              //  Mod
            x11_dl::keysym::XK_Alt_L => K::Alt,                //  Alt
            x11_dl::keysym::XK_Alt_R => K::RightAlt,           //  RightAlt
            x11_dl::keysym::XK_Home => K::Home,                //  Home
            x11_dl::keysym::XK_End => K::End,                  //  End
            x11_dl::keysym::XK_Print => K::Print,
            x11_dl::keysym::XK_Escape => K::Escape,
            _ => K::Unknown, //  Print
        }
    }
}
