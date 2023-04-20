/// https://www.hackerrank.com/challenges/arrays-ds/
pub fn reverse_array(a: &[i32]) -> Vec<i32> {
    a.iter().copied().rev().collect()
}

#[cfg(test)]
mod reverse_array_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(reverse_array(&[1, 4, 3, 2]), vec![2, 3, 4, 1]);
    }
}

/// https://www.hackerrank.com/challenges/array-left-rotation/
pub fn rotate_left(d: i32, arr: &[i32]) -> Vec<i32> {
    let (left, right) = arr.split_at(d as usize);
    let mut right_vector: Vec<i32> = right.to_vec();
    right_vector.append(&mut left.to_vec());
    right_vector
}

#[cfg(test)]
mod rotate_left_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rotate_left(4, &[1, 2, 3, 4, 5]), vec![5, 1, 2, 3, 4]);
    }
}
