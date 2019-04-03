# Running Angora

## Run in Docker Container 

The Dockerfile provided allows Angora to run in a container.

*Caution: Angora assigns different threads with specific process affinity levels.
Running multiple Angora instances in different containers can result in decreased
efficiency.*

```
echo core | sudo tee /proc/sys/kernel/core_pattern
docker build -t angora ./
docker run --privileged -v /path-to-code-and-seed:/data -it --rm angora /bin/bash
```


## Tests

A number of tests have been provided. Feel free to add your own to test the 
capabilities of Angora.

```
cd tests
./test.sh <test_name>
```

## Run alongside AFL

If you are running AFL and its output directory is `output`, run
```
./angora_fuzzer -i input -o output -t path-to-taint-program --sync_afl -- program args(..)
```

Since the implementation of AFL mutation approach in Angora is too simple, the best practice is run it together with AFL, and use `-A` to disable Angora's AFL approach.
