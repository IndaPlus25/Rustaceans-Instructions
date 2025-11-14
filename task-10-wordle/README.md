# DD1338 Week 10 | Wordle Project

To wrap up DD1337, you will embark on a fun mini project. See the intended learning outcomes as defined in the [course syllabus](https://www.kth.se/student/kurser/kurs/DD1337?l=en): 

> On completion of the course, course participant should be able to
> - use common computer tools, especially the computer environment at D,
> - design and implement simple sequential programs
> - use programming to solve problems,
> 
> in order to be able to
> - find and use correct programming techniques for a given problem,
> - take advanced courses within computer science and numerical analysis.

To encompass/prove these learning outcomes, you are tasked to write Wordle.

Due to the course content stating: "The programming language Java is used." Java is permitted.

## Prepare for your assigment

1) Create a repository named `task-10-<KTH_ID>`.
2) Clone your regular assignment repository.
    ```sh
    git clone git@gits-15.sys.kth.se:inda-25/<KTH_ID>-task-10.git
    ```
3) Add the upstream for `task-10-<KTH_ID>` to your local repository.
    ```sh
    git remote add plus git@github.com:IndaPlus25/task-10-<KTH_ID>.git
    ```
4) This is your project workspace. Organise it however you want.

Go nuts! o(￣▽￣)ｄ

## Assignment

There are numerous Wordle implementations:
* [The original from NYT](https://www.nytimes.com/games/wordle/index.html)
* [Absurdle](https://absurdleonline.github.io/). The game activly changes the target word to extend the game as long as possible.
* [Don't Wordle](https://dontwordle.com/). Try not to guess the correct word.
* [Nerdle](https://nerdlegame.com/). Guess math equation.
* [Lewdle](https://wordleplay.com/lewdle). Wordle that only accept lewd words.
* [Wizarding Wordle](https://www.harrypotterwordle.com/). Wordle that only accept Harry Potter terminology.

Create your own Wordle implementation with the following criterias:
* Use a language Rust, C, Prolog, or Java.
* You may write either for CLI or a GUI.
* List your sources, both for help and any word databases.
  * Exempel på svenska ordlistor:
    * https://github.com/hising/svenska-ord.txt
    * https://github.com/titoBouzout/Dictionaries/blob/master/Swedish.txt 

_Challenge_: Make an implementation that switch the target word mid-game, or that generate a target value rather than fetching one.