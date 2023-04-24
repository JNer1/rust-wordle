# rust-wordle

A wordle-like game in Rust

## How to Play

When you run the program with `cargo run` you will be prompted to guess:

```
Guess a five-letter word:
```

After you make a guess, you will be shown how your guess matches with the answer.

| Symbol | Description        |
|--------|--------------------|
| /      | Letter is correct  |
| X      | Letter is incorect |

```
Guess a five-letter word:
guess

XXXX/

2/3 lives
```

## Rules

- You must guess a five-letter word
- You only have three guesses
