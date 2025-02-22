struct Empty;

enum Action {
    DoNothing(Empty),
    DoSomething(i32)
}

fn main() {
    let action = Action::DoNothing(Empty);

    match action {
        Action::DoNothing(_) => println!("Do nothing!"),
        Action::DoSomething(num) => println!("DoSomething with {}",num),
    }
}