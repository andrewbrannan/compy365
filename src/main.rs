extern crate regex;

use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

use regex::Regex;

/* Token type holds all the supported tokens, anything not supported is ignored */
#[derive(Debug)]  // So we can print the list and see whats in it.
#[derive(PartialEq)] // So we can compare them
enum Token{
    OpenBrace,
    CloseBrace,
    OpenParenth,
    CloseParenth,
    Semicolon,
    KeywordInt,
    KeywordReturn,
    Identifier(String),
    Integer(u32),
}

/*Stucts for each supported AST node*/
struct Program {
    function: Function,
}

struct Function {
    id: String,
    statement: Statement,
}

struct Statement {
    expression: Expression,
}

struct Expression {
    int: u32,
}

fn parse_program<I>(token_list: I) -> Program
where
    I: Iterator<Item = Token>,
    I: Copy {
    return Program{
        function: parse_function(token_list),
    }
}

fn parse_function<I>(mut token_list: I) -> Function
where
    I: Iterator<Item = Token>,
    I: Copy {
    let function_string: String;

    //Consume the 'int' keyword
    match token_list.next() {
        Some(t) => {
            match t {
                Token::KeywordInt => (),
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }

    //Consume the id string
    match token_list.next() {
        Some(t) => {
            match t {
                Token::Identifier(s) => {
                    function_string = s;
                },
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }

    //Consume the open parenth
    match token_list.next() {
        Some(t) => {
            match t {
                Token::OpenParenth => (),
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }

    //Consume the close parenth
    match token_list.next() {
        Some(t) => {
            match t {
                Token::CloseParenth => (),
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }

    //Consume the open brace
    match token_list.next() {
        Some(t) => {
            match t {
                Token::OpenBrace => (),
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }

    // Pull out the statement
    let function_statement = parse_statement(token_list);

    //Consume the close brace
    match token_list.next() {
        Some(t) => {
            match t {
                Token::CloseBrace => (),
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }

    return Function{
        id: function_string,
        statement: function_statement,
    }
}

fn parse_statement<I>(mut token_list: I) -> Statement
where
    I: Iterator<Item = Token>,
    I: Copy {

    //Consume the 'return' keyword
    match token_list.next() {
        Some(t) => {
            match t {
                Token::KeywordReturn => (),
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }

    //Pull out the expression
    let statement_expression = parse_expression(token_list);

    //Consume the semi-colon keyword
    match token_list.next() {
        Some(t) => {
            match t {
                Token::Semicolon => (),
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }

    return Statement {
        expression: statement_expression,
    }
}

fn parse_expression<I>(mut token_list: I) -> Expression
where
    I: Iterator<Item = Token>,
    I: Copy {
    //pull out the int
    match token_list.next() {
        Some(t) => {
            match t {
                Token::Integer(i) => return Expression{int: i},
                _ => panic!("Error parsing function"),
            }
        }
        None => panic!("Error parsing function"),
    }
}

fn parse(tokens: &Vec<Token>) -> Program {
    return parse_program(&mut tokens.iter_mut());
}

/** @brief  Lexes/Tokenizes a string into a list
 *  of symbols (tokens) for later processing.
 *
 *  Used this http://keepcalmandlearnrust.com/2016/08/iterator-and-peekable/
 *  as inspiration, though I implemented the whole thing myself in what is
 *  probably a less functional/idiomatic way to keep it simple while learning
 *  rust.  I'll probably come back later and make it nicer, use a custom iterator
 *  etc...
 *
 *  @param A reference to the string to be tokenized
 *  @return A vector of the tokens found in the string
 */
fn lex(source: &String) -> Vec<Token>{

    let mut tokens: Vec<Token> = Vec::new();
    let mut c = source.chars().peekable();

    let re_alpha = Regex::new(r"[a-zA-Z]").unwrap();
    let re_ints = Regex::new(r"[0-9]+").unwrap();

    //Start iterating through the string
    //Loop
        //Is the current char a symbol? ('{' | '}' | '(' | ')' | ';')
            //If Y: add it to token list
            //Continue
        //Is the current char in [a-zA-Z]?
            //If Y: Start building a word token
            //Read through the string while the current char matches [a-zA-Z]
            //Add the word to the token list
        //Is the current char an integer literal? ([0-9]+)
            //If Y: Start building a number token
            //Read through the rest of the string where char matches [0-9]+
            //Add the number to the token list
            //Continue


    loop{
        match c.next(){
            Some(ch)   => {
                if ch == '{' {
                    tokens.push(Token::OpenBrace);
                } else if ch == '}' {
                    tokens.push(Token::CloseBrace);
                } else if ch == '(' {
                    tokens.push(Token::OpenParenth);
                } else if ch == ')' {
                    tokens.push(Token::CloseParenth);
                } else if ch == ';' {
                    tokens.push(Token::Semicolon);
                } else if re_alpha.is_match(&ch.to_string()) {
                    let mut s: String = ch.to_string();

                    while c.peek() != None && re_alpha.is_match(&c.peek().unwrap().to_string()){
                        s = s + &c.next().unwrap().to_string();
                    }

                    if s == "int" {
                        tokens.push(Token::KeywordInt);
                    } else if s == "return" {
                        tokens.push(Token::KeywordReturn);
                    } else {
                        tokens.push(Token::Identifier(s));
                    }

                } else if re_ints.is_match(&ch.to_string()) {

                    let mut s: String = ch.to_string();

                    while c.peek() != None && re_ints.is_match(&c.peek().unwrap().to_string()){
                        s = s + &c.next().unwrap().to_string();
                    }

                    tokens.push(Token::Integer(s.parse::<u32>().unwrap()));
                }
            }
            None   => break,
        }
    }

    return tokens;

}

fn main() {
    println!("###  COMPY 365 C COMPILER ###");
    println!("");

    println!("### STEP 1: READ SOURCE FILE ###");

    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        panic!("ERROR: Source file to be compiled must be passed as the first argument");
    } else if args.len() > 2 {
        panic!("ERROR: Only one source can be compiled")
    }

    //Try to open the source file
    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("ERROR: Couldn't open {}: {}", display,why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string s
    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(why) => panic!("ERROR: Couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("DONE\n"),
    };

    println!("### STEP 2: TOKENIZE THE SOURCE FILE ###");

    let tokens: Vec<Token> = lex(&s);

    println!("DONE");
}
