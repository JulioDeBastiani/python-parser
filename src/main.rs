extern crate clap;

use clap::{Arg, App};

use std::fmt;
use std::fs::File;
use std::io::{BufRead, Write, BufReader, BufWriter};

static RESERVED_WORDS: [(&'static str, &'static str); 33] = [
    ("and", "RWORD{AND}"),
    ("as", "RWORD{AS}"),
    ("assert", "RWORD{ASSERT}"),
    ("break", "RWORD{BREAK}"),
    ("class", "RWORD{CLASS}"),
    ("continue", "RWORD{CONTINUE}"),
    ("def", "RWORD{DEF}"),
    ("del", "RWORD{DEL}"),
    ("elif", "RWORD{ELIF}"),
    ("else", "RWORD{ELSE}"),
    ("except", "RWORD{EXCEPT}"),
    ("exec", "RWORD{EXEC}"),
    ("finally", "RWORD{FINALLY}"),
    ("for", "RWORD{FOR}"),
    ("from", "RWORD{FROM}"),
    ("global", "RWORD{GLOBAL}"),
    ("if", "RWORD{IF}"),
    ("import", "RWORD{IMPORT}"),
    ("in", "RWORD{IN}"),
    ("is", "RWORD{IS}"),
    ("lambda", "RWORD{LAMBDA}"),
    ("none", "RWORD{NONE}"),
    ("nonlocal", "RWORD{NONLOCAL}"),
    ("not", "RWORD{NOT}"),
    ("or", "RWORD{OR}"),
    ("pass", "RWORD{PASS}"),
    ("print", "RWORD{PRINT}"),
    ("raise", "RWORD{RAISE}"),
    ("return", "RWORD{RETURN}"),
    ("try", "RWORD{TRY}"),
    ("while", "RWORD{WHILE}"),
    ("with", "RWORD{WITH}"),
    ( "yield", "RWORD{YIELD}")
];

static OPERATORS: [(&'static str, &'static str); 44] = [
    ("+", "OPERATOR{MAIS}"),
    ("-", "OPERATOR{MENOS}"),
    ("*", "OPERATOR{VEZES}"),
    ("/", "OPERATOR{BARRA}"),
    ("%", "OPERATOR{PORCENTO}"),
    ("&", "OPERATOR{ECOMERCIAL}"),
    ("|", "OPERATOR{PIPE}"),
    ("^", "OPERATOR{CIRCUMFLEXO}"),
    ("~", "OPERATOR{TIL}"),
    ("<", "OPERATOR{MENOR}"),
    (">", "OPERATOR{MAIOR}"),
    ("(", "OPERATOR{PARENTESES_ESQUERDO}"),
    (")", "OPERATOR{PARENTESES_DIREITO}"),
    ("[", "OPERATOR{COLCHETES_ESQUERDO}"),
    ("]", "OPERATOR{COLCHETES_DIREITO}"),
    ("{", "OPERATOR{CHAVES_ESQUERDA}"),
    ("}", "OPERATOR{CHAVES_DIREITA}"),
    (",", "OPERATOR{VIRGULA}"),
    (":", "OPERATOR{DOIS_PONTOS}"),
    (".", "OPERATOR{PONTO}"),
    (";", "OPERATOR{PONTO_VIRGULA}"),
    ("@", "OPERATOR{ARROBA}"),
    ("=", "OPERATOR{IGUAL}"),
    ("**", "OPERATOR{NOME_PARAMETRO}"),
    ("//", "OPERATOR{BARRA_DUPLA}"),
    ("<<", "OPERATOR{SHIFT_LEFT}"),
    (">>", "OPERATOR{SHIFT_RIGHT}"),
    ("<=", "OPERATOR{MENOR_IGUAL}"),
    (">=", "OPERATOR{MAIOR_IGUAL}"),
    ("==", "OPERATOR{IGUAL_IGUAL}"),
    ("!=", "OPERATOR{DIFERENTE}"),
    ("+=", "OPERATOR{MAIS_IGUAL}"),
    ("-=", "OPERATOR{MENOS_IGUAL}"),
    ("*=", "OPERATOR{VEZES_IGUAL}"),
    ("/=", "OPERATOR{BARRA_IGUAL}"),
    ("//=", "OPERATOR{BARRA_DUPLA_IGUAL}"),
    ("%=", "OPERATOR{PORCENTO_IGUAL}"),
    ("@=", "OPERATOR{ARROBA_IGUAL}"),
    ("&=", "OPERATOR{ECOMERCIAL_IGUAL}"),
    ("|=", "OPERATOR{PIPE_IGUAL}"),
    ("^=", "OPERATOR{CIRCUMFLEXO_IGUAL}"),
    (">>=", "OPERATOR{SHIFT_RIGHT_IGUAL}"),
    ("<<=", "OPERATOR{SHIFT_LEFT_IGUAL}"),
    ("**=", "OPERATOR{DUPLO_ASTERISCO_IGUAL}")
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tp = match &self.tk_type {
            TkType::Indentaion => "INDENT",
            TkType::Dedentation => "DEDENT",
            TkType::ReservedWord(w) => w,
            TkType::Operator(w) => w,
            TkType::Literal(t) => match t {
                LiteralTypes::Int => "LITERAL{INT}",
                LiteralTypes::Float => "LITERAL{FLOAT}",
                LiteralTypes::String => "LITERAL{STRING}",
            },
            TkType::Identifier => "ID",
        };
        
        write!(f, "Token: {: <40} {: <60} {:0>3} {:0>3}", tp, self.lexema, self.row, self.col)
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

fn get_string_literal(line: &Vec<char>, delimiter: char, col: usize, row: usize) -> Option<(Token, usize)> {
    if line[col] != delimiter {
        return None;
    }
    
    let mut lexema = String::default();
    lexema.push(delimiter);
    let mut icol = col + 1;
    
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
            c => {
                icol += 1;

                if c == delimiter {
                    break;
                }
            }
        }
    }

    let token = Token::new(TkType::Literal(LiteralTypes::String), lexema, row, col);
    Some((token, icol))
}

fn get_int_literal(line: &Vec<char>, col: usize, row: usize) -> Option<(Token, usize)> {
    let mut icol = col;

    if !line[icol].is_numeric() {
        return None;
    }
    
    let mut lexema = String::default();
    lexema.push(line[col]);
    icol += 1;

    loop {
        let c = line[icol];

        if c.is_numeric() {
            lexema.push(c);
        } else if char_acts_as_separator(c) {
            break;
        } else if char_defines_operator(c) {
            break;
        } else {
            // TODO tratar os erros igual gente decente
            panic!("Invalid literal at: row {}, col {}", row, col);
        }

        icol += 1;
    }

    if let Ok(_) = lexema.parse::<i32>() {
        let token = Token::new(TkType::Literal(LiteralTypes::Int), lexema, row, col);
        Some((token, icol))
    } else {
        None
    }
}

fn get_float_literal(line: &Vec<char>, col: usize, row: usize) -> Option<(Token, usize)> {
    let mut icol = col;

    if line[icol] != '.' && !line[icol].is_numeric() {
        return None;
    }
    
    let mut lexema = String::default();
    lexema.push(line[icol]);
    let mut had_dot = line[icol] == '.';
    icol += 1;

    loop {
        let c = line[icol];

        if c.is_numeric() {
            lexema.push(c);
        } else if c == '.' && !had_dot {
            lexema.push('.');
            had_dot = true;
        } else if char_acts_as_separator(c) {
            break;
        } else if char_defines_operator(c) {
            break;
        } else {
            // TODO tratar os erros igual gente decente
            panic!("Invalid literal at: row {}, col {}", row, col);
        }

        icol += 1;
    }

    if had_dot {
        let token = Token::new(TkType::Literal(LiteralTypes::Float), lexema, row, col);
        Some((token, icol))
    } else {
        None
    }
}

fn get_operator(line: &Vec<char>, col: usize, row: usize) -> Option<(Token, usize)> {
    let mut icol = col;

    if !char_defines_operator(line[icol]) {
        return None;
    }

    let mut lexema = String::default();
    lexema.push(line[col]);
    icol += 1;

    loop {
        let c = line[icol];

        if char_defines_operator(c) {
            lexema.push(c);
        } else {
            break;
        }

        if !OPERATORS.iter().any(|op| op.0 == lexema) {
            lexema.pop();
            break;
        }

        icol += 1;
    }

    if !lexema.is_empty() {
        let id = match OPERATORS.iter().find(|op| op.0 == lexema) {
            Some(op) => op.1,
            // TODO tratar os erros igual gente decente
            None => panic!("Invalid operation at: row {}, col {}", row, col)
        };

        let token = Token::new(TkType::Operator(id), lexema, row, col);
        Some((token, icol))
    } else {
        None
    }
}

fn get_reserved_word_or_identifier(line: &Vec<char>, col: usize, row: usize) -> Option<(Token, usize)> {
    let mut icol = col;

    if !line[icol].is_ascii_alphabetic() {
        return None;
    }
    
    let mut lexema = String::default();
    lexema.push(line[icol]);
    icol += 1;

    loop {
        let c = line[icol];

        if char_defines_operator(c) {
            break;
        }

        if char_acts_as_separator(c) {
            break;
        }

        icol += 1
    }

    let tp = match RESERVED_WORDS.iter().find(|i| i.0 == lexema) {
        Some(i) => TkType::ReservedWord(i.1),
        None => TkType::Identifier,
    };
        
    let token = Token::new(tp, lexema, row, col);
    Some((token, icol))
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

        loop {
            match line[col] {
                ' ' => {
                    col += 1;
                },
                '\t' => {
                    col += 1;
                },
                '\n' => {
                    break;
                },
                '#' => {
                    break;
                },
                '\'' => {
                    // TODO bloco de comentario
                    match get_string_literal(&line, '\'', col, row) {
                        Some((token, icol)) => {
                            tokens.push(token);
                            col = icol;
                        }
                        // TODO errors
                        None => panic!("Invalid Token at: row {}, col {}", row, col)
                    }
                },
                '"' => {
                    // TODO bloco de comentario
                    match get_string_literal(&line, '"', col, row) {
                        Some((token, icol)) => {
                            tokens.push(token);
                            col = icol;
                        }
                        // TODO errors
                        None => panic!("Invalid Token at: row {}, col {}", row, col)
                    }
                },
                _ => {
                    if let Some((token, icol)) = get_float_literal(&line, col, row) {
                        tokens.push(token);
                        col = icol;
                        continue;
                    }

                    if let Some((token, icol)) = get_int_literal(&line, col, row) {
                        tokens.push(token);
                        col = icol;
                        continue;
                    }

                    if let Some((token, icol)) = get_operator(&line, col, row) {
                        tokens.push(token);
                        col = icol;
                        continue;
                    }

                    if let Some((token, icol)) = get_reserved_word_or_identifier(&line, col, row) {
                        tokens.push(token);
                        col = icol;
                        continue;
                    }

                    // TODO errors
                    panic!("Unidentified Token at: row {}, col {}", row, col)
                }
            }
        }

        row += 1;
        
        buf = l.into_bytes();
        buf.clear();
    }
    
    Ok(tokens)
}

fn dump_tokens(tokens: &Vec<Token>, filename: &str) -> std::io::Result<()> {
    let mut out = BufWriter::new(File::create(filename)?);

    for token in tokens.iter() {
        let line = format!("{}\n", token);
        out.write_all(line.as_bytes())?;
    }
    
    Ok(())
}

fn run(src_file: &str, out_dir: &str) -> std::io::Result<()> {
    let tokens = generate_tokens(src_file)?;

    let mut filename = out_dir.to_owned();
    filename.push_str("/out.lex");
    dump_tokens(&tokens, &filename)?;

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
