fn main() {
    // arrays

    let numeros_inteiros = [1, 2, 3, 4, 5];

    println!("{}", numeros_inteiros[0]);

    for i in 0..numeros_inteiros.len() {
        println!("{}", numeros_inteiros[i])
    };

    // ou 

    for n in numeros_inteiros.iter() {
        println!("{}", n)
    };

    let varios_numeros = [2;3];
    println!("{:?}", varios_numeros);
}
