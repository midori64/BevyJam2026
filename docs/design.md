Word based game, the core mechanic is combining two (possibly more) words together in order to produce required one.

You own a magic shop, customers come in and request some keyword (or a couple of words maybe). You don't know the requests beforehand. Every request has to be completed, there's no ability to skip it. The score can be tracked in multiple ways, but one is based on how many extra letters (waste/leftovers) are produced when combining words.

## Word combination
The simplest model I can think of:
each word is a bag of it's letters. A bag is like a set, but can contain repeated number of identical elements `(multiset for the nerds out there)`. The combination of the words is the union of the bags - `[a, b, c] + [c, d] = [a, b, c, c, d]`. Order doesn't matter.

## Rules
Player tries to come up with a optimal combination of available words. The One that contains the required bag of letters, but also has minimal extra letters.

# Implementation


### Todo
- [ ] Decide where the source words come from
- [ ] Decide if customers say a couple of sentences about why they need a thing (do we have the time to write it?)
