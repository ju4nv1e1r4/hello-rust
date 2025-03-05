fn dobro(num: i32) -> i32 {
    return num*2
}

fn some_function(a: i32, b: i32) -> i32 {
    let a_a = a;
    let b_b = b;
    let subtrair = a_a - b_b;
    println!("Está função retorna um inteiro de 32 bits. Params -> {} - {} = {}.", a_a, b_b, subtrair);
    15.0 as i32
}

fn main() {
    let numero = 5;
    println!("O dobro de {} é {}", numero, dobro(numero));

    let some_fn = some_function(5, 10);
    println!("Função {}", some_fn);
}
