/*
TODO's:
* add a raw_write and a raw_write_line functions. will be the same as write and write_lines, but
will print with out the space in between the tokens.
*/

use clay_lib::{Nargs, Token};

use std::collections::HashMap;
use std::io::{self, Write};

// use std::env;
// use std::path::Path;

// type func = fn(Vec<Token>) -> Option<()>;

// the deffinitions of all standard library functions go here.

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
    let _ = printf(lines);
    println!();
    return Ok(None);
}

#[no_mangle]
pub fn write<'a>(lines: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    // print!("in the write function ");
    let _ = printf(lines);
    return Ok(None);
}

fn printf(tokens: &Vec<Token>) {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    // println!("write args: {:?}", tokens);
    let mut loc_lines = tokens.clone();
    // println!("loc_lines: {:?} ", loc_lines);
    let last_i = loc_lines.len();
    // println!("{} ", last_i);

    for i in 0..last_i {
        let line = loc_lines.pop().unwrap();
        let _ = write!(handle, "{}", print_tok(line));
        if i != last_i - 1 {
            let _ = write!(handle, " ");
        }
    }
    handle.flush();
}

fn print_tok(line: Token) -> String {
    match line {
        Token::Symbol(output) => output,
        Token::Number(output) => output,
        Token::Str(output) => output,
        // Token::Form(form) => print_form(*form),
        Token::Bool(truth) => {
            if truth {
                "t".to_string()
            } else {
                "nil".to_string()
            }
        }
        _ => panic!("ERROR: in print_tok. you can't print parens and EOFs"), // {} //return Err("ERROR: on write_line. you can't print that."),
    }
}

// #[no_mangle]
// fn print_form(form: Vec<Token>) {
//     let mut loc_form = form.clone();
//     loc_form.reverse();
//     print!("`(");
//     let _ = write(&loc_form);
//     print!(")");
// }

#[no_mangle]
pub fn terpri<'a>(_things: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    println!();
    return Ok(None);
}

fn add_fn(dict: &mut HashMap<String, (Nargs, String)>, lisp: &str, num_args: Nargs, rust: &str) {
    dict.insert(lisp.to_string(), (num_args, rust.to_string()));
}

/*
#[no_mangle]
pub fn get_funcs<'a>() -> Vec<(
    String,
    Nargs,
    String,
    //&'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
)> {
    let mut std_funcs: Vec<(
        String,
        Nargs,
        // String,
        &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
    )> = Vec::new();

    // base lib
    // std_funcs.push(("write", Nargs::INF, &write));
    // std_funcs.push(("print", Nargs::INF, &write));
    // std_funcs.push(("write-line", Nargs::INF, &write_line));
    // std_funcs.push(("terpri", Nargs::Num(0), &terpri));
    // std_funcs.push(("not", Nargs::Num(1), &not));
    // std_funcs.push(("!", Nargs::Num(1), &not));
    std_funcs.push(("write".to_string(), Nargs::INF, "write".to_string()));
    std_funcs.push(("print".to_string(), Nargs::INF, "write".to_string()));
    std_funcs.push((
        "write-line".to_string(),
        Nargs::INF,
        "write_line".to_string(),
    ));
    std_funcs.push(("terpri".to_string(), Nargs::Num(0), "terpri".to_string()));
    std_funcs.push(("not".to_string(), Nargs::Num(1), "not".to_string()));
    std_funcs.push(("!".to_string(), Nargs::Num(1), "not".to_string()));

    return std_funcs;
}
*/

#[no_mangle]
pub fn get_funcs<'a>() -> HashMap<
    String,
    (
        Nargs,
        String,
        // &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
    ),
> {
    let mut std_funcs: HashMap<
        String,
        (
            Nargs,
            String,
            // &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    > = HashMap::new();

    // base lib
    // std_funcs.push(("write", Nargs::INF, &write));
    // std_funcs.push(("print", Nargs::INF, &write));
    // std_funcs.push(("write-line", Nargs::INF, &write_line));
    // std_funcs.push(("terpri", Nargs::Num(0), &terpri));
    // std_funcs.push(("not", Nargs::Num(1), &not));
    // std_funcs.push(("!", Nargs::Num(1), &not));

    // std_funcs.insert("write".to_string(), (Nargs::INF, &write));
    // std_funcs.insert("print".to_string(), (Nargs::INF, &write));
    // std_funcs.insert("write-line".to_string(), (Nargs::INF, &write_line));
    // std_funcs.insert("writeln".to_string(), (Nargs::INF, &write_line));
    // std_funcs.insert("terpri".to_string(), (Nargs::Num(0), &terpri));
    // std_funcs.insert("not".to_string(), (Nargs::Num(1), &not));
    // std_funcs.insert("!".to_string(), (Nargs::Num(1), &not));

    add_fn(&mut std_funcs, "write", Nargs::INF, "write");
    add_fn(&mut std_funcs, "print", Nargs::INF, "write");
    add_fn(&mut std_funcs, "write-line", Nargs::INF, "write_line");
    add_fn(&mut std_funcs, "writeln", Nargs::INF, "write_line");
    add_fn(&mut std_funcs, "terpri", Nargs::INF, "terpri");
    add_fn(&mut std_funcs, "not", Nargs::INF, "not");
    add_fn(&mut std_funcs, "!", Nargs::INF, "not");

    return std_funcs;
}
