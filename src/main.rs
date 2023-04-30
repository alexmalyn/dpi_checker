use std::thread::sleep;
use std::time::Duration;

use mouse_position::mouse_position::{Mouse};

// no error handling to allow empty input. poor design
fn get_user_input<T>(message: &str) -> T 
    where T: std::str::FromStr,
          T: std::default::Default,
{
    let mut input = String::new();
    println!("{}: ", message);
    std::io::stdin().read_line(&mut input).unwrap();
    let output = input.trim().parse::<T>();
    match output {
        Ok(res) => return res,
        Err(_) => {
            let default: T = T::default();
            return default;
        },
    }
}

fn get_mouse_pos() -> (i32, i32) {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => (x,y),
        Mouse::Error => panic!("Error getting mouse position"),
   }
}

fn calculate_euclidean_distance(start: &(i32,i32), end: &(i32,i32)) -> f64 {
    let x = (end.0 - start.0).pow(2);
    let y = (end.1 - start.1).pow(2);
    ((x + y) as f64).sqrt()
}

fn calculate_dpi_deviation(measured_dpi: f64, set_dpi: f64) -> f64 {
    ((measured_dpi / set_dpi) - 1.0) * 100.0
}

fn main() {
    println!("Hello, welcome to dpi checker!");

    let dpi: u32 = get_user_input("Please enter your set DPI");
    let distance: f64 = get_user_input("Please enter the desired distance");
    
    println!("You will now be asked to move your mouse the desired distance, \
    make sure you have enough space so you do not hit any screen borders! A countdown will start soon.");
    sleep(Duration::from_secs(3));

    println!("Starting in 3...");
    sleep(Duration::from_secs(1));
    println!("2...");
    sleep(Duration::from_secs(1));
    println!("1...");
    sleep(Duration::from_secs(1));

    let mut starting_coordinates: (i32,i32) = (0,0);
    let mut ending_coordinates: (i32, i32) = (0,0);

    starting_coordinates = get_mouse_pos();

    get_user_input::<i32>("Move your mouse the desired distance and press enter");

    ending_coordinates = get_mouse_pos();

    let measured_dpi = calculate_euclidean_distance(&starting_coordinates, &ending_coordinates) / distance;
    let dpi_deviation = calculate_dpi_deviation(measured_dpi, dpi as f64);

    println!("Measured DPI is: {:.2}", measured_dpi);
    println!("Measured DPI deviation is: {:.1}%", dpi_deviation);

}
