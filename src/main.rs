use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let mut line_nums = String::new();
    let num: u32;
    let mut final_term: HashSet<String> = HashSet::new();
    let mut final_not_term: HashSet<String> = HashSet::new();
    let terminals = def_terminal_map();

    match std::io::stdin().read_line(&mut line_nums) {
        Ok(_) => {
            num = line_nums.trim().parse::<u32>().expect("Not valid input");
        }
        Err(error) => {
            panic!("{error}");
        }
    };

    for _ in 0..num {
        let mut temp_str = String::new();
        std::io::stdin()
            .read_line(&mut temp_str)
            .expect("failed to read from stdin");
        let temp: Vec<&str> = temp_str.trim().split(&[' ', '\n']).collect();
        // println!("{:?}", temp);
        for s in temp {
            // println!("{s}");
            match s {
                "->" => (),
                // "expr" => final_not_term.insert(s.to_string()),
                "expr" | "smt" => {
                    let _ = final_not_term.insert(s.to_string());
                }
                t if terminals.contains_key(&t.to_string()) => {
                    final_term.insert(s.to_string());
                }
                t if t.chars().all(|x| x.is_lowercase()) => {
                    final_not_term.insert(s.to_string());
                }
                t if t.chars().all(|x| x.is_uppercase()) => {
                    final_not_term.insert(s.to_string());
                }
                t if t == '\''.to_string() => (),
                _ => {
                    final_not_term.insert(s.to_string());
                }
            }
        }
    }

    print_res(&final_not_term, &final_term);
}
fn print_res(final_not_term: &HashSet<String>, final_term: &HashSet<String>) {
    print!("No Terminales: ");
    for i in final_not_term {
        print!("{i} ");
    }
    println!();
    print!("Terminales: ");
    for i in final_term {
        print!("{i} ");
    }
    println!();
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
        ("true".to_string(), true),
        ("false".to_string(), true),
    ])
}
