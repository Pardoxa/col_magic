# Command line tool for tables

Useful for tables stored as .txt or .csv if you don't want to 
open a gui program and just want to do quick calculations from the terminal,
e.g., for plotting with gnuplot


WORK IN PROGRESS

## Usage:

Used to do table calculations on a text file. For each command you give, a column will be outputted

    C0 stands for the index
    C1 stands for column 1
    C<n> stands for column n (exchange <n> for the column number!)

Currently implemented operators: + - * /

```bash
col_magic table.dat "C0 + C1 * C2 ^ C3"
```

You can also use brackets

```bash
col_magic table.dat "(C0 + C1) * C2 ^ C3"
```

There are also functions implemented: abs, sin, sinh, cos, cosh, ln, log10, asin, asinh, acos, acosh, sqrt, cbrt,
signum, round, floor, ceil

```bash
col_magic table.dat "sin(ln(C0 + C1) * C2) ^ round(C3)"
```

You can also use numbers and pi will be changed to the respective value like so:
```bash 
col_magic table.dat "2/pi+sin(ln(C0 + C1) * C2) ^ round(C3)*0.231"
```

Lastly there are functions that need two inputs:

min, max

They can be used like so:

```bash
col_magic table.dat "min(C0,pi)" "max(C0, pi)" "max(C1, C2)" "min(C0*C1+23.9,C0*abs(C1)/0.1*min(C0,C1))"
```

USAGE:
    col_magic [FLAGS] <file> [commands]...

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            Prints information during command parsing


ARGS:
    <file>           
            

    <commands>... 