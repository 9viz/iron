fn main() {
    let line: String = String::from("static const char *colors[SchemeLast][2] = {");

    let matches: Vec<_> = line.match_indices(char::is_numeric).collect();
    let mut prev_index = 0;

    for m in matches.iter() {
        // get the index number
        let ind = m.0;
        // get the matching character
        let mut ch = String::from(m.1);

        // check if the previous char is an X
        let prev_ch = line.get(ind-1..ind).unwrap_or("");

        // if it is not, skip the iteration
        if prev_ch != "X" {
            continue;
        }

        // get the next char and see if it is numeric
        let next_ch = line.get(ind+1..ind+2).unwrap_or("");
        let next_ch: Vec<_> = next_ch.matches(char::is_numeric).collect();

        // if it is numeric, then set the approriate prev_index and skip
        // the iteration
        if next_ch.len() > 0 {
            prev_index = ind;
            continue;
        }

        // check if the previous index is the one we stored
        // if it is, then it means we have to prepend it
        if ind-1 == prev_index {
            ch = String::from(format!("{}{}", line.get(prev_index..ind).unwrap(), ch));
            //println!("holla");
        }

        // update the prev_index
        prev_index = ind;

        println!("{}", ch);
    }
}
