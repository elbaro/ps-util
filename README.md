## Install
```
cargo install --git https://github.com/elbaro/ps-util
```

## Usage


### Judge List

- supproted:
- not implemented: codeforces, topcoder, acmicpc, domjudge, cms, ..

### Data Generation

```
psutil generate tree 100
psutil generate tree 100 -i 1 1000
psutil generate convex 100 -f -100 100
```



| **generate** |             |                                           | note                                                     |
| ------------ | ----------- | ----------------------------------------- | -------------------------------------------------------- |
|              | **tree**    | <n> (-i min max) (-f min max)             | uniform sampling                                         |
|              | graph (WIP) | <n> (<m>) (--connected) (--directed)      |                                                          |
|              | **convex**  | <n> (-i min max) (-f min max)             | uniform sampling, not uniform when using int coordinates |
|              | **points**  | <n> (--no-same) (-i min max) (-f min max) |                                                          |
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
psutil validate ./input_validator data/A --filter .in
psutil validate ./output_validator data/A --filter .out
```

check the solution

```
psutil eval ./solution data/A --in .in --out .out
psutil eval ./solution data/A  --eval ./eval
```


## Judge

```
psutil new
psutil new --from cf 1004H
psutil submit cf 1004H code.cpp
```

| submit |      |      |          |
| ------ | ---- | ---- | -------- |
|        | cf   | 204A | code.cpp |
|        | tc   |      |          |
|        | bj   |      |          |
|        |      |      |          |
|        |      |      |          |



### download

```
./code.cpp
./problem.txt
./input1.txt
./input2.txt
./output1.txt
./output2.txt
```

| download |      |      |            |
| -------- | ---- | ---- | ---------- |
|          | cf   | 204A | (dest_dir) |
|          | cf   | 204  | (dest_dir) |
|          |      |      |            |
|          |      |      |            |
|          |      |      |            |



### contest

```

```

