use std::io;


fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut name = "Rust";
    println!("Hello, {}.", name);
    name = "Rust beginner";
    println!("Hello, {}.", name);

    let number1 = 24;
    let number2 = 42;

    if number1 > number2 {
        println!("{} > {}", number1, number2)
    } 
    else {
        println!("{} <= {}", number1, number2)
    }

    let mut number_1 = String::new();
    io::stdin().read_line(&mut number_1).expect("Erro ao ler número 1.");
    let mut number_2 = String::new();
    io::stdin().read_line(&mut number_2).expect("Erro ao ler número 2.");

    if convert_to_int(&number_1) > convert_to_int(&number_2) {
        println!("O número {} é maior que {}.", number_1, number_2);
    }
    else {
        println!("O nṹmero {} é menor ou igual a {}.", number_1, number_2);
    }

    let mut soma = 0;
    let mut valor_entrada = String::new();

    io::stdin().read_line(&mut valor_entrada).expect("Erro ao ler valor de entrada.");

    let mut valor_int = convert_to_int(&valor_entrada);

    while valor_int != 0 {
        let mut r = valor_int % 10;
        soma = soma + r;
        valor_int = valor_int / 10;

    }

    println!("O valor da soma dos dígitos é {}", soma)
}
