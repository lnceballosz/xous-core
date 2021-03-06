use crate::{
    map_memory, send_message, Error, MemoryFlags, MemoryMessage, MemoryRange, MemorySize, Message,
    Result, CID,
};

pub struct String<'a> {
    raw_slice: &'a mut [u8],
    s: &'a str,
    len: usize,
}

impl<'a> String<'a> {
    pub fn new(max: usize) -> String<'a> {
        let mem = map_memory(None, None, max, MemoryFlags::R | MemoryFlags::W).unwrap();
        let p = mem.as_mut_ptr();
        for i in 0..max {
            unsafe { p.add(i).write_volatile(0) };
        }
        String {
            raw_slice: unsafe { core::slice::from_raw_parts_mut(mem.as_mut_ptr(), max) },
            s: unsafe {
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(mem.as_ptr(), 0))
            },
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Convert a `MemoryMessage` into a `String`
    pub fn from_message(
        message: &'a mut MemoryMessage,
    ) -> core::result::Result<String<'a>, core::str::Utf8Error> {
        let raw_slice =
            unsafe { core::slice::from_raw_parts_mut(message.buf.as_mut_ptr(), message.buf.len()) };
        let starting_length = message.valid.map(|x| x.get()).unwrap_or(0);
        Ok(String {
            raw_slice,
            s: core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(message.buf.as_ptr(), starting_length)
            })?,
            len: 0,
        })
    }

    /// Perform an immutable lend of this String to the specified server.
    /// This function will block until the server returns.
    pub fn lend(
        &self,
        connection: CID,
        id: crate::MessageId,
    ) -> core::result::Result<Result, Error> {
        let memory_range =
            MemoryRange::new(self.raw_slice.as_ptr() as _, self.raw_slice.len()).unwrap();
        let msg = MemoryMessage {
            id,
            buf: memory_range,
            offset: None,
            valid: MemorySize::new(self.len).map(Some).unwrap_or(None),
        };
        send_message(connection, Message::Borrow(msg))
    }

    /// Move this string from the client into the server.
    pub fn send(
        self,
        connection: CID,
        id: crate::MessageId,
    ) -> core::result::Result<Result, Error> {
        let memory_range =
            MemoryRange::new(self.raw_slice.as_ptr() as _, self.raw_slice.len()).unwrap();
        let msg = MemoryMessage {
            id,
            buf: memory_range,
            offset: None,
            valid: MemorySize::new(self.len).map(Some).unwrap_or(None),
        };
        send_message(connection, Message::Move(msg))
    }

    /// Clear the contents of this String and set the length to 0
    pub fn clear(&mut self) {
        self.len = 0;
        self.s = unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.raw_slice.as_ptr(),
                self.len,
            ))
        };
    }
}

impl<'a> core::fmt::Display for String<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl<'a> core::fmt::Write for String<'a> {
    fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
        for c in s.bytes() {
            if self.len < self.raw_slice.len() {
                self.raw_slice[self.len] = c;
                self.len += 1;
            }
        }
        self.s = unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.raw_slice.as_ptr(),
                self.len,
            ))
        };
        Ok(())
    }
}
