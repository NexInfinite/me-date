extern crate astro;

use astro::*;
use std::io;
use std::io::*;


fn main() {
    // Welcome  message
    println!("So you want to have a Julian day (Me day)?");

    loop {

        // Declaring variables
        let mut year: i16 = 0;
        let mut month: u8 = 0;
        let mut day: u8 = 0;


        loop {
            // Get the year
            print!("Ok so, whats the year?                                                  ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            year = input.trim().parse().unwrap();

            // Get the month
            print!("Ah so we are time travelling to {}! But like what month?              ", year);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            month = input.trim().parse().unwrap();

            // Get the day
            print!("Really? Month {} of {}? Alright but be more specific - what day?       ", month, year);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            day = input.trim().parse().unwrap();

            // Funni message to confirm all the inputs
            print!("Alright, we will go to {}-{}-{}. Is this correct? Y/n                   ", day, month, year);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let is_correct = input.trim().to_lowercase();

            if is_correct == "y"{
                break
            } else {
                println!("\n\nAlright, I'll let you pick another time to become a me day.n")
            }
        }

        // Generating the day as an obj
        let day_of_month = time::DayOfMonth{day: day, hr: 12, min: 0, sec: 0.0, time_zone: 0.0};
        let date = time::Date{year: year, month: month, decimal_day: time::decimal_day(&day_of_month), cal_type: time::CalType::Gregorian};

        // Getting the Julian Date and letting the user know thats the date
        let julian_day = time::julian_day(&date);
        println!("\n\nYour me date is: {}\n\n", julian_day);

        // Ask if the user want to go again
        print!("So erm, kind of awkward but, do you want to go again? Y/n               ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let go_again = input.trim().to_lowercase();

        if go_again == "y" {
            println!("Ok I'll let you go again...\n\n");
        } else {
            break
        }
    }
}
