# jonlang

## What is it?

**jonlang** is a deliberately simple programming language built as a joke.
Despite the playful syntax, programs are parsed, compiled into bytecode, and executed by a small virtual machine.

The goal of jonlang is readability and simplicity.  Programs read almost like plain English while still supporting variables, arithmetic, input, and output.

---

# How to Use It

jonlang programs follow a few simple rules:

* Every program must begin with
  `hi, jon!`

* Every program must end with
  `bye, jon!`

* Every executable line must:

  * start with `jon,`
  * end with `!`

Example:

```
hi, jon!
jon, say "Hello, World!"!
bye, jon!
```

---

# Commands

## `say` : Output and Input

The `say` command is used for console output and user input.

| Command                                       | Description                                          | Example                                         |
| --------------------------------------------- | ---------------------------------------------------- | ----------------------------------------------- |
| `say <msg>`                                   | Prints a message to the console                      | `jon, say "Hello, World!"!`                     |
| `say <msg> and read into <varname> as <type>` | Prompts the user and stores input in a variable      | `jon, say "Name: " and read into name as text!` |
| `say <msg> and <varname> aloud`               | Prints a message followed by the value of a variable | `jon, say "Your name is:" and name aloud!`      |

### Example

```
jon, say "Enter your age: " and read into age as number!
jon, say "You entered:" and age aloud!
```

### Supported Input Types

The `read into` command requires a type:

| Type     | Description   |
| -------- | ------------- |
| `number` | Numeric input |
| `text`   | String input  |

---

# `remember` : Variable Control

The `remember` command creates and updates variables.

| Command                                        | Description                             | Example                                                |
| ---------------------------------------------- | --------------------------------------- | ------------------------------------------------------ |
| `remember that <varname> is <value>`           | Assigns a value                         | `jon, remember that pi is 3.14!`                       |
| `remember that <varname> will be <expression>` | Assigns the result of a math expression | `jon, remember that area will be radius times radius!` |

### Math Expressions

Expressions follow this format:

```
<value> <operation> <value>
```

Supported operations:

| Operation      | Keyword |
| -------------- | ------- |
| Addition       | `plus`  |
| Subtraction    | `minus` |
| Multiplication | `times` |
| Division       | `by`    |

Example:

```
jon, remember that total will be price plus tax!
jon, remember that square will be x times x!
```

---

# Example Program

```
hi, jon!

jon, say "Area Calculator"!
jon, remember that pi is 3.14!

jon, say "Radius: " and read into radius as number!
jon, remember that answer will be radius times radius!
jon, remember that answer will be answer times pi!

jon, say "Area is:" and answer aloud!

bye, jon!
```

This program:

1. Asks the user for a radius
2. Calculates the area of a circle
3. Prints the result

---

# Summary

jonlang keeps programming intentionally simple:

* Programs read like sentences
* Variables are created using `remember`
* Input and output use `say`
* Arithmetic uses natural keywords like `plus` and `times`

Behind the scenes, jonlang programs are compiled into bytecode and executed by a small virtual machine.
To see this bytecode, append `--debug` to the end of the command
