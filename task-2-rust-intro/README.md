# DD1337 Task 2 | Rust Introduction

_Author_: Viola Söderlund, violas@kth.se
_Modified by_: Isak Larsson

## Getting started

### Install Rust

1) Install [Rustup](https://rustup.rs/).
2) _If you're running Windows_, install [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
3)  _If you're developing in Visual Studio Code_, install the *superior* [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Prepare for your assigment

Your first assignment is turned in by uploading it to a repository named `task-2-<KTH_ID>` (ex. `task-2-violaso`) under the `IndaPlus25` organisation. Be careful to get the spelling right.

1) Create a repository named `task-2-<KTH_ID>`.
2) Clone your regular assignment repository.
```sh
git clone git@gits-15.sys.kth.se:inda-25/<KTH_ID>-task-2.git
```
3) Add the upstream for `task-2-<KTH_ID>` to your local repository.
```sh
git remote add plus git@github.com:IndaPlus25/task-2-<KTH_ID>.git
```
4) The repository is your workspace. You may organise it however you want. I recommend to seperate the regular assignment from the plus assignment in two seperate directories. 
5) Create one Rust crate (term for application or library) per subassignment (per Kattis solution).

See guides and manuals for Rust: https://www.rust-lang.org/learn.

#### How to create a Rust application (_binary_) crate

The _binary crate_ is the Rust term for an application root directory.

1) Navigate in your terminal/shell/CMD into your inda workspace.
2) Initialise your Rust crate.
```sh
cargo init task-2-<KTH_ID>
```
3) Navigate into your project root.
4) Build and run your application.
```sh
cargo run
```

Your clean build should look like:
```
task-2-<KTH_ID>
|- src
  |- main.rs
|- target
|- Cargo.lock
|- Cargo.toml
```

Write your source code in `src`, where the `main` function is located in `src/main.rs`. To make it easier, begin by copying the contents of `./kattis_template/src/main.rs` into your `main.rs` file.

## Assignment

### Kattis problems

This week, you're going to learn the basics of Rust by solving easier [Kattis](https://kth.kattis.com) problems. For each problem, create one Rust crate in your repo. Include a screenshot of your Kattis submission to prove solution. See `./minimal_scalar_product` for a Kattis solution example.

Solve at least two of the following problems:
- [Summera tal](https://kth.kattis.com/courses/DD2016/plusplus24/assignments/q4npcz/problems/kth.javap.sumsort)
- [Avstånd till kanten](https://kth.kattis.com/courses/DD2016/plusplus24/assignments/q4npcz/problems/kth.javap.kant)
- [Cyber-Clara och anmälningslistorna](https://kth.kattis.com/courses/DD2016/plusplus24/assignments/q4npcz/problems/kth.grupdat.anmalningslistorna)

_(optional fun)_:
- [Game Rank](https://open.kattis.com/problems/gamerank)
- [Quantum](https://open.kattis.com/problems/quantum)

_(optional challenge)_:
- _Cyber-Clara och anmälningslistorna_ is a special problem. The Rust [statistics board](https://kth.kattis.com/problems/kth.grupdat.anmalningslistorna/statistics) is littered with +- and ++-students. **Take them down!**
- A shitty example solution to the Kattis problem [Minimal Scalar Product](https://open.kattis.com/problems/minimumscalar) can be found in `./minimal_scalar_product`. This solution runs at 0.06s. See the [statistics](https://open.kattis.com/problems/minimumscalar/statistics) for the Rust language. As you can see, it's possible to solve this problem in much less time. Write your own solution, which may be based on the example solution, and which runs quicker than 0.06s.

### Questions

Be prepared to answer the following questions during the next övning.

#### Method chaining

Observe the following code:

```rust
let input = io::stdin();

let mut lines = input
    .lock()
    .lines()
    .map(|_line| _line.ok().unwrap().to_string());

// for every line, assuming input strings with the characters '0' and '1' seperated by whitelines

let binary_line = lines
    .next().unwrap()
    .split(" ")
    .map(|_title| {
        _title
            .chars()
            .map(|_character| {
                match _character {
                    '0' => false,
                    _ => true
                }
            })
            .collect::<Vec<bool>>()
    })
    .collect::<Vec<Vec<bool>>>();
```

Know the answer of the following question:
- What is the value of `binary_line`?

#### Iterables

Observe the following code:

```rust
use std::collections::{ HashMap, HashSet };

/*...*/

let limit: usize = 10;

let mut index_store: HashMap<usize, usize> = HashMap::with_capacity(limit + 1);
let mut value_store: Vec<HashSet<usize>> = Vec::with_capacity(limit + 1);
        
for value in 1..(limit + 1) {
    index_store.insert(value, value - 1);

    let mut sequence: HashSet<usize> = HashSet::new();
    sequence.insert(value);

    value_store.push(sequence);
}
```

Know the answer of the following questions:
- What is the value of `index_store`?
- What is the value of `value_store`?
