/// https://www.hackerrank.com/challenges/mini-max-sum/
pub fn mini_max_sum(arr: &[i32]) -> [i64; 2] {
    let sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let maybe_max = arr.iter().max();
    let maybe_min = arr.iter().min();

    if let (Some(max), Some(min)) = (maybe_max, maybe_min) {
        let min_sum = sum - i64::from(*max);
        let max_sum = sum - i64::from(*min);
        print!("{} {}", min_sum, max_sum);
        return [min_sum, max_sum]
    }

    [0, 0]
}

#[cfg(test)]
mod mini_max_sum_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(mini_max_sum(&[1, 2, 3, 4, 5]), [10, 14]);
    }
}

/// https://www.hackerrank.com/challenges/staircase/
pub fn staircase(n: i32) -> String {
    let x: usize = n.try_into().unwrap();
    let mut stairs = "".to_owned();
    
    for i in 1..(x + 1) {
        stairs.push_str(format!("{}{}\n", " ".repeat(x - i), "#".repeat(i)).as_str());
    }
    
    println!("{}", stairs);
    stairs
}

#[cfg(test)]
mod staircase_tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected = "   #
  ##
 ###
####
";
        assert_eq!(expected, staircase(4));
    }
}