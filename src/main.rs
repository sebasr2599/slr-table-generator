use indexmap::{IndexMap, IndexSet};
use regex::Regex;
use std::collections::HashSet;
static EPSILON: &str = "#";
//to fix, change from hashmap, hashset to linked structures
fn main() {
    let (final_term, final_not_term, productions) = get_terminal_and_not_terminal();
    let mut first_and_follows: IndexMap<String, (HashSet<String>, HashSet<String>)> =
        IndexMap::new();
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
    // if its the first follow add $
    first_and_follows
        .entry(final_not_term.iter().next().unwrap().clone())
        .and_modify(|tup|{ 
            _ = tup.1.insert("$".to_string())
        });

    // println!("\nPrinting map of productions");
    let inverted_production_map: IndexMap<String, String> = inverted_production_map(&productions);
    // for (key, value) in &inverted_production_map {
    //     println!("Production {} produced by <- {}", key, value);

    // }

    // println!("\nFollows process");
    for b in &final_not_term {
        _ = follow(
            &b,
            &inverted_production_map,
            &mut first_and_follows,
            &final_not_term,
        );
    }
    print_first_and_follows(&first_and_follows);
}

fn follow(
    b: &String,
    map: &IndexMap<String, String>,
    first_and_follows: &mut IndexMap<String, (HashSet<String>, HashSet<String>)>,
    final_not_term: &IndexSet<String>,
) -> HashSet<String> {
    let formatted = format!(r"(.*) \b{}\b (.*)", b.to_string()); // A = αBβ
    let re_1 = Regex::new(formatted.as_str()).unwrap();
    let formatted = format!(r"\b{}\b (.*)", b.to_string()); // A = Bβ
    let re_1_2 = Regex::new(formatted.as_str()).unwrap();
    let formatted = format!(r"(.*) \b{}\b", b.to_string()); // A = αB
    let re_2 = Regex::new(formatted.as_str()).unwrap();

    // println!("\nChecking for {b}");
    for (prod, a) in map {
        // println!("Checking for th B: {b} in prod: {prod} gets created by: {a}");
        if re_1.is_match(prod) || re_1_2.is_match(prod) {
            // println!("{prod} matches rule 1");
            //make into array
            let arr = prod.split(' ').collect::<Vec<&str>>();
            let pos = arr.iter().position(|&x| x == b).unwrap() + 1;
            let beta = arr[pos].to_string();
            //check if beta its not terminal
            if final_not_term.contains(&beta) {
                // println!("{beta} is not terminal");
                //check if the first of beta contains EPSILON
                if first_and_follows.get(&beta).unwrap().0.contains(EPSILON) {
                    // println!("beta contains EPSILON");
                    //add the first of beta except EPSILON in the follow of b
                    //get the frist of beta
                    let aux = first_and_follows.get(&beta).unwrap().0.clone(); //returns a set of the first of beta
                                                                               //just add the first of beta is the follow of b
                    first_and_follows.entry(b.to_string()).and_modify(|tup| {
                        tup.1.extend(aux.clone());
                    });
                    // add the follow of the producer A to follow of b

                    // Check if follow of A has been calculted before?
                    if !first_and_follows.get(a).unwrap().1.is_empty() {
                        // if so, just append it
                        // println!("The follow of A {a} is not empty!");
                        let aux = first_and_follows.get(a).unwrap().1.clone(); //returns a set of the first of beta
                                                                               // println!("appending the the first of A: to the follow of B {:?}", aux);
                        first_and_follows.entry(b.to_string()).and_modify(|tup| {
                            tup.1.extend(aux);
                        });
                    } else {
                        // else append the calculation of the follow of A to the follow of B
                        // println!("The follow of A {a}  empty!");
                        let aux = follow(a, map, first_and_follows, final_not_term);
                        first_and_follows.entry(b.to_string()).and_modify(|tup| {
                            tup.1.extend(aux);
                        });
                    }
                } else {
                    // println!("beta does not contains EPSILON");
                    //get the frist of beta
                    let aux = first_and_follows.get(&beta).unwrap().0.clone(); //returns a set of the first of beta
                                                                               //just add the first of beta is the follow of b
                    first_and_follows.entry(b.to_string()).and_modify(|tup| {
                        tup.1.extend(aux);
                    });
                }
            } else {
                first_and_follows
                    .entry(b.to_string())
                    .and_modify(|tup|{
                        _ = tup.1.insert(beta.to_string())
                    });
            }
        } else if re_2.is_match(prod) {
            // println!("{prod} matches rule 2");
            // else append the calculation of the follow of A to the follow of B
            // Check if follow of A has been calculted before?
            if !first_and_follows.get(a).unwrap().1.is_empty() {
                // if so, just append it
                // println!("The follow of A {a} is not empty!");
                let aux = first_and_follows.get(a).unwrap().1.clone(); //returns a set of the first of beta
                // println!("appending the the follow of A {a}: to the follow of B{b} {:?}",aux);
                first_and_follows.entry(b.to_string()).and_modify(|tup| {
                    tup.1.extend(aux);
                });
            } else {
                // else append the calculation of the follow of A to the follow of B
                // println!("The follow of A {a}  empty!");
                let aux = follow(a, map, first_and_follows, final_not_term);
                first_and_follows.entry(b.to_string()).and_modify(|tup| {
                    tup.1.extend(aux);
                });
            }
        }
    }
    first_and_follows.get(b).unwrap().1.clone()
}
fn inverted_production_map(
    productions_map: &IndexMap<String, Vec<String>>,
) -> IndexMap<String, String> {
    let mut out: IndexMap<String, String> = IndexMap::new();
    for (key, value) in productions_map {
        for str_vec in value {
            out.entry(str_vec.to_string()).or_insert(key.to_string());
            // println!("{str_vec} -> {key}");
        }
    }
    out
}
fn first(
    not_term: &String,
    productions: &Vec<String>,
    terminals: &IndexSet<String>,
    output_map: &mut IndexMap<String, (HashSet<String>, HashSet<String>)>,
    productions_map: &IndexMap<String, Vec<String>>,
) -> HashSet<String> {
    for production in productions {
        let front = production.split(' ').collect::<Vec<&str>>()[0].to_string();
        if terminals.contains(&front) {
            output_map
                .entry(not_term.clone())
                .and_modify(|(frst, _)|{ _ = frst.insert(front.clone())
                })
                .or_insert((HashSet::from([front.clone()]), HashSet::new()));
        } else {
            // println!("Checking for A {not_term} in production {production} front {front}");
            if front != not_term.to_string() {
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
    }
    output_map.get(not_term).unwrap().0.clone()
}
fn get_terminal_and_not_terminal() -> (
    IndexSet<String>,
    IndexSet<String>,
    IndexMap<String, Vec<String>>,
) {
    let mut final_term: IndexSet<String> = IndexSet::new();
    let mut final_not_term: IndexSet<String> = IndexSet::new();
    let mut productions: IndexMap<String, Vec<String>> = IndexMap::new();

    let mut line_nums = String::new();
    let num: u32;
    // let terminals = def_terminal_set();
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
        temp_str = temp_str.replace("\' \'", EPSILON);
        // let temp: Vec<&str> = temp_str.trim().split(&[' ', '\n']).collect();
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
            .entry(left.to_string())
            .and_modify(|vs| vs.push(right.clone()))
            .or_insert(vec![right.clone()]);

        final_not_term.insert(left.to_string());
        let aux: Vec<&str> = right.split(' ').collect();
        for x in aux {
            final_term.insert(x.to_string());
        }
    }
    for x in &final_not_term {
        if final_term.contains(x) {
            let i = x.clone();
            _ = final_term.shift_remove(&i);
        }
    }
    return (final_term, final_not_term, productions);
}
fn _print_terminal_and_not_terminals(
    final_not_term: &IndexSet<String>,
    final_term: &IndexSet<String>,
) {
    print!("Terminales: ");
    for (i, item) in final_term.iter().enumerate() {
        if i != 0 {
            print!(",");
        }
        if item == EPSILON {
            print!("\' \'");
        } else {
            print!("{item}");
        }
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
fn print_first_and_follows(
    first_and_follows: &IndexMap<String, (HashSet<String>, HashSet<String>)>,
) {
    for (key, value) in first_and_follows {
        print!("{} FIRST => {{", key);

        for (i, item) in value.0.iter().enumerate() {
            if i != 0 {
                print!(",");
            }
            if item == EPSILON {
                print!("\' \'");
            } else {
                print!("{item}");
            }
        }

        print!("}}, FOLLOWS = {{");

        for (i, item) in value.1.iter().enumerate() {
            if i != 0 {
                print!(",");
            }
            if item != EPSILON {
                print!("{item}");
            }
        }

        println!("}}");
    }
}
