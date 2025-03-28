use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

fn count_lines_on_files(f_path: &str) -> Result<usize, std::io::Error> {
    let f = File::open(f_path)
    .expect("Não foi possível abrir o arquivo");

    let reader = BufReader::new(f);

    let lines = reader.lines().count();

    Ok(lines)
}

fn main() {
    let dados = File::open("dados.txt")
        .expect("Não foi possível abrir o arquivo");

    println!("DEBUG: {:?}", dados);

    let mut f = File::options().append(true).open("dados.txt")
        .expect("Não foi possível adicionar mais linhas ao arquivo");

    match writeln!(&mut f, "\nTo see more: https://www.arewelearningyet.com/") {
        Ok(_) => println!("Sucesso ao escrever mais uma linha"),
        Err(e) => println!("Nao foi possível adicionar mais uma linha: {e}")
    }

    match count_lines_on_files("dados.txt") {
        Ok(count) => println!("O arquivo tem {} linhas.", count),
        Err(e) => println!("Falha ao contar linhas do arquivo: {}", e)
    }
}
