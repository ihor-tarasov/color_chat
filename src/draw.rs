use std::borrow::Cow;

use colored::{Color, Colorize};
use textwrap::wrap;

use crate::styles::STYLES;

pub struct FrameStyle {
    pub left_up: char,
    pub right_up: char,
    pub left_down: char,
    pub right_down: char,
    pub left: char,
    pub right: char,
    pub up: char,
    pub down: char,
    pub frame_color: Color,
    pub nickname_color: Color,
}

const TEXT_WIDTH: usize = 76;

fn get_text_width(nickname: &str, text: &str) -> usize {
    text.chars().count().max(nickname.chars().count() + 2).min(TEXT_WIDTH)
}

fn draw_top(buffer: &mut String, style: &FrameStyle, nickname: &str, width: usize) {
    buffer.clear();
    buffer.push(style.left_up);
    buffer.push(style.up);
    print!("{} ", buffer.color(style.frame_color));
    print!("{} ", nickname.color(style.nickname_color));
    buffer.clear();
    let l = nickname.chars().count();
    let count = width - l - 2;
    for _ in 0..count {
        buffer.push(style.up);
    }
    buffer.push(style.up);
    buffer.push(style.right_up);
    println!("{}", buffer.color(style.frame_color));
}

fn draw_line(buffer: &mut String, style: &FrameStyle, line: &str, width: usize) {
    buffer.clear();
    buffer.push(style.left);
    print!("{} {}", buffer.color(style.frame_color), line);
    buffer.clear();
    let count = width - line.chars().count();
    for _ in 0..count {
        buffer.push(' ');
    }
    buffer.push(' ');
    buffer.push(style.right);
    println!("{}", buffer.color(style.frame_color));
}

fn draw_down(buffer: &mut String, style: &FrameStyle, width: usize) {
    buffer.clear();
    buffer.push(style.left_down);
    for _ in 0..(width + 2) {
        buffer.push(style.down);
    }
    buffer.push(style.right_down);
    println!("{}", buffer.color(style.frame_color));
}

pub fn frame(style: usize, nickname: &str, text: &str) {
    let style = &STYLES[style];
    let mut buffer = String::new();

    let width = get_text_width(nickname, text);
    draw_top(&mut buffer, style, nickname, width);

    for line in wrap(text, TEXT_WIDTH) {
        match line {
            Cow::Borrowed(s) => draw_line(&mut buffer, style, s, width),
            Cow::Owned(s) => draw_line(&mut buffer, style, s.as_str(), width),
        }
    }

    draw_down(&mut buffer, style, width);
}
