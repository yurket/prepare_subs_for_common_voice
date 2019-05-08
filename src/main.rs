use std::fs;


// TODO: remove "р-р-р"
fn check_string_conformance(s: &str) -> bool {
    if s.len() == 0 {
        println!("skipping due to zero length: {}", s);
        return false;
    }

    let contains_numbers = s.chars()
        .map(|c| c.is_numeric())
        .collect::<Vec<bool>>()
        .contains(&true);
    if contains_numbers {
        println!("skipping due to numbers: {}", s);
        return false;
    }

    if s.split_whitespace().count() > 14 {
        println!("skipping due to too many words (> 14): {}", s);
        return false;
    }

    let contains_english = s.chars()
        .map(|c| c.is_ascii_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&true);
    if contains_english {
        println!("skipping due to ascii alphabetic letters: {}", s);
        return false;
    }

    true
}


fn main() {
    let filename = "/home/lite/!_coding/rust/mozilla_common_voice_subs/Rebellion.S02E05.720p.WEB-DL.srt";
    // let filename = "test.srt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut chunks: Vec<String> = Vec::new();
    for chunk in content.split("\r\n\r\n") {
        let purely_numeric_strings_to_skip = 2;
        let t = chunk
            .lines()
            .skip(purely_numeric_strings_to_skip)
            .collect::<Vec<&str>>()
            .join(" ");
        // println!("{:?}", t);
        chunks.push(t);
    }

    let text = chunks.join(" ");
    // println!("{}", text);
    let mut sentence_candidates: Vec<&str> = Vec::new();
    let mut sentence_start = 0;
    for (i, _) in text.match_indices(|c| c == '.' || c == '?' || c == '!') {
        // println!("i: {}, match: {}", i, m);

        let candidate = text.get(sentence_start+2..i+1);
        match candidate {
            Some(x) => sentence_candidates.push(x),
            None    => println!("Couldn't parse match \"{:?}\"", candidate)
        }
        // println!("{:?}", candidate);
        sentence_start = i;
    }

    println!("Sentence candidates: {:?}", sentence_candidates);

    for s in sentence_candidates {
        if check_string_conformance(s) {
            println!("ok: {}", s);
        }
    }
}
