use colored::Colorize;

mod hackerrank;
mod rust_lang;

fn main() {
    println!("{}", "Hacker Rank Challenges\n".blue().bold());

    println!("{}", "Problem Solving\n".green().bold());

    println!("{}", "Bon Appetit".yellow().bold());
    hackerrank::problem_solving::bon_appetit(&[3, 10, 2, 9], 1, 7);
    println!();

    println!("{}", "Grading Students".yellow().bold());
    hackerrank::problem_solving::grading_students(&[73, 67, 38, 33]);
    println!();

    println!("{}", "Min & Max Sum".yellow().bold());
    hackerrank::problem_solving::mini_max_sum(&[1, 2, 3, 4, 5]);
    println!();
    println!();

    println!("{}", "Plus Minus".yellow().bold());
    hackerrank::problem_solving::plus_minus(&[-4, 3, -9, 0, 4, 1]);
    println!();

    println!("{}", "Sock Merchant".yellow().bold());
    println!("{}", hackerrank::problem_solving::sock_merchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]));
    println!();

    println!("{}", "Staircase".yellow().bold());
    hackerrank::problem_solving::staircase(5);

    println!("{}", "Time Conversion".yellow().bold());
    println!("{}", hackerrank::problem_solving::time_conversion("12:40:22AM"));
    println!();

    println!("{}", "Data Structures\n".green().bold());

    println!("{}", "Reverse Array".yellow().bold());
    println!("{:?}", hackerrank::data_structures::reverse_array(&[1, 4, 3, 2]));
    println!();

    println!("{}", "Roatate Left".yellow().bold());
    println!("{:?}", hackerrank::data_structures::rotate_left(4, &[1, 2, 3, 4, 5]));
    println!();

    println!("{}", "Rust Language Book Programming\n".blue().bold());

    println!("{}", "Secret Number".yellow().bold());
    rust_lang::secret_number();
}
