use std::fs;

use colored::*;

fn check_string_conformance(s: &str) -> bool {
    if s.len() == 0 {
        println!("{}: {}", "skipping due to zero length".yellow(), s);
        return false;
    }

    let contains_numbers = s.chars()
        .map(|c| c.is_numeric())
        .collect::<Vec<bool>>()
        .contains(&true);
    if contains_numbers {
        println!("{}: {}", "skipping due to numbers".yellow(), s);
        return false;
    }

    let word_count = s.split_whitespace().count();
    if word_count < 3 {
        println!("{}: {}", "skipping due to too few words (< 3)".yellow(), s);
        return false;
    } else if word_count > 14 {
        println!("{}: {}", "skipping due to too many words (> 14)".yellow(), s);
        return false;
    }

    let contains_english = s.chars()
        .map(|c| c.is_ascii_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&true);
    if contains_english {
        println!("{}: {}", "skipping due to ascii alphabetic letters".yellow(), s);
        return false;
    }

    if s.starts_with("- ") {
        println!("{}: {}", "skipping due direct speech".yellow(), s);
        return false;
    }

    if s.contains("р-р-р") {
        println!("{}: {}", "skipping due \"р-р-р\"".red(), s);
        return false;
    }

    println!("ok: {}", s);
    true
}

fn save_valid_sentences(filename: &str, valid_sentences: Vec<&str>) {
    fs::write(filename, valid_sentences.join("\n"));
}

fn main() {
    let filename = "/home/lite/!_coding/rust/mozilla_common_voice_subs/Rebellion.S02E05.720p.WEB-DL.srt";
    // let filename = "test.srt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut chunks: Vec<String> = Vec::new();
    for subtitle_chunk in content.split("\r\n\r\n") {
        let metainfo_lines_to_skip = 2;
        let t = subtitle_chunk
            .lines()
            .skip(metainfo_lines_to_skip)
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

    let mut valid_sentences: Vec<&str> = Vec::new();
    for s in sentence_candidates {
        if check_string_conformance(s) {
            valid_sentences.push(s);
        }
    }

    save_valid_sentences("out.txt", valid_sentences);
}
