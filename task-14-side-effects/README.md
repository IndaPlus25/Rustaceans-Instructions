# DD1338 Week 14 | Shared Resource Management

I hope you have had a great time with your exams and holiday celebrations. Now its time to get on coding again. Yay! 

## Multi-threading and Side Effects

This weeks task is to work with communication over sockets! To do this, you will have to select a programming language of your choice and figure out how sockets work in that language. Most languages, like Java, have built in objects for sockets that are really easy to work with. Rust offers networking functionality via its standard library. I strongly suggest that you challenge yourself to a system development language, like C++, C, or Rust.

To get started, see the Rust book's [multithreaded server example project](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html). 

### Prepare your assignment

1) Create a repository named `task-14-<KTH_ID>`.
2) Clone your regular assignment repository.
    ```sh
    git clone git@gits-15.sys.kth.se:inda-25/<KTH_ID>-task-14.git
    ```
3) Add the upstream for `task-14-<KTH_ID>` to your local repository.
    ```sh
    git remote add plus git@github.com:IndaPlus25/task-14-<KTH_ID>.git
    ```
4) Navigate into your newly created repository and start writing.

I have touched up a very simple Rust server and a client binary under `./rust-example`. Check it out! You are permitted to use this as a template, given that you can explain how it works. 

## Assignment

Your task is to write two programs: a client and a server. Make these a chat platform.

Requirements:
1. The server must support multiple clients.
2. The clients shoud be able to update a shared resource on the server, for example a poke counter.
3. The server should be written so that the shared resource is memory safe (protected from data races).

The point is for you to work with multi-threading and protected resources using channels and semaphores.

Of course you may implement whatever you like, file transfers, markdown, etc.

## Grading

I should be able to run your application locally on my Windows system. If you require me to use any specific libraries or compiler that I may have to download, specify this in a README file.