use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    println!("Hello, world!");

    println!("list {:?}", make_vec("eff-word-list.txt"));
    search_for_sum_words(make_vec("eff-word-list.txt"));
}

fn make_vec(filename: &str) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        word_list.push(l);
    }
    return word_list;
}

fn search_for_sum_words(word_list: Vec<String>) -> Vec<String> {
    let mut sum_words: Vec<String> = [].to_vec();
    for word in word_list {
        let length: usize = word.len();
        let vec: Vec<&str> = word.split("").collect();
        // (1) Iterate  as many times as length
        for i in 1..length {
            let first_split = vec[1..i + 1].join("");
            for word_to_check_first_split in &word_list {
                // (2) compare first split to every word in word list
                if &first_split == word_to_check_first_split {
                    // (3) if a match, immediately compare second split to
                    // every word in word list
                    let second_split = vec[i..length + 1].join("");
                    for word_to_check_second_split in &word_list {
                        if &second_split == word_to_check_second_split {
                            // (4) if find a match there, add to sum_words vec
                            sum_words.push(word);
                            // and (5) move to next word
                        }
                    }
                }
            }
        }
    }
    return sum_words;
}
