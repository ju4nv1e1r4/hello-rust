fn lucro_maximo(precos: Vec<i32>) -> i32 {
    if precos.is_empty(){
        return 0;
    }

    let mut preco_min = precos[0];
    let mut lucro_max = 0;

    for preco in precos.iter().skip(1) {
        let lucro_atual = preco - preco_min;
        lucro_max = lucro_max.max(lucro_atual);
        preco_min = preco_min.min(*preco);
    }

    lucro_max
}

fn main() {
    let book1 = vec![7, 6, 10, 4, 5, 6, 1, 11, 5, 4, 1, 16, 15, 14, 10];

    let lucro1 = lucro_maximo(book1.clone());
    println!("O lucro máximo é: R$ {}", lucro1);

    let book2 = vec![7, 6, 5, 4, 2, 1, 1, 1];

    let lucro2 = lucro_maximo(book2.clone());
    println!("O lucro máximo é: R$ {}", lucro2);
}
