fn main() {
    let mut x = 5;
    { // Scope the mutable borrow
        let y = &mut x;
        *y += 1;
    }
    { // Scope the mutable borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
//Alternative solution using cloning
fn main() {
    let mut x = 5;
    let mut y = x;
    let mut z = x;
    y += 1;
    z += 1;
    x = y + z -10;
    println!("x = {}", x);
}