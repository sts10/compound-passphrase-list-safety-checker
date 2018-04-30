use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    // let (single_bad_words, double_bad_words) = split_and_search(make_vec("bad_word_test.txt"));
    let (single_bad_words, double_bad_words) = split_and_search(make_vec("agile_words.txt"));
    println!(
        "Recommend you remove: {:?}",
        find_words_to_remove(single_bad_words, double_bad_words)
    );
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

fn split_and_search(words: Vec<String>) -> (Vec<String>, Vec<Vec<String>>) {
    let mut single_bad_words: Vec<String> = [].to_vec();
    let mut double_bad_words: Vec<Vec<String>> = [].to_vec();
    for mut word in words {
        println!("Starting search of {}", word);
        let mut second_half = "".to_string();
        for _i in 0..word.len() {
            let length = &word.len();
            second_half = format!("{}{}", &word.split_off(length - 1), second_half);
            if search(&word) {
                println!("I found {} as its own word. second half is {} and I should search for that now", word, second_half);
                if search(&second_half) {
                    single_bad_words.push(word.to_string());
                    single_bad_words.push(second_half.to_string());
                    double_bad_words.push(vec![word.to_string(), second_half.to_string()]);
                }
            }
        }
    }
    println!("Here are all the bad words I found {:?}", single_bad_words);
    (single_bad_words, double_bad_words)
}

fn search(target_word: &str) -> bool {
    let words = make_vec("agile_words.txt");
    // let words = make_vec("bad_word_test.txt");
    for word in words {
        if target_word == word {
            return true;
        }
    }
    return false;
}

fn find_words_to_remove(
    single_bad_words: Vec<String>,
    double_bad_words: Vec<Vec<String>>,
) -> Vec<String> {
    let mut words_to_remove: Vec<String> = [].to_vec();
    for word_vec in double_bad_words {
        let mut first_word_appearances = 0;
        let mut second_word_appearances = 0;
        for word in &single_bad_words {
            if &word_vec[0] == word {
                first_word_appearances = first_word_appearances + 1;
            }
            if &word_vec[1] == word {
                second_word_appearances = second_word_appearances + 1;
            }
        }
        if first_word_appearances >= second_word_appearances {
            words_to_remove.push(word_vec[0].to_string());
        } else {
            words_to_remove.push(word_vec[1].to_string());
        }
    }

    words_to_remove.sort();
    words_to_remove.dedup();
    return words_to_remove;
}
