# Play a game of guess that number!

To make this easier on me since I'm new,
if whatever you enter isn't convertable to a float
I will default to 128 and you will get however many guesses
128 requires

# Help output when nothing is given

```
error: The following required arguments were not provided:
    <num>

USAGE:
    gg <num>

For more information try --help
```

# Default when input is broken
```
gg ff?
Let's play "Guess that numbrer!" 7
I'm thinking of a number from 1 to 128
You get 7 chances before I start calling you names
Chance 1, Please input your guess:
```

# Examples

```
gg 500
Let's play "Guess that number!"
I'm thinking of a number from 1 to 500
You get 9 chances before I start calling you names
Chance 1, Please input your guess:
```
```
gg 500000
Let's play "Guess that number!"
I'm thinking of a number from 1 to 500000
You get 19 chances before I start calling you names
Chance 1, Please input your guess:
```
