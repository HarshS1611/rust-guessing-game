## 🎯 Guess the Number (Rust)

A simple command-line guessing game written in **Rust**.
The player has **10 attempts** to guess a randomly generated number between **1 and 100**.

### 🚀 How to Run

```bash
cargo build
cargo run
```

### 🕹 How It Works

* The game generates a secret number between 1–100
* You get **10 guesses**
* After each guess, you’ll be told if it’s **too small** or **too big**
* Guess correctly to win, or lose after all attempts are used

### 📦 Requirements

* Rust (stable)
* `rand` crate

Add to `Cargo.toml`:

```toml
rand = "0.8.5"
```

### 📄 Example Output

```
Attempts left: 7
Enter your guess: 42
⬆️ Too small!
```

### 🎉 Win Condition

Guess the number before your attempts run out.

