use colored::Colorize;

mod hackerrank;
mod rust_lang;

fn main() {
    println!("{}", "Hacker Rank Challenges\n".blue().bold());

    println!("{}", "Bon Appetit".yellow().bold());
    hackerrank::bon_appetit(&[3, 10, 2, 9], 1, 7);
    println!();

    println!("{}", "Grading Students".yellow().bold());
    hackerrank::grading_students(&[73, 67, 38, 33]);
    println!();

    println!("{}", "Min & Max Sum".yellow().bold());
    hackerrank::mini_max_sum(&[1, 2, 3, 4, 5]);
    println!();
    println!();

    println!("{}", "Plus Minus".yellow().bold());
    hackerrank::plus_minus(&[-4, 3, -9, 0, 4, 1]);
    println!();

    println!("{}", "Sock Merchant".yellow().bold());
    println!("{}", hackerrank::sock_merchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]));
    println!();

    println!("{}", "Staircase".yellow().bold());
    hackerrank::staircase(5);

    println!("{}", "Time Conversion".yellow().bold());
    println!("{}", hackerrank::time_conversion("12:40:22AM"));
    println!();

    println!("{}", "Rust Language Book Programming\n".blue().bold());

    println!("{}", "Secret Number".yellow().bold());
    rust_lang::secret_number();
}
