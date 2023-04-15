/// https://www.hackerrank.com/challenges/mini-max-sum/
pub fn mini_max_sum(arr: &[i32]) {
    let sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let maybe_max = arr.iter().max();
    let maybe_min = arr.iter().min();

    if let (Some(max), Some(min)) = (maybe_max, maybe_min) {
        print!("{} {}", sum - i64::from(*max), sum - i64::from(*min));
    }
}

/// https://www.hackerrank.com/challenges/staircase/
pub fn staircase(n: i32) {
    let x = n.try_into().unwrap();

    for i in 1..(x + 1) {
        print!("{}{}", " ".repeat(x - i), "#".repeat(i));
        
        if i < x {
            println!();
        }
    }
}
