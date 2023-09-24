use std::cmp::min;

use super::{Gui, GuiMessage};
use crate::data::extraction::get_digits;
use iced::widget::{Column, Text};

impl Gui {
    pub(super) fn recite_view(&self) -> iced::Element<'_, GuiMessage> {
        Column::new()
            .push(Text::new(format!("Next position: {}", self.recite_pos)))
            .push(Text::new(self.number_text()))
            .padding(5)
            .spacing(5)
            .into()
    }

    fn number_text(&self) -> String {
        let number = match self.number {
            Some(n) => n,
            None => return String::from(""),
        };

        let name = number.name;
        let amount = min(self.recite_pos, 10);
        let start = self.recite_pos - amount;
        let mut recited_digits = get_digits(number, start, amount);
        if start == 0 && !recited_digits.is_empty() {
            recited_digits.insert(1, '.');
        } else if start > 0 {
            for _ in 0..3 {
                recited_digits.insert(0, '.');
            }
        }
        let recited_digits = String::from(recited_digits.iter().collect::<String>());
        format!("{}={}", name, recited_digits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::e::E;
    use iced::Sandbox;

    #[test]
    fn test_number_text() {
        let mut gui = Gui::new();
        gui.number = Some(&E);
        gui.recite_pos = 0;
        assert_eq!(gui.number_text(), "e=");
        gui.recite_pos = 1;
        assert_eq!(gui.number_text(), "e=2.");
        gui.recite_pos = 2;
        assert_eq!(gui.number_text(), "e=2.7");
        gui.recite_pos = 3;
        assert_eq!(gui.number_text(), "e=2.71");
        gui.recite_pos = 4;
        assert_eq!(gui.number_text(), "e=2.718");
        gui.recite_pos = 5;
        assert_eq!(gui.number_text(), "e=2.7182");
        gui.recite_pos = 6;
        assert_eq!(gui.number_text(), "e=2.71828");
        gui.recite_pos = 7;
        assert_eq!(gui.number_text(), "e=2.718281");
        gui.recite_pos = 8;
        assert_eq!(gui.number_text(), "e=2.7182818");
        gui.recite_pos = 9;
        assert_eq!(gui.number_text(), "e=2.71828182");
        gui.recite_pos = 10;
        assert_eq!(gui.number_text(), "e=2.718281828");
        gui.recite_pos = 11;
        assert_eq!(gui.number_text(), "e=...7182818284");
    }
}
