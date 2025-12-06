#[derive(PartialEq, Clone)]
pub enum MapSymbol {
    Empty,
    Scroll,
    Oob,
}

pub fn parse_char_in_input(c: char) -> MapSymbol {
    match c {
        '.' => MapSymbol::Empty,
        '@' => MapSymbol::Scroll,
        _ => unreachable!(),
    }
}
