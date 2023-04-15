use colored::Colorize;

mod hackerrank;
mod rust_lang;

fn main() {
    println!("{}", "Hacker Rank Challenges\n".blue().bold());

    println!("{}", "Min & Max Sum".yellow().bold());
    hackerrank::mini_max_sum(&[1, 2, 3, 4, 5]);
    println!();

    print!("{}", "\nStaircase".yellow().bold());
    hackerrank::staircase(5);
    println!();

    println!("{}", "\nRust Language Book Programming\n".blue().bold());

    println!("{}", "Secret Number".yellow().bold());
    rust_lang::secret_number();
}
