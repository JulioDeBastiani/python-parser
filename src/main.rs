extern crate clap;

use clap::{Arg, App};

use std::fs::File;
// use std::collections::HashMap;
use std::io::{BufRead, BufReader};

static RESERVED_WORDS: [(&'static str, &'static str); 33] = [
    ("and", "TK.AND"),
    ("as", "TK.AS"),
    ("assert", "TK.ASSERT"),
    ("break", "TK.BREAK"),
    ("class", "TK.CLASS"),
    ("continue", "TK.CONTINUE"),
    ("def", "TK.DEF"),
    ("del", "TK.DEL"),
    ("elif", "TK.ELIF"),
    ("else", "TK.ELSE"),
    ("except", "TK.EXCEPT"),
    ("exec", "TK.EXEC"),
    ("finally", "TK.FINALLY"),
    ("for", "TK.FOR"),
    ("from", "TK.FROM"),
    ("global", "TK.GLOBAL"),
    ("if", "TK.IF"),
    ("import", "TK.IMPORT"),
    ("in", "TK.IN"),
    ("is", "TK.IS"),
    ("lambda", "TK.LAMBDA"),
    ("none", "TK.NONE"),
    ("nonlocal", "TK.NONLOCAL"),
    ("not", "TK.NOT"),
    ("or", "TK.OR"),
    ("pass", "TK.PASS"),
    ("print", "TK.PRINT"),
    ("raise", "TK.RAISE"),
    ("return", "TK.RETURN"),
    ("try", "TK.TRY"),
    ("while", "TK.WHILE"),
    ("with", "TK.WITH"),
    ( "yield", "TK.YIELD")
];

static OPERATORS: [(&'static str, &'static str); 44] = [
    ("+", "TK.MAIS"),
    ("-", "TK.MENOS"),
    ("*", "TK.VEZES"),
    ("/", "TK.BARRA"),
    ("%", "TK.PORCENTO"),
    ("&", "TK.ECOMERCIAL"),
    ("|", "TK.PIPE"),
    ("^", "TK.CIRCUMFLEXO"),
    ("~", "TK.TIL"),
    ("<", "TK.MENOR"),
    (">", "TK.MAIOR"),
    ("(", "TK.PARENTESES_ESQUERDO"),
    (")", "TK.PARENTESES_DIREITO"),
    ("[", "TK.COLCHETES_ESQUERDO"),
    ("]", "TK.COLCHETES_DIREITO"),
    ("{", "TK.CHAVES_ESQUERDA"),
    ("}", "TK.CHAVES_DIREITA"),
    (",", "TK.VIRGULA"),
    (":", "TK.DOIS_PONTOS"),
    (".", "TK.PONTO"),
    (";", "TK.PONTO_VIRGULA"),
    ("@", "TK.ARROBA"),
    ("=", "TK.IGUAL"),
    ("**", "TK.NOME_PARAMETRO"),
    ("//", "TK.BARRA_DUPLA"),
    ("<<", "TK.SHIFT_LEFT"),
    (">>", "TK.SHIFT_RIGHT"),
    ("<=", "TK.MENOR_IGUAL"),
    (">=", "TK.MAIOR_IGUAL"),
    ("==", "TK.IGUAL_IGUAL"),
    ("!=", "TK.DIFERENTE"),
    ("+=", "TK.MAIS_IGUAL"),
    ("-=", "TK.MENOS_IGUAL"),
    ("*=", "TK.VEZES_IGUAL"),
    ("/=", "TK.BARRA_IGUAL"),
    ("//=", "TK.BARRA_DUPLA_IGUAL"),
    ("%=", "TK.PORCENTO_IGUAL"),
    ("@=", "TK.ARROBA_IGUAL"),
    ("&=", "TK.ECOMERCIAL_IGUAL"),
    ("|=", "TK.PIPE_IGUAL"),
    ("^=", "TK.CIRCUMFLEXO_IGUAL"),
    (">>=", "TK.SHIFT_RIGHT_IGUAL"),
    ("<<=", "TK.SHIFT_LEFT_IGUAL"),
    ("**=", "TK.DUPLO_ASTERISCO_IGUAL")
];

fn char_defines_operator(c: char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        '%' => true,
        '&' => true,
        '|' => true,
        '^' => true,
        '~' => true,
        '<' => true,
        '>' => true,
        '(' => true,
        ')' => true,
        '[' => true,
        ']' => true,
        '{' => true,
        '}' => true,
        ',' => true,
        ':' => true,
        '.' => true,
        ';' => true,
        '@' => true,
        '=' => true,
        _ => false
    }
}

fn char_acts_as_separator(c: char) -> bool {
    match c {
        ' ' => true,
        '\t' => true,
        '\n' => true,
        _ => false
    }
}

#[derive(Debug, PartialEq)]
enum LiteralTypes {
    Int = 1,
    Float = 2,
    String = 4
}

#[derive(Debug, PartialEq)]
enum TkType {
    Indentaion,
    Dedentation,
    ReservedWord(&'static str),
    Operator(&'static str),
    Literal(LiteralTypes),
    Identifier,
}

#[derive(Debug)]
struct Token {
    tk_type: TkType,
    lexema: String,
    row: usize,
    col: usize
}

impl Token {
    fn new(tk_type: TkType, lexema: String, row: usize, col: usize) -> Token {
        Token {
            tk_type: tk_type,
            lexema: lexema.to_owned(),
            row: row,
            col: col
        }
    }
}

// TODO passar um option com o char de indentacao do arquivo
fn get_line_indentation(line: &Vec<char>) -> usize {
    let mut ind: usize = 0;

    for c in line.iter() {
        match c {
            ' ' => ind += 1,
            '\t' => ind += 1,
            _ => break,
        }
    }

    ind
}

fn get_string_literal(line: &Vec<char>, delimiter: char, col: &mut usize, row: usize) -> String {
    let mut lexema = String::default();
    lexema.push(delimiter);
    let mut icol = *col + 1;
    
    loop {
        lexema.push(line[icol]);
        
        match line[icol] {
            // TODO tratar os erros igual gente decente
            '\n' => panic!("Unexpected end of line at: row {}, col {}", row, icol),
            '\\' => {
                if line[icol + 1] == delimiter {
                    lexema.push(delimiter);
                    icol += 2;
                }
            },
            delimiter => {
                icol += 1;
                break;
            },
            _ => {
                icol += 1;
            }
        }
    }

    *col = icol;
    lexema
}

fn get_lexema_as_int_literal(line: &Vec<char>, col: &mut usize, row: usize) -> Option<String> {
    let mut lexema = String::default();
    lexema.push(line[*col]);
    let mut icol = *col + 1;

    loop {
        let c = line[icol];

        if c.is_numeric() {
            lexema.push(c);
        } else if char_acts_as_separator(c) {
            if lexema.is_empty() {
                return None;
            } else {
                break;
            }
        } else if char_defines_operator(c) {
            if lexema.is_empty() {
                return None;
            } else {
                break;
            }
        } else {
            if lexema.is_empty() {
                return None;
            } else {
                // TODO tratar os erros igual gente decente
                panic!("Invalid literal at: row {}, col {}", row, col);
            }
        }
    }

    *col = icol;
    Some(lexema)
}

fn generate_tokens(src_file: &str) -> std::io::Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut src = BufReader::new(File::open(src_file)?);

    // TODO anotar o processo melhor
    // TODO `log` seria uma boa ideia
    println!("Tokenazing: \"{}\"", src_file);

    let mut buf = Vec::<u8>::new();
    let mut ind = Vec::new();

    let mut row: usize = 0;

    while src.read_until(b'\n', &mut buf)? != 0 {
        // TODO tratar os erros igual gente decente
        let l = String::from_utf8(buf).expect("source file is not UTF-8");

        let line: Vec<char> = l.chars().collect();

        // Indentacao
        let line_indentation = get_line_indentation(&line);
        let next_char_col = line_indentation + 1;

        // Ignora se for uma linha em branco
        if line.len() >= next_char_col && line[next_char_col] != '\n' {
            // TODO rename
            let tot_ind = ind.iter().sum();

            if line_indentation < tot_ind {
                let difference = tot_ind - line_indentation;

                if Some(&difference) != ind.last() {
                    // TODO tratar os erros igual gente decente
                    panic!("Invalid indentation at: row {}, col {}", row, line_indentation);
                }
                
                ind.pop();
                tokens.push(Token::new(TkType::Dedentation, "".to_owned(), row, 0));
            } else if line_indentation > tot_ind {
                let difference = line_indentation - tot_ind;
                ind.push(difference);
                tokens.push(Token::new(TkType::Indentaion, "".to_owned(), row, 0));
            }

        }
        
        let mut col = line_indentation;
        let mut col_ini = col;

        loop {
            let mut currtk = String::default();

            match line[col] {
                ' ' => {
                    // TODO salvar token
                },
                '\t' => {
                    // TODO salvar token
                },
                '\n' => {
                    // TODO salvar token
                    break;
                },
                '#' => {
                    // TODO salvar token
                    break;
                },
                '\'' => {
                    // TODO bloco de comentario
                    let lexema = get_string_literal(&line, '\'', &mut col, row);
                    tokens.push(Token::new(TkType::Literal(LiteralTypes::String), lexema, row, col_ini));
                },
                '"' => {
                    // TODO bloco de comentario
                    let lexema = get_string_literal(&line, '\'', &mut col, row);
                    tokens.push(Token::new(TkType::Literal(LiteralTypes::String), lexema, row, col_ini));
                },
                c => {

                }
            }
        }
        
        buf = l.into_bytes();
        buf.clear();
    }
    
    Ok(tokens)
}

fn run(src_file: &str, out_dir: &str) -> std::io::Result<()> {
    let tokens = generate_tokens(src_file)?;
    
    // let mut tokens = Vec::new();
    // let mut ind = Vec::new();
    // let mut currtk = String::default();
    
    // println!("Tokenazing: \"{}\"", src_file);
    // let mut src = BufReader::new(File::open(src_file).expect("Could not open source file"));
    // let mut buf = Vec::<u8>::new();

    // let mut row = 0;

    while src.read_until(b'\n', &mut buf).expect("could not read line") != 0 {

        let mut col_ini = col;
        let mut isop = false;
        let mut is_comment = false;
        let mut is_string_literal = false;
        let mut literal_char = '"';

        loop {
            match curc {
                ' ' => {
                    if is_string_literal {
                        currtk.push(' ');
                    } else if is_comment {
                        currtk.clear();
                    } else if !currtk.is_empty() {
                        let tkt = get_token_type(&currtk);
                        tokens.push(Token::new(tkt, &currtk, row, col_ini));
                        currtk.clear();
                    }
                },
                '\t' => {
                    if is_string_literal {
                        currtk.push(' ');
                    } else if is_comment {
                        currtk.clear();
                    } else if !currtk.is_empty() {
                        let tkt = get_token_type(&currtk);
                        tokens.push(Token::new(tkt, &currtk, row, col_ini));
                        currtk.clear();
                    }
                },
                '\n' => {
                    if is_string_literal {
                        currtk.push(' ');
                    } else if is_comment {
                        currtk.clear();
                    } else if !currtk.is_empty() {
                        let tkt = get_token_type(&currtk);
                        tokens.push(Token::new(tkt, &currtk, row, col_ini));
                        currtk.clear();
                    }
                },
                '#' => {
                    if is_string_literal {
                        currtk.push(' ');
                    } else if is_comment {
                        currtk.clear();
                    } else if !currtk.is_empty() {
                        let tkt = get_token_type(&currtk);
                        tokens.push(Token::new(tkt, &currtk, row, col_ini));
                        currtk.clear();
                    }

                    break;
                },
                // TODO delimitadores de strings e comentarios
                c => {
                    if is_string_literal {
                        currtk.push(c);

                        if c == literal_char {
                            let tkt = get_token_type(&currtk);
                            tokens.push(Token::new(tkt, &currtk, row, col_ini));
                            currtk.clear();
                            is_string_literal = false;
                        }
                    } else if is_comment {

                    }
                    
                    
                    
                    
                    
                    
                    let lisop = OPERATORS.iter().any(|o| o[0].chars().next().unwrap() == c);

                    if currtk.is_empty() {
                        isop = lisop;
                    } else if isop != lisop {
                        if c == '.'{
                            if let Ok(_) = currtk.parse::<i32>() {
                                currtk.push(c);
                            } else {
                                let tkt = get_token_type(&currtk);
                                tokens.push(Token::new(tkt, &currtk, row, col_ini));
                                currtk.clear();
                                currtk.push(c);
                            }
                        }
                    }
                }
            }

            match gen.next() {
                Some(c) => curc = c,
                None => break
            }

            col += 1;
        }

        row += 1;
        buf = l.into_bytes();
        buf.clear();
    }
    
    Ok(())
}

fn main() {
    let matches = App::new("pytthon-parser")
        .version("0.1")
        .author("Julio De Bastiani <julioc.debastiani@gmail.com>")
        .about("simple python parser")
        .arg(Arg::with_name("INPUT")
            .help("input file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("output_dir")
            .short("o")
            .long("output_dir")
            .value_name("OUTDIR")
            .takes_value(true)
            .help("output directory"))
        .get_matches();

    let src_file = matches.value_of("INPUT").unwrap();
    let out_dir = matches.value_of("OUTDIR").unwrap_or("out");
    
    run(src_file, out_dir).unwrap();
}
