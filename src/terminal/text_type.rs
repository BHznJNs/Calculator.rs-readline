use std::fmt::Display;

use crossterm::style::Stylize;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TextType {
    Hint,

    Variable,
    Keyword,

    Didider,
    Comment,

    NumberLiteral,
    StringLiteral,
}

pub fn match_tx_type<T: Stylize>(text: T,type__: TextType) -> <T as Stylize>::Styled
    where <T as Stylize>::Styled: Display
{
    match type__ {
        TextType::Hint     => text.dim(),

        TextType::Variable => text.underlined(),
        TextType::Keyword  => text.dark_cyan(),

        TextType::Didider  => text.white(),
        TextType::Comment  => text.dark_green(),

        TextType::NumberLiteral => text.yellow(),
        TextType::StringLiteral => text.dark_yellow(),
    }
}