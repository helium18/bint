use std::{env, fs::File, path::Path, io::Read, collections::HashMap};

const OP_EQ: &str = "=";

fn main() {
    let cts = strip_cmts(&script_rd());
    let vars = variables(&cts);
    // run(vars, &cts);
    println!("{vars:#?}");
}

// fn run(vars: HashMap<String, String>, cts: &str) {}

fn variables(cts: &str) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    for i in cts.lines() {
        for w in i.split_whitespace() {
            if w.contains(OP_EQ) {
                let mut expr = w.split(OP_EQ);
                assert!(expr.nth(0).is_some() && expr.nth(1).is_some()); // `expr` needs two sides to be mapped 
                vars.insert(
                    expr.nth(0).unwrap().to_string(), 
                    expr.nth(1).unwrap().to_string(),
                );
            }
        } 
    }
    vars
}

fn strip_cmts(cts: &str) -> String {
    const CMT: &str = "#";
    const NL: &str = "\n";
    let mut res = String::new();

    for i in cts.lines() {
        for w in i.split_whitespace() {
            match w {
                w if w.starts_with(CMT) => break,
                _ => res += &(w.to_owned() + " "),
            }
        } 
        res += NL;
    };

    res.trim().to_string()
}

fn script_rd() -> String {
    let args = env::args();

    let path = args.into_iter().nth(1).unwrap(); 
    let path = Path::new(&path); 

    let mut script = File::open(path).expect(&format!("cannot open {:?}", path));
    let mut contents = String::new();

    script.read_to_string(&mut contents).expect("cannot read script to string");

    contents
}
