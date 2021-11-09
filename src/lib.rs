#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/*
TODO's:
* add a raw_write and a raw_write_line functions. will be the same as write and write_lines, but
will print with out the space in between the tokens.
*/

use clay_lib::{Nargs, Token};

use std::collections::HashMap;
use std::env;
use std::path::Path;

// type func = fn(Vec<Token>) -> Option<()>;

// the deffinitions of all standard library functions go here.

fn make_bool(input: &str) -> bool {
    match input {
        "t" => true,
        "nill" => false,
        _ => panic!("external library function returned a not boolean but clamed it as a bool."),
    }
}

#[no_mangle]
pub fn not<'a>(truth: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    /*
    takes a boolean val as an arguement returns its negation.
    work inprogress.
    */
    let negation: bool = match truth[0] {
        Token::Bool(val) => !val,
        _ => panic!("cannot negate a non boolean value."),
    };
    return Ok(Some(Token::Bool(negation)));
}

#[no_mangle]
pub fn write_line<'a>(lines: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    let _ = write(lines);
    println!();
    return Ok(None);
}

#[no_mangle]
pub fn write<'a>(lines: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    // println!("write args: {:?}", lines);
    let mut loc_lines = lines.clone();
    let last_i = loc_lines.len();
    for i in 0..last_i {
        let line = loc_lines.pop().unwrap();
        print_line(line);
        if i != last_i - 1 {
            print!(" ");
        }
    }
    return Ok(None);
}

#[no_mangle]
fn print_line(line: Token) {
    match line {
        Token::Symbol(output) => print!("{}", output),
        Token::Number(output) => print!("{}", output),
        Token::Str(output) => print!("{}", output),
        Token::Form(form) => print_form(*form),
        Token::Bool(truth) => print!(
            "{}",
            match truth {
                true => "t",
                false => "nil",
            }
        ),
        _ => {} //return Err("ERROR: on write_line. you can't print that."),
    }
}

#[no_mangle]
fn print_form(form: Vec<Token>) {
    let mut loc_form = form.clone();
    loc_form.reverse();
    print!("`(");
    let _ = write(&loc_form);
    print!(")");
}

#[no_mangle]
pub fn terpri<'a>(_things: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    println!();
    return Ok(None);
}

#[no_mangle]
pub fn get_funcs<'a>() -> HashMap<
    &'a str,
    (
        Nargs,
        &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
    ),
> {
    let mut std_funcs: HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    > = HashMap::new();

    // base lib
    std_funcs.insert("write", (Nargs::INF, &write));
    std_funcs.insert("print", (Nargs::INF, &write));
    std_funcs.insert("write-line", (Nargs::INF, &write_line));
    std_funcs.insert("terpri", (Nargs::Num(0), &terpri));
    std_funcs.insert("not", (Nargs::Num(1), &not));
    std_funcs.insert("!", (Nargs::Num(1), &not));

    return std_funcs;
}
