use std::collections::HashMap;

/// https://www.hackerrank.com/challenges/bon-appetit/
pub fn bon_appetit(bill: &[i32], k: i32, b: i32)-> String {
    let ignore: i32 = bill[k as usize];
    let katies_bill = (bill.iter().sum::<i32>() - ignore) / 2;

    if katies_bill == b {
        const SUCCESS: &str = "Bon Appetit";
        println!("{}", SUCCESS);
        return SUCCESS.to_string();
    }
    
    let diff = format!("{}", b - katies_bill);
    println!("{}", diff);
    diff
}

#[cfg(test)]
mod bon_appetit_tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(bon_appetit(&[3, 10, 2, 9], 1, 7), "Bon Appetit");
    }

    #[test]
    fn wrong() {
        assert_eq!(bon_appetit(&[3, 10, 2, 9], 1, 12), "5");
    }
}

/// https://www.hackerrank.com/challenges/grading/
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut results = Vec::new();
    const GRADE_GAP: i32 = 5;

    for grade in grades {
        let modulo = grade % GRADE_GAP;

        if grade > &37 && modulo > 2 {
            let rounded = *grade + (GRADE_GAP - modulo);
            results.push(rounded);
            println!("{}", rounded);
        } else {
            results.push(*grade);
            println!("{}", grade);
        }
    }

    results
}

#[cfg(test)]
mod grading_students_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(grading_students(&[73, 67, 38, 33]), [75, 67, 40, 33]);
    }
}

/// https://www.hackerrank.com/challenges/mini-max-sum/
pub fn mini_max_sum(arr: &[i32]) -> [i64; 2] {
    let sum = arr.iter().fold(0i64, |sum, &val| sum + i64::from(val));
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

/// https://www.hackerrank.com/challenges/sock-merchant/
pub fn sock_merchant(_: i32, ar: &[i32]) -> i32 {
    let mut sock_counts: HashMap<i32, i32> = HashMap::new();
    
    for i in ar {
        let maybe_value = sock_counts.get(i);

        if let Some(value) = maybe_value {
            sock_counts.insert(*i, value + 1);
        } else {
            sock_counts.insert(*i, 1);
        }
    }

    sock_counts
        .iter()
        .fold(
            0,
            |agg, (_, value)| if value > &0 { agg + (value / 2) } else { agg }
        )
}

#[cfg(test)]
mod sock_merchant_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sock_merchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]), 3);
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