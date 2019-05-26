use std::env;
use std::fs;
use std::path::Path;

use colored::*;


fn count_quotes(s: &str) -> usize {
    s.chars().filter(|&c| c == '"').collect::<Vec<char>>().len()
}

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
        println!("{}: {}", "skipping due \"р-р-р\"".yellow(), s);
        return false;
    }

    if s.contains("…") {
        println!("{}: {}", "skipping due \"…\"".yellow(), s);
        return false;
    }

    if count_quotes(s) % 2 != 0 {
        println!("{}: {}", "skipping due to odd number of quotes".yellow(), s);
        return false;
    }

    println!("ok: {}", s);
    true
}

fn save_valid_sentences(filename: &str, valid_sentences: Vec<&str>) {
    fs::write(filename, valid_sentences.join("\n"));
}

fn collect_valid_sentences_from_srt(filename: &str) {
    assert!(!filename.is_empty(), "Error: Empty filename");

    let path = Path::new(filename).canonicalize().unwrap();
    assert!(path.is_file(), "Error: Wrong file type");

    let content = fs::read_to_string(&path).expect("Something went wrong reading the file");

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

    let out_filename = path.file_stem().unwrap().to_str().unwrap().to_owned() + "_out.txt";
    let out_path = path.with_file_name(out_filename);
    save_valid_sentences(&out_path.to_str().unwrap(), valid_sentences);
}

fn main() {
    collect_valid_sentences_from_srt(&env::args().nth(1).unwrap());
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    #[should_panic(expected = "Error: Empty filename")]
    fn test_empty_filename() {
        collect_valid_sentences_from_srt("")
    }

    #[test]
    #[should_panic]
    fn test_file_not_exists() {
        let filename = "some_non_existant_filename";
        collect_valid_sentences_from_srt(&filename)
    }

    #[test]
    #[should_panic(expected = "Error: Wrong file type")]
    fn test_dir() {
        let filename = "/tmp";
        collect_valid_sentences_from_srt(&filename)
    }

    #[test]
    fn test_odd_quotes_number() {
        let s = r#"При встрече с тобой я поняла, что все эти годы любила только тебя"."#;
        assert_eq!(check_string_conformance(s), false)
    }
}
