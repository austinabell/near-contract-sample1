# Near contract sample1

This repo is only for repreducing the misuse of `UnorderedSet`.

After deploy and new the contract, you can call `set_status({"message":"hello"})` to store a message to storage successfully, but the `unique_values_count` will return `0` and the `contains_message({"message":"hello"})` will return `true`.

When you change the state of a collection struct like `Vector`, `UnorderedSet` and `UnorderedMap`, that is, when the `len` of `Vector` or the `len` of `Vector` inside `UnorderedSet` and `UnorderedMap` changes, you must ensure to store the state of `Vector`, `UnorderedSet` and `UnorderedMap` to storage too. Otherwise, it will result in inconsistent state of these collections, just like this repo shows.
