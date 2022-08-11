`ps-util` is a CLI tool for algorithm problem solving (TopCoder, Codeforces, ACM-ICPC, ..). It can generate common test cases such as tree, or 2d points. Also you can judge your solution against the test cases.

## Install
```
cargo install --git https://github.com/elbaro/ps-util
```

## Usage

### Data Generation

```
psutil generate tree 100
psutil generate tree 100 -i 1 1000
psutil generate convex 100 -f -100 100
```



| **generate** |             |                                           | note                                                     |
| ------------ | ----------- | ----------------------------------------- | -------------------------------------------------------- |
|              | **tree**    | `<n>` (-i min max) (-f min max)             | uniform sampling                                         |
|              | graph (WIP) | `<n>` (<m>) (--connected) (--directed)      |                                                          |
|              | **convex**  | `<n>` (-i min max) (-f min max)             | uniform sampling, not uniform when using int coordinates |
|              | **points**  | `<n>` (--no-same) (-i min max) (-f min max) |                                                          |
|              |             |                                           |                                                          |
|              |             |                                           |                                                          |
|              |             |                                           |                                                          |



### Data Validation

check for CRLF, newline at EOF, unicode, etc

```
psutil sanitize data/A --ext txt,in,out
psutil sanitize data/A --ext txt,in,out --confirmed
```

check for data format (recommends testlib)

```
psutil validate ./input_validator data/A --filter ".*\\.in"
psutil validate ./output_validator data/A --filter ".*\\.in"
```

### Solution Validation

check the solution

```
psutil eval ./solution data/A --in .in --out .out --time 1.5 --memory 64
psutil eval ./solution data/A --in input_ --out output_ --eval ./eval
```


## Judge

boilerplate

```
psutil new H --python
psutil new H --from cf 1004 (WIP)
psutil new H --from cf 1004H
```

submit (WIP)

```
psutil submit cf 1004H code.cpp
```
