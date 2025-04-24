
fn return_five() -> i32 {
    5
}

fn main() {
    let x:i32 = return_five();

    let new_value = plus_one(x);
    println!("The value of x is {new_value}");

}

fn plus_one(x:i32) -> i32 {
    x+1
}
