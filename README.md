# adventofcode2023
These are my, [Niklas Hallqvist](https://github.com/niklasha) solutions to
[Advent of code 2023](https://adventofcode.com/2023).
They are written in [Rust](https://rust-lang.org).

My reason for doing these are, besides the fact that I like puzzle solving, I want to test my skills in Rust.

You need Rust, [rustup](https://rustup.rs/) is the suggested way to install Rust, that is about it.
You may need to add some SSL libraries, depending on operating system, but the installation process will tell you, if so.

Run all the days with:
```
cargo run input/
```

Where "input/" is a prefix for the days' inputs, named 01, 02, etc.
The tests (the examples given in the days' descriptions) can be run with:
```
cargo test
```

For every day, the first commit will be the solution with which I solved the puzzle.
After that, I may still revise the code to be more idiomatic or just nicer.


```
My results were:
      --------Part 1--------   --------Part 2--------
Day       Time   Rank  Score       Time   Rank  Score
 25   10:57:56   6617      0          -      -      -
 24   03:00:36   3855      0       >24h   6606      0
 23   01:07:43   2636      0          -      -      -
 22   02:23:35   3031      0   02:54:49   2756      0
 21   00:57:52   4238      0       >24h  11132      0
 20   03:49:00   5340      0   15:05:50   8866      0
 19   03:29:34   8221      0   13:22:21  10432      0
 18   02:29:33   6057      0   17:27:45  13905      0
 17   08:26:49   7913      0       >24h  22056      0
 16   01:10:19   4203      0   01:23:15   3911      0
 15   00:21:36   5814      0   01:39:19   6889      0
 14   01:08:56   7245      0   03:09:07   6630      0
 13   02:16:34   7753      0   02:49:59   6449      0
 12   01:03:58   4437      0       >24h  22652      0
 11   01:06:34   7485      0   01:10:30   5976      0
 10   01:41:14   6880      0   03:24:37   4645      0
  9   01:38:52  10900      0   01:48:21  10694      0
  8   01:01:01  11488      0   02:51:27  11133      0
  7   01:41:48  10518      0   02:55:03  11291      0
  6   00:47:05  11404      0   00:55:43  10971      0
  5   01:37:16  11446      0   04:03:41   8577      0
  4   00:17:15   6531      0   01:44:06  13299      0
  3   01:17:33   8649      0   01:47:05   8019      0
  2   01:35:08  14691      0   02:05:23  15580      0
  1   00:24:32   9109      0   00:47:44   5506      0