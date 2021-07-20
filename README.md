# rndmator

Command-line utility for choosing randomly from a list, rolling a die or tossing a coin.

## Installation

```
cargo install rndmator
```

## Usage

```
rndmator 0.1.0
Aman Harwara <amanharwara@protonmail.com>
Command-line utility to randomly choose from a list, roll dice or toss a coin.

USAGE:
    rndmator [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --repeat <n>    Repeat action <n> number of times

SUBCOMMANDS:
    -c, coin      Toss a coin
    -d, dice      Roll a dice
    -l, list      Randomly choose an item from a list
    -n, number    Random number between range
    help          Prints this message or the help of the given subcommand(s)
```

## Toss a coin

```
USAGE:
    rndmator {coin, -c} [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --repeat <n>         Repeat action <n> number of times
    -t, --tosses <tosses>    Number of tosses [default: 1]
```

## Roll a dice

```
USAGE:
    rndmator {dice, -d} [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --repeat <n>       Repeat action <n> number of times
    -s, --sides <sides>    Number of sides of the dice [default: 6]
```

## Choose from a list

```
USAGE:
    rndmator {list, -l} [OPTIONS] <LIST>

ARGS:
    <LIST>    The list to choose an item from

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --amount <amount>      Amount of items to choose from list [default: 1]
    -d, --delim <delimiter>    The delimiter used to separate items [default: ,]
    -r, --repeat <n>           Repeat action <n> number of times
```

## Random number between a range

```
USAGE:
    rndmator {number, -n} [OPTIONS] <RANGE>

ARGS:
    <RANGE>    Range to choose from. e.g. 1..10

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --repeat <n>    Repeat action <n> number of times
```
