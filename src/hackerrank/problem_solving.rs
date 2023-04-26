use std::{collections::HashMap, cmp::Ordering};
use urlencoding;

pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let maybe_max = candles.iter().max();

    if let Some(max) = maybe_max {
        return candles.iter().fold(0, | count, current | if current == max { count + 1 } else { count });
    }

    0
}

#[cfg(test)]
mod birthday_cake_candles_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(birthday_cake_candles(&[3, 2, 1, 3]), 2);
    }
}

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
            results.push(*grade + (GRADE_GAP - modulo));
        } else {
            results.push(*grade);
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

pub fn migratory_birds(arr: &[i32]) -> i32 {
    // Convert the array to map of counts.
    let mut bird_counts: HashMap<i32, i32> = HashMap::new();

    for i in arr {
        let maybe_value = bird_counts.get(i);

        if let Some(value) = maybe_value {
            bird_counts.insert(*i, value + 1);
        } else {
            bird_counts.insert(*i, 1);
        }
    }

    let mut lowest_id = i32::MAX;
    let mut highest_count = 0;

    for (current_id, current_count) in bird_counts {
        // Always set the lowest ID if it has a higher count.
        if current_count > highest_count {
            lowest_id = current_id;
        }

        // Use the lower ID if the counts are equal.
        if current_count == highest_count && current_id < lowest_id {
            lowest_id = current_id;
        }

        // Check if this is a new highest count.
        if current_count > highest_count {
            highest_count = current_count;
        }
    }

    lowest_id
}

#[cfg(test)]
mod migratory_birds_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
        assert_eq!(migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);
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

/// https://www.hackerrank.com/challenges/plus-minus/
pub fn plus_minus(arr: &[i32]) -> (String, String, String) {
    let mut positive: f32 = 0.0;
    let mut negative: f32 = 0.0;
    let mut zero: f32 = 0.0;

    for i in arr {
        match i.cmp(&0) {
            Ordering::Less => negative += 1.0,
            Ordering::Greater => positive += 1.0,
            Ordering::Equal => zero += 1.0
        }
    }

    let array_length = arr.len() as f32;
    let truncated_ratio = |f: f32| format!("{:.6}", f / array_length);

    let postive_ratio = truncated_ratio(positive);
    let negative_ratio = truncated_ratio(negative);
    let zero_ratio = truncated_ratio(zero);

    println!("{} {} {}", postive_ratio, negative_ratio, zero_ratio);
    (postive_ratio, negative_ratio, zero_ratio)
}

#[cfg(test)]
mod plus_minus_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            plus_minus(&[-4, 3, -9, 0, 4, 1]),
            ("0.500000".to_string(), "0.333333".to_string(), "0.166667".to_string())
        );
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
#[utoipa::path(
    get,
    params(
        (
            "stairs" = &i32,
            Path,
            description = "Numbers of stair levels to generate, e.g. 5"
        )
    ),
    path = "/staircase/{stairs}",
    responses(
        (
            body = String,
            description = "Generated staircase",
            status = 200
        )
    )
)]
pub fn staircase(n: i32) -> String {
    let x = n as usize;
    let mut stairs = String::new();

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

/// Converts 12-hour to 24-hour time.
/// Deliberately not using e.g. chrono.
/// Will decode a URL parameter from e.g. http://127.0.0.1:3030/time_conversion/12%3A59%3A00AM
/// https://www.hackerrank.com/challenges/time-conversion/
#[utoipa::path(
    get,
    params(
        (
            "time" = &str,
            Path,
            description = "12-hour time to convert, e.g. 12:59:00AM"
        )
    ),
    path = "/time_conversion/{time}",
    responses(
        (
            body = String,
            description = "Converted time, e.g. 00:59:00",
            status = 200
        )
    )
)]
pub fn time_conversion(s: &str) -> String {
    let decoded = urlencoding::decode(s).expect("UTF-8");
    // "12:01:00PM" => ["12", "01", "00PM"]
    let time_parts = decoded.split(":").collect::<Vec<&str>>();

    if let (Some(hour), Some(minute), Some(second_end)) = (time_parts.get(0), time_parts.get(1), time_parts.get(2)) {
        // Convert the 12-hour hour to 24 (military...)-hour.
        let mut military_hour = hour.parse::<i32>().unwrap();

        if second_end.ends_with("PM") && military_hour < 12 {
            military_hour += 12;
        } else if second_end.ends_with("AM") && military_hour == 12 {
            military_hour = 0;
        }

        // "59AM" => "59"...
        let mut seconds = second_end.to_string();
        seconds.pop();
        seconds.pop();

        return format!("{:0>2}:{}:{}", military_hour, minute, seconds)
    }

    s.to_string()
}

#[cfg(test)]
mod time_conversion_tests {
    use super::*;

    #[test]
    fn morning() {
        assert_eq!(time_conversion("12:01:00PM"), "12:01:00");
    }

    #[test]
    fn afternoon() {
        assert_eq!(time_conversion("03:59:59PM"), "15:59:59");
    }

    #[test]
    fn past_midnight() {
        assert_eq!(time_conversion("12:01:00AM"), "00:01:00");
    }

    #[test]
    fn past_midnight_again() {
        assert_eq!(time_conversion("12:40:22AM"), "00:40:22");
    }
}
