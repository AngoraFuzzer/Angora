# UI Terminology

## Layout
```
   ANGORA    (\_/)
   FUZZER    (x'.')
 -- OVERVIEW --
    TIMING |     RUN: [00:00:05],     TRACK: [00:00:00]
  COVERAGE |    EDGE:   10.50,   DENSITY:    0.00%
    EXECS  |   TOTAL:      27,     ROUND:      10,     MAX_R:       1
    SPEED  |  PERIOD:    5.40r/s    TIME:  212.40us,
    FOUND  |    PATH:      10,     HANGS:       0,   CRASHES:       0
 -- FUZZ --
   EXPLORE | CONDS:       8, EXEC:      22, TIME: [00:00:00], FOUND:       8 -       0 -       0
   EXPLOIT | CONDS:       0, EXEC:       0, TIME: [00:00:00], FOUND:       0 -       0 -       0
     CMPFN | CONDS:       0, EXEC:       0, TIME: [00:00:00], FOUND:       0 -       0 -       0
       LEN | CONDS:       1, EXEC:       4, TIME: [00:00:00], FOUND:       1 -       0 -       0
       AFL | CONDS:       0, EXEC:       0, TIME: [00:00:00], FOUND:       0 -       0 -       0
     OTHER | CONDS:       0, EXEC:       1, TIME: [00:00:00], FOUND:       1 -       0 -       0
 -- SEARCH --
    SEARCH | CMP:       8 /       8, BOOL:       0 /       0, SW:       0 /       0
   UNDESIR | CMP:       0 /       0, BOOL:       0 /       0, SW:       0 /       0
   ONEBYTE | CMP:       0 /       0, BOOL:       0 /       0, SW:       0 /       0
  INCONSIS | CMP:       0 /       0, BOOL:       0 /       0, SW:       0 /       0
  -- STATE -- 
          |    NORMAL:      40d -     104p,   NORMAL_END:       0d -       0p,   ONE_BYTE:     486d -     530p
          |       DET:       0d -       0p,    TIMEOUT:       0d -       0p,     UNSOLVABLE:       0d -       0p
```

## Terminology
- `OVERVIEW`: Overall Stats
  - `TIMING`: Timing stats
    - `ALL`: Elapsed fuzzing time
    - `TRACK`: Accumulated taint tracking time
  - `COVERAGE`: Program branch coverage status
    - `EDGE`: Average edge coverage
    - `DENSITY`: Coverage map density
  - `EXECS`: Execution statistics
    - `TOTAL`: Total execution count 
    - `ROUND`: Current round execution count
    - `MAX_R`: Maximum rounds
  - `SPEED`: Execution speed statistics 
    - `PERIOD`: Executions per second
    - `TIME`: Average execution time for fast pass
  - `FOUND`: Fuzzing results
    - `PATH`: Total path count
    - `HANGS`: Total timeout count
    - `CRASHES`: Total crash count
- `FUZZ`: Fuzzing Strategy Statistics
  - Methods:
    - `EXPLORE`: Exploration strategies
    - `EXPLOIT`: Exploitation strategies
    - `CMPFN`: `*cmp` function strategies
    - `LEN`: Length exploitation 
    - `AFL`: Strategies derived from AFL
    - `OTHER`: Other strategies
  - Metrics:
    - `CONDS`: Conditions fuzzed under this strategy
    - `EXEC`: Executions done under this strategy
    - `TIME`: Accumulated fuzzing time elapsed under this strategy
    - `FOUND`: Fuzzing results under this strategy
- `SEARCH`: Search statistics
  - Types:
    - `SEARCH`: Ordinary comparisons
    - `UNDESIR`: Undesirable comparisons
    - `ONEBYTE`: One byte fuzzing
    - `INCONSIS`: Inconsistencies of ids between programs
  - Metrics:
    - `CMP`: Comparison evaluations solved / all
    - `BOOL`: Boolean values solved / all
    - `SW`: Switch comparisons solved / all
- `STATE`: State for the constraints
    - `d`: Solved
    - `p`: Pending, not solved.