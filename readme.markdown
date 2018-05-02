# Compound Passphrase List Safety Checker

This command line tool checks whether a given passphrase word list (such as a diceware word list) has any words that can be combined to make another word on the list. It's written in Rust, which I am new to, so please use this with caution.

Initially I wanted to make sure that no two words in [the EFF's long diceware word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) could be combined to make another word on the list. I later checked other lists.

**Disclosure**: I am not a professional researcher or statistician, and frankly I'm pretty fuzzy on some of this math. This code/theory/explanation could be very wrong (but hopefully not harmful?). If you think it could be wrong or harmful, please leave an issue! 

Further disclosure: see "Known issue" and "Caveat" sections below.

## What is "compound-safety"? 

I made up the term. Here's what I mean by it: 

> A passphrase word list is "compound-safe" if it does NOT contain any pairs of words that can be combined to make another word on the list. 

For example, if a word list included "under", "dog", and "underdog" as three separate words, it would NOT be compound-safe, since "under" and "dog" can be combined to make the word "underdog".

## Why is this attribute of a passphrase word list notable? 

Let's say we're using the word list described above, which has "under", "dog" and "underdog" in it. A user might randomly get "under" and "dog" in a row, for example in the six-word passphrase "crueltyfrailunderdogcyclingapostle". The user might assume they had six words worth of entropy. But really, an attacker brute forcing their way through five-word passphrases would eventually crack the passphrase. We can call this event "a compounding".

**It's important to note** that if the passphrase has any punctuation (for example, a period, comma, hyphen, space) between words, this issue goes away completely. "cruelty frail under dog cycling apostle" is indeed a six-word passphrase, and an attacker who tries "underdog" as the third word does not get a match.

To summarize: When creating passphrases without punctuation between the words with a word list that is NOT compound-safe, there's a small risk that users will put two words together that form another word on this list. When this happens (which we might call "a compounding"), they lose one word's worth of entropy from their password. However if they used a compound-safe list, they can safely not use punctuation between words, since compounding cannot occur (or at least, are less likely to occur -- see "Known issue" and "Caveat" sections below).

Again, it's super important to understand that putting a hyphen or space between the words ("cruelty frail under dog cycling apostle") eliminates this problem completely.

I heard of this potential issue in [this YouTube video](https://youtu.be/Pe_3cFuSw1E?t=8m36s). 

## Does a compounding make a passphrase weaker? Does this matter?

Is "crueltyfrailunderdogcyclingapostle" a "weaker" passphrase than a 6-word phrase that does not have a compounding in it? Honestly I'm not sure. The only concrete situation where it might is if an attacker is brute forcing through all 5-word phrases before brute forcing all 6-word phrases, and in this case would break the compounded 6-word phrase slightly earlier than a non-compounded 6-word phrase. (How much earlier?)

But **if an attacker knew your passphrase was 6 words, I'm not sure if a phrase with a compounding is "worse" (i.e. going to be cracked earlier) or as good as one without**.

## The "Venn-diagram" problem (or, the "left-eye, right-eye" problem)

What if we generated a passphrase and, at some point in it, got "paperboyhood"? As we've learned, if "paper", "boy", and "paperboy" are all words in our list, we have a compounding. Likewise if "boy", "hood", and "boyhood" are all on our list we have a second compounding. 

However, if our list is: 

```
paper
paperboy
boyhood
hood
``` 

within the 2-phrase guess space we'll have "paperboyhood" appear twice: once as [paperboy][hood] and again as [paper][boyhood]. Unfortunately the current version of this tool would NOT do anything to fix this list.

Note: A Fediverse user brought this situation to my attention.

## What this tool does

This tool takes a word list (as a text file) as an input. It then searches the given list for words that can be combined to make other words on the list.

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

I did not find any compound-unsafe pairs of words in the [EFF long word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases). In other words, according to my script, the EFF long word list is compound-safe.

However, in the 1Password list (labeled `word_lists/agile_words.txt` in this project, copied from [this 1Password challenge](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt)), I found 2,661 compound words (see: `scrap-lists-of-compound-words-and-components/agile_double_bad_words.txt`), made up of 1,511 unique bad single words (see: `scrap-lists-of-compound-words-and-components/agile_single_bad_words.txt`). 

NOTE: 1Password's software, as far as I know, does NOT allow users to generate random passphrase without punctuation between words. Users _must_ choose to separate words with a period, hyphen, space, comma, or underscore. So these findings do NOT constitute a security issue with 1Password.

## An example

The aim of the tool is to  is to remove the fewest number of these bad words to make the list compound-safe (see the `find_words_to_remove` function). When I ran it on the 1Password wordlist, I got 498 words back, which I dumped in to `scrap-lists-of-compound-words-and-components/words_to_remove_from_agile_list.txt`. 

Then the `make_clean_list` function removes these 498 words, giving us the list found in `word_lists/agile_words.txt.compound-safe`, a list of 17,830 words compound-safe words.

Now, we should note that reducing the length of the list from 18,328 words to 17,830 has a cost. Given 1Password's current list of 18,328 words, when a user adds one of these words to their passphrase, they're adding about 14.162 bits of entropy to their passphrase. Using the shortened, compound-safe 17,830 word list, each randomly generated word would add about 14.122 bits to the passphrase. Of course, Agile Bits/1Password could replace the 498 words while keeping the list compound-safe.

## Known issues

See: Venn diagram problem described above

## A caveat

We've explored "two-word compounding", where two words are actually one, but is there a possibility of a three-word compounding -- where three words become two? This tool does NOT currently check for this, so I can't actually guarantee that the lists outputted by the tool are completely compound-safe.

## To do

- Use multiple threads to speed up the process. 
- Make the command line text output during the process cleaner and more professional-looking.
- Make the Rust code simpler and/or more idiomatic.
- Explore the caveat listed above.

## Lingering questions

1a. Given a word list that is not compound safe, calculate the probability of a compounding (generating a non-safe pair in a passphrase)? 
1b. Given this probability, does it make sense, or is it useful, to calculate a revised bits-per-word measure of the list? (For the record I think this would be harmful, but I pose it here for inspiration.)


