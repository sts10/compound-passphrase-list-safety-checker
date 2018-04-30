# Half-Entropy Diceware Word Checker

This Rust scripts checks whether a given diceware word list has any words that can be combined to make another word on the list.

Initially I wanted to make sure that no two words in [the EFF's long diceware word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) could be combined to make another word on the list. I later checked the list that 1Password uses.

## Why might these "half-entropy words be bad?

If, in randomly generating a passphrase, two of these half-entropy words happen to appear next to each other -- **_without_ a separating punctuation mark** -- the user would only be adding one word's worth of entropy to their password instead of the expected two words of entropy. 

An example of this would be **if** a word list included "under", "dog", and "underdog", a user might randomly get "under" and "dog" in a row. The generated passphrase could be "crueltyfrailunderdogcyclingapostle", and the user might assume it had six words worth of entropy. But really, an attacker brute forcing their way through _five_-word passphrases would crack it at some point, with the third guess being the compound word "underline". 

Again, it's super important to understand that putting a hyphen or space between the words ("cruelty frail under dog cycling apostle") eliminates this problem completely.

I heard of this potential issue in [this YouTube video](https://youtu.be/Pe_3cFuSw1E?t=8m36s). 

## Findings

I did not find any "bad" pairs of words in the EFF long word list.

However, in the 1Password list (labeled `agile_words.txt`), I found 2,661 compound words (see: `findings/agile_double_bad_words.txt`), made up of 1,511 unique bad single words (see: `findings/agile_single_bad_words.txt`). 

NOTE: 1Password's software, as far as I know, does NOT allow users to generate random passphrase without punctuation between words. Users _must_ choose to separate words with a period, hyphen, space, comma, or underscore. So these findings do NOT constitute a security issue with 1Password.

However, if 1Password wanted to safely offer users the choice of having no separator between words, they could remove the 1,511 bad single words from their list. This would take their list down from 18,328 words to 16,817. 

This reduction does have a cost, however. Given the current list of 18,328 words, when a user adds one of these words to their passphrase, they're adding about 14.162 bits of entropy to their passphrase. Using the shortened, 16,817 word list, each randomly generated word would add about 14.038 bits to the passphrase. Of course, alternatively, Agile/1Password could replace the 1,511 words with words that cannot be combined to make other words on their list.


