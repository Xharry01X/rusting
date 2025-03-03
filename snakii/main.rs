#[derive(PartialEq, Debug)]
struct Student {
    id: i32,
    grade: char,
}

fn find_match<T: PartialEq>(a: T, b: T) -> bool {
    a == b 
}

fn main() {
    let student1 = Student { id: 101, grade: 'A' };
    let student2 = Student { id: 101, grade: 'A' };

    let are_same = find_match(student1, student2);

    println!("Are they the same? {}", are_same); 
}