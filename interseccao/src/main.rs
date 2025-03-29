use std::collections::HashSet;

fn interseccao(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let convert1: HashSet<_> = nums1.into_iter().collect();
    let convert2: HashSet<_> = nums2.into_iter().collect();

    let response: Vec<_> = convert1.intersection(&convert2).cloned().collect();

    response
}

fn main() {
    let vetor1 = vec![1, 1, 2, 2, 0];
    let vetor2 = vec![10, 6, 3, 4, 5];

    let result = interseccao(vetor1, vetor2);
    println!("Intersecção: {:?}", result)
}
