#![allow(warnings)]

mod message;

use iced::Sandbox;

fn main() {
    println!("Hello, world!");
}

struct Calculator {}

impl Sandbox for Calculator {
    type Message = message::CalculatorMessage;

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}
