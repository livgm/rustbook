use std::io;

fn main() {
    let mut scale = String::new();
    let mut temperature = String::new();

        println!("Please input C if your number is in Celcius and F if your number is in Fahrenheit.");

        match io::stdin().read_line(&mut scale){
            Ok(_) => {}
            Err(_) => {
                println!("Couldn't parse the scale.");
                return;
            }
        }

    println!("Please input your temperature (as an integer).");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Couldn't parse the temperature");

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Couldn't parse the temperature");
                    return;}
    };

    if scale.trim() == "C"{
        println!("{}° Celcius are equivalent to {}° Fahrenheit.", temperature, ctof(temperature));
    }else if scale.trim() == "F"{
        println!("{}° Fahrenheit are equivalent to {}° Celcius.", temperature, ftoc(temperature));
    }else{
        println!("Not a valid scale.");
    }
}

fn ctof(t: f32) -> f32{
    t*1.8 + 32.0
}

fn ftoc(t: f32) -> f32{
    (t-32.0)/1.8
}
