# Initializing a git repository

Because the `git-scm` application was built using rust you must build the project 

```shell
cargo build --release # for an optimised build
```

Then add an alias if your are using Git Bash (Windows) or a Linux bash terminal so that you can run the command 
using the `git-scm` command. Make sure you set the correct path if you are on Windows.

```shell
alias git-scm="/home/picasso/Desktop/src/hackathons/git-scm/target/release/git"
```

Create an example react app to demonstrate how the `git-scm` application works

```
npm create vite@latest react-app -- --template react-ts
```

So now you can move into that directory and install all dependencies

```
cd react-app
npm install
```

If we list all the directories we will have the following list

```
❯ ls -al
total 164
drwxrwxr-x   5 picasso picasso   4096 Dec  4 15:49 .
drwxrwxr-x   3 picasso picasso   4096 Dec  4 15:31 ..
-rw-rw-r--   1 picasso picasso    734 Dec  4 15:31 eslint.config.js
-rw-rw-r--   1 picasso picasso    253 Dec  4 15:31 .gitignore
-rw-rw-r--   1 picasso picasso    366 Dec  4 15:31 index.html
drwxrwxr-x 130 picasso picasso   4096 Dec  4 15:49 node_modules
-rw-rw-r--   1 picasso picasso    681 Dec  4 15:31 package.json
-rw-rw-r--   1 picasso picasso 110115 Dec  4 15:49 package-lock.json
drwxrwxr-x   2 picasso picasso   4096 Dec  4 15:31 public
-rw-rw-r--   1 picasso picasso   1607 Dec  4 15:31 README.md
drwxrwxr-x   3 picasso picasso   4096 Dec  4 15:31 src
-rw-rw-r--   1 picasso picasso    665 Dec  4 15:31 tsconfig.app.json
-rw-rw-r--   1 picasso picasso    119 Dec  4 15:31 tsconfig.json
-rw-rw-r--   1 picasso picasso    593 Dec  4 15:31 tsconfig.node.json
-rw-rw-r--   1 picasso picasso    161 Dec  4 15:31 vite.config.ts

```

Now initialise a git repository with our `git-scm` binary

```
git-scm init
Initialized empty Git repository in .

```

When we check our list again, you should have a `.git-scm` folder listed together with the previous ones


```
❯ ls -al
total 168
drwxrwxr-x   6 picasso picasso   4096 Dec  4 15:51 .
drwxrwxr-x   3 picasso picasso   4096 Dec  4 15:31 ..
-rw-rw-r--   1 picasso picasso    734 Dec  4 15:31 eslint.config.js
-rw-rw-r--   1 picasso picasso    253 Dec  4 15:31 .gitignore
drwxrwxr-x   4 picasso picasso   4096 Dec  4 15:51 .git-scm
-rw-rw-r--   1 picasso picasso    366 Dec  4 15:31 index.html
drwxrwxr-x 130 picasso picasso   4096 Dec  4 15:49 node_modules
-rw-rw-r--   1 picasso picasso    681 Dec  4 15:31 package.json
-rw-rw-r--   1 picasso picasso 110115 Dec  4 15:49 package-lock.json
drwxrwxr-x   2 picasso picasso   4096 Dec  4 15:31 public
-rw-rw-r--   1 picasso picasso   1607 Dec  4 15:31 README.md
drwxrwxr-x   3 picasso picasso   4096 Dec  4 15:31 src
-rw-rw-r--   1 picasso picasso    665 Dec  4 15:31 tsconfig.app.json
-rw-rw-r--   1 picasso picasso    119 Dec  4 15:31 tsconfig.json
-rw-rw-r--   1 picasso picasso    593 Dec  4 15:31 tsconfig.node.json
-rw-rw-r--   1 picasso picasso    161 Dec  4 15:31 vite.config.ts

```


