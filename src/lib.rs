use mpi::ffi::MPI_Comm;
use mpi::topology::{Communicator, UserCommunicator};
use std::mem::ManuallyDrop;

#[no_mangle]
pub extern "C" fn say_rank(comm: usize) {
    // We need to mark the communicator as ManuallyDrop since
    // otherwise its drop method will clean up the MPI system when
    // it goes out of scope. But this should be handled on the Python
    // side by mpi4py.

    // The communicator is passed as pointer that has been converted
    // to an unsigned integer. We need to convert this pointer back
    // and dereference to get the actual communicator.

    // By only passing integers from Python to Rust maturin does not
    // need to know details of the MPI data structures and the bindings
    // can be automatically generated independently of the underlying
    // MPI implementation.

    let comm = ManuallyDrop::new(unsafe {
        UserCommunicator::from_raw(*(comm as *const MPI_Comm)).unwrap()
    });
    println!("My rank is {}", comm.rank());
}
