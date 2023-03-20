#[derive(Debug)]
struct Lexer {
    input: String,
    position: Option<usize>,
    readPosition: usize,
    ch: Option<char>,
}


impl Lexer {
    // Create a new lexer object with an input string
    fn new(input_string: String) -> Lexer {
        return Lexer {input: input_string, position: None, readPosition: 0, ch: None};
    }
    
    // Advance pointer in lexer
    fn read_char(&self){
        if self.readPosition > self.input.len() {
            self.ch = Some('\0');
        } else {
            self.ch = Some(self.input.chars().nth(self.readPosition).unwrap());
        }

        self.position = Some(self.readPosition);
        self.readPosition += 1;
    }
}

fn main() {
    let l = Lexer::new("={),=".toString());
    
    l.read_char();
    println!("{}", l.ch);
}
