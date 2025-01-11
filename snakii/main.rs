

fn main() {
    let age = 12;
    finding_age(age);
}

fn finding_age(age: i32) {
    for n in 1..=15 {
        if n > age {
            println!("you are senior to me: {}", n);
        }else if n == age {
            println!("your age is similar to me: {}", n);
        }else {
            println!("You are junior to  me: {}", n);
        }
    }
}
