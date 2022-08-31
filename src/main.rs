extern crate rand;
extern crate math;

use rand::prelude::*;
use std::io;
use math::round;
use std::process;

// function for creating random number
fn create_random_value() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(1.00..10.00);
    y
}

// function for comparing input price and generated random price
fn compare(price_origin: f64, price_input: f64) -> bool {
    price_input >= price_origin
}

fn input_payment(price_origin: f64) -> (f64, usize) {

    let mut price_input: f64;
    let mut tried_cnt: usize = 0;

    loop {
        println!("Please input value for payment. Or please press 'q' to quit.");
        let mut str_input = String::new();

        // input string
        io::stdin()
            .read_line(&mut str_input)
            .expect("Cannot read line");

        // check if input 'q'
        let pp = str_input.as_bytes();
        if pp[0] == b'q' {
            println!("Do you really want to exit? y/n");

            // confirm quit
            str_input = String::from("");
            io::stdin()
                .read_line(&mut str_input)
                .expect("Cannot read line");
            
            let pp = str_input.as_bytes();
            if pp[0] == b'y' {
                process::exit(1);
            } else {
                continue;
            }
        }
        
        price_input = match str_input.trim().parse() {
            Ok(pay) => {
                if pay <= 0.0 || pay >= 1000.0 {
                    println!("Invalid input! Must be between 1.00 to 999.99");
                    continue;
                }

                // check format 000.00
                let check_pay = round::ceil(pay, 2);
                if check_pay * 100.0 != pay * 100.0 {
                    println!("Invalid input format! Must be format like : 000.00");
                    continue;
                }
                tried_cnt += 1;
                pay
            },
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        
        if compare(price_origin, price_input) {
            break;
        }
        
        println!("Price is not enough! Tried count : {tried_cnt}");

    }

    (price_input, tried_cnt)
}

fn main() {
    let price_origin = create_random_value();
    let price_origin = round::floor(price_origin, 2);

    let (price_input, tried_cnt) = input_payment(price_origin);

    println!("{price_input}, {price_origin}, You succeed in {tried_cnt} times.");
}