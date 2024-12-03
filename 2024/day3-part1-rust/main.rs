use std::{
    fs::{self},
    num::ParseIntError,
    u8,
};

struct MultStatement {
    left_value: usize,
    right_value: usize,
}

struct Parser<'a> {
    text_length: usize,
    text_as_bytes: &'a [u8],
    current_position: usize,
    current_char: char,
    sum: usize,
}

impl Parser<'_> {
    fn read_char(&mut self) -> Result<char, &str> {
        let previous_char = self.current_char;

        self.current_position += 1;
        if self.current_position >= self.text_length {
            return Err("out of bounds");
        }

        self.current_char = self.text_as_bytes[self.current_position] as char;

        Ok(previous_char)
    }

    fn expect_next_char(&mut self, c: char) -> bool {
        match self.read_char() {
            Ok(char) => return char == c,
            Err(_) => return false,
        }
    }

    fn read_mult_statement(&mut self) -> Option<MultStatement> {
        if !self.expect_next_char('u') {
            return None;
        }
        if !self.expect_next_char('l') {
            return None;
        }
        if !self.expect_next_char('(') {
            return None;
        }

        let left_number = self.read_number(3, ',');
        if left_number.is_err() {
            return None;
        }
        let left_value = left_number.unwrap();

        let right_number = self.read_number(3, ')');
        if right_number.is_err() {
            return None;
        }
        let right_value = right_number.unwrap();

        Some(MultStatement {
            left_value,
            right_value,
        })
    }

    fn read_number(&mut self, max_digits: u8, break_on_char: char) -> Result<usize, &str> {
        let mut char_vector: Vec<char> = Vec::new();
        let mut number_len = 0;

        // NOTE: number must start with digit
        match self.read_char() {
            Ok(c) => {
                if !c.is_digit(10) {
                    return Err("number did not start with digit");
                }
                number_len += 1;
                char_vector.push(c);
            }
            Err(_) => return Err("failed reading char"),
        }

        while number_len <= max_digits {
            // NOTE: if reached max digits and next char is correct, build number
            if number_len == max_digits {
                if !self.expect_next_char(break_on_char) {
                    return Err("");
                }
                let number: Result<usize, ParseIntError> =
                    char_vector.iter().cloned().collect::<String>().parse();
                match number {
                    Ok(num) => return Ok(num),
                    Err(_) => return Err("error forming number"),
                }
            }
            match self.read_char() {
                Ok(c) => {
                    // NOTE: if reached break char, build number
                    if c == break_on_char {
                        // convert char_vector to number
                        let number: Result<usize, ParseIntError> =
                            char_vector.iter().cloned().collect::<String>().parse();
                        match number {
                            Ok(num) => return Ok(num),
                            Err(_) => return Err("error forming number"),
                        }
                    }

                    if !c.is_digit(10) {
                        return Err("expected digit or break_on_char");
                    }

                    char_vector.push(c);
                    number_len += 1;
                }
                Err(_) => return Err("failed reading char"),
            }
        }

        // NOTE: build number
        let number: Result<usize, ParseIntError> =
            char_vector.iter().cloned().collect::<String>().parse();
        match number {
            Ok(num) => return Ok(num),
            Err(_) => return Err("error forming number"),
        }
    }
}

fn main() {
    let text = fs::read_to_string("input1.txt").unwrap();

    let mut parser = Parser {
        text_as_bytes: text.as_bytes(),
        text_length: text.len(),
        current_position: 0,
        current_char: 's',
        sum: 0,
    };

    while parser.current_position < parser.text_length {
        match parser.read_char() {
            Ok(c) => {
                if c == 'm' {
                    let mult_statement = parser.read_mult_statement();
                    match mult_statement {
                        Some(m) => parser.sum = parser.sum + m.left_value * m.right_value,
                        None => (),
                    }
                }
            }
            Err(_) => (),
        }
    }

    println!("{}", parser.sum);
}
