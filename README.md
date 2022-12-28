Install:  
`~$ git clone https://github.com/lukakalinovcic/aoc2022.git`

Run:  
`~/aoc2022$ cargo run --release`

Output:  
```
Day #01 ... (done in 0.00s) => 68923, 200044
Day #02 ... (done in 0.00s) => 12645, 11756
Day #03 ... (done in 0.00s) => 7990, 2602
Day #04 ... (done in 0.00s) => 569, 936
Day #05 ... (done in 0.00s) => VCTFTJQCG, GCFGLDNJZ
Day #06 ... (done in 0.00s) => 1920, 2334
Day #07 ... (done in 0.00s) => 2061777, 4473403
Day #08 ... (done in 0.00s) => 1832, 157320
Day #09 ... (done in 0.00s) => 6011, 2419
Day #10 ... (done in 0.00s) => 14720, 
####.####.###..###..###..####.####.####.
#.......#.#..#.#..#.#..#.#.......#.#....
###....#..###..#..#.###..###....#..###..
#.....#...#..#.###..#..#.#.....#...#....
#....#....#..#.#....#..#.#....#....#....
#....####.###..#....###..#....####.#....
Day #11 ... (done in 0.08s) => 62491, 17408399184
Day #12 ... (done in 0.00s) => 420, 414
Day #13 ... (done in 0.00s) => 5208, 25792
Day #14 ... (done in 0.30s) => 592, 30367
Day #15 ... (done in 1.95s) => 4748135, 13743542639657
Day #16 ... (done in 0.25s) => 1896, 2576
Day #17 ... (done in 0.01s) => 3085, 1535483870924
Day #18 ... (done in 0.01s) => 4282, 2452
Day #19 ... (done in 2.08s) => 1528, 16926
Day #20 ... (done in 0.58s) => 8372, 7865110481723
Day #21 ... (done in 0.02s) => 379578518396784, 3353687996514
Day #22 ... (done in 0.00s) => 133174, 15410
Day #23 ... (done in 0.59s) => 3766, 954
Day #24 ... (done in 0.09s) => 277, 877
Day #25 ... (done in 0.00s) => 20=02=120-=-2110-0=1, Start the Blender
```

Run selected days:  
`~/aoc2022$ cargo run --release -- -d15 -d19 -d20`

Output:
```
Day #15 ... (done in 1.97s) => 4748135, 13743542639657
Day #19 ... (done in 2.03s) => 1528, 16926
Day #20 ... (done in 0.57s) => 8372, 7865110481723
```

Run on problem examples:  
`~/aoc2022$ cargo test`

Output:
```
running 30 tests
test day02::tests::day02 ... ok
test day04::tests::day04 ... ok
test day01::tests::day01 ... ok
test day05::tests::day05 ... ok
test day06::tests::day06_1 ... ok
test day06::tests::day06_2 ... ok
test day08::tests::day08 ... ok
test day06::tests::day06_4 ... ok
test day07::tests::day07 ... ok
test day06::tests::day06_5 ... ok
test day03::tests::day03 ... ok
test day06::tests::day06_3 ... ok
test day09::tests::day09_1 ... ok
test day12::tests::day12 ... ok
test day13::tests::day13 ... ok
test day10::tests::day10 ... ok
test day09::tests::day09_2 ... ok
test day14::tests::day14 ... ok
test day18::tests::day18 ... ok
test day21::tests::day21 ... ok
test day20::tests::day20 ... ok
test day15::tests::day15 ... ok
test day25::tests::day25 ... ok
test day23::tests::day23 ... ok
test day24::tests::day24 ... ok
test day22::tests::day22 ... ok
test day17::tests::day17 ... ok
test day11::tests::day11 ... ok
test day16::tests::day16 ... ok
test day19::tests::day19 ... ok
```