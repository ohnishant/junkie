pub fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

pub fn is_digit(ch: char) -> bool {
    return ch.is_numeric();
    // return ch >= '0' && ch <= '9';
}
