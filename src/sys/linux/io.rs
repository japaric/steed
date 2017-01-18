use io::ErrorKind;
use io::Read;
use io;
use slice::from_raw_parts_mut;

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

// Provides read_to_end functionality over an uninitialized buffer.
// This function is unsafe because it calls the underlying
// read function with a slice into uninitialized memory. The default
// implementation of read_to_end for readers will zero out new memory in
// the buf before passing it to read, but avoiding this zero can often
// lead to a fairly significant performance win.
//
// Implementations using this method have to adhere to two guarantees:
//  *  The implementation of read never reads the buffer provided.
//  *  The implementation of read correctly reports how many bytes were written.
pub unsafe fn read_to_end_uninitialized(r: &mut Read, buf: &mut Vec<u8>) -> io::Result<usize> {

    let start_len = buf.len();
    buf.reserve(16);

    // Always try to read into the empty space of the vector (from the length to the capacity).
    // If the vector ever fills up then we reserve an extra byte which should trigger the normal
    // reallocation routines for the vector, which will likely double the size.
    //
    // This function is similar to the read_to_end function in std::io, but the logic about
    // reservations and slicing is different enough that this is duplicated here.
    loop {
        if buf.len() == buf.capacity() {
            buf.reserve(1);
        }

        let buf_slice = from_raw_parts_mut(buf.as_mut_ptr().offset(buf.len() as isize),
                                           buf.capacity() - buf.len());

        match r.read(buf_slice) {
            Ok(0) => { return Ok(buf.len() - start_len); }
            Ok(n) => { let len = buf.len() + n; buf.set_len(len); },
            Err(ref e) if e.kind() == ErrorKind::Interrupted => { }
            Err(e) => { return Err(e); }
        }
    }
}
