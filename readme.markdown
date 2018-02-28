# Half-Entropy Diceware Word Checker

I wanted to make sure that no two words in [the EFF's long diceware word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) could be combined to make another word on the list. 

This is an important quality of a diceware word list, since if these two words happen to appear next to each other in a passphrase (_without_ a separating punctuation mark), the user would only be adding about 12.92 bits of entropy to their password instead of the expected 25.85 bits of entropy. 

An example of this would be **if** a word list included "under", "line", and "underline", a user might randomly get "under" and "line" in a row. The generated passphrase could be "crueltyfrailunderlinecyclingapostle", and the user might assume it had six words worth of entropy, or 77.55 bits of entropy. But really, an attacker brute forcing their way through _five_ word passphrases would crack it at some point, with the third guess being the compound word "underline". That would mean the passphrase actually only has 64.62 bits of entropy.

I heard of this problem in [this YouTube video](https://youtu.be/Pe_3cFuSw1E?t=8m36s), but I guessed that the folks who made the EFF list would be too smart to make that mistake.

*Spoiler alert*: I did **NOT** find any 12.92 bits pairs of words in the EFF long word list! They were smart when they made it! 

## How I did it

I used Rust to check for these words. 

I _definitely_ did not do this the most efficient way-- it ended up taking about 11 hours. I did it by mashing all the words together, then checking that list against the word list. 

For example, if "under", "line", and "underline" were all in the list, my code would mash the "under" and "line" together, then eventually check that again all the words. The script would find that "underline" == "underline", and add the word to a vector called `bad_words`. 

But, again, my script, when run on the EFF long list, didn't find any bad words.

As with all Cargo Rust projects, the main code is in `src/main.rs`
