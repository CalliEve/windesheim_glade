# Windesheim Glade Interpreter

This is an interpreter for test running the sort-of pseudo-code used for the a-mazing challenge of the introduction week of year 2 HBO-ICT of University of Applied Sciences Windesheim. Included is example code in `instructions.txt` and an example glade in `glade.csv` to run it against.

## csv syntax

| letter | object         | number                                          |
| ------ | -------------- | ----------------------------------------------- |
| q      | obstacle       |
| x      | bomb           | seconds (steps) till explosions                 |
| t      | target (doel)  | number of target                                |
| m      | money (bonus)  | 2 ^ x is the bonus gained                       |
| d      | turner (draai) | 1-3 times turning to the left, 0 is random turn |
| s      | start          | 0-3 is direction, clock-wise with 0 being north |
| w      | white square   |
| g      | gray square    |
| r      | red square     |
| o      | orange square  |
| y      | yellow square  |
| e      | green square   |
| b      | blue square    |
| p      | purple square  |
| l      | black square   |

## code syntax

Uses the syntax (including bugs) of "taal 20", a simple language made for the a-mazing challenge for the introduction week of year 2 HBO-ICT of University of Applied Sciences Windesheim. One addition is the `print` statement which can take a variable or expression and debug print it.

## notes

- compiling requires rust nightly
- csv file can be passed by using `-g <filepath>` and defaults to `glade.csv`
- code file can be passed by using `-c <filepath>` and defaults to `instructions.txt`
- needs to be ran from the command line in the directory with the csv and txt file
