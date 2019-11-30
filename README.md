# Python Parser

## Como executar

É necessário instalar o compilador e o gerenciador de pacotes da linguagem rust, que podem ser encontrados [aqui](https://www.rust-lang.org/tools/install).
Para rodar basta ir para a pasta que contém o arquivo `Cargo.toml` e rodar o comando `cargo run -- <INPUT>` passando o arquivo fonte de python no lugar de `<INPUT>`

## Erros na gramática

- Esqueci de fazer as definições e chamadas de funções aceitarem listas de parâmetros vazias
- Por algum motivo o lexer não está colocando `EOS` depois de `else:`

## Notas

- seria **muito** bom se houvesse uma forma de exportar as tabelas no ParsingEdu