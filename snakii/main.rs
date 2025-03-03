

#[derive(Clone)]

struct Inventory {
    items: Vec<String>,
    count: i32
}

fn duplicate_inventory<T: Clone>(inv: T) -> (T,T){
  
  let inv2 = inv.clone();
  (inv,inv2)
}

fn main(){
    let stock = Inventory{ items: vec![String::from("clothes"), String::from("Headphones")],count: 2 };

     let (original, copy) = duplicate_inventory(stock);
     println!(" original: {} items, Copy: {} items", original.count, copy.count);
}