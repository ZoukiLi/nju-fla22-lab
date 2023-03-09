# Turing Machine Simulator

This is a simple cross-platform Turing Machine simulator implemented in Rust, using the trm_sim library. It includes a Machine struct which can read in a model from a file and run the machine based on that model.

## Usage
To use the Turing Machine simulator, you need to provide a model file that defines the machine's states and transitions. Here is an example of a model file:

```toml
[[state]]
name = 'A'
start = true

[[state.transitions]]
next = 'B'
cons = 'b'
prod = '_'
move = 'L'

[[state.transitions]]
next = 'C'
cons = '*'
prod = '*'
move = 'S'

[[state]]
name = 'B'
final = false

[[state]]
name = 'C'
final = true
```

The above model file defines a simple machine with three states, where the first state is the start state and the third state is a final state. The machine reads in an input and replaces the first occurrence of the character b with an underscore, and replaces all occurrences of the character * with itself, while not moving the tape head.

To run the machine with this model file, you can use the cli tool:

```bash
> trm_sim_cli -h

A cross-platform CLI tool for Turing machine simulator

Usage: trm_sim_cli [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>    The path for turing machine definition file
  -e, --ext <EXT>      The extension of the file, if not provided, will be inferred from the file path. Now only supports [json, yaml, toml]
  -v, --verbose        If provided, the machine will be run in verbose mode, every step will be printed
  -i, --input <INPUT>  The input string for the machine, if not provided, will be read from stdin
  -h, --help           Print help
  -V, --version        Print version
```


## Contributing
Contributions are welcome! If you have any ideas or suggestions for improvement, feel free to open an issue or submit a pull request.