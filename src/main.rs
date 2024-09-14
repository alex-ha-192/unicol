use colored::Colorize;
use core::str;
use rand::{thread_rng, Rng};
use std::process::Command;

fn main() {
    let mut rng = thread_rng();

    let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .expect("`uname -r` failed!");

    let kernel = str::from_utf8(kernel.stdout.as_slice())
        .expect("Conversion of kernel version to string failed!");

    let kernel_elements: Vec<&str> = kernel.split(['.']).collect();

    let kernel_length = kernel_elements.len();
    for i in 0..kernel_length {
        print!(
            "{}",
            kernel_elements[i].truecolor(
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255)
            )
        );
        if i < kernel_length - 1 {
            print!(
                "{}",
                ".".truecolor(
                    rng.gen_range(0..=255),
                    rng.gen_range(0..=255),
                    rng.gen_range(0..=255)
                )
            );
        }
    }
}
