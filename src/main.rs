extern crate clap;

use clap::{Arg, App};

use std::fmt;
use std::fs::File;
use std::io::{BufRead, Write, BufReader, BufWriter};
use std::collections::HashMap;

enum CompilationError {
    ParseError(String),
    SintaxError(String)
}

static RESERVED_WORDS: [(&'static str, &'static str); 32] = [
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
    // ("print", "RWORD{PRINT}"),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum LiteralTypes {
    Int = 1,
    Float = 2,
    String = 4
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TkType {
    Indentaion,
    Dedentation,
    ReservedWord(&'static str),
    Operator(&'static str),
    Literal(LiteralTypes),
    Identifier,
    EOS,
    END
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
            TkType::EOS => "EOS",
            TkType::END => "END"
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
                } else {
                    icol += 1;
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
            if lexema == "." {
                return None;
            }
            
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

    if !line[icol].is_ascii_alphabetic() && line[icol] != '_' {
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

        lexema.push(c);
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
    let mut scope = Vec::<char>::new();

    let mut row: usize = 0;

    while src.read_until(b'\n', &mut buf)? != 0 {
        // TODO tratar os erros igual gente decente
        let l = String::from_utf8(buf).expect("source file is not UTF-8");
        
        let mut line: Vec<char> = l.chars().collect();

        // FIXME por favor remover essa gambiarra
        if line.last() != Some(&'\n') {
            line.push('\n');
        }

        // Indentacao
        let line_indentation = get_line_indentation(&line);

        // Ignora se for uma linha em branco ou estiver dentro de um escopo
        if line.len() >= line_indentation && line[line_indentation] != '\n' && scope.len() == 0 {
            // TODO rename
            let tot_ind = ind.iter().sum();

            if line_indentation < tot_ind {
                let mut difference = tot_ind - line_indentation;

                while difference > 0 {
                    let last = match ind.last() {
                        Some(&i) => i,
                        None => panic!("Invalid indentation at: row {}, col {}", row, line_indentation),
                    };
                    
                    if difference < last {
                        // TODO tratar os erros igual gente decente
                        panic!("Invalid indentation at: row {}, col {}", row, line_indentation);
                    }
                
                    ind.pop();
                    difference -= last;
                    tokens.push(Token::new(TkType::Dedentation, "".to_owned(), row, 0));
                }
            } else if line_indentation > tot_ind {
                let difference = line_indentation - tot_ind;
                ind.push(difference);
                tokens.push(Token::new(TkType::Indentaion, "".to_owned(), row, 0));
            }
        }

        let mut col = line_indentation;
        let mut eos = false;

        loop {
            match line[col] {
                ' ' => {
                    col += 1;
                },
                '\t' => {
                    col += 1;
                },
                '\n' => {
                    if scope.len() == 0 && eos {
                        tokens.push(Token::new(TkType::EOS, "".to_string(), row, col));
                    }
                    
                    break;
                },
                '#' => {
                    break;
                },
                '(' => {
                    let (_, optype) = OPERATORS[11];
                    scope.push(')');
                    tokens.push(Token::new(TkType::Operator(optype), "(".to_string(), row, col));
                    col += 1;
                },
                ')' => {
                    if let Some(&s) = scope.last() {
                        if s != ')' {
                            panic!("Expected '{}' but found ')' at: row {}, col {}", s, row, col)
                        }
                    } else {
                        panic!("Unexpected ')' at: row {}, col {}", row, col)
                    }
                    
                    let (_, optype) = OPERATORS[12];
                    scope.pop();
                    tokens.push(Token::new(TkType::Operator(optype), ")".to_string(), row, col));
                    col += 1;
                },
                '[' => {
                    let (_, optype) = OPERATORS[13];
                    scope.push(']');
                    tokens.push(Token::new(TkType::Operator(optype), "[".to_string(), row, col));
                    col += 1;
                },
                ']' => {
                    if let Some(&s) = scope.last() {
                        if s != ']' {
                            panic!("Expected '{}' but found ']' at: row {}, col {}", s, row, col)
                        }
                    } else {
                        panic!("Unexpected ']' at: row {}, col {}", row, col)
                    }
                    
                    let (_, optype) = OPERATORS[14];
                    scope.pop();
                    tokens.push(Token::new(TkType::Operator(optype), "]".to_string(), row, col));
                    col += 1;
                },
                '{' => {
                    let (_, optype) = OPERATORS[15];
                    scope.push('}');
                    tokens.push(Token::new(TkType::Operator(optype), "{".to_string(), row, col));
                    col += 1;
                },
                '}' => {
                    if let Some(&s) = scope.last() {
                        if s != '}' {
                            panic!("Expected '{}' but found '}}' at: row {}, col {}", s, row, col)
                        }
                    } else {
                        panic!("Unexpected '}}' at: row {}, col {}", row, col)
                    }
                    
                    let (_, optype) = OPERATORS[12];
                    scope.pop();
                    tokens.push(Token::new(TkType::Operator(optype), "}".to_string(), row, col));
                    col += 1;
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

            eos = true;
        }

        row += 1;
        
        buf = l.into_bytes();
        buf.clear();
    }

    for _ in ind.iter() {
        tokens.push(Token::new(TkType::Dedentation, "".to_owned(), row, 0));
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum PossibleStates {
    STATEMENT_LIST,
    STATEMENT,
    STATEMENT_LIST_E,
    SCOPE,
    ID_LIST,
    EXPRESSION_LIST,
    EXPRESSION_STATEMENT,
    EXPRESSION,
    ELSE_STATEMENT,
    ID_LIST_D,
    EXPRESSION_STATEMENTL,
    ASSIGNMENT_EXPRESSIONL,
    ID_OR_FCALL_D,
    ID_OR_FCALL,
    EXPRESSION_LIST_d,
    ASSIGNMENT_EXPRESSION,
    EXPRESSION_A,
    EXPRESSIONL,
    EXPRESSION_B,
    EXPRESSION_AL,
    EXPRESSION_C,
    EXPRESSION_BL,
    EXPRESSION_D,
    EXPRESSION_CL,
    Terminal(TkType),
    NOP
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct HmIndex {
    state: PossibleStates,
    token: TkType
}

fn generate_lookup_table() -> HashMap<HmIndex, Vec<PossibleStates>> {
    let mut hm = HashMap::new();

    let prod1 = vec![PossibleStates::STATEMENT, PossibleStates::STATEMENT_LIST_E];
    let prod2 = vec![PossibleStates::STATEMENT, PossibleStates::STATEMENT_LIST_E];
    let prod3 = vec![PossibleStates::NOP];
    let prod4 = vec![PossibleStates::Terminal(TkType::Indentaion), PossibleStates::STATEMENT_LIST, PossibleStates::Terminal(TkType::Dedentation)];
    let prod5 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{DEF}")), PossibleStates::Terminal(TkType::Identifier), PossibleStates::Terminal(TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")), PossibleStates::ID_LIST, PossibleStates::Terminal(TkType::Operator("OPERATOR{PARENTESES_DIREITO}")), PossibleStates::Terminal(TkType::Operator("OPERATOR{DOIS_PONTOS}")), PossibleStates::Terminal(TkType::EOS), PossibleStates::SCOPE];
    let prod6 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{BREAK}")), PossibleStates::Terminal(TkType::EOS)];
    let prod7 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{CONTINUE}")), PossibleStates::Terminal(TkType::EOS)];
    let prod8 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{RETURN}")), PossibleStates::EXPRESSION_LIST, PossibleStates::Terminal(TkType::EOS)];
    let prod9 = vec![PossibleStates::EXPRESSION_STATEMENT, PossibleStates::Terminal(TkType::EOS)];
    let prod10 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{FOR}")), PossibleStates::Terminal(TkType::Identifier), PossibleStates::Terminal(TkType::ReservedWord("RWORD{IN}")), PossibleStates::Terminal(TkType::Identifier), PossibleStates::Terminal(TkType::Operator("OPERATOR{DOIS_PONTOS}")), PossibleStates::Terminal(TkType::EOS), PossibleStates::SCOPE];
    let prod11 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{WHILE}")), PossibleStates::EXPRESSION, PossibleStates::Terminal(TkType::Operator("OPERATOR{DOIS_PONTOS}")), PossibleStates::Terminal(TkType::EOS), PossibleStates::SCOPE];
    let prod12 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{IF}")), PossibleStates::EXPRESSION, PossibleStates::Terminal(TkType::Operator("OPERATOR{DOIS_PONTOS}")), PossibleStates::Terminal(TkType::EOS), PossibleStates::SCOPE, PossibleStates::ELSE_STATEMENT];
    let prod13 = vec![PossibleStates::Terminal(TkType::Identifier), PossibleStates::ID_LIST_D];
    let prod14 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{VIRGULA}")), PossibleStates::Terminal(TkType::Identifier), PossibleStates::ID_LIST_D];
    let prod15 = vec![PossibleStates::NOP];
    let prod16 = vec![PossibleStates::Terminal(TkType::Identifier), PossibleStates::EXPRESSION_STATEMENTL];
    let prod17 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")), PossibleStates::EXPRESSION_LIST, PossibleStates::Terminal(TkType::Operator("OPERATOR{PARENTESES_DIREITO}"))];
    let prod18 = vec![PossibleStates::ASSIGNMENT_EXPRESSIONL];
    let prod19 = vec![PossibleStates::Terminal(TkType::Identifier), PossibleStates::ID_OR_FCALL_D];
    let prod20 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")), PossibleStates::EXPRESSION_LIST, PossibleStates::Terminal(TkType::Operator("OPERATOR{PARENTESES_DIREITO}"))];
    let prod21 = vec![PossibleStates::NOP];
    let prod22 = vec![PossibleStates::EXPRESSION, PossibleStates::EXPRESSION_LIST_d];
    let prod23 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{VIRGULA}")), PossibleStates::EXPRESSION, PossibleStates::EXPRESSION_LIST_d];
    let prod24 = vec![PossibleStates::NOP];
    let prod25 = vec![PossibleStates::Terminal(TkType::Identifier), PossibleStates::ASSIGNMENT_EXPRESSIONL];
    let prod26 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{IGUAL}")), PossibleStates::EXPRESSION];
    let prod27 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{MAIS_IGUAL}")), PossibleStates::EXPRESSION];
    let prod28 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{MENOS_IGUAL}")), PossibleStates::EXPRESSION];
    let prod29 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{VEZES_IGUAL}")), PossibleStates::EXPRESSION];
    let prod30 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{BARRA_IGUAL}")), PossibleStates::EXPRESSION];
    let prod31 = vec![PossibleStates::EXPRESSION_A, PossibleStates::EXPRESSIONL];
    let prod32 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{AND}")), PossibleStates::EXPRESSION];
    let prod33 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{OR}")), PossibleStates::EXPRESSION];
    let prod34 = vec![PossibleStates::NOP];
    let prod35 = vec![PossibleStates::EXPRESSION_B, PossibleStates::EXPRESSION_AL];
    let prod36 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{NOT}")), PossibleStates::EXPRESSION_B];
    let prod37 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{IGUAL_IGUAL}")), PossibleStates::EXPRESSION_A];
    let prod38 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{DIFERENTE}")), PossibleStates::EXPRESSION_A];
    let prod39 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{MENOR}")), PossibleStates::EXPRESSION_A];
    let prod40 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{MENOR_IGUAL}")), PossibleStates::EXPRESSION_A];
    let prod41 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{MAIOR}")), PossibleStates::EXPRESSION_A];
    let prod42 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{MAIOR_IGUAL}")), PossibleStates::EXPRESSION_A];
    let prod43 = vec![PossibleStates::NOP];
    let prod44 = vec![PossibleStates::EXPRESSION_C, PossibleStates::EXPRESSION_BL];
    let prod45 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{VEZES}")), PossibleStates::EXPRESSION_B];
    let prod46 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{BARRA}")), PossibleStates::EXPRESSION_B];
    let prod47 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{CIRCUMFLEXO}")), PossibleStates::EXPRESSION_B];
    let prod48 = vec![PossibleStates::NOP];
    let prod49 = vec![PossibleStates::EXPRESSION_D, PossibleStates::EXPRESSION_CL];
    let prod50 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{MAIS}")), PossibleStates::EXPRESSION_C];
    let prod51 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{MENOS}")), PossibleStates::EXPRESSION_C];
    let prod52 = vec![PossibleStates::NOP];
    let prod53 = vec![PossibleStates::Terminal(TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")), PossibleStates::EXPRESSION, PossibleStates::Terminal(TkType::Operator("OPERATOR{PARENTESES_DIREITO}"))];
    let prod54 = vec![PossibleStates::ID_OR_FCALL];
    let prod55 = vec![PossibleStates::Terminal(TkType::Literal(LiteralTypes::Int))];
    let prod56 = vec![PossibleStates::Terminal(TkType::Literal(LiteralTypes::Float))];
    let prod57 = vec![PossibleStates::Terminal(TkType::Literal(LiteralTypes::String))];
    let prod58 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{ELSE}")), PossibleStates::Terminal(TkType::Operator("OPERATOR{DOIS_PONTOS}")), PossibleStates::Terminal(TkType::EOS), PossibleStates::SCOPE];
    let prod59 = vec![PossibleStates::Terminal(TkType::ReservedWord("RWORD{ELIF}")), PossibleStates::EXPRESSION, PossibleStates::Terminal(TkType::Operator("OPERATOR{DOIS_PONTOS}")), PossibleStates::Terminal(TkType::EOS), PossibleStates::SCOPE, PossibleStates::ELSE_STATEMENT];
    let prod60 = vec![PossibleStates::NOP];

    // dedent
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::Dedentation
    }, prod3.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::Dedentation
    }, prod60.clone());

    // indent
    hm.insert(HmIndex {
        state: PossibleStates::SCOPE,
        token: TkType::Indentaion
    }, prod4.clone());

    // def
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST,
        token: TkType::ReservedWord("RWORD{DEF}")
    }, prod1.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT,
        token: TkType::ReservedWord("RWORD{DEF}")
    }, prod5.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::ReservedWord("RWORD{DEF}")
    }, prod2.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::ReservedWord("RWORD{DEF}")
    }, prod60.clone());

    // id
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST,
        token: TkType::Identifier
    }, prod1.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT,
        token: TkType::Identifier
    }, prod9.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::Identifier
    }, prod2.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ID_LIST,
        token: TkType::Identifier
    }, prod13.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST,
        token: TkType::Identifier
    }, prod22.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_STATEMENT,
        token: TkType::Identifier
    }, prod16.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION,
        token: TkType::Identifier
    }, prod31.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::Identifier
    }, prod60.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL,
        token: TkType::Identifier
    }, prod19.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ASSIGNMENT_EXPRESSION,
        token: TkType::Identifier
    }, prod25.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_A,
        token: TkType::Identifier
    }, prod35.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_B,
        token: TkType::Identifier
    }, prod44.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_C,
        token: TkType::Identifier
    }, prod49.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_D,
        token: TkType::Identifier
    }, prod54.clone());

    // (
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST,
        token: TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")
    }, prod22.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION,
        token: TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")
    }, prod31.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_STATEMENTL,
        token: TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")
    }, prod17.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")
    }, prod20.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_A,
        token: TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")
    }, prod35.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_B,
        token: TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")
    }, prod44.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_B,
        token: TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")
    }, prod49.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_D,
        token: TkType::Operator("OPERATOR{PARENTESES_ESQUERDO}")
    }, prod53.clone());

    // )
    hm.insert(HmIndex {
        state: PossibleStates::ID_LIST_D,
        token: TkType::Operator("OPERATOR{PARENTESES_DIREITO}")
    }, prod15.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{PARENTESES_DIREITO}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST_d,
        token: TkType::Operator("OPERATOR{PARENTESES_DIREITO}")
    }, prod24.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSIONL,
        token: TkType::Operator("OPERATOR{PARENTESES_DIREITO}")
    }, prod34.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{PARENTESES_DIREITO}")
    }, prod43.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{PARENTESES_DIREITO}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{PARENTESES_DIREITO}")
    }, prod52.clone());

    // :
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{DOIS_PONTOS}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSIONL,
        token: TkType::Operator("OPERATOR{DOIS_PONTOS}")
    }, prod34.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{DOIS_PONTOS}")
    }, prod43.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{DOIS_PONTOS}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{DOIS_PONTOS}")
    }, prod52.clone());

    // EOS
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::EOS
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST_d,
        token: TkType::EOS
    }, prod24.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSIONL,
        token: TkType::EOS
    }, prod34.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::EOS
    }, prod43.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::EOS
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::EOS
    }, prod52.clone());

    // ,
    hm.insert(HmIndex {
        state: PossibleStates::ID_LIST_D,
        token: TkType::Operator("OPERATOR{VIRGULA}")
    }, prod14.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{VIRGULA}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST_d,
        token: TkType::Operator("OPERATOR{VIRGULA}")
    }, prod23.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSIONL,
        token: TkType::Operator("OPERATOR{VIRGULA}")
    }, prod34.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{VIRGULA}")
    }, prod43.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{VIRGULA}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{VIRGULA}")
    }, prod52.clone());

    // and
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::ReservedWord("RWORD{AND}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSIONL,
        token: TkType::ReservedWord("RWORD{AND}")
    }, prod33.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::ReservedWord("RWORD{AND}")
    }, prod43.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::ReservedWord("RWORD{AND}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::ReservedWord("RWORD{AND}")
    }, prod52.clone());

    // or
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::ReservedWord("RWORD{OR}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSIONL,
        token: TkType::ReservedWord("RWORD{OR}")
    }, prod33.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::ReservedWord("RWORD{OR}")
    }, prod43.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::ReservedWord("RWORD{OR}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::ReservedWord("RWORD{OR}")
    }, prod52.clone());

    // ==
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{IGUAL_IGUAL}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{IGUAL_IGUAL}")
    }, prod37.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{IGUAL_IGUAL}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{IGUAL_IGUAL}")
    }, prod52.clone());

    // !=
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{DIFERENTE}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{DIFERENTE}")
    }, prod38.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{DIFERENTE}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{DIFERENTE}")
    }, prod52.clone());

    // <
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{MENOR}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{MENOR}")
    }, prod39.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{MENOR}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{MENOR}")
    }, prod52.clone());

    // <=
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{MENOR_IGUAL}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{MENOR_IGUAL}")
    }, prod40.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{MENOR_IGUAL}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{MENOR_IGUAL}")
    }, prod52.clone());

    // >
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{MAIOR}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{MAIOR}")
    }, prod41.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{MAIOR}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{MAIOR}")
    }, prod52.clone());

    // >=
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{MAIOR_IGUAL}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_AL,
        token: TkType::Operator("OPERATOR{MAIOR_IGUAL}")
    }, prod42.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{MAIOR_IGUAL}")
    }, prod48.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{MAIOR_IGUAL}")
    }, prod52.clone());

    // *
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{VEZES}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{VEZES}")
    }, prod45.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{VEZES}")
    }, prod52.clone());

    // /
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{BARRA}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{BARRA}")
    }, prod46.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{BARRA}")
    }, prod52.clone());

    // ^
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{CIRCUMFLEXO}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_BL,
        token: TkType::Operator("OPERATOR{CIRCUMFLEXO}")
    }, prod47.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{CIRCUMFLEXO}")
    }, prod52.clone());

    // +
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{MAIS}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{MAIS}")
    }, prod50.clone());

    // -
    hm.insert(HmIndex {
        state: PossibleStates::ID_OR_FCALL_D,
        token: TkType::Operator("OPERATOR{MENOS}")
    }, prod21.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_CL,
        token: TkType::Operator("OPERATOR{MENOS}")
    }, prod51.clone());

    // =
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_STATEMENTL,
        token: TkType::Operator("OPERATOR{IGUAL}")
    }, prod18.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ASSIGNMENT_EXPRESSIONL,
        token: TkType::Operator("OPERATOR{IGUAL}")
    }, prod26.clone());

    // +=
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_STATEMENTL,
        token: TkType::Operator("OPERATOR{MAIS_IGUAL}")
    }, prod18.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ASSIGNMENT_EXPRESSIONL,
        token: TkType::Operator("OPERATOR{MAIS_IGUAL}")
    }, prod27.clone());

    // -=
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_STATEMENTL,
        token: TkType::Operator("OPERATOR{MENOS_IGUAL}")
    }, prod18.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ASSIGNMENT_EXPRESSIONL,
        token: TkType::Operator("OPERATOR{MENOS_IGUAL}")
    }, prod28.clone());

    // *=
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_STATEMENTL,
        token: TkType::Operator("OPERATOR{VEZES_IGUAL}")
    }, prod18.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ASSIGNMENT_EXPRESSIONL,
        token: TkType::Operator("OPERATOR{VEZES_IGUAL}")
    }, prod29.clone());

    // /=
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_STATEMENTL,
        token: TkType::Operator("OPERATOR{BARRA_IGUAL}")
    }, prod18.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ASSIGNMENT_EXPRESSIONL,
        token: TkType::Operator("OPERATOR{BARRA_IGUAL}")
    }, prod30.clone());

    // not
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST,
        token: TkType::ReservedWord("RWORD{NOT}")
    }, prod22.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION,
        token: TkType::ReservedWord("RWORD{NOT}")
    }, prod31.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_A,
        token: TkType::ReservedWord("RWORD{NOT}")
    }, prod36.clone());

    // int
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST,
        token: TkType::Literal(LiteralTypes::Int)
    }, prod22.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION,
        token: TkType::Literal(LiteralTypes::Int)
    }, prod31.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_A,
        token: TkType::Literal(LiteralTypes::Int)
    }, prod35.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_B,
        token: TkType::Literal(LiteralTypes::Int)
    }, prod44.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_C,
        token: TkType::Literal(LiteralTypes::Int)
    }, prod49.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_D,
        token: TkType::Literal(LiteralTypes::Int)
    }, prod55.clone());

    // float
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST,
        token: TkType::Literal(LiteralTypes::Float)
    }, prod22.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION,
        token: TkType::Literal(LiteralTypes::Float)
    }, prod31.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_A,
        token: TkType::Literal(LiteralTypes::Float)
    }, prod35.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_B,
        token: TkType::Literal(LiteralTypes::Float)
    }, prod44.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_C,
        token: TkType::Literal(LiteralTypes::Float)
    }, prod49.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_D,
        token: TkType::Literal(LiteralTypes::Float)
    }, prod56.clone());

    // string
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_LIST,
        token: TkType::Literal(LiteralTypes::String)
    }, prod22.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION,
        token: TkType::Literal(LiteralTypes::String)
    }, prod31.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_A,
        token: TkType::Literal(LiteralTypes::String)
    }, prod35.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_B,
        token: TkType::Literal(LiteralTypes::String)
    }, prod44.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_C,
        token: TkType::Literal(LiteralTypes::String)
    }, prod49.clone());
    hm.insert(HmIndex {
        state: PossibleStates::EXPRESSION_D,
        token: TkType::Literal(LiteralTypes::String)
    }, prod57.clone());

    // break
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST,
        token: TkType::ReservedWord("RWORD{BREAK}")
    }, prod1.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT,
        token: TkType::ReservedWord("RWORD{BREAK}")
    }, prod6.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::ReservedWord("RWORD{BREAK}")
    }, prod2.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::ReservedWord("RWORD{BREAK}")
    }, prod60.clone());

    // continue
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST,
        token: TkType::ReservedWord("RWORD{CONTINUE}")
    }, prod1.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT,
        token: TkType::ReservedWord("RWORD{CONTINUE}")
    }, prod7.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::ReservedWord("RWORD{CONTINUE}")
    }, prod2.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::ReservedWord("RWORD{CONTINUE}")
    }, prod60.clone());

    // return
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST,
        token: TkType::ReservedWord("RWORD{RETURN}")
    }, prod1.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT,
        token: TkType::ReservedWord("RWORD{RETURN}")
    }, prod8.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::ReservedWord("RWORD{RETURN}")
    }, prod2.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::ReservedWord("RWORD{RETURN}")
    }, prod60.clone());

    // for
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST,
        token: TkType::ReservedWord("RWORD{FOR}")
    }, prod1.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT,
        token: TkType::ReservedWord("RWORD{FOR}")
    }, prod10.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::ReservedWord("RWORD{FOR}")
    }, prod2.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::ReservedWord("RWORD{FOR}")
    }, prod60.clone());

    // while
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST,
        token: TkType::ReservedWord("RWORD{WHILE}")
    }, prod1.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT,
        token: TkType::ReservedWord("RWORD{WHILE}")
    }, prod11.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::ReservedWord("RWORD{WHILE}")
    }, prod2.clone());

    // if
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST,
        token: TkType::ReservedWord("RWORD{IF}")
    }, prod1.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT,
        token: TkType::ReservedWord("RWORD{IF}")
    }, prod12.clone());
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::ReservedWord("RWORD{IF}")
    }, prod2.clone());

    // else
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::ReservedWord("RWORD{ELSE}")
    }, prod58.clone());

    // elif
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::ReservedWord("RWORD{ELIF}")
    }, prod59.clone());

    // $
    hm.insert(HmIndex {
        state: PossibleStates::STATEMENT_LIST_E,
        token: TkType::END
    }, prod3.clone());
    hm.insert(HmIndex {
        state: PossibleStates::ELSE_STATEMENT,
        token: TkType::END
    }, prod60.clone());

    hm
}

fn parse(tokens: &Vec<Token>) -> Result<(), CompilationError> {
    let hm = generate_lookup_table();
    let mut stack = Vec::<PossibleStates>::new();

    println!("empilha $");
    stack.push(PossibleStates::Terminal(TkType::END));
    println!("empilha produção inicial");
    stack.push(PossibleStates::STATEMENT_LIST);

    for tk in tokens.iter() {
        println!("token {:?}", tk);

        loop {
            let last_state = stack.last().unwrap().clone();

            if let PossibleStates::Terminal(tk_type) = last_state {
                if tk.tk_type != tk_type {
                    return Err(CompilationError::SintaxError(format!("Token mismatch, expected, '{:?}' but found '{:?}', at: row {}, col {}", tk_type, tk.tk_type, tk.row, tk.col)));
                }
                
                stack.pop();
                break;
            }

            let prox = HmIndex {
                state: last_state,
                token: tk.tk_type.clone()
            };

            println!("prox {:?}", prox);

            match hm.get(&prox) {
                Some(p) => {
                    stack.pop();
                    println!("desempilha");
                    
                    for s in p.iter().rev() {
                        let cp = *s;

                        if cp != PossibleStates::NOP {
                            println!("empilha {:?}", cp);
                            stack.push(cp);
                        } else {
                            println!("NOP");
                        }
                    }
                },
                None => {
                    println!("stack {:?}", stack);
                    return Err(CompilationError::SintaxError(format!("Unexpected state for token at: row {}, col {}", tk.row, tk.col)))
                },
            }
        }
    }
    
    println!("valid!!!");
    Ok(())
}

fn run(src_file: &str, out_dir: &str) -> std::io::Result<()> {
    let tokens = generate_tokens(src_file)?;

    let mut filename = out_dir.to_owned();
    filename.push_str("/out.lex");
    dump_tokens(&tokens, &filename)?;
    let res = parse(&tokens);

    if let Err(CompilationError::SintaxError(error)) = res {
        println!("Syntax error: {}", error);
    }

    Ok(())
}

fn main() {
    let matches = App::new("python-parser")
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
