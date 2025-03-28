fn k_esimo_maior(j: &mut Vec<i32>, k: usize) -> i32 {
    j.sort_by(|a, b| b.cmp(a));

    return j[k-1];
}

fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
 
    let resultado = k_esimo_maior(&mut nums.clone(), k);
 
    println!("O {}º maior elemento é: {:?}", k, resultado);
}
