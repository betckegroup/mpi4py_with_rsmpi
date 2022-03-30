# Passing an MPI communicator from mpi4py to Rust rsmpi

This small example demonstrates how to pass a communicator
from Python to Rust and use it with rsmpi from within Rust.

It is developed so that maturin can automatically generate the
correct cffi bindings. To compile just call

```
maturin develop -b cffi
```

This compiles the Rust part, generates a pip module and installs
the pip module. To execute the code create a small file called
`say_rank.py` which contains

```
from mpi4py_with_rsmpi import say_rank, COMM

say_rank(COMM)
```

and call it for example with

```
mpirun -n 4 python say_rank.py
```

This will initialize the MPI system within Python. But the rank of each
process is printed out from Rust with rsmpi.



