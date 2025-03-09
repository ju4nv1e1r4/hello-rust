/*
Sobre const, blocos, static e afins
*/

const DESCONTO: f32 = 0.1;
static DESCONTO2: f64 = 0.5;

fn calcular_desconto(preco: f32) -> f32 {
    let c = preco*DESCONTO;
    c
}

fn calcular_desconto2(preco: f32) -> f32 {
    let c = preco*DESCONTO2 as f32;
    c
}

fn main() {
    let p = 50.00;
    let valor_de_desconto = calcular_desconto(p);
    println!("Valor de desconto: {}", valor_de_desconto);

    let texto = String::from("Fora do bloco");
    println!("Texto -> {}", texto);

    {
        let p2 = 45.00;
        let valor_de_desconto = calcular_desconto(p2);
        println!("Valor de desconto: {}", valor_de_desconto);

        let texto = String::from("Dentro do bloco");
        println!("Texto com shadowing-> {}", texto);
    }

    println!("Texto após o bloco, mas fora dele -> {}", texto);

    let valor_de_desconto2 = calcular_desconto2(p);
    println!("Calculando um desconto usando static: {}", valor_de_desconto2)
}
