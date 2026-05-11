# E-Book parsing: Parallel vs Sequential Parsing

## General Overview of problem

The ebook parsing logic used to be a single for loop iterating through every single book, checking which filetype it is and running one of the valid parsers it belongs to.

I thought employing parallelism via multithreading (well not true parallelism only if threads are available at time of running the programming) might help speed up this process.

I built two binaries - one with the parallelism logic and the other with the sequential logic and tested both on Linux with `perf`. The results are present in the logs directory but I will be talking more about them in this .md file.

## Information on files
I tested these on 94 book files downloaded from gutenberg.org - ranging from a wide variety of novels such as "All of Shakespeare's works" to "Email 101"

## Test cases

1. No config or app directory created, no books present in default source
Files: `logs/parallel-brand-new-no-books-results.txt`, `logs/sequential-brand-new-no-books-results.txt` | Surprisingly the sequential operation is better here but not at a big enough margin to debate which one is better (~300-400ms). In my testing, the change was not very noticeable. By pure metrics, sequential won here but other parallelism may also be favored as an alternative

2. No config or app directory created, 94 books present in default source
File: `logs/parallel-brand-new-books-present-results.txt`, `logs/sequential-brand-new-books-present-results.txt` | Here is where parallelism has a marginal improvement (~8s faster). Although CPU overhead has increased due to the number of threads we are using - the performance gain is considerable. This will also scale as books for users grow

3. existing config or app directory already created, 94 books present in default source
File: `logs/parallel-already-existing-results.txt`, `logs/sequential-already-existing-results.txt` | Parallel wins here again (~10s faster). CPU utilized is 0.1% lesser in case of sequential but that is expected as we are using more threads in the parallel program. The usage is not astronomically bigger so we can choose to not worry about it right now. It is to be seen how CPU usage increases as books increase.

## Conclusion

Going to be opting for the parallelism approach for ebook parsing.

In the future however, we need to evaluate how CPU usage is affected by the number of books in this approach.
