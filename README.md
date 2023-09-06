# clap-env-test

Thanks to [tangowithfoxtrot](https://github.com/tangowithfoxtrot) for originally discovering this!

**Introduction**

This repository was created to test the existence of a bug in the [clap](https://github.com/clap-rs/clap) crate, and to be an aid in testing. Let's first define the correct behavior for printing help text in a CLI.

**What is the correct behavior?**

The help text in a CLI should be printed when no options are provided. (Additionally, when `-h` or `--help` flags are provided.) This is the usual industry standard, and it's also documented [here](https://clig.dev/#help).

**Where does the behavior begin to fail?**

When using [clap](https://github.com/clap-rs/clap), the help text is successfully printed when no options are provided, with one exception. If an environment variable is set in the shell that the program also expects is present, the help text is no longer printed.

**How to reproduce**

To reproduce this, please build the Rust binary in this repository with `cargo build`. You can then navigate to `target/debug` and run the program like the examples below.

The failure cases will occur if you set the `ACCESS_TOKEN` environment variable, like so: `export ACCESS_TOKEN="12345"`

**Success Cases**

```
% ./clap-env-test
Usage: clap-env-test [OPTIONS] <COMMAND>

Commands:
  employee  Commands related to employees
  email     Commands related to emails
  help      Print this message or the help of the given subcommand(s)

Options:
  -t, --access-token <ACCESS_TOKEN>  [env: ACCESS_TOKEN=]
  -h, --help                         Print help
  -V, --version                      Print version
```

```
% ./clap-env-test employee
Usage: clap-env-test employee [OPTIONS] <COMMAND>

Commands:
  get     
  create  
  help    Print this message or the help of the given subcommand(s)

Options:
  -t, --access-token <ACCESS_TOKEN>  [env: ACCESS_TOKEN=]
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
```

**Failure Cases**

```
% ./clap-env-test            
error: 'clap-env-test' requires a subcommand but one was not provided
  [subcommands: employee, email, help]

Usage: clap-env-test [OPTIONS] <COMMAND>

For more information, try '--help'.
```

```
% ./clap-env-test employee   
error: 'clap-env-test employee' requires a subcommand but one was not provided
  [subcommands: get, create, help]

Usage: clap-env-test employee [OPTIONS] <COMMAND>

For more information, try '--help'.
```