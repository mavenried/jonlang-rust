# jonlang

## What is it?

**jonlang** is a deliberately simple programming language originally created as a joke. Despite its playful syntax, it behaves like a real programming language: programs are parsed, compiled into bytecode, and executed by a small virtual machine.

The goal of jonlang is simplicity. Programs read almost like plain English while still supporting variables, arithmetic, input, and output.

---

# How to Use It

jonlang programs follow a few simple structural rules:

* Every program must begin with
  `hi, jon!`

* Every program must end with
  `bye, jon!`

* Every executable line must:

  * start with `jon,`
  * end with an exclamation mark `!`

Example:

```
hi, jon!
jon, say "Hello, World!"
bye, jon!
```

---

# Commands

## `say` - Output and Input

The `say` command is used to display messages and interact with the user.

| Command                                       | Description                                          | Example                                         |
| --------------------------------------------- | ---------------------------------------------------- | ----------------------------------------------- |
| `say <msg>`                                   | Prints a message to the console                      | `jon, say "Hello, World!"!`                     |
| `say <msg> and read into <varname> as <type>` | Prints a prompt and stores user input in a variable  | `jon, say "Name: " and read into name as text!` |
| `say <msg> and read aloud <varname>`          | Prints a message followed by the value of a variable | `jon, say "Your name is:" and read aloud name!` |

### Example

```
jon, say "Enter your age: " and read into age as number!
jon, say "You entered:" and read aloud age!
```

### Supported Input Types

`read into` requires a type specifier:

| Type     | Description   |
| -------- | ------------- |
| `number` | Numeric input |
| `text`   | String input  |

---

# `remember` - Variable Control

The `remember` command creates and updates variables.

| Command                                        | Description                             | Example                                                |
| ---------------------------------------------- | --------------------------------------- | ------------------------------------------------------ |
| `remember that <varname> is <value>`           | Assigns a value to a variable           | `jon, remember that pi is 3.14!`                       |
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

jon, say "Area is:" and read aloud answer!

bye, jon!
```

This program:

1. Prompts the user for a radius
2. Calculates the area of a circle
3. Prints the result

---

# Summary

jonlang keeps programming intentionally simple:

* Programs are structured like sentences
* Variables are created using `remember`
* Input and output are handled using `say`
* Arithmetic uses natural-language keywords

Behind the scenes, jonlang programs are compiled into bytecode and executed by a small virtual machine, giving the language a real execution model while keeping the syntax approachable and fun.
