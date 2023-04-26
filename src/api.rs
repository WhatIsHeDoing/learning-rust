use crate::hackerrank::*;
use warp::{ Filter, Rejection, Reply };

/// See examples at https://github.com/seanmonstar/warp/tree/master/examples.
pub fn handlers() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let bon_appetit_path =
        warp::path!("bon_appetit")
            .map(|| problem_solving::bon_appetit(&[3, 10, 2, 9], 1, 7));

    let rotate_left_path =
        warp::path!("rotate_left" / i32)
            .map(| rotation | warp::reply::json(&data_structures::rotate_left(rotation, &[1, 2, 3, 4, 5])));

    let staircase_path =
        warp::path!("staircase" / i32)
            .map(| levels | problem_solving::staircase(levels));

    let time_conversion_path =
        warp::path!("time_conversion" / String)
            .map(| time: String | problem_solving::time_conversion(&time.to_string()));

    bon_appetit_path
        .or(rotate_left_path)
        .or(staircase_path)
        .or(time_conversion_path)
}
