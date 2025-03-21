fn main() {
    let mut vetores1 = vec![1, 2, 3, 4];
    // let mut vetores2: Vec<i32> = Vec::new();

    println!("{}", vetores1[1]);

    vetores1.push(5);
    println!("{}", vetores1[1]);

    vetores1.remove(1);
    println!("{:?}", vetores1);

    for i in vetores1.iter(){
        println!("{}", i);
    }
}
