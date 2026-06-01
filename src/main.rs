// Distro-welcomer is a simple GUI interface to display a welcome display memory safe and written in Rust
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.



use iced::widget::{button, column, container, row, scrollable, text, Button};
use iced::{Background, Border, Color, Element, Length, Theme};
use std::fs;
use std::process::Command;
use std::path::Path;
use std::env;
use iced::widget::text_input;
use iced::Task;

#[derive(Debug, Clone)]
pub enum Message {
    Click,
}

//#[derive(Default)]
struct Widgets {
    website: String,
    documentation: String,
    community: String,
    image: String,
    systemname: String,
}

impl Default for Widgets {
    fn default() -> Self {
        Self {
            systemname: fs::read_to_string("/etc/os-release")
                .unwrap()
                .lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .unwrap()
                .to_string()
                .split_once("\"")
                .map(|(_, name)| name)
                .unwrap()
                .split_once("\"")
                .map(|(name, _)| name).unwrap().to_string(),
            website: String::new(),
            documentation: String::new(),
            community: String::new(),
            image: String::new(),
        }
    }
}


impl Widgets {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            text(format!("Welcome to : {}", self.systemname.to_string()))
        ]
        .into()
        
    }





    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Click => {
                println!("ok");
                Task::none()
            }
        }
    }

}

fn theme(_app: &Widgets) -> Theme {
    Theme::Dark
}

fn main() -> iced::Result {
    iced::run(Widgets::update, Widgets::view)
}

