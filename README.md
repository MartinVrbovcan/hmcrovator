# Hmcrovator

Hmcrovator (short for **h**o**m**e **cro**atian e**valuator**) is a simple and quick command line code evaluator built for
Croatian coding competitions (specifically designed to work for the test cases
of [COCI](https://hsin.hr/honi/)/[HONI](https://hsin.hr/coci/) and
[The National Croatian Computer Science Competition](https://informatika.azoo.hr/)).

**NOTE:** Hmcrovator does **not check the evaluated program's memory usage**, and **only loosely checks program running time** (meaning the measurement is not that precise, IE. the actual running time may be a bit faster or slower), **nor does it use sandboxing** (meaning the program can still create files, network requests, spawn new threads and other prohibited things). That said, it should work well enough for cases where you want to quickly check if your program passes some test cases (this is the "home" part of hmcrovator). The test case inputs and outputs are tested through standard input and output.

## Installation

The prebuilt program binaries are available at `/dist` (currently only built for 64 bit WIN10). Simply copy the appropriate binary to be able to use hmcrovator.


### Building your own binary

If a prebuilt binary is not compatible, or if you would like to build the binary yourself, you will need to have [Rust](https://www.rust-lang.org/tools/install) installed. To build your own binary, `git clone` this repo and run `cargo build --release` in the cloned repository.

## Usage

To run your program, hmcrovator takes an "action command" which tells it how
to start your program. The action is enclosed in quotes and is thus able to ignore spaces. An example action command would be `"python ./myfile.py"`, or
`"./myfilecompiled.exe"`.

After the action command, you also need to specify the test case(s). To specify a single test case, use the `-i` and `-o` arguments. The `-i` argument takes the path to the test case input, and the `-o` argument takes the path to the test case output. An example:
```bash
> ./hmcrovator.exe "./taskA.exe" -i "./tasks/a/testcases/a.1.in" -o "./tasks/a/testcases/a.1.out"
```

More often than not, you will have a lot of test cases and testing them individually would be time consuming. This is where the `-iod` argument comes in. The `-iod` argument (*input/output directory*) takes the path to a directory containing the test case inputs and outputs (note that the test case files are not checked recursively) and automatically tests all test cases that are inside the directory. The correct test case inputs and outputs are paired according to filename. They must be in the format **[something].(in/out).[id]** or **[something].[id].(in/out)** and should work on the above noted Croatian programming competitions. An example:

```bash
> ./hmcrovator.exe "./taskF.exe" -iod "./f/testcases/"
```

With the `-t` argument you can set the time limit for a task. For example, with `-t 5` every test case will wait for the program to terminate within 5 seconds, or it will fail with `TIME LIMIT EXCEEDED`.

## Full example usage

Test case file structure:
```
+ testcases
|
|- a.1.in
|- a.1.out
|- a.2.in
|- a.2.out
|- ...
```

```bash
> ./hmcrovator.exe "python ./programs/taskA.py" -iod "./tasks/a/testcases/" -t 1
a.1.in a.1.out
ACCEPTED
a.2.in a.2.out
WRONG ANSWER
a.3.in a.3.out
TIME LIMIT EXCEEDED
a.4.in a.4.out
ACCEPTED
```

Crovator recognises test case formats used by the above noted competitions.
`Accepted` means that the program output was matched exactly, `Time limit exceeded` means the program did not terminate in the given time limit, and `Wrong answer` means the program terminated, but its output does not match the test case correct output.

## All arguments
```
Options:
    -t The time limit for each test case (a whole number, in seconds, the default is 1)
    -i Test case input (if using on 1 test case)
    -o Test case correct output (if using on 1 test case)
    -iod Input/Ouput directory (overwrites -i and -o) - a directory containing the program
        (test case) inputs and outputs, in the format (something).in.[identifier],
        and (something).out.[identifier].
        (NOTE: also works if the .in and .out extensions are at the end of the filename)
        The corresponding inputs and outputs are matched according to the identifier.
        The test cases are processed alphabetically according to the identifiers
    -v Verbose output - output the expected test case output and output returned from the program in the case of a wrong answer
    -c Colored output - highlight output with colors and styling (may not display properly on all consoles)
```

## Contributing
If you notice a bug or have a question, feel free to open an issue.

This is just a pet project of mine and is my first dip into Rust. I am aware that it isn't efficient or super maintainable, so if you have some tips or would like to refactor the code to something better fell free to do so :)

## License

[MIT](https://choosealicense.com/licenses/mit/)
