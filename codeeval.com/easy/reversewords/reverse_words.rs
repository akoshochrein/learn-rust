
use std::io;

fn main() {
    
    let line = io::stdin().read_line().ok().expect("FAIL");
    let words: Vec<&str> = line.as_slice().split(' ').collect();

    let words_len = words.len();
    let mut reverse_words = "".to_string();
    for i in range(0, words_len) {
        reverse_words = reverse_words + words[words_len-i-1].trim() + " ";
    }

    println!("{}", reverse_words);
}
