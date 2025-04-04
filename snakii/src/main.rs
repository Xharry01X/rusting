mod lib;


fn main(){
    let addi = lib::add(3, 2);
    let subbi = lib::sub(5, 2);

    println!("{}",addi);
    println!("{}",subbi);
}


