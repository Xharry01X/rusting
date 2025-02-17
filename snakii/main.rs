struct Temperature {
    degrees_celsius: f64,
}

impl Temperature {
    // Constructor that ensures the temperature is within a valid range
    fn new(degrees_celsius: f64) -> Result<Self, String> {
        if (-273.15..=1_000.0).contains(&degrees_celsius) {
            Ok(Self { degrees_celsius })
        } else {
            Err(String::from("Temperature must be between -273.15 and 1000.0 degrees Celsius."))
        }
    }
}

fn main() {
    match Temperature::new(-300.0) {
        Ok(temp) => println!("Temperature set to {}°C", temp.degrees_celsius),
        Err(e) => println!("Error: {}", e),
    }
}
