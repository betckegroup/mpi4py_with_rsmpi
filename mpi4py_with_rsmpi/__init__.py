from mpi4py import MPI

from .mpi4py_with_rsmpi import ffi, lib

COMM = MPI.COMM_WORLD


def say_rank(comm):
    """Print the rank from Rust"""

    comm_ptr = ffi.cast('uintptr_t', MPI._addressof(comm))
    lib.say_rank(comm_ptr)
