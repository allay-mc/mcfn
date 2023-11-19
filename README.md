# mcfn - An mcfunction preprocessor

> [!NOTE]
> This preprocessor is primarily designed for Bedrock Edition and may not work for
> Java Edition due to its different command syntax.

![Crates.io](https://img.shields.io/crates/v/mcfn?style=for-the-badge)
![GitHub Repo stars](https://img.shields.io/github/stars/allay-mc/mcfn?style=for-the-badge)


## Overview

<!-- TODO: image of example program -->
<!-- TODO: gitattributes vendor built mcfunctions in examples -->


## Philosophy

Instead of inventing an entire new programming language like
[mcscript](https://mcscript.stevertus.com/) for Java Edition, `mcfn` extends the
`mcfunction` format with several macros to avoid repetitive tasks in a readable way.

`mcfn` primarily makes use of the `execute` and `scoreboard` commands which make it possible to achieve conditions at runtime.


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

```mcfunction
# included in output file

#!! not included in output file
````


### `with`

The `with` macro preprends each line of the block with its argument.

```mcfunction
#!with execute as @a run
  say Hello
  say World
#!end

#!! same as

executa as @a run say Hello
execute as @a run say World
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
#!call foo
#^^^^^^^^^ error

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
  #!if scores @s _arg_a matches 3
  #!then
    say Hi
  #!else
    say Hello
  #!end
#!end

scoreboard players set @s _arg_a 3
#!call fizzbuzz
scoreboard players reset @s _arg_a

scoreboard players set @s _arg_a 20
#!call fizzbuzz
scoreboard players reset @s _arg_a
```


### `log`

The `log` macro prints its value to the console at transpile time whenever it is
reached. This may be useful for debugging.

```mcfunction
#!log hi
```


### `if`, `ifn` and `else`

You can use `if` to verify a condition is met and `ifn` to verify the opposite. If multiple `if` or `ifn`
macros are chained, then the block is executed when **one** of the conditions is met.

```mcfunction
#!if score @s foo matches 3
#!ifn score @s bar matches 5
#!then
  say Hello
  say World
#!end
```

You can also add an `else`-block which will run if none of the conditions were met.

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


### `when` and `else`

If you have used programming languages like C or Rust before you might be familar with
conditional compilation. Conditional compilation allows you to build variable programs
depending on configuration settings or the OS for instance.

```rust
if cfg!(any(target_os = "windows", target_os = "linux")) {
    // do performant stuff
} else if cfg!(any(target_os = "android", target_os = "macos")) {
    // do less performant stuff
}
```

This can be implemented in `mcfn` in a way as well by using the `when` macro.

```mcfunction
#!when MCFN_TARGET windows
#!when MCFN_TARGET linux
#!then
  say Hello
#!else
  say Hi
#!end
```

Such conditions compare a specified environment variable (such as `MCFN_TARGET`) with
a string (such as `windows`). The block is rendered when the condition is met and
ignored otherwise (also if the environment variable is not defined). The structure
matches [normal conditions](#if-ifn-and-else) with the exception that `when`
is used instead of `if` and negated `when` macros do not exist. Note that `when` branches
still run `log` and `include` macros and validate the blocks. The **rendered** block will
just be ignored during the tranpilation step.

You can then control the transpilation with bash for example by using this syntax:

```bash
MCFN_TARGET="windows" mcfn script.mcfn
```


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

Add the following ruby script to your script directory and refer to it in `allay.toml`.

```ruby
# TODO
```
