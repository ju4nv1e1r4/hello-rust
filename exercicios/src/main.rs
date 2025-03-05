use std::io;

fn found_biggest_number(numbers_of_vector: &[i32]) -> i32{
   let mut bigger_than_all = numbers_of_vector[0];
   for i in numbers_of_vector {
       if i > &bigger_than_all{
        bigger_than_all = *i; 
       }
   }

   bigger_than_all
}

fn tabuada(numero: i32) {
    for i in 1..11 {
        let result = numero*i;
        println!("{} x {} = {}", numero, i, result)
    }
}

fn somando_pares() {
    println!("Digite uma sequência de números reais:");
    println!("> ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("ERROR");

    let mut numbers: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map( |x| x.parse::<f64>().expect("ERROR"))
        .collect();

    let mut sum: f64 = 0.0;

    for i in &numbers{
        if  i % 2.0 == 0.0 {
            sum += i;
        }
    }

    println!("A soma dos números pares é: {}", sum);
}

fn media_de_notas(n1: f64, n2: f64, n3: f64) -> f64 {
    let media = (n1+n2+n3)/3.0;
    media
}

fn main() {
    // #1 - Contando de 1 a 10 em loops - usando FOR
    println!("=> Contando de 1 a 10 usando laço 'for'");
    for i in 1..11 {
        println!("> {}", i)
    }

    // #2 - Contando de 1 a 10 em loops - usando WHILE
    println!("=> Contando de 1 a 10 usando laço 'while'");
    let mut x = 0;
    while x < 10 {
        x += 1;
        println!("> {}", x);
    }

    // #3 - Encontrando o maior número em um vetor
    println!("=> Encontrando o maior número em um vetor.");
    let numbers = [2, 3, 4, 5, 0, 15, 1];
    println!("Vetor: {:?}", numbers);
    let biggest = found_biggest_number(&numbers);
    println!("O maior número do vetor é: {:?}", biggest);

    // #4 - Tabuada em Rust
    let number = 10;
    println!("=> Tabuada de {}:", number);
    tabuada(number);

    // #5 - Somando numeros pares de um vetor
    println!("=> Somando os números pares de um vetor.");
    somando_pares();

    // #6 - Calculando a média
    println!("=> Calculando a média.");
    let media = media_de_notas(5.5, 6.5, 9.0);
    println!("A média é: {}", media);

}

