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

---

## Stage 3: REPL (Read-Eval-Print Loop)

A REPL is an interactive loop that reads user input, evaluates it, prints the result, and then waits for the next input.

### 📕 References

- https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop

---

## Stage 4: Exit Command

The `exit` command causes the shell to exit. It returns an integer exit code. An exit code of `0` indicates success, and any other value indicates an error.

### 📕 References

- https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#exit
- https://en.wikipedia.org/wiki/Exit_status

---

## Stage 5: Echo Command

The `echo` utility writes its arguments to standard output, followed by a `<newline>`. If there are no arguments, only the `<newline>` is written.

### 📕 References

- https://pubs.opengroup.org/onlinepubs/9699919799/utilities/echo.html

---

## Stage 6: Type Command

The `type` builtin command is used to identify how a command name is interpreted by the shell. It returns whether the command is a shell builtin or an unknown command.

### 📕 References

- https://pubs.opengroup.org/onlinepubs/9699919799/utilities/type.html

---

## Stage 7: Type Command: Executables

Extend the `type` builtin command to identify executable programs (using the `PATH` environment variable). The `type` builtin should search the [`PATH`][PATH] for the command name and print the path to the executable if found.

> [!NOTE]
> [`PATH`][PATH] is an environment-variable that specifies the set of directories where executable programs are located.

### 📕 References

- [Wikipedia: PATH (variable)][PATH]

---

## Stage 8: Run External Programs

Implement the ability to run external programs. The shell should be able to run any program that is in the `PATH` environment variable. The arguments are passed to the program.

---

## Stage 9: PWD Command

The `pwd` utility writes the absolute pathname of the current working directory to the standard output.

### 📕 References

- https://en.wikipedia.org/wiki/Pwd
- https://pubs.opengroup.org/onlinepubs/9699919799/utilities/pwd.html

---

## Stage 10: CD Command (Absolute Paths)

The `cd` utility changes the current working directory to the directory specified by the `PATH` argument. This step only deals with absolute paths.

---

## Stage 11: CD Command (Relative Paths)

Extend the `cd` utility to handle relative paths. The `cd` utility changes the current working directory to the directory specified by the `PATH` argument.
e.g. `cd ./dir` and `cd ..`

---

## Stage 12: CD Command (Home Directory)

The `~` character is a shorthand for the home directory. Extend the `cd` utility to handle the `~` character.

## Stage 13; Escaping Single Quotes

- https://www.gnu.org/software/bash/manual/bash.html#Single-Quotes

---

## 📕 References

- https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html

<!-- ----- -->
<!-- LINKS -->
<!-- ----- -->

[PATH]: https://en.wikipedia.org/wiki/PATH_(variable)
