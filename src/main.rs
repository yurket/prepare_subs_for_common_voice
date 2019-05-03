use std::fs;

fn main() {
    let filename = "/home/lite/!_coding/rust/mozilla_common_voice_subs/Rebellion.S02E05.720p.WEB-DL.srt";
    let  content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{}", content);
}
