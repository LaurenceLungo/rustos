use crate::atags::raw;
use core::str;
use core::slice;

pub use crate::atags::raw::{Core, Mem};

/// An ATAG.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None,
}

impl Atag {
    /// Returns `Some` if this is a `Core` ATAG. Otherwise returns `None`.
    pub fn core(self) -> Option<Core> {
        match self {
            Atag::Core(core) => return Some(core),
            Atag::Mem(_)|Atag::Cmd(_)|Atag::Unknown(_)|Atag::None => return None,
        }
    }

    /// Returns `Some` if this is a `Mem` ATAG. Otherwise returns `None`.
    pub fn mem(self) -> Option<Mem> {
        match self {
            Atag::Mem(mem) => return Some(mem),
            Atag::Core(_)|Atag::Cmd(_)|Atag::Unknown(_)|Atag::None => return None,
        }
    }

    /// Returns `Some` with the command line string if this is a `Cmd` ATAG.
    /// Otherwise returns `None`.
    pub fn cmd(self) -> Option<&'static str> {
        match self {
            Atag::Cmd(cmd) => return Some(cmd),
            Atag::Mem(_)|Atag::Core(_)|Atag::Unknown(_)|Atag::None => return None,
        }
    }
}

// FIXME: Implement `From<&raw::Atag> for `Atag`.
impl From<&'static raw::Atag> for Atag {
    fn from(atag: &'static raw::Atag) -> Atag {
        // FIXME: Complete the implementation below.

        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => return Atag::Core(core),
                (raw::Atag::MEM, &raw::Kind { mem }) => return Atag::Mem(mem),
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => {
                    let mut ptr = cmd as *const raw::Cmd as usize;
                    let mut len = 1;
                    while *(ptr as *const char) != '\0'{
                        ptr = ptr + 8 as usize;
                        len += 1;
                    }
                    let cmd_slice = slice::from_raw_parts(cmd as *const raw::Cmd as *const _, len);
                    let cmd_str = str::from_utf8(cmd_slice).expect("failed to cast cmd_slice to cmd_str");
                    return Atag::Cmd(cmd_str);

                },
                (raw::Atag::NONE, _) => Atag::None,
                (id, _) => return Atag::Unknown(id),
            }
        }
    }
}
