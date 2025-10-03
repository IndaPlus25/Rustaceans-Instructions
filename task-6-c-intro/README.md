# DD1337 Week 6 | C Introduction

## Getting started with C

### Install GCC toolchain

1) Install [MingW Installation Manager](http://mingw.org/). *As a pleb (Windows user), you are entitled to [the poor Windows user's fast download track](https://osdn.net/projects/mingw/downloads/68260/mingw-get-setup.exe/)*. 
2) In the installation manager application, install the `mingw32-gcc-objc-bin` under *Basic Setup*.
3) Test your installation by entering `gcc --help` in your terminal or command prompt(*host pleb *host).

Congrats dear programmer. You can now compile your C files by entering the following command.
```
gcc ./my_program.c -o my_executable
```

### Prepare for your assigment

1) Create a repository named `task-7-<KTH_ID>`.
2) Clone your regular assignment repository.
```sh
git clone git@gits-15.sys.kth.se:inda-25/<KTH_ID>-task-7.git
```
3) Add the upstream for `task-7-<KTH_ID>` to your local repository.
```sh
git remote add plus git@github.com:IndaPlus25/task-7-<KTH_ID>.git
```
4) The repository is your workspace. You may organise it however you want. I recommend to seperate the regular assignment from the plus assignment in two seperate directories. 
5) Create one `.c` file per subassignment. Name then descriptivly.   

For help with code setup, begin by copying the contents of `./kattis_template/main.c` into your `.c` file.

## Assignment

What!? This seems very familiar! Yes ettan. Your assignment this week it to complete the Kattis problems which you did not complete during week 2, but this time in C. Good luck!

See `./minimal_scalar_product` for a Kattis solution example.

**Don't forget** to include screenshots of Kattis to prove that your solutions work.

### Kattis problems

Solve two of the following problems:
- [Summera tal](https://kth.kattis.com/problems/kth.javap.sumsort)
- [Avstånd till kanten](https://kth.kattis.com/problems/kth.javap.kant)
- [Cyber-Clara och anmälningslistorna](https://kth.kattis.com/problems/kth.grupdat.anmalningslistorna)
- [A Different Problem](https://kth.kattis.com/problems/different)

_(optional fun)_:
- [Game Rank](https://open.kattis.com/problems/gamerank)
- [Quantum](https://open.kattis.com/problems/quantum)

_(optional challenge)_:
- Rust, rightfully, dominates the [Cyber-Clara och anmälningslistorna statistics board](https://kth.kattis.com/problems/kth.grupdat.anmalningslistorna/statistics). Respectfully put C on the list.

### Questions

#### Data control

Observe the following code:

```c++
int length = 0;
scanf("%d", &length); 

int *vector = malloc(length * sizeof(int));
for (int i = 0; i < length; i++) 
    scanf("%d", &vector[i]);

free(vector);
```

Know the answer of the following question:
- What is happening line for line?

#### Gammal hederlig läsförståelse

Observe the following function:

```c++
#include <stdio.h>
#include <time.h>

// Assume that f is a function which takes longer time to execute
// for a larger value n.

void someFunction(void (*f)(int), int milliseconds, int n) {
    int milliseconds_since = 1;
    int end = 1;

    do {
        n = (int)(n * (double)end / milliseconds_since);

        milliseconds_since = clock() * 1000 / CLOCKS_PER_SEC;
        end = milliseconds_since + milliseconds;

        f(n);

        milliseconds_since = clock() * 1000 / CLOCKS_PER_SEC;
    } while (end - milliseconds_since > 100);

    printf("\nLargest n: %d", n);
}
```

Know the answer of the following question:
- What does `someFunction` do?

#### Data types are like whaaat?

Observe the following function:

```c++
int x;

printf("Nothing: %d\nGive x a value: ", x);

scanf("%d", x);

printf("\nYour value is: %d\n", x);
```

Know the answer of the following question:
- What is printed?