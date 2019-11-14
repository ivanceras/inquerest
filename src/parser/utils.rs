pub(crate) fn to_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub(super) fn alpha_or_underscore(ch: char) -> bool {
    pom::char_class::alpha(ch as u8) || underscore(ch)
}

pub(super) fn alphanum_or_underscore(ch: char) -> bool {
    pom::char_class::alphanum(ch as u8) || underscore(ch)
}

pub(super) fn underscore(ch: char) -> bool {
    ch == '_'
}
