extern crate clap;

use clap::{Arg, App};

use std::fs::File;
// use std::collections::HashMap;
use std::io::{BufRead, BufReader};

static reserved_words: [[&'static str; 2]; 33] = [
    ["and", "TK.AND"],
    ["as", "TK.AS"],
    ["assert", "TK.ASSERT"],
    ["break", "TK.BREAK"],
    ["class", "TK.CLASS"],
    ["continue", "TK.CONTINUE"],
    ["def", "TK.DEF"],
    ["del", "TK.DEL"],
    ["elif", "TK.ELIF"],
    ["else", "TK.ELSE"],
    ["except", "TK.EXCEPT"],
    ["exec", "TK.EXEC"],
    ["finally", "TK.FINALLY"],
    ["for", "TK.FOR"],
    ["from", "TK.FROM"],
    ["global", "TK.GLOBAL"],
    ["if", "TK.IF"],
    ["import", "TK.IMPORT"],
    ["in", "TK.IN"],
    ["is", "TK.IS"],
    ["lambda", "TK.LAMBDA"],
    ["none", "TK.NONE"],
    ["nonlocal", "TK.NONLOCAL"],
    ["not", "TK.NOT"],
    ["or", "TK.OR"],
    ["pass", "TK.PASS"],
    ["print", "TK.PRINT"],
    ["raise", "TK.RAISE"],
    ["return", "TK.RETURN"],
    ["try", "TK.TRY"],
    ["while", "TK.WHILE"],
    ["with", "TK.WITH"],
    ["yield", "TK.YIELD"]
];

// fn get_reserved_words() -> HashMap<&'static str, &'static str> {
//     let mut rwords = HashMap::new();
//     rwords.insert("and", "TK.AND");
//     rwords.insert("as", "TK.AS");
//     rwords.insert("assert", "TK.ASSERT");
//     rwords.insert("break", "TK.BREAK");
//     rwords.insert("class", "TK.CLASS");
//     rwords.insert("continue", "TK.CONTINUE");
//     rwords.insert("def", "TK.DEF");
//     rwords.insert("del", "TK.DEL");
//     rwords.insert("elif", "TK.ELIF");
//     rwords.insert("else", "TK.ELSE");
//     rwords.insert("except", "TK.EXCEPT");
//     rwords.insert("exec", "TK.EXEC");
//     rwords.insert("finally", "TK.FINALLY");
//     rwords.insert("for", "TK.FOR");
//     rwords.insert("from", "TK.FROM");
//     rwords.insert("global", "TK.GLOBAL");
//     rwords.insert("if", "TK.IF");
//     rwords.insert("import", "TK.IMPORT");
//     rwords.insert("in", "TK.IN");
//     rwords.insert("is", "TK.IS");
//     rwords.insert("lambda", "TK.LAMBDA");
//     rwords.insert("none", "TK.NONE");
//     rwords.insert("nonlocal", "TK.NONLOCAL");
//     rwords.insert("not", "TK.NOT");
//     rwords.insert("or", "TK.OR");
//     rwords.insert("pass", "TK.PASS");
//     rwords.insert("print", "TK.PRINT");
//     rwords.insert("raise", "TK.RAISE");
//     rwords.insert("return", "TK.RETURN");
//     rwords.insert("try", "TK.TRY");
//     rwords.insert("while", "TK.WHILE");
//     rwords.insert("with", "TK.WITH");
//     rwords.insert("yield", "TK.YIELD");
//     rwords
// }

static operators: [[&'static str; 2]; 44] = [
    ["+", "TK.MAIS"],
    ["-", "TK.MENOS"],
    ["*", "TK.VEZES"],
    ["/", "TK.BARRA"],
    ["%", "TK.PORCENTO"],
    ["&", "TK.ECOMERCIAL"],
    ["|", "TK.PIPE"],
    ["^", "TK.CIRCUMFLEXO"],
    ["~", "TK.TIL"],
    ["<", "TK.MENOR"],
    [">", "TK.MAIOR"],
    ["(", "TK.PARENTESES_ESQUERDO"],
    [")", "TK.PARENTESES_DIREITO"],
    ["[", "TK.COLCHETES_ESQUERDO"],
    ["]", "TK.COLCHETES_DIREITO"],
    ["{", "TK.CHAVES_ESQUERDA"],
    ["}", "TK.CHAVES_DIREITA"],
    [",", "TK.VIRGULA"],
    [":", "TK.DOIS_PONTOS"],
    [".", "TK.PONTO"],
    [";", "TK.PONTO_VIRGULA"],
    ["@", "TK.ARROBA"],
    ["=", "TK.IGUAL"],
    ["**", "TK.NOME_PARAMETRO"],
    ["//", "TK.BARRA_DUPLA"],
    ["<<", "TK.SHIFT_LEFT"],
    [">>", "TK.SHIFT_RIGHT"],
    ["<=", "TK.MENOR_IGUAL"],
    [">=", "TK.MAIOR_IGUAL"],
    ["==", "TK.IGUAL_IGUAL"],
    ["!=", "TK.DIFERENTE"],
    ["+=", "TK.MAIS_IGUAL"],
    ["-=", "TK.MENOS_IGUAL"],
    ["*=", "TK.VEZES_IGUAL"],
    ["/=", "TK.BARRA_IGUAL"],
    ["//=", "TK.BARRA_DUPLA_IGUAL"],
    ["%=", "TK.PORCENTO_IGUAL"],
    ["@=", "TK.ARROBA_IGUAL"],
    ["&=", "TK.ECOMERCIAL_IGUAL"],
    ["|=", "TK.PIPE_IGUAL"],
    ["^=", "TK.CIRCUMFLEXO_IGUAL"],
    [">>=", "TK.SHIFT_RIGHT_IGUAL"],
    ["<<=", "TK.SHIFT_LEFT_IGUAL"],
    ["**=", "TK.DUPLO_ASTERISCO_IGUAL"]
];

// fn get_simple_operators() -> HashMap<char, &'static str> {
//     let mut rwords = HashMap::new();
//     rwords.insert('+', "TK.MAIS");
//     rwords.insert('-', "TK.MENOS");
//     rwords.insert('*', "TK.VEZES");
//     rwords.insert('/', "TK.BARRA");
//     rwords.insert('%', "TK.PORCENTO");
//     rwords.insert('&', "TK.ECOMERCIAL");
//     rwords.insert('|', "TK.PIPE");
//     rwords.insert('^', "TK.CIRCUMFLEXO");
//     rwords.insert('~', "TK.TIL");
//     rwords.insert('<', "TK.MENOR");
//     rwords.insert('>', "TK.MAIOR");
//     rwords.insert('(', "TK.PARENTESES_ESQUERDO");
//     rwords.insert(')', "TK.PARENTESES_DIREITO");
//     rwords.insert('[', "TK.COLCHETES_ESQUERDO");
//     rwords.insert(']', "TK.COLCHETES_DIREITO");
//     rwords.insert('{', "TK.CHAVES_ESQUERDA");
//     rwords.insert('}', "TK.CHAVES_DIREITA");
//     rwords.insert(',', "TK.VIRGULA");
//     rwords.insert(':', "TK.DOIS_PONTOS");
//     rwords.insert('.', "TK.PONTO");
//     rwords.insert(';', "TK.PONTO_VIRGULA");
//     rwords.insert('@', "TK.ARROBA");
//     rwords.insert('=', "TK.IGUAL");
//     rwords
// }

// fn get_composed_operators() -> HashMap<&'static str, &'static str> {
//     let mut rwords = HashMap::new();
//     rwords.insert("**", "TK.NOME_PARAMETRO");
//     rwords.insert("//", "TK.BARRA_DUPLA");
//     rwords.insert("<<", "TK.SHIFT_LEFT");
//     rwords.insert(">>", "TK.SHIFT_RIGHT");
//     rwords.insert("<=", "TK.MENOR_IGUAL");
//     rwords.insert(">=", "TK.MAIOR_IGUAL");
//     rwords.insert("==", "TK.IGUAL_IGUAL");
//     rwords.insert("!=", "TK.DIFERENTE");
//     rwords.insert("+=", "TK.MAIS_IGUAL");
//     rwords.insert("-=", "TK.MENOS_IGUAL");
//     rwords.insert("*=", "TK.VEZES_IGUAL");
//     rwords.insert("/=", "TK.BARRA_IGUAL");
//     rwords.insert("//=", "TK.BARRA_DUPLA_IGUAL");
//     rwords.insert("%=", "TK.PORCENTO_IGUAL");
//     rwords.insert("@=", "TK.ARROBA_IGUAL");
//     rwords.insert("&=", "TK.ECOMERCIAL_IGUAL");
//     rwords.insert("|=", "TK.PIPE_IGUAL");
//     rwords.insert("^=", "TK.CIRCUMFLEXO_IGUAL");
//     rwords.insert(">>=", "TK.SHIFT_RIGHT_IGUAL");
//     rwords.insert("<<=", "TK.SHIFT_LEFT_IGUAL");
//     rwords.insert("**=", "TK.DUPLO_ASTERISCO_IGUAL");
//     rwords
// }

// static comment_ln_delimiter: &'static str = "#";

// fn get_comment_line_delimiters() -> Vec<&'static str> {
//     vec!["#"]
// }

// static comment_block_delimiters: [&'static str; 2] = ["'''", "\"\"\""];

// fn get_comment_block_delimiters() -> Vec<&'static str> {
//     vec!["'''", "\"\"\""]
// }

// static string_literal_delimiters: [&'static str; 2] = ["'", "\""];

// fn get_string_literal_delimiters() -> Vec<&'static str> {
//     vec!["'", "\""]
// }

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
    row: u32,
    col: u32
}

impl Token {
    fn new(tk_type: TkType, lexema: &str, row: u32, col: u32) -> Token {
        Token {
            tk_type: tk_type,
            lexema: lexema.to_owned(),
            row: row,
            col: col
        }
    }
}

fn get_token_type(lexema: &str) -> TkType {
    if let Some(i) = operators.iter().position(|o| o[0] == lexema) {
        TkType::Operator(operators[i][1])
    } else if let Some(i) = operators.iter().position(|w| w[0] == lexema) {
        TkType::ReservedWord(operators[i][1])
    } else if let Ok(_) = lexema.parse::<i32>() {
        TkType::Literal(LiteralTypes::Int)
    } else if let Ok(_) = lexema.parse::<f32>() {
        TkType::Literal(LiteralTypes::Float)
    } else {
        match lexema.chars().next() {
            Some('"') => TkType::Literal(LiteralTypes::String),
            Some('\'') => TkType::Literal(LiteralTypes::String),
            _ => TkType::Identifier
        }
    }
}

fn run(src_file: &str, out_dir: &str) -> std::io::Result<()> {
    // let reserved_words = get_reserved_words();
    // let simple_operators = get_simple_operators();
    // let composed_operators = get_composed_operators();
    // let comment_line_delimiters = get_comment_line_delimiters();
    // let comment_block_delimiters = get_comment_block_delimiters();
    // let string_literal_delimiters = get_string_literal_delimiters();

    let mut tokens = Vec::new();
    let mut ind = Vec::new();
    let mut currtk = String::default();
    
    println!("Tokenazing: \"{}\"", src_file);
    let mut src = BufReader::new(File::open(src_file).expect("Could not open source file"));
    let mut buf = Vec::<u8>::new();

    let mut row = 0;

    while src.read_until(b'\n', &mut buf).expect("could not read line") != 0 {
        let l = String::from_utf8(buf).expect("source file is not UTF-8");

        let mut gen = l.chars();
        let mut curc;

        match gen.next() {
            Some(c) => curc = c,
            None => break
        }

        let mut col = 0;
        let mut curind = 0;

        loop {
            match curc {
                ' ' => curind += 1,
                '\t' => curind += 1,
                _ => break
            }

            match gen.next() {
                Some(c) => curc = c,
                // FIXME tecnicamente e valido, so nao deveria gerar indentacao, pensar em uma maneira de sair dos dois loops
                None => panic!("Invalid EOF at: row {}, col {}", row, col)
            }

            col += 1;
        }

        // Se for so uma linha com espacos/tabs, ignorar a indentacao
        if curc != '\n' {
            let tot_ind = ind.iter().sum();

            if curind < tot_ind {
                let difference = tot_ind - curind;

                if Some(&difference) != ind.last() {
                    panic!("Invalid indentation at: row {}, col {}", row, col);
                }
                
                ind.pop();
                tokens.push(Token::new(TkType::Dedentation, "", row, col));
            } else if curind > tot_ind {
                let difference = curind - tot_ind;
                ind.push(difference);
                tokens.push(Token::new(TkType::Indentaion, "", row, col));
            }
        }

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
                    
                    
                    
                    
                    
                    
                    let lisop = operators.iter().any(|o| o[0].chars().next().unwrap() == c);

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
