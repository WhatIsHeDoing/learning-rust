use colored::Colorize;
use warp::Filter;

mod hackerrank;
mod rust_lang;

use hackerrank::*;
use rust_lang::*;

#[tokio::main]
async fn main() {
    println!("{}", "Hacker Rank Challenges\n".blue().bold());

    println!("{}", "Problem Solving\n".green().bold());

    println!("{}", "Birthday Candles".yellow().bold());
    println!("{}", problem_solving::birthday_cake_candles(&[3, 2, 1, 3]));
    println!();

    println!("{}", "Bon Appetit".yellow().bold());
    problem_solving::bon_appetit(&[3, 10, 2, 9], 1, 7);
    println!();

    println!("{}", "Grading Students".yellow().bold());
    problem_solving::grading_students(&[73, 67, 38, 33]);
    println!();

    println!("{}", "Migratory Birds".yellow().bold());
    println!("{}", problem_solving::migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]));
    println!();

    println!("{}", "Min & Max Sum".yellow().bold());
    problem_solving::mini_max_sum(&[1, 2, 3, 4, 5]);
    println!();
    println!();

    println!("{}", "Plus Minus".yellow().bold());
    problem_solving::plus_minus(&[-4, 3, -9, 0, 4, 1]);
    println!();

    println!("{}", "Sock Merchant".yellow().bold());
    println!("{}", problem_solving::sock_merchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]));
    println!();

    println!("{}", "Staircase".yellow().bold());
    problem_solving::staircase(5);

    println!("{}", "Time Conversion".yellow().bold());
    println!("{}", problem_solving::time_conversion("12:40:22AM"));
    println!();

    println!("{}", "Data Structures\n".green().bold());

    println!("{}", "Reverse Array".yellow().bold());
    println!("{:?}", data_structures::reverse_array(&[1, 4, 3, 2]));
    println!();

    println!("{}", "Rotate Left".yellow().bold());
    println!("{:?}", data_structures::rotate_left(4, &[1, 2, 3, 4, 5]));
    println!();

    println!("{}", "Rust Language Book Programming\n".blue().bold());

    println!("{}", "Enums".yellow().bold());
    println!("{}", enums(WebEvent::Click { x: 20, y: 80 }));
    println!();

    println!("{}", "Secret Number".yellow().bold());
    secret_number();
    println!();

    println!("{}", "Warp\n".blue().bold());
    println!();

    // See examples at https://github.com/seanmonstar/warp/tree/master/examples.
    let bon_appetit_path = warp::path!("bon_appetit")
        .map(|| problem_solving::bon_appetit(&[3, 10, 2, 9], 1, 7));

    let rotate_left_path = warp::path!("rotate_left" / i32)
        .map(| rotation | warp::reply::json(&data_structures::rotate_left(rotation, &[1, 2, 3, 4, 5])));

    let staircase_path = warp::path!("staircase" / i32)
        .map(| levels | problem_solving::staircase(levels));

    let time_conversion_path = warp::path!("time_conversion" / String)
        .map(| time: String | problem_solving::time_conversion(&time.to_string()));

    let routes = warp::get()
        .and(bon_appetit_path)
        .or(rotate_left_path)
        .or(staircase_path)
        .or(time_conversion_path);

    println!("{}", "🚀 Serving from http://127.0.0.1:3030/ - try:");
    println!("{}", "  ➡️ http://127.0.0.1:3030/bon_appetit/");
    println!("{}", "  ➡️ http://127.0.0.1:3030/rotate_left/4/");
    println!("{}", "  ➡️ http://127.0.0.1:3030/staircase/6/");
    println!("{}", "  ➡️ http://127.0.0.1:3030/time_conversion/12:40:22AM/");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
