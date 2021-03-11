# cli-template

This is a template to build a CLI with Rust. As it is, the generated CLI do nothing.  
Please refer to [structopt](https://docs.rs/structopt/0.3.21/structopt/) documentation
for adding **command** and **option**.

## Creating a new repository from template

Assuming you have a recent version of rust and cargo installed (via [rustup](https://rustup.rs)),  
then the following should get you a new repository:

First, install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate). Unless you did that before, run this line now:

``` sh
cargo install cargo-generate --features vendored-openssl
```

Go to the folder in which you want to place it and run:

``` sh
cargo generate --git https://github.com:alekece/cli-template --name YOUR_NAME_HERE
```

You will now have a new folder called `YOUR_NAME_HERE` (please change that name to something else)  
containing a simple CLI that you can customize.

## Add a remote repository

After generating, you have a local *git* repository, but no commits, and no remote.  
To add a new upstream repository (called `YOUR_GIT_URL` below), run the following :

``` sh
cargo check
git checkout -b master
git add .
git commit -m "Initial commit"
git remote add origin YOUR_GIT_URL
git push -u origin master
```
