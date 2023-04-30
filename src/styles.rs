use colored::Color;
use crate::draw::FrameStyle;

pub const STYLES: [FrameStyle; 2] = [
    FrameStyle {
        left_up: '╔',
        right_up: '╗',
        left_down: '╚',
        right_down: '╝',
        left: '║',
        right: '║',
        up: '═',
        down: '═',
        frame_color: Color::Cyan,
        nickname_color: Color::Red,
    },
    FrameStyle {
        left_up: '╔',
        right_up: '╗',
        left_down: '╚',
        right_down: '╝',
        left: '║',
        right: '║',
        up: '═',
        down: '═',
        frame_color: Color::Blue,
        nickname_color: Color::Green,
    },
];
