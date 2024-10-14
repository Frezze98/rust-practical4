fn main() {
    const SIZE: usize = 6;

    // Верхня частина ромба
    for i in 1..=SIZE {
        for _ in 0..(SIZE - i) {
            print!(" ");
        }
        for _ in 0..i {
            print!("* ");
        }
        println!();
    }

    // Нижня частина ромба
    for i in (1..SIZE).rev() {
        for _ in 0..(SIZE - i) {
            print!(" ");
        }
        for _ in 0..i {
            print!("* ");
        }
        println!();
    }
}