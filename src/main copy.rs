#![allow(warnings)]

mod message;

use iced::{
    widget::{self, text, button},
    Sandbox, Settings, Theme,
};

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

// This struct holds the current state of the calculator.
// For now, it just contains a string opening
struct Calculator {
    opening: String,
}

// Sandbox is a trait that allows you to define a simple application structure
// It provides key methods that define how the calculator behaves,
// updates, and displays its interface.
impl Sandbox for Calculator {
    // Events or user interactions that the application can handle or react to,
    // any event that can change a state of the application
    // E.g. user pressing a button would be a message
    type Message = message::CalculatorMessage;

    // notatka:
    // kompilator krzyczał gdy było tutaj "fn new() -> Self {}",
    // bo gdy scope jest pusty to evaluuje się do () (unit type),
    // a przecież obiecałaś mu, że zwrócisz Self (czyli Calculator)

    // kompilator krzyczał jeśli zrobisz "fn new() {}",
    // bo mimo, że zwracany typ () zgadza się z zadeklarowanym implicitly
    // to funkcja "new" z interfejsu (traita) Sandbox MUSI zwracać Self
    // (mieć dokładnie taką sygnaturę: "fn new() -> Self")

    // zadanie domowe:
    // figure out poprzez googlanie lub rozmowę z Chatem GPT
    // zadając pytania "why" and "how" co tu się odpierdala,
    // masz to rozumieć i wiedzieć co i jak i rozwikłać ten problem
    // dzięki wiedzy, a nie dzięki znalezionym odpowiedziom.
    // (najlepiej unikaj zaglądania do poprzedniego projektu,
    //  raczej chcemy byś spróbowała się nauczyć sama od nowa)

    // This is the state we want our app to be in when it starts
    // Creates a new instance of Calculator and sets the initial
    // state (message: "Ready to calculate").
    fn new() -> Self {
        Calculator {
            opening: "Ready to calculate".to_string(),
        }
    }

    // Sets the title of the calculator window
    fn title(&self) -> String {
        String::from("Kalkulator")
    }

    // Method where user interactions (messages) will be handled.
    // The Self::Message type refers to the CalculatorMessage enum,
    // meaning this method will react to the Calculate message when
    // the user triggers it.
    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    // Application View
    // - how your app looks like
    // - what's there? buttons, text fields, etc...
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut button1 = widget::button("1").on_press(message::CalculatorMessage::Calculate);

        widget::container(button1).into()
    }

    fn theme(&self) -> Theme {
        iced::Theme::Dracula
    }

    // i skompiluj i uruchom, żeby zobaczyć czy się włącza
    // nie musi nic robić, ma się pojawić okienko i tyle <3
}
