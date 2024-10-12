use colored::Colorize;
use core::str;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::env;
use std::process::Command;

// UNICOL - The kernel version recolourer

fn main() {
    let args: Vec<String> = env::args().collect();
    let opt = &args[1];

    if (opt == "-g" || opt == "--greet") {
        println!("Welcome to UNICOL - the kernel version recolourer!");
    }

    let mut rng = thread_rng();

    let (kernel_elements, kernel_separators) = parse_kernel();

    let kernel_length = kernel_elements.len();

    for i in 0..kernel_length {
        let color_1 = generate_random_rgb(&mut rng);
        let color_2 = generate_random_rgb(&mut rng);

        print!(
            "{}",
            kernel_elements[i].truecolor(color_1.0, color_1.1, color_1.2)
        );
        if i < kernel_length - 1 {
            print!(
                "{}",
                kernel_separators[i]
                    .to_string()
                    .truecolor(color_2.0, color_2.1, color_2.2)
            );
        }
    }
}

fn generate_random_rgb(rng: &mut ThreadRng) -> (u8, u8, u8) {
    (
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
    )
}

fn parse_kernel() -> (Vec<String>, Vec<char>) {
    let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .expect("`uname -r` failed!");

    let kernel: String = str::from_utf8(kernel.stdout.as_slice())
        .expect("Conversion of kernel version to string failed!")
        .to_owned();

    let ref_kernel_elements: Vec<&str> = kernel.split(['.', '-', '_']).collect();
    let kernel_elements: Vec<String> = ref_kernel_elements.iter().map(|v| v.to_string()).collect();
    let kernel_separators: Vec<char> = kernel
        .chars()
        .filter(|x| x.is_ascii_punctuation())
        .collect();
    (kernel_elements, kernel_separators)
}
