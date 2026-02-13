# DD1338 Week 18

## Dynamic Programming (mostly)! 

You think you're a good programmer eh? Well think again!

![](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.redd.it%2Foy8znwiq1b111.png&f=1&nofb=1)

## Assignment
This week, like some other weeks, you have two different assignments:

- Solve an ADK exercise problem and some other Kattis problem.

Remember that the plus-assignment is about doing your best. You may fail and still pass.

### Prepare for your assigment

1) Create a repository named `task-18-<KTH_ID>`.
2) Clone your regular assignment repository.
    ```sh
    git clone git@gits-18.sys.kth.se:inda-25/<KTH_ID>-task-18.git
    ```
3) Add the upstream for `task-18-<KTH_ID>` to your local repository.
    ```sh
    git remote add plus git@github.com:IndaPlus25/task-18-<KTH_ID>.git
    ```
4) Navigate into your newly created repository and start writing.

See `./kattis_template` for start with the Kattis assignment in Rust.

See `./korpus.txt`, `./token.c`, and `./token.zip` for dependencies to the Concordance assignment.

To make your Kattis-experience easier, see `./kattis_template/src/main.rs`. If you're mot all förmodan using Java, make use of [this utility class](https://open.Kattis.com/download/Kattio.java?1a0093=) to simplify input-output fast enough to satisfy the judge.

### ADK exersice + Kattis

If the above ADK problem is too difficult, we have provided you an alternative!

- Solve at least two in total, where
    - at least one problem is from `./Assignment.pdf` and
    - at least one is of the followning Kattis problem list regarding graphs and dynamic programming:
      - [Single source shortest path, non-negative weights](https://open.kattis.com/problems/shortestpath1)
      - [All Pairs Shortest Path](https://open.kattis.com/problems/allpairspath)
      - [Proving Equivalences](https://open.kattis.com/problems/equivalences)
      - [Bond](https://open.kattis.com/problems/bond)
      - [Entertainment Box](https://open.kattis.com/problems/entertainmentbox)

The first problem from the PDF is the easiest. There's plenty of Kattis problems to choose from. Feel free to solve muliple of them.

### Grading

If you decide to do any of the non-Kattis exercises from the Multiple Problems category, we require you to write tests to prove the validity of your code.
