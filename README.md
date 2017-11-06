cask-cli
=======================================================================

### Overview

cask-cli is a simple interactive and batch shell for [cask key-value store](https://github.com/andrebeat/cask).

### Build

~~~
git clone https://github.com/nak3/cask-cli.git
cd cask-cli
cargo build
~~~

Optionally `cargo build --release`.

### Usage

~~~
$ ./target/debug/cask-i -h
cask-cli 1.0
cask-cli is a command line interface for cask key value store.

USAGE:
    cask-cli [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --db <db>    specify database directory. default "cask.db"

SUBCOMMANDS:
    delete    delete the value with specified key.
    get       get the value from specified key.
    help      Prints this message or the help of the given subcommand(s)
    put       put the value with specified key.
~~~

### Example

#### interactive mode

~~~
mkdir -p /tmp/cask-test
~~~

1) start interactive mode

~~~
$ ./cask-cli --db=/tmp/cask-test
>
~~~

2) put items by key and value 

~~~
> put apple 100
> put orange 200
> put grape 300
~~~

3) get item by key

~~~
> get apple
100
~~~

4) Exit interactive mode

~~~
> quit
~~~


#### batch mode

1) set item by key and value 

~~~
./cask-cli --dir=/tmp/cask-test put banana 500
~~~

2) get item by key

~~~
./cask-cli --dir=/tmp/cask-test get banana 
500
~~~

