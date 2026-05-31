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
    Install,
    Select(String),
    Uninstall,
    Upgrade,
    Search(String),
    UpgradeDone,
}

struct App {
    packages: Vec<String>,
    selected: Option<String>,
    pkglistwhole: Vec<String>,
    search: String,
    status: String,
    statusapp: String,
}

impl Default for App {
    fn default() -> Self {
        
        Self {
            packages: fs::read_dir("/var/lib/pkg/DB")
                .unwrap()
                .filter_map(|e| Some(e.ok()?.file_name().to_string_lossy().to_string()))
                .collect(),
            selected: None,
            pkglistwhole,
            search: String::new(),
            status: String::new(),
            statusapp: String::new(),
        }
    }
}

fn pkg_button(label: &str, active: bool) -> Button<'_, Message> {
    button(
        text(label).size(13)
    )
    .on_press(Message::Select(label.to_string()))
    .width(Length::Fill)
    .style(move |_theme, status| {
        let bg = if active {
            Color::from_rgb(0.11, 0.62, 0.46)
        } else if matches!(status, button::Status::Hovered) {
            Color::from_rgb(0.15, 0.15, 0.17)
        } else {
            Color::from_rgb(0.10, 0.10, 0.12)
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: if active {
                Color::WHITE
            } else {
                Color::from_rgb(0.75, 0.75, 0.78)
            },
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        
    }





    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Search(query) => { self.search = query; Task::none() }
            Message::Select(pkg) => { self.selected = Some(pkg); Task::none() }
            Message::Install => {
                if let Some(pkg) = &self.selected {
                    Command::new("pkexec").args(["cards", "install", pkg]).status().ok();
                }
                self.statusapp = String::from("Installed");
                Task::none()
            }
            Message::Uninstall => {
                if let Some(pkg) = &self.selected {
                    Command::new("pkexec").args(["cards", "remove", pkg]).status().ok();
                }
                self.statusapp = String::from("Not installed");
                Task::none()
            }
            Message::Upgrade => {
                Task::perform(
                    async {
                        tokio::process::Command::new("pkexec")
                            .args(["cards", "upgrade", "--proceed"])
                            .status()
                            .await
                            .ok();
                    },
                    |_| Message::UpgradeDone,
                )
            }
            Message::UpgradeDone => {
                self.status = String::from("Upgrade terminé");
                Task::none()
            }
        }
    }

}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
