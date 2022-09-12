use std::collections::BTreeSet;
fn main() {
    let mut line_nums = String::new();
    let num: u32;
    let mut final_term: BTreeSet<String> = BTreeSet::new();
    let mut final_not_term: BTreeSet<String> = BTreeSet::new();
    let terminals = def_terminal_set();

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
                t if terminals.contains(&t.to_string()) => {
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
fn print_res(final_not_term: &BTreeSet<String>, final_term: &BTreeSet<String>) {
    print!("Terminales: ");
    for (i, item) in final_term.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{item}");
    }
    print!("\nNo Terminales: ");
    for (i, item) in final_not_term.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{item}");
    }
    println!();
}

fn def_terminal_set() -> BTreeSet<String> {
    // 0 not terminal
    // 1 Terminal
    let mut set = BTreeSet::new();

    set.insert("+".to_string());
    set.insert("-".to_string());
    set.insert("*".to_string());
    set.insert("/".to_string());
    set.insert("%".to_string());
    set.insert("a".to_string());
    set.insert("b".to_string());
    set.insert("c".to_string());
    set.insert("d".to_string());
    set.insert(".".to_string());
    set.insert(",".to_string());
    set.insert(";".to_string());
    set.insert("(".to_string());
    set.insert(")".to_string());
    set.insert("{".to_string());
    set.insert("}".to_string());
    set.insert("0".to_string());
    set.insert("1".to_string());
    set.insert("2".to_string());
    set.insert("3".to_string());
    set.insert("4".to_string());
    set.insert("5".to_string());
    set.insert("6".to_string());
    set.insert("7".to_string());
    set.insert("8".to_string());
    set.insert("9".to_string());
    set.insert("id".to_string());
    set.insert("if".to_string());
    set.insert("true".to_string());
    set.insert("false".to_string());
    set.insert("and".to_string());
    set.insert("or".to_string());
    set.insert("not".to_string());

    set
}
