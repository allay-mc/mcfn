# mcfn - A mcfunction preprocessor

> **Note**
> This preprocessor is primarily designed for Bedrock Edition and may not work for
> Java Edition due to its different command syntax.


## Overview

<!-- TODO: image of example program -->


## Philosophy

Instead of inventing an entire new programming language like
[mcscript](https://mcscript.stevertus.com/) for Java Edition, `mcfn` extends the
`mcfunction` to save repetitive tasks in a more readable way. If you want more
controll, check out the
[`templating` script for Allay](https://github.com/allay-mc/scripts/blob/master/templating.rb).

`mcfn` makes use of the `execute` and `scoreboard` commands to simulate a runtime
within Minecraft. This allows dynamic conditional programming with several macros.


## Installtion

### Cargo

```
cargo install -f mcfn
```


## Usage

To transpile all `*.mcfn` files within the `functions` directory, the following
command can be used.

```console
mcfn functions/**/*.mcfn
```


## Reference

### Indention

Leading spaces from each line in the program is removed in the built file. This
allows nesting code in blocks for a better visual view.

```mcfunction
#!proc greet
say Hello
say World
#!end

#!! can be written like below as well

#!proc greet
  say Hello
  say World
#!end
```


### Comments

There are two different kinds of comments. A normal comment starts with a hash sign
(`#`) as usual but must not be followed by a bang (`!`). They are included in the
built file and can be used wherever full commands can be used. In fact, they can be
used at the end of a command. The other kind of comment starts with a hash sign and
two bangs (`#!!`). They must be used as a whole line meaning they cannot be appended
to a macro or a command.


### `with`

The `with` macro preprends each line of the block with its argument.

```mcfunction
#!with execute as @a
  say Hello
  say World
#!end

#!! same as

executa as @a say Hello
execute as @a say World
```


### `proc` and `call`

The `proc` macro creates a procedure. Its content can be called at a later point in
the program any amount of times by using the `call` macro.

```mcfunction
#!proc greet
  say Hello
  say World
#!end

#!call greet
#!call greet
```

Unlike most programming languages, procedures are always global, meaning a procedure
defined within a different procedure can be called from outside as long as the call
is present after the procedure has been defined.

```mcfunction
#!! ERROR: #!call foo

#!proc foo
  #!proc bar
    say Hello
    say World
  #!end
#!end

#!call bar
```

Procedures cannot have parameters. However you can implements such behaviour as shown
below:

```mcfunction
scoreboard objectes add _internal dummy

#!proc fizzbuzz
  #!switch score _internal _arg_a
    #!case matches 3
      say Fizz
    #!case matches 5
      say Buzz
    #!default
      say None
  #!end
#!end

scoreboard players set _internal _arg_a 3
#!call fizzbuzz

scoreboard players set _internal _arg_a 20
#!call fizzbuzz
```


### `log`

The `log` macro prints its value to the console at transpile time whenever it is
reached. This may be useful for debugging.

```mcfunction
#!log hi
```


### `if`, `ifn` and `else`

You can use `if` to verify a condition is met and `ifn` to verify the opposite.

```mcfunction
#!if score @s foo matches 3
#!ifn score @s bar matches 5
#!then
  say Hello
  say World
#!end
```

You can also add an `else`-block.

```mcfunction
#!if score @s foo matches 3
#!ifn score @s bar matches 5
#!then
  say Hello
  say World
#!else
  say Bye
  say World
#!end
```

If you change the state in the first block that the condition depends on the else
block is still executed.


### `include`

You can include another file relative to the current one by using the `include`
macro.

```mcfunction
#!include path/to/file.in
```

Note that including `mcfn` files will not be rendered so you may only use this with
`mcfunction` programs that do not need to be transpiled initially (in case that step
happens after including the file).

This macro can also be used to include some sort of header for example by creating
a `mcfunction` program consisting of only comments.


### `declare` and `delete`

A quick way of creating scores is by using the `declare` macro. Removing a score is
possible with the `delete` macro.

```mcfunction
#!declare x
#!delete x
```

The result would be:

```mcfunction
scoreboard objectives add x dummy
scoreboard objectives remove x
```


### `switch`, `case`, `ncase` and `default`

Using `if`, `ifn` and `else` for multiple different branches may result in a deeply
nested unreadable and repetitive tree of conditions and blocks. In such scenario a
`switch` macro may be useful.

```mcfunction
#!switch score @s money
  #!case matches 69
    say Nice
    #!then
  #!ncase matches 69
    say Ok
    #!then
  #!case matches 1000..
    say Rich
    #!end
  #!case matches ..0
    say I need to get a job
    #!end
  #!default
    say Life is good, you know what I mean?
#!end
```

The argument of the `switch` macro defines the first part of the condition. Each
case contains the second half of the condition as an argument. The above snippet
matches the pseudo code below.

```rust
if "scores @s money matches 69" {
  run("say Nice");
}
if !"score @s money matches 69" {
  run("say Ok");
}
if "score @s money matches 1000.." {
  run("say Rich");
} else if "score @s matches ..0" {
  run("say I need to get a job");
} else {
  run("say Life is good, you know what I mean?");
}
```


#### Behavior of `case` and `ncase`

In most programming languages, a `case` usually ends with a `break` statement which
exits the entire switch block. In `mcfn` the equal is the `end` macro. If a case
block however does not end with a `break`, then other `case`s following are tried to
match as well. In `mcfn` this can be achieved by ending the case block with the
`then` macro. Each `case` or `ncase` must end with either `end` or `then` whereas
the optional `default` macro at the end must not end with either of them.


## Good To Know

### Highlighting

The preprocessor language is designed to look good with the `mcfunction` highlighter
as well so you can use it in your repositories. Both `.mcfn` and `.mcfn.mcfunction`
are therefore supported as file extensions.


### Magic variables

The score `__mcfn_internal_n` where n may be any natural number and the "fake player"
name `__mcfn_internal` are used by `mcfn` and should not be used within a program or
else it might result in unexpected behaviour.


### Usage in Allay

TODO
