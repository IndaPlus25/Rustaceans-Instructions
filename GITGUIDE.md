# Git Good Guide

Good version control habits are key to keeping your future coworkers sane.

_Author_: Viola Söderlund, violaso@kth.se

## How to Repository

### 0. Configure your Github accounts.

1. Add your system user's SSH key to your KTH Github accout in [settings](https://gits-15.sys.kth.se/settings/keys). 
2. Then, add the same key to your public Github account in [settings](https://github.com/settings/keys).

### 1. Configure your repository.

Lets clone your git repository for the regular assignment from the _inda-25_ organisation at KTH Github.
```sh
git clone git@gits-15.sys.kth.se:inda-25/<your-assignment-repository>.git
```

Confirm your remote `origin` destination.
```sh
git remote -v
```

You should get:
```
C:\[...]> git remote -v
origin  git@gits-15.sys.kth.se:inda-25/<your-assignment-repository>.git (fetch)
origin  git@gits-15.sys.kth.se:inda-25/<your-assignment-repository>.git (push)
```

This is the inherited upstream from your regular assignment repository. I strongly encourage you to participate in collaborative learning. You are able to share your solution with your course buddies by adding an addional public upstream to the [IndaPlus25 organisation](https://github.com/IndaPlus25). 

> You might think it scary to share your crappy code with strangers. Think about it; haven't you learned to code by reading others' code? Your future career will demand you to collaborate with, and judge, the work of people that you barely know. Only practise make perfect.

Create a new _public_ repository in the [IndaPlus25 organisation](https://github.com/IndaPlus25). Then, add the new upstream to your local repository. This will allow you to push your solution to both upstreams.

* _(optional)_ Rename the `origin` upstream to `kth`: 
```sh
git remote rename origin kth
```
* Add the new upstream to _INDAPlus25_ as `plus`:
```sh
git remote add plus git@github.com:IndaPlus25/<your-public-repository>.git
```
* Confirm your changes:
```
C:\[...]> git remote -v
kth  git@gits-15.sys.kth.se:inda-25/<your-assignment-repository>.git (fetch)
kth  git@gits-15.sys.kth.se:inda-25/<your-assignment-repository>.git (push)
plus  git@github.com:IndaPlus25/<your-public-repository>.git (fetch)
plus  git@github.com:IndaPlus25/<your-public-repository>.git (push)
```

Now you're able to push commits to both organisations.

> To keep up with the rules of EECS, we have to utilise _KTH Github_. However, this doesn't allow us to keep things public and open for co-development. This is why were operating with double upstreams.
>
> The development of a public Github portfolio will also help you in the hunt for interships and part-time jobs.

### 2. Prepare your repository.

#### Lay the Foundation for Good Practise

The Internet is littered with guides on how to write good code. If in doubt, search for others' opinions.

I recommend the following industry-common practises:

* __Maintain your own seperate documentation.__ This might be in a markdown file (_DOCS.md_, _README.md_, _GUIDE.md_, or _SPECS.md_). The document may contain a notes on how to start development, how to build and execute your application, how to maintain your work, or a to-do list, among else.
* __Write issues.__ Github allow you to write issues, on which you can comment, link commits, label, and close upon irrelevance. These enable you to follow the development process of your project.
* _(optional)_ __Write milestones.__ Github allow you to define milestones that group issues towards  abigger goal.
* _(optional)_ __Declare projects.__ Github projects allow you to use several tools to organise your development together with potential teammates. My favourite is the カンバン board feature (think Trello), that can link cards to issues, among else.
* __Maintain a `.gitignore` file.__ This file lets you exclude local files from your git repository. Highly recommended to ignore for executable files, temporary files, private notes, and more.
* __Maintain a standard for file header comments.__ These comments gives credit to the creators of a file, in case other developers (including AI) would like to use your code. Furthermore, it should also contain a context for the existence of the code.
* __Add a `LICENSE` file.__ Clear any possible confusions by adding a license file to the root directory. This file contains legal text for future use, development, and distribution of your source code.
  * Common permissive licenses:
    * [MIT](https://mit-license.org/)
    * [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.html)
  * Common restricted licenses:
    * [Creative Commons](https://sv.wikipedia.org/wiki/Creative_Commons#Licenser)
  * Otherwise:
    * Add the line `All rights reserved.` to your documentation and file header comments.
    * Example: `Copyright (C) 2025 Viola Söderlund - All rights reserved.`
* _(optional)_ __Refuse AI.__ Include the line `Use of this code or any derivatives thereof for the purpose of training, fine-tuning, or evaluation of machine learning or artificial intelligence models is expressly prohibited.` in any documentation and file header comments.
* __Give credit.__ When due, give credit to your fellow collaborators and influencers, both active and passive.
* _(optional)_ __Maintain a version standard.__ There exists multiple version standards. 
  * Example format $V.S.F.P$ (initial commit is version $0.0.1.0$):
    * $V$: Release version.
    * $S$: Development stage; pre-alpha $=0$; alpha $=1$; beta $=2$; etc.
    * $F$: Latest new feature by index.
    * $P$: Latest patch or bug-fix by index.
  * Example format for an INDA assignment $V.T.S.P$ (initial commit is version $0.1.0.0$):
    * $V$: Release version. 
    * $T$: Task by index or count.
    * $S$: Sub-task by index or count.
    * $P$: Latest patch or bug-fix by index.

##### Java File Header Comment Example

```java
/**
 * File: <filename>
 * Description: <description>
 * 
 * Assignment: <assignment>
 * Course: <course>
 * KTH Royal Institute of Technology
 * 
 * Authors: <author-name> <author-contact-information>, [...]
 * License: <license>
 * Created On: <date>
 * 
 * Version History:
 * <version> - Initial implementation.
 * <version> - <message>
 * [...]
 * 
 * Copyright (C) 2025 <authors-names> 
 */
```

Note: The version history should mirror that of your git repository.

## How to Synchronise your Work

Assume that you have written and saved some code locally! Lets structure the changes in different versions. In short:
1. `git add`: Stage code for the next version.
2. `git commit`: Create a new version (also called a _commit_) of your code base.
3. `git push`: Synchronise your commit history with that of your upstream.

View which of your files are changed, and staged and not:
```sh
git status
```

### 0. Spectate differences between versions.

View differences in your saved code next to other versions.

* Between your saved code and the latest commit:
```sh
git diff <.|path>
```
* Between your saved code and the staged changes:
```sh
git diff --staged <.|path>
```
* Between your saved code and a specific commit:
```sh
git diff <commit-id> <.|path>
```

### 1. Stage your saved code.

You can choose to stage all files for the next commit, or only specific files, or even only part of files.

* Stage all files:
```sh
git add .
```
* Stage one file/directory:
```sh
git add <path>
```
* Stage specific changes in a file (called _add patch_):
```sh
git add -p <path>
```

ULikewise, use the command `git reset` to unstage any code.

### 2. Commit a new version.

You may commit separate staged files at a time. I recommend the following: To master the staging process to only commit everything staged at once.

Commit every staged file at once as a new version with a message:
```sh
git commit -m "<message>"
```

I also recommend you to keep a standard format to your messages. For example: `"<version>: #<issue-id>: <message>"`. The `#<id>` will automatically link your commit to the issue of that index.

Commited files are also called _tracked files_.

### 3. Regret your latest commit.

Lets say that you find a mistake in your code. This blunder can be fixed into your latest commit.

1. Update and save your code.
2. Stage your changes.
3. Amend:
```sh
git commit --amend
```

### 4. Push your new commits to your upstream.

Lets synchronise your changes.

```sh
git push <upstream> <branch>
```

Example: `git push kth main` and `git push plus main`.

> `main` or `master` are industry standard names for the default branch of your code base, whereas `master` is the legacy option.

If you get an error asking for a "`pull`" to merge, the upstream version tree doesn't match that of your local clone. In other words, some change to the files upstream has occured since your last pull. Then merge with upstream before pushing:
```sh
git pull <upstream> <branch>
```

If you get a merge error, you have to help two version trees to merge.
1. Resolve any merge errors.
2. Stage and commit your merge fix.
3. Try to push again.

### 5. Admire your version tree.

Look at your current version branch:
```sh
git log --oneline
```

## Git Methodologies

A git methodology is a framework for how you organise your versions in branches. Branching becomes necessary when multiple teammates are working on the same code base. They allow for different people to work on seperate features without disrupting each others code.

This only becomes relevant in our last INDA course, _DD1349 Project in Introduction to Computer Science_.

Learn git branching: https://learngitbranching.js.org/



### Git Flow

The default branch is used for tracking versions for release to the public, while development is based from a _development_ branch.

![](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fjeffkreeftmeijer.com%2Fgit-flow%2Fgit-flow.png&f=1&nofb=1&ipt=28bb5694e23cf5e9a833840fe6ef5932c0f4419300a2bccec9911c22920e6754)

### Trunk-Based Development

Instead of having the default branch track release versions, the default branch becomes the development branch. Release versions have their own branches, on which release-specific patches may be commited. Hotfixes should however be merged back into the development branch.

The advantage of this approach is that old releases can be further developed without forking the repository.

![](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fd33wubrfki0l68.cloudfront.net%2F2b214bae5e64192033f1d06cc50d977c0b547313%2F84498%2Fimg%2Fdiagram-of-trunk-based-development.png&f=1&nofb=1&ipt=1078c2a81924e52872fe05d8a288f38afe4a66cd5660cda24136273dbc32879b)