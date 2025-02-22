enum Weather {
    Sunny,
    Rainy(i32),
    Cloudy(Empty)
}

struct Empty;

fn main () {
    let today = Weather::Rainy(15);

    match today {
        Weather::Sunny => println!("It's sunny"),
        Weather::Cloudy(Empty) => println!("It is weather shit"),
        Weather::Rainy(amount) => println!("It is raining {} mm",amount),
    }
}