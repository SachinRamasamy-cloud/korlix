use colored::Colorize;

pub fn print_banner() {
    println!();
    println!("  {}  Korlix v0.1.0", "◈".cyan().bold());
    println!("  {}", "Ultra-light frontend language compiler".dimmed());
    println!();
}

#[allow(dead_code)]
pub fn print_success(msg: &str) {
    println!("  {} {}", "✓".green().bold(), msg.white());
}

pub fn print_error(msg: &str) {
    eprintln!("  {} {}", "✕".red().bold(), msg.white());
}

#[allow(dead_code)]
pub fn print_info(msg: &str) {
    println!("  {} {}", "◦".cyan(), msg.dimmed());
}

#[allow(dead_code)]
pub fn print_step(step: &str, msg: &str) {
    println!("  {} {} {}", "→".dimmed(), step.cyan().bold(), msg.dimmed());
}
