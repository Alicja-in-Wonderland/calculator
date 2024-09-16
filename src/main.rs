#![allow(warnings)]

mod message;

use iced::{widget::{self, text}, Sandbox, Settings, Theme};

fn main() {
    Calculator::run(Settings::default());
}

struct Calculator {
    opening: String,
}

impl Sandbox for Calculator {
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
    
    // Here is where you should return the initial state of your app.
    fn new() -> Self {
        Calculator {
            opening: "Ready to calculate".to_string(),
        }
    }

    // no i usuń tutaj todo i zrób to samo co powyżej
    fn title(&self) -> String {
        "Kalkulator".to_string()
    }

    // i tu też
    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    // no i tu. :)
    // Application View
    // - how your app looks like
    // - what's there? buttons, text fields, etc...
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let button1 = widget::button("1").on_press(1);

        widget::container(button1).into()

    }

    fn theme(&self) -> Theme {
        iced::Theme::SolarizedLight
    }

    // i skompiluj i uruchom, żeby zobaczyć czy się włącza
    // nie musi nic robić, ma się pojawić okienko i tyle <3
}
