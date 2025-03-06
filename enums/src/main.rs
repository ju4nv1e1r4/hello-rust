#[derive(Debug)]
enum Airplane {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
enum Payment {
    Cash(bool, f64),
    Credit(bool, f64)
}

fn main() {
    let my_airplane: Airplane = Airplane::Up;
    let my_airplane_d: Airplane = Airplane::Down;
    let my_airplane_l: Airplane = Airplane::Left;
    let my_airplane_r: Airplane = Airplane::Right;

    match my_airplane{
        Airplane::Right => println!("The airplane is going to the Right"),
        Airplane::Left => println!("The airplane is going to the Left"),
        Airplane::Up => println!("The airplane is going Up"),
        Airplane::Down => println!("The airplane is going Down"),
    }

    println!("Others:\n{:?}\n{:?}\n{:?}", my_airplane_d, my_airplane_l, my_airplane_r);

    println!("---\n");

    let order1: Payment = Payment::Credit(false, 0.);
    let order2: Payment = Payment::Cash(true, 543.50);

    match order2 {
        Payment::Cash(x, y) => println!("Status: {}\nTotal: {}", x, y),
        Payment::Credit(z, w) => println!("Status: {}\nTotal: {}", z, w)
    }

    println!("-----\nOrder\n-----\n {:?}", order1)
}
