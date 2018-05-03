# Compound Passphrase List Safety Checker

This command line tool checks whether a given passphrase word list (such as a diceware word list) has any words that can be combined to make another word on the list. It's written in Rust, which I am new to, so please use this with caution.

Initially I wanted to make sure that no two words in [the EFF's long diceware word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) could be combined to make another word on the list. I later checked other lists.

**Disclosure**: I am not a professional researcher or statistician, and frankly I'm pretty fuzzy on some of this math. This code/theory/explanation could be very wrong (but hopefully not harmful?). If you think it could be wrong or harmful, please leave an issue! 

Further disclosure: see "Known issue" and "Caveat" sections below.

## What is "compound-safety"? 

I made up the term. Here's what I mean by it: A passphrase word list is "compound-safe" if it...

1. does NOT contain any pairs of words that can be combined to make another word on the list. (We'll call this a "compounding")

2. does Not contain any pairs of words that can be combined such that they can be guessed in two distinct ways (We'll call this a "problematic overlap").

## Brief examples of each of these conditions being violated

**An example of condition #1**: If a word list included "under", "dog", and "underdog" as three separate words, it would NOT be compound-safe, since "under" and "dog" can be combined to make the word "underdog". A user not using spaces between words might get a passphrase that included the character string "underdog" as two words, but a brute-force attack would guess it as one word. Therefore this word list would NOT be compound-safe. (I refer to this as a "compounding".)

I heard of this potential issue in [this YouTube video](https://youtu.be/Pe_3cFuSw1E?t=8m36s). 

**An example of condition #2**: Let's say a word list included "paper", "paperboy", "boyhood", and "hood". A user not using spaces between words might get the following two words next to each other in a passphrase: "paperboyhood", which would be able to be brute-force guessed as both `[paperboy][hood]` and `[paper][boyhood]`. Therefore this word list would NOT be compound-safe. (I call this a "problematic overlap".)

## Why is the compound-safety of a passphrase word list notable? 

Let's say we're using the word list described above, which has "under", "dog" and "underdog" in it. A user might randomly get "under" and "dog" in a row, for example in the six-word passphrase "crueltyfrailunderdogcyclingapostle". The user might assume they had six words worth of entropy. But really, an attacker brute forcing their way through five-word passphrases would eventually crack the passphrase. We can call this event "a compounding".

Likewise if we got the 6-word phrase "divingpaperboyhoodemployeepastelgravity", an attacker running through six-word combinations would have two chances of guessing "paperboyhood" rather than one.

**It's important to note** that if the passphrase has any punctuation (for example, a period, comma, hyphen, space) between words, both of these issues go away completely. If our passphrase is "cruelty under dog daylight paper boyhood": (1) an attacker who tries "underdog" as the third word does not get a match, (2) and the attacker likewise does not get a match if "paperboy" is guessed in the fifth slot and "hood is guessed as the sixth.

## Are compound-safe passphrases "stronger" or "better" than non-compound-safe passphrases?

Is "crueltyfrailunderdogcyclingapostle" a "weaker" passphrase than a 6-word phrase that does not have a compounding in it? Honestly I'm not sure. 

But **if an attacker knew your passphrase was 6 words, I'm not sure if a phrase with a compounding is "worse" (i.e. going to be cracked earlier) or as good as one without**.

## What about a "problematic overlap"?

I think a passphrase with a problematic overlap is a clearer issue. This means that, in the same word-length guess space, this passphrase will appear twice rather than once. 

## Realistically, what are the odds of either a compounding or a problematic overlap occurring in a randomly generated passphrase?

I don't know! If you think you have a formula for calculating this on a per-list basis, feel free to submit an issue or pull request!

## What this tool does

This tool takes a word list (as a text file) as an input. It then searches the given list for both compoundings and problematic overlaps (see above).

Next, it attempts to find the smallest number of words that need to be removed in order to make the given word list "compound-safe". Finally, it prints out this new, shorter, compound-safe list to a new text file. In this way it makes word lists "compound-safe" (or at least more safe-- see "Known issue" and "Caveat" sections below).

## How to use this tool to check a word list

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

I found the [EFF long word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) to be compound-safe (which is really cool!).

However, in the 1Password list (labeled `word_lists/agile_words.txt` in this project, copied from [this 1Password challenge](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt)), I found 2,661 compound words (see: `scrap-lists-of-compound-words-and-components/agile_double_bad_words.txt`), made up of 1,511 unique bad single words (see: `scrap-lists-of-compound-words-and-components/agile_single_bad_words.txt`). 

NOTE: 1Password's software, as far as I know, does NOT allow users to generate random passphrase without punctuation between words. Users _must_ choose to separate words with a period, hyphen, space, comma, or underscore. So these findings do NOT constitute a security issue with 1Password.

## An example

The aim of the tool is to  is to remove the fewest number of these bad words to make the list compound-safe (see the `find_words_to_remove` function). When I ran it on the 1Password wordlist, I got 498 words back, which I dumped in to `scrap-lists-of-compound-words-and-components/words_to_remove_from_agile_list.txt`. 

Then the `make_clean_list` function removes these 498 words, giving us the list found in `word_lists/agile_words.txt.compound-safe`, a list of 17,830 words compound-safe words.

Now, we should note that reducing the length of the list from 18,328 words to 17,830 has a cost. Given 1Password's current list of 18,328 words, when a user adds one of these words to their passphrase, they're adding about 14.162 bits of entropy to their passphrase. Using the shortened, compound-safe 17,830 word list, each randomly generated word would add about 14.122 bits to the passphrase. Of course, Agile Bits/1Password could replace the 498 words while keeping the list compound-safe.

## Caveats / Known issues

We've explored "two-word compounding", where two words are actually one, but is there a possibility of a three-word compounding -- where three words become two? This tool does NOT currently check for this, so I can't actually guarantee that the lists outputted by the tool are completely compound-safe.

Also, currently this script runs really slowly on lists with a lot of overlaps (problematic or not). Using threads in Rust would help, but I'm sure there's a more efficient way to check for problematic overhangs.

## To do

- Use multiple threads to speed up the process. 
- Make the command line text output during the process cleaner and more professional-looking.
- Make the Rust code simpler and/or more idiomatic.
- Explore the caveat listed above.

## Lingering questions

1a. Given a word list that is not compound-safe, calculate the probability of a compounding (generating a non-safe pair in a passphrase)? 
1b. Given this probability, does it make sense, or is it useful, to calculate a revised bits-per-word measure of the list? (For the record I think this would be harmful, but I pose it here for inspiration.)


