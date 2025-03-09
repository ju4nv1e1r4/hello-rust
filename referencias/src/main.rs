fn main() {
    let a = String::from("Esta é uma referência da variável a");
    let b = &a;

    println!("b carrega uma referência de a: {}", b);

    let mut c = "Esta é uma referência da variável";
    let d = &mut c;

    println!("d carrega uma referência de c, que é mutável: {}", d);

    let mut n1 = 0;
    let n2 = &mut n1;
    *n2 += 1;

    println!("{}", n2)
}
