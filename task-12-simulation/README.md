# DD1338 Week 12

## The Nature of Code

With all the BAS-MATTE, LINALG, and zombified Fysik 1-2, you are ready to take on AD through the lense of simulation; in this case, extremely basic simulation.

_The Nature of Code_ is a 14 week course written by [Daniel Shiffman](https://www.youtube.com/c/TheCodingTrain/featured). It covers the basics of physics simulation, including particle systems and flocking behaiviour. Your assignment for the next week is to follow and implement one chapter of _The Nature of Code_. Yes, this do include the front-end samples.

Book: https://natureofcode.com/book/preface/ 

### Prepare Assignment

1) Create a repository named `task-12-<KTH_ID>`.
2) Clone your regular assignment repository.
    ```sh
    git clone git@gits-15.sys.kth.se:inda-25/<KTH_ID>-task-12.git
    ```
3) Add the upstream for `task-12-<KTH_ID>` to your local repository.
    ```sh
    git remote add plus git@github.com:IndaPlus25/task-12-<KTH_ID>.git
    ```
4) Navigate into your newly created repository and start writing.
    - Implement the logic of a chapter of choise.
    - Visualise the logic, either like the book samples, or something more creative.
    - Consider writing unit tests as practise.

**Goldstar** for the Rustacean who explore the wonders of Rust by implementing the solution using Rust WebAssembly (logic implementation in Rust and frontend in Javascript). 

**Note**: You may implemet your solution using any language (this includes Python *_host_ psudokod *_host_ *_host_). The only drawback is that the book assume that you run OOP (C++). Obviously, this is not the case (for me). However, you should be able to do your own interpretations.

#### Rust Game Engines

There are several Rust game engines to choose from. Shortlist example: https://blog.logrocket.com/5-rust-game-engines-consider-next-project/

See `./nature-of-code-introduction` for three examples in Piston.

- _GGEZ_: Interface with game loop through implementation of `EventHandler`. May not work on every obscure Linux configuration.
- _Piston_: Events as a stream iterator. You write the event loop. Don't handle text with grace.

#### Rust WebAssembly

To prepare for Rust WebAssembly, do the following:
1) Install the [Rust WebAssembly framework](https://rustwasm.github.io/wasm-pack/installer/).
2) Install the [latest version of the Javascript package manager `npm`](https://nodejs.org/en/download/).
3) Follow the tutorial of your choise. 
    - See a [tutorial for game of life](https://rustwasm.github.io/docs/book/game-of-life/introduction.html).

### Grading

Because your solution can be implemented using any language, write in a README file of how to build and run your application (that includes how to run any unit tests and your frontend application).