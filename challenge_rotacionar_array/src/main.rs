fn rotacionar_array(my_array: &mut [i32; 7], k: usize) {
    let n  = my_array.len();

    if n == 0 {
        return;
    }

    let k = k % n;

    my_array.reverse();
    my_array[0..k].reverse();
    my_array[k..].reverse();
}
fn main() {
    let mut array = [1, 2, 3, 4, 5, 6, 7];
    let k = 5;
 
    println!("Array original: {:?}", array);
 
    rotacionar_array(&mut array, k);
 
    println!("Array rotacionado: {:?}", array);
}
