#![allow(warnings)]

// mod message;

use iced::{
    widget::{self, button, column, container, text},
    Sandbox, Settings, Theme,
};

fn main() -> iced::Result {
    TextPrinter::run(Settings::default())
}

struct TextPrinter {
    text: String,
    theme: Theme,
    base_title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrinterMessage {
    Print,
    ChangeTheme,
}

impl Sandbox for TextPrinter {
    type Message = PrinterMessage;

    fn new() -> Self {
        println!("new()");

        Self {
            text: String::new(),
            theme: Theme::Dracula,
            base_title: String::from("Printer"),
        }
    }

    fn update(&mut self, message: Self::Message) {
        println!("update({:?})", message);

        use PrinterMessage::*;
        if message == ChangeTheme {
            if self.theme == Theme::Dracula {
                self.theme = Theme::CatppuccinFrappe
            } else {
                self.theme = Theme::Dracula
            }
        } else if message == Print {
            self.text.push_str("printu-sprintu ");
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        println!("view()");

        use PrinterMessage::*;
        let mut text = text(self.text.clone());
        let mut btn_change_theme = button("Change Theme").on_press(ChangeTheme);
        let mut btn_print = button("Print").on_press(Print);

        column!(btn_change_theme, btn_print, text).into()
    }

    fn title(&self) -> String {
        println!("title()");

        let appendix = format!(" - with {} theme!", self.theme);

        let mut new_title = self.base_title.clone();
        new_title.push_str(&appendix);

        new_title
    }

    fn theme(&self) -> Theme {
        println!("theme()");

        self.theme.clone()
    }
}
