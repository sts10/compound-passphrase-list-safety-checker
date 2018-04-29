use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    // println!(
    //     "Mashed word list is {:?}",
    //     mash_words(make_vec("eff-word-list.txt"))
    // );

    // println!(
    //     "Bad words are {:?}",
    //     search(
    //         make_vec("eff-word-list.txt"),
    //         mash_words(make_vec("eff-word-list.txt"))
    //     )
    // );
    split_and_search(make_vec("agile_words.txt"));
    // split_and_search(make_vec("bad_word_test.txt"));
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

fn mash_words(word_list: Vec<String>) -> Vec<String> {
    let mut mashed_words: Vec<String> = [].to_vec();

    for first_word in &word_list {
        for second_word in &word_list {
            // should only push if word is same length or shorter than the longest word on the list
            mashed_words.push(first_word.to_owned() + &second_word);
        }
    }

    return mashed_words;
}

// fn search(word_list: Vec<String>, mashed_words_list: Vec<String>) -> Vec<String> {
//     let mut bad_words: Vec<String> = [].to_vec();
//     for mashed_word in mashed_words_list {
//         for word in &word_list {
//             if word == &mashed_word {
//                 bad_words.push(word.to_string());
//             }
//         }
//         println!("Done with {}", mashed_word);
//     }
//     return bad_words;
// }

fn split_and_search(words: Vec<String>) {
    let mut bad_words: Vec<String> = [].to_vec();
    for mut word in words {
        println!("Starting search of {}", word);
        let mut second_half = "".to_string();
        for i in 0..word.len() {
            let length = &word.len();
            second_half = format!("{}{}", &word.split_off(length - 1), second_half);
            if (search(&word)) {
                println!("I found {} as its own word. second half is {} and I should search for that now", word, second_half);
                if (search(&second_half)) {
                    bad_words.push(word.to_string());
                    bad_words.push(second_half.to_string());
                }
            }
        }
    }
    println!("Here are all the bad words I found {:?}", bad_words);
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
