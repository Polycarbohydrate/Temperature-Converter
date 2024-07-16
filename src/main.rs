// Made by @Polycarbohydrate
// https://github.com/polycarbohydrate/temperature-converter/
// MIT LICENCE
// If you use this project or repost it somewhere else, please give credit by using the first 2 lines/comments.

use std::io;

fn main()	{
	// This part just lets the users pick which conversion to chose from.
	loop	{
		println!("Press 1 for Fahrenheit to Celsius conversion.");
		println!("Press 2 for Celsius to Fahrenheit conversion.");

		let mut user_in = String::new();

		io::stdin()
			.read_line(&mut user_in)
			.expect("Failed to read line");

		let user_in: u64 = match user_in.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You pressed {user_in}.");
		// User conversion from Fahrenheit to Celsius.
		// °C = [(°F-32)×5]/9
		if user_in == 1	{
			println!("Enter the temperature in Fahrenheit to convert to Celsius. (e.g: 20, 30.45, 0 ,-13)");
			
			let mut f_in = String::new();

			io::stdin()
				.read_line(&mut f_in)
				.expect("Failed to read line");

			let f_in: f64 = match f_in.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
			};

            let sub: f64 = f_in - 32.0;

            let multi = sub * 5.0;

            let div = multi / 9.0;

            println!("                                                         ");
            println!("{f_in} degrees Fahrenheit is {div} degrees Celsius.");
            println!("The final conversion from Fahrenheit to Celsius is {div}.");
            println!("                                                         ");
            println!("Press 1 to exit the program. Press any other key to continue.");

            let mut exitf = String::new();

            io::stdin()
                .read_line(&mut exitf)
                .expect("Failed to read line");

            let exitf: u64 = match exitf.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if exitf == 1   {
                break
            }
            else    {
               continue;
            }
		}
        // User conversion from Fahrenheit to Celsius
        // °F = (°C × 9/5) + 32
		else if user_in == 2 {
			println!("Enter the temperature in Celsius to convert to Fahrenheit. (e.g: 86, 98.60, 0, -26)");

            let mut c_in = String::new();

            io::stdin()
                .read_line(&mut c_in)
                .expect("Failed to read line");

            let c_in: f64 = match c_in.trim().parse()   {
            Ok(num) => num,
            Err(_) => continue,
            };

            let cdiv: f64 = 9.0/5.0;

            let cmulti = c_in * cdiv;

            let add = cmulti + 32.0;

            println!("                                                         ");
            println!("{c_in} degress Celsius is {add} degrees Fahrenheit.");
            println!("The final conversion from Celsius to Fahrenheit is {add}.");
            println!("                                                         ");
            println!("Press 1 to exit the program. Press any other key to continue.");

            let mut exitf = String::new();

            io::stdin()
                .read_line(&mut exitf)
                .expect("Failed to read line");

            let exitf: u64 = match exitf.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if exitf == 1   {
                break
            }
            else    {
               continue;
            }
		}

		else	{
			println!("Enter a valid number either 1 or 2.");
			continue;
		}
	}
}
