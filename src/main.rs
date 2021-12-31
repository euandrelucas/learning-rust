fn main() {
    let mut count = 0u32;
    loop {
        count += 1;
        println!("{}", count);
        if count == 9999999 {
            println!("OK, chega.");
            break;
        }
    }
}
