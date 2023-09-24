use iced::{
    widget::{Button, Column, Row, Text},
    Sandbox,
};

use crate::data::{e::E, extraction::get_digits, pi::PI};

pub(crate) struct Gui {
    number: Option<&'static str>,
    mode: GuiMode,
    pos: u32,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui {
            number: None,
            mode: GuiMode::Browse,
            pos: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Irrational Pal")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            GuiMessage::PickedNumber(number) => self.number = Some(number),

            GuiMessage::PickedMode(mode) => self.mode = mode,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let button_e = Button::new(Text::new("e")).on_press(GuiMessage::PickedNumber(E));
        let button_pi = Button::new(Text::new("Pi")).on_press(GuiMessage::PickedNumber(PI));
        let button_browse =
            Button::new(Text::new("Browse")).on_press(GuiMessage::PickedMode(GuiMode::Browse));
        let button_recite =
            Button::new(Text::new("Recite")).on_press(GuiMessage::PickedMode(GuiMode::Recite));
        let mut col = Column::new().push(
            Row::new()
                .push(button_e)
                .push(button_pi)
                .padding(5)
                .spacing(5),
        );
        if self.number.is_some() {
            col = col.push(
                Row::new()
                    .push(button_browse)
                    .push(button_recite)
                    .padding(5)
                    .spacing(5),
            );
            match self.mode {
                GuiMode::Browse => {
                    col = col.push(self.digit_view());
                }
                GuiMode::Recite => {}
            }
        }
        col.padding(5).spacing(5).into()
    }
}

impl Gui {
    fn digit_view(&self) -> iced::Element<'_, GuiMessage> {
        let number = self.number.unwrap_or("");
        let shown_digits = get_digits(number, self.pos, 10);
        let mut row = Row::new();
        let button_left = Button::new(Text::new("<"));
        let button_right = Button::new(Text::new(">"));
        row = row.push(button_left);
        for d in shown_digits.iter() {
            row = row.push(Text::new(d.to_string()));
        }
        row = row.push(button_right);
        row.padding(5).spacing(5).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    PickedNumber(&'static str),
    PickedMode(GuiMode),
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMode {
    Browse,
    Recite,
}
