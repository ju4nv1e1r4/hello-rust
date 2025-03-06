fn main() {
    let tupla = (1.2, "string", 3, (4, 0, 5.8), true);

    // acessando tuplas
    println!("{}", tupla.4);
    println!("{:?}", tupla.3);

    // acessando tuplas dentro de tuplas
    println!("{}", (tupla.3).2);

    // também temos esta forma de acessar e de referenciar tuplas 
    let (a, b, c, d, e) = tupla;

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {:#?}", d);
    println!("e: {}", e);
}
