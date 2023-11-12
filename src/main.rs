use std::io::stdin;

fn main() {
    let mut f_degrees  = String::new();
    loop {
    println!("please input the farenheit degrees you wanted to convert.");
        stdin()
            .read_line(&mut f_degrees)
            .expect("failed to read farenheit");

        let f_degrees:i32 = f_degrees.trim()
            .parse()
            .expect("please type a number");

        let conversion_to_c: i32 = (f_degrees - 32) * 5/9;
        println!("the celcius value is : {conversion_to_c}");
    }
}
