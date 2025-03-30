use std::collections::HashMap;

pub fn media(nums: &Vec<i32>) -> f64 {
    let mut soma = 0;
    for i in nums {
        soma += i;
    }

    soma as f64 / nums.len() as f64
}

pub fn mediana(nums: &Vec<i32>) -> f64 {
    let mut nums_sorted = nums.clone();
    nums_sorted.sort();

    let find_center= nums.len() / 2;
    if  nums_sorted.len() % 2 == 0 {
        return media(&vec![nums_sorted[find_center], nums_sorted[find_center-1]]);
    }

    nums_sorted[find_center] as f64
}

pub fn moda(nums: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in nums{
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    
    let mut major_value = 0;
    let mut major_key = 0;

    for (k, v) in map{
        if v > major_value{
            major_value = v;
            major_key = *k;
        }
    }

    major_key
}