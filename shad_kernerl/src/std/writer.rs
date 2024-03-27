pub mod constants;
mod frame_buffer_writer;

use frame_buffer_writer::FrameBufferWriter;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub(crate) static ref FRAME_BUFFER_WRITER: Mutex<FrameBufferWriter> =
        Mutex::new(FrameBufferWriter::empty());
}

macro_rules! println  {
    ($content:expr) => {
        use core::fmt::Write;
        //below requires this
        writeln!($crate::FRAME_BUFFER_WRITER.lock(), "{}", $content).unwrap();
    };

    ($fmt_str:expr, $($args:expr),*) => {
        if let frame_buffer =  unsafe{&mut crate::FRAME_BUFFER_WRITER_INSTANCE}{
            use core::fmt::Write;
            writeln!(frame_buffer,"{}", format_args!($fmt_str, $($args),*)
                    ).unwrap();
        }
    };
}

macro_rules! move_cursor {
    ($x:expr,$y:expr) => {
        if let Some(frame_buffer) =unsafe{&mut crate::std::FRAME_BUFFER_WRITER_INSTANCE}{
            frame_buffer.set_pos($x,$y);
        }
    };
}

// Expose to crate lvl
pub(crate) use println; 
pub(crate) use move_cursor;

