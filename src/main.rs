fn main() {
    let mut str = String::new();
    let arr = [1, 2, 3, 4, 5];
    std::io::stdin().read_line(&mut str)
        .expect("should input something");
    let idx: usize = str.trim()
        .parse()
        .expect("should input a number");
    println!("you chosen value is {}", arr[idx]);
}
