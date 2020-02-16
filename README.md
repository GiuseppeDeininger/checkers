# Checkers in Rust

Checkers game implemented in Rust

## Rules 

The general rules of checker were applied. Some of them are:

- Regular checker can only move forward and one slot per time
- Regular checkers can only make forward captures
- When reaching the furthes row, the checker turns into a king
- The king can move as many slots as desired, both backwards and forward
- The king can capture checkers over any diagonal line, as long as it lands on an empty slot and captures a single checker per time without going over an ally
- It is possible to make multiple jumps to capture multiple checkers, as long as the moving limitations are respected

Additionally, rules that may change from country to country were selected based on [this](https://www.thesprucecrafts.com/play-checkers-using-standard-rules-409287) website

# How to play

