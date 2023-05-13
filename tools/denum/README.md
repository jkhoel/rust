# DENUM

A directory enumerator written in Rust

```command
$ ./denum --help             
denum 0.1.0

USAGE:
    denum [OPTIONS] --list <WORDLIST>

OPTIONS:
    -a, --address <HOSTADDRESS>         Ommit to target local paths
        --csv                           Outputs to CSV format
    -h, --help                          Print help information
        --json                          Outputs to JSON format
    -l, --list <WORDLIST>               Path to wordlist
    -o, --output <FILENAME>             Outputs to file
    -s, --status <STATUSCODE>           Limit output to this status code
    -t, --threads <THREAD_POOL_SIZE>    Number of threads to spawn (default is 12)
    -V, --version                       Print version information
```
