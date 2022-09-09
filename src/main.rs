use std::collections::HashMap;
fn main() {
    let mut line_nums = String::new();
    let num: u32;
    match std::io::stdin().read_line(&mut line_nums) {
        Ok(_) => {
            num = line_nums.trim().parse::<u32>().expect("Not valid input");
        }
        Err(error) => {
            panic!("{error}");
        }
    };
    // let num:u32 = match line_nums.trim().parse::<u32>() {
    //     Ok(i) => i,    //     Err(_) => panic!("Not a valid input"),
    //     Err(_) => panic!("Not a valid input"),
    // };
    println!("The num is  {num}");
    for _ in 0..num {
        let mut temp_str = String::new();
        std::io::stdin()
            .read_line(&mut temp_str)
            .expect("failed to read from stdin");
        let temp: Vec<&str> = temp_str.trim().split(&[' ', '\n']).collect();
        println!("{:?}", temp);
    }
}
fn def_terminal_map() -> HashMap<String, bool> {
    // 0 not terminal
    // 1 Terminal
    HashMap::from([
        ("+".to_string(), true),
        ("-".to_string(), true),
        ("*".to_string(), true),
        ("/".to_string(), true),
        ("%".to_string(), true),
        ("a".to_string(), true),
        ("b".to_string(), true),
        ("c".to_string(), true),
        (".".to_string(), true),
        (",".to_string(), true),
        (";".to_string(), true),
        ("(".to_string(), true),
        (")".to_string(), true),
        ("{".to_string(), true),
        ("}".to_string(), true),
        ("0".to_string(), true),
        ("1".to_string(), true),
        ("2".to_string(), true),
        ("3".to_string(), true),
        ("4".to_string(), true),
        ("5".to_string(), true),
        ("6".to_string(), true),
        ("7".to_string(), true),
        ("8".to_string(), true),
        ("9".to_string(), true),
        ("id".to_string(), true),
        ("if".to_string(), true),
    ])
}
