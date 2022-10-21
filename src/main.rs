use std::collections::{BTreeSet, HashMap, HashSet};
fn main() {
    let (final_term, _final_not_term, productions) = get_terminal_and_not_terminal();
    let mut first_and_follows: HashMap<String, (HashSet<String>, HashSet<String>)> = HashMap::new();
    // print_terminal_and_not_terminals(&final_not_term, &final_term);

    for (key, value) in &productions {
        // println!("{} produces -> {:?}", key, value);
        first(
            &key,
            &value,
            &final_term,
            &mut first_and_follows,
            &productions,
        );
    }
    for (key, value) in &first_and_follows {
        print!("{} FIRST = {{", key);

        for (i, item) in value.0.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("{item}");
        }

        print!("}} , FOLLOWS = {{\n");
    }
}
fn first(
    not_term: &String,
    productions: &Vec<String>,
    terminals: &BTreeSet<String>,
    output_map: &mut HashMap<String, (HashSet<String>, HashSet<String>)>,
    productions_map: &HashMap<String, Vec<String>>,
) -> HashSet<String> {
    for production in productions {
        let front = production.split(' ').collect::<Vec<&str>>()[0].to_string();
        if terminals.contains(&front) {
            output_map
                .entry(not_term.clone())
                .and_modify(|(frst, _)| {_ = frst.insert(front.clone())})
                .or_insert((HashSet::from([front.clone()]), HashSet::new()));
        } else {
            let t = first(
                &front,
                productions_map.get(&front).unwrap(),
                &terminals,
                output_map,
                &productions_map,
            );
            output_map
                .entry(not_term.clone())
                .or_insert((t.clone(), HashSet::new()));
        }
    }
    output_map.get(not_term).unwrap().0.clone()
}
fn _print_terminal_and_not_terminals(
    final_not_term: &BTreeSet<String>,
    final_term: &BTreeSet<String>,
) {
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
fn get_terminal_and_not_terminal() -> (
    BTreeSet<String>,
    BTreeSet<String>,
    HashMap<String, Vec<String>>,
) {
    let mut final_term: BTreeSet<String> = BTreeSet::new();
    let mut final_not_term: BTreeSet<String> = BTreeSet::new();
    let mut productions: HashMap<String, Vec<String>> = HashMap::new();

    let mut line_nums = String::new();
    let num: u32;
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
        temp_str = temp_str.replace("\' \'", "#");
        let temp: Vec<&str> = temp_str.trim().split(&[' ', '\n']).collect();
        // println!("{:?}", temp);
        // println!("{:?}", temp_str);

        let left = temp_str.split("->").collect::<Vec<&str>>()[0]
            .trim()
            .to_string();
        // println!("term: {:?}", left);

        let right = temp_str
            .split("->")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .trim()
            .to_string();
        // println!("production: {:?}", right);

        productions
            .entry(left)
            .and_modify(|vs| vs.push(right.clone()))
            .or_insert(vec![right.clone()]);

        //old way of dividing into term and !term
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
    return (final_term, final_not_term, productions);
}
fn def_terminal_set() -> BTreeSet<String> {
    // 0 not terminal
    // 1 Terminal
    let mut set = BTreeSet::new();

    set.insert("+".to_string());
    set.insert("#".to_string());
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
