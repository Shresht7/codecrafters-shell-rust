[![progress-banner](https://backend.codecrafters.io/progress/shell/f49d30d6-4531-4bfb-b3e0-e3ee631cbae2)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

_Add a description of your course here_

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.


## 📏 Guidelines

- Ensure you have `cargo (1.70)` installed locally
- Commit your changes and run `git push origin master` to submit your solution to CodeCrafters. Test output will be streamed to your terminal.

---

## Stage 1: Print a prompt

We print a shell prompt (`$ `) and wait for the user's input.

---

## Stage 2: Handle Missing Commands

We handle the case where the user enters a command that doesn't exist (which right now, is every command!). We print an error message and continue to wait for the user's input instead of letting the shell crash.

## Stage 3: REPL (Read-Eval-Print Loop)

A REPL is an interactive loop that reads user input, evaluates it, prints the result, and then waits for the next input.

### 📕 References

- https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop

---

## 📕 References

- https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html
