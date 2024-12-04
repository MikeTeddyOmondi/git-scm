# Ignoring, Staging & Committing Files

`staging` prepares files to be added to the next commit history and `committing` adds & records the changes to the commit history. You can also `ignore` files that can be recreated once again after another user clones a repository.

Continuing from our previous example `react-app` folder to:

Add the files we want to stage and commit...

```
git-scm add .
Files staged successfully
```

Commit the changes 

```
git-scm commit message --message "initial commit"
Commit created: dd88c78906a8ab0c
```

To ignore files, you have to add a `.gitignore` file in the current directory and they will be ignored while `staging`

