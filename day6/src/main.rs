use std::collections::HashMap;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input")?;
    let mut debug = String::new();
    let mut count = 0;
    let mut questions_answered = HashMap::<char, ()>::new();

    for line in input.lines() {
        if line == "" {
            count += questions_answered.len();
            debug.push_str(&format!(
                "Group questions answered: {}\n",
                questions_answered.len()
            ));
            questions_answered.clear();
        } else {
            for chr in line.chars() {
                if !questions_answered.contains_key(&chr) {
                    questions_answered.insert(chr, ());
                }
            }
        }
        debug.push_str(line);
        debug.push('\n');
    }
    count += questions_answered.len();
    debug.push_str(&format!(
        "Group questions answered: {}\n",
        questions_answered.len()
    ));

    println!("Total: {}", count);

    fs::write("debug", debug)?;

    Ok(())
}
