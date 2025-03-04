use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    println!("==> Calculadora de Fatorial <== \n");
    println!("> Digite um número:");

    let mut entrada_fatorial = String::new();
    io::stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada fatorial.");

    let mut fatorial = 1;
    let mut entrada_int = convert_to_int(&entrada_fatorial); 

    while entrada_int > 1 {
        fatorial = fatorial * entrada_int;
        entrada_int = entrada_int - 1;
    }

    println!("Resultado: {}", fatorial)
}
