fn main() {
    let my_array = [5, 21, 3, 8, 11];

    let mut num = my_array[0];

    for i in my_array{
        if i > num {
            num = i;
            println!("{num}");
        }
        // println!("{}",i);
    }
}
