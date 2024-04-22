# 🎄 Advent of Code 2023 🎄

<img src="./img/rustacean-flat-happy.png" width="164">

First time I do the advent of code. This year the goal is to finish all the problems in Rust to learn this language.

Link to the website with puzzle: [Advent Of Code 2023](https://adventofcode.com/2023)

## Use template

To create a new day simply run: ```.\new-day.ps1 NUMBER_OF_DAY```.

## Use Algorithm

During this challenge I use a lot of graph and simple puzzle translation puzzle into code without over-engineering. However, here is a small list of interesting algorithm used:

Algorithm|File|Small description
--|--|--
[Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)|[day 17](./day-17/src/main.rs) [day 23 part 1](./day-23/src/main.rs)|Path finding.
[Stoer–Wagner algorithm](https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm)|[day 25](./day-25/src/main.rs)|Graph min cut. (Splitting a graph in two)
[Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)|[day 18](./day-18/src/main.rs)|Area of simple polygon.

