mod stats;

fn main() {
    let nums  = vec![1, 2, 6, 6, 7, 4, 4, 10];

    println!("Média: {:?}", stats::media(&nums));

    println!("Mediana: {}", stats::mediana(&nums));

    println!("Moda: {}", stats::moda(&nums))

}
