# Compound Passphrase List Safety Checker

This Rust scripts checks whether a given passphrase word list (such as a diceware word list) has any words that can be combined to make another word on the list.

Initially I wanted to make sure that no two words in [the EFF's long diceware word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) could be combined to make another word on the list. I later checked the list that 1Password uses.

## What is "compound-safety"? 

I made up the term. Here's what I mean by it: 

> A passphrase word list is "compound-safe" if it does NOT contain any pairs of words that can be combined to make another word on the list. 

For example, if a word list included "under", "dog", and "underdog" as three separate words, it would NOT be compound-safe, since "under" and "dog" can be combined to make the word "underdog".

## Why is this attribute of a passphrase word list notable? 

Let's say we're using the word list described above, which has "under", "dog" and "underdog" in it. A user might randomly get "under" and "dog" in a row, for example in the six-word passphrase "crueltyfrailunderdogcyclingapostle". The user might assume they had six words worth of entropy. But really, an attacker brute forcing their way through five-word passphrases would eventually crack the passphrase.

It's important to note that if the passphrase has any punctuation (for example, a period, comma, hyphen, space) between words, this issue goes away completely. "cruelty frail under dog cycling apostle" is indeed a six-word passphrase, and an attacker who tries "underdog" as the third word does not get a match.

To summarize: When creating passphrases without punctuation between the words with a word list that is NOT compound-safe, there's a small risk that users will put two words together that form another word on this list. When this happens, they lose one word's worth of entropy from their password. However if they used a compound-safe list, they can safely not use punctuation between words.

Again, it's super important to understand that putting a hyphen or space between the words ("cruelty frail under dog cycling apostle") eliminates this problem completely.

I heard of this potential issue in [this YouTube video](https://youtu.be/Pe_3cFuSw1E?t=8m36s). 

## What this script does

This Rust script takes a word list (as a text file) as an input. It then searches the given list for words that can be combined to make other words on the list.

Next, it attempts to find the smallest number of words that need to be removed in order to make the given word list "compound-safe". Finally, it prints out this new, shorter, compound-safe list to a new text file. In this way it makes word lists "compound-safe".

## How to use this script to check a word list

First you'll need to [install Rust](https://www.rust-lang.org/en-US/install.html). Make sure running the command `cargo --version` returns something that starts with something like `cargo 0.26.0`. 

Next, clone down this repo. To run the script, cd into the repo's directory and run:

```
cargo run <wordlist.txt>
```

This will create a file named `wordlist.txt.compound-safe` that is the compound-safe list of your word list (obviously may be shorter). 

You can also specify a specific output file location:

```
cargo run <wordlist-to-check.txt> <output.txt>
```


## Some initial findings

I did not find any compound-unsafe pairs of words in the [EFF long word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases). In other words, according to my script, the EFF long word list is compound-safe.

However, in the 1Password list (labeled `agile_words.txt` in my project, copy from [this 1Password challenge](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt)), I found 2,661 compound words (see: `scrap-lists-of-compound-words-and-components/agile_double_bad_words.txt`), made up of 1,511 unique bad single words (see: `scrap-lists-of-compound-words-and-components/agile_single_bad_words.txt`). 

NOTE: 1Password's software, as far as I know, does NOT allow users to generate random passphrase without punctuation between words. Users _must_ choose to separate words with a period, hyphen, space, comma, or underscore. So these findings do NOT constitute a security issue with 1Password.

## 1Password: An example and suggestion

The aim of the `find_words_to_remove` function is to remove the fewest number of these bad words to make the list compound-safe. When I ran it on the 1Password wordlist, I got 498 words back, which I dumped in to `scrap-lists-of-compound-words-and-components/words_to_remove_from_agile_list.txt`. 

Then the `make_clean_list` function removes these 498 words, giving us the list found in `compound-safe_word_lists/agile_words_compound-safe.txt`, a list of 17,830 words compound-safe words.

Now, we should note that reducing the length of the list from 18,328 words to 17,830 has a cost. Given 1Password's current list of 18,328 words, when a user adds one of these words to their passphrase, they're adding about 14.162 bits of entropy to their passphrase. Using the shortened, compound-safe 17,830 word list, each randomly generated word would add about 14.122 bits to the passphrase. Of course, Agile Bits/1Password could replace the 498 words while keeping the list compound-safe.


