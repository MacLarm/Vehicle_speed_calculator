use std::io::{self, Write};
use std::f32::consts::PI;

fn main() {
    let speed = find_speed();
    println!("With those numbers, you will be doing {:.1} mph", speed);
}

//
// Here we put it all together. We calculate the speed a vehicle would be traveling
// with the height of their tires, the engine RPM, and the final drive ratio.
// 
// # Returns: speed in Miles per Hour. 
//
fn find_speed() -> f32 {
    // Get circuference of tires
    let height = get_height();
    let mut circumference = calc_circumference(height); // Circumference in inches
    circumference = calc_miles_per_rotation(circumference); // Circumgference in miles

    // Figure the speed of the wheel based on engine RPM
    let engine_rpm = get_rpm();
    let final_drive = calc_final_drive(get_trans_gear(), 
                                get_t_case_gear(), get_axle_gear());
    let wheel_rpm = calc_wheel_rpm(engine_rpm, final_drive);

    // Calculate vehicle speed.
    let mph = wheel_rpm * circumference * 60.0;
    return mph;
}

//
// Get the enginge RPM from the user.
//
fn get_rpm() -> i32 {
    // Declare line to write to.
    let mut line = String::new();
    
    print!("Enter the RPM of your motor: ");
    // Ensures the prompt is printed before reading input
    io::stdout().flush().unwrap();  
    // Error check
    io::stdin().read_line(&mut line).expect("Failed to read line");
    // Try parseing the string into a f32, if not recursive call to try again.
    match line.trim().parse::<i32>() {
        Ok(rpm) => rpm, // if successful return rpm
        Err(_) => {
            println!("Error: Please provide a valid floating-point number.");
            return get_rpm(); // recursive call.
        }
    }
}

//
// Get the height of the tire from the user.
//
fn get_height() -> u16 {
    // Declare line to write to.
    let mut line = String::new();
    
    print!("Enter the height (diameter) of your tire: ");
    // Ensures the prompt is printed before reading input
    io::stdout().flush().unwrap();  
    // Error check
    io::stdin().read_line(&mut line).expect("Failed to read line");
    // Try parseing the string into a f32, if not recursive call to try again.
    match line.trim().parse::<u16>() {
        Ok(height) => height, // if successful return height
        Err(_) => {
            println!("Error: Please provide a valid floating-point number.");
            return get_height(); // recursive call.
        }
    }
}

//
// Calculate the circumference with PI*diameter.
//
fn calc_circumference(diameter: u16) -> f32 {
    let circumference = diameter as f32 * PI;
    return circumference;
}

//
// Caluclate how many miles a circle travels in one rotation
//
// # Paramaters: tire_circumference (in inches)
// # Returns: circumference of the tire in miles.
//
fn calc_miles_per_rotation(tire_circumference: f32) -> f32 {
    return tire_circumference / 63360.0; 
}

//
// Find the rate the wheels are turning from engine speed and final drive ratio
// using:         Wheel RPM = Engine RPM / Final Drive
//
fn calc_wheel_rpm(engine_rpm: i32, final_drive: f32 ) -> f32 {
    let wheel_rpm = engine_rpm as f32 / final_drive;
    return wheel_rpm;
}

//
// Find the final drive ratio given all the ratios.
//
fn calc_final_drive(trans_gear:f32, t_case_gear:f32, axle_gear: f32) -> f32 {
    let final_drive = trans_gear * t_case_gear * axle_gear;
    return final_drive;
}

//
// Get the gear ratio of a gear in your transmission as a floating point number.
//
fn get_trans_gear() -> f32 {
    // Declare line to write to.
    let mut line = String::new();
    
    print!("Enter your transmission gear ratio: ");
    // Ensures the prompt is printed before reading input
    io::stdout().flush().unwrap();  
    // Error check
    io::stdin().read_line(&mut line).expect("Failed to read line");
    // Try parseing the string into a f32, if not recursive call to try again.
    match line.trim().parse::<f32>() {
        Ok(gear_ratio) => gear_ratio, // if successful retrn value
        Err(_) => {
            println!("Error: Please provide a valid floating-point number.");
            return get_trans_gear(); // recursive call.
        }
    }
}

//
// Get the transfer case gear ratio as a floating point number.
//
fn get_t_case_gear() -> f32 {
    // Declare line to write to.
    let mut line = String::new();
    
    print!("Enter your Transfer case gear ratio: ");
    // Ensures the prompt is printed before reading input
    io::stdout().flush().unwrap();  
    // Error check
    io::stdin().read_line(&mut line).expect("Failed to read line");
    // Try parseing the string into a f32, if not recursive call to try again.
    match line.trim().parse::<f32>() {
        Ok(gear_ratio) => gear_ratio, // if successful retrn value
        Err(_) => {
            println!("Error: Please provide a valid floating-point number.");
            return get_t_case_gear(); // recursive call.
        }
    }
}

//
// Get the differential gear ratio as a floating point number.
//
fn get_axle_gear() -> f32 {
    // Declare line to write to.
    let mut line = String::new();
    
    print!("Enter your differential gear ratio: ");
    // Ensures the prompt is printed before reading input
    io::stdout().flush().unwrap();  
    // Error check
    io::stdin().read_line(&mut line).expect("Failed to read line");
    // Try parseing the string into a f32, if not recursive call to try again.
    match line.trim().parse::<f32>() {
        Ok(gear_ratio) => gear_ratio, // if successful retrn value
        Err(_) => {
            println!("Error: Please provide a valid floating-point number.");
            return get_axle_gear(); // recursive call.
        }
    }
}