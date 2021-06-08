use std::io;

fn main() {
    let mut scale = String::new();
    let mut temperature = String::new();

    while scale.trim() != "C" && scale.trim() != "F"{
        println!("Please input C if your number is in Celcius and F if your number is in Fahrenheit.");

        io::stdin()
            .read_line(&mut scale)
            .expect("Couldn't parse whether it was Celcius or Fahrenheit");

    }

    println!("Please input your temperature (as an integer).");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Couldn't parse the temperature");

    //let temperature: i32 = match temperature.trim().parse() {
    //    Ok(num) => num,
    //    Err(_) => continue,
    //};

    if scale.trim() == "C"{
        println!("Celcius");
    }else{
        println!("Fahrenheit");
    }
}
