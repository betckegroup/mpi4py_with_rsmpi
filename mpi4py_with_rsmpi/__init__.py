from mpi4py import MPI

from .mpi4py_with_rsmpi import ffi, lib

COMM = MPI.COMM_WORLD


def say_rank(comm):
    """Print the rank from Rust"""

    # Here, we take the address of the pointer and then
    # cast the address into a uintptr_t, which can be
    # passed into Rust.
    comm_ptr = ffi.cast('uintptr_t', MPI._addressof(comm))
    lib.say_rank(comm_ptr)
