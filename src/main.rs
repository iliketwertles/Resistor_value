use std::env::args;

fn get_value(args: &Vec<String>) -> i64 {
    let mut total = String::new();
    let mut count = 0;
    for arg in &args[1..3] {
        let value = match arg.as_str() {
            "brown" => 1,
            "red" => 2,
            "orange" => 3,
            "yellow" => 4,
            "green" => 5,
            "blue" => 6,
            "violet" => 7,
            "grey" => 8,
            "white" => 9,
            _ => panic!("not a valid color"),
        };
        count +=1;
        total += &value.to_string();
        if count == 2 {
            return total.parse::<i64>().unwrap();
        }
    }
    return -999
}

fn get_xten(color: &str) -> i64 {
    let val = match color {
        "brown" => 1,
        "red" => 2,
        "orange" => 3,
        "yellow" => 4,
        "green" => 5,
        "blue" => 6,
        "violet" => 7,
        "grey" => 8,
        "white" => 9,
        _ => panic!("not a valid color"),
    };
    return val;
}

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        4 | 5 => {
            let mut per = 0;
            if args.len() == 4 {
                let _ = per += 20;
            } else if args.len() == 5 {
                let peradd = match args[4].as_str() {
                    "silver" => 10,
                    "gold" => 5,
                    "red" => 2,
                    _ => panic!("Invalid percentage color")
                };
                let _ = per += peradd;
            }
            let value: i64 = get_value(&args);
            let power: i64 = get_xten(args[3].as_str());
            let res = value * 10i64.pow(power.try_into().unwrap());
            println!("Resistance: {}Ω\nAccuracy: {}%", res, per)

        },
        _ => println!("Incorrect amount of args"),
    }

}
