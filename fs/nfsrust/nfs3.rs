/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * NFSv3 protocol definitions
 */

use kernel::prelude::*;
use kernel::bindings::{PATH_MAX, NAME_MAX};

module! {
    type: RustNFS3,
    name: "rust_NFS3",
    author: "Rust for Linux Contributors",
    description: "Rust implementation of NFS3",
    license: "GPL",
}

struct RustNFS3 {
    numbers: Vec<i32>,
}

impl kernel::Module for RustNFS3 {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust minimal sample (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        let mut numbers = Vec::new();
        numbers.try_push(72)?;
        numbers.try_push(108)?;
        numbers.try_push(200)?;

        Ok(RustMinimal { numbers })
    }
}

impl Drop for RustMinimal {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust minimal sample (exit)\n");
    }
}

pub(crate) const PORT: i32 = 2049;
pub(crate) const MAXDATA: i32 = 32768;
pub(crate) const MAXPATHLEN: u32 = PATH_MAX; 
pub(crate) const MAXNAMLEN: u32 = NAME_MAX;
pub(crate) const MAXGROUPS: i32 = 16;
pub(crate) const FHSIZE: i32 = 64;
pub(crate) const COOKIESIZE: i32 = 4;
pub(crate) const CREATEVERFSIZE: i32 = 8;
pub(crate) const COOKIEVERFSIZE: i32 = 8;
pub(crate) const WRITEVERFSIZE: i32 = 8;
pub(crate) const FIFO_DEV: i32 = -1;
pub(crate) const NFS3MODE_FMT: i32 = 0170000;
pub(crate) const NFS3MODE_DIR: i32 = 0040000;
pub(crate) const NFS3MODE_CHR: i32 = 0020000;
pub(crate) const NFS3MODE_BLK: i32 = 0060000;
pub(crate) const NFS3MODE_REG: i32 = 0100000;
pub(crate) const NFS3MODE_LNK: i32 = 0120000;
pub(crate) const NFS3MODE_SOCK: i32 = 0140000;
pub(crate) const NFS3MODE_FIFO: i32 = 0010000;

/* Flags for access() call */
pub(crate) const ACCESS_READ: i32 = 0x0001;
pub(crate) const ACCESS_LOOKUP: i32 = 0x0002;
pub(crate) const ACCESS_MODIFY: i32 = 0x0004;
pub(crate) const ACCESS_EXTEND: i32 = 0x0008;
pub(crate) const ACCESS_DELETE: i32 = 0x0010;
pub(crate) const ACCESS_EXECUTE: i32 = 0x0020;
pub(crate) const ACCESS_FULL: i32 = 0x003f;

/* Flags for create mode */
pub(crate) enum CreateMode {
    UNCHECKED = 0,
    GUARDED = 1,
    EXCLUSIVE = 2,
}

/* NFSv3 file system properties */
pub(crate) const FSF_LINK: i32 = 0x0001;
pub(crate) const FSF_SYMLINK: i32 = 0x0002;
pub(crate) const FSF_HOMOGENEOUS: i32 = 0x0008;
pub(crate) const FSF_CANSETTIME: i32 = 0x0010;

/* Some shorthands. See fs/nfsd/nfs3proc.c */
pub(crate) const FSF_DEFAULT: i32 = 0x001B;
pub(crate) const FSF_BILLYBOY: i32 = 0x0018;
pub(crate) const FSF_READONLY: i32 = 0x0008;


pub(crate) enum Stat {
    OK = 0,
    PERM = 1,
    NOENT = 2,
    IO = 5,
    NXIO = 6,
    EAGAIN = 11,
    ACCES = 13,
    EXIST = 17,
    XDEV = 18,
    NODEV = 19,
    NOTDIR = 20,
    ISDIR = 21,
    INVAL = 22,
    FBIG = 27,
    NOSPC = 28,
    ROFS = 30,
    MLINK = 31,
    OPNOTSUPP = 45,
    NAMETOOLONG = 63,
    NOTEMPTY = 66,
    DQUOT = 69,
    STALE = 70,
    REMOTE = 71,
    WFLUSH = 99,
    BADHANDLE = 10001,
    NOT_SYNC = 10002,
    BAD_COOKIE = 10003,
    NOTSUPP = 10004,
    TOOSMALL = 10005,
    SERVERFAULT = 10006,
    BADTYPE = 10007,
    JUKEBOX = 10008,
}

pub(crate) enum FType {
    NON = 0,
    REG = 1,
    DIR = 2,
    BLK = 3,
    CHR = 4,
    LNK = 5,
    SOCK = 6,
    FIFO = 7,
    BAD  = 8,
}

pub(crate) enum TimeHow {
    DONT_CHANGE = 0,
    SET_TO_SERVER_TIME = 1,
    SET_TO_CLIENT_TIME = 2,
}

pub(crate) struct FH {
    size: u16,
    data: [u8; FHSIZE as usize],
}

pub(crate) const VERSION: i32 = 3;
pub(crate) const PROC_NULL: i32 = 0;
pub(crate) const PROC_GETATTR: i32 = 1;
pub(crate) const PROC_SETATTR: i32 = 2;
pub(crate) const PROC_LOOKUP: i32 = 3;
pub(crate) const PROC_ACCESS: i32 = 4;
pub(crate) const PROC_READLINK: i32 = 5;
pub(crate) const PROC_READ: i32 = 6;
pub(crate) const PROC_WRITE: i32 = 7;
pub(crate) const PROC_CREATE: i32 = 8;
pub(crate) const PROC_MKDIR: i32 = 9;
pub(crate) const PROC_SYMLINK: i32 = 10;
pub(crate) const PROC_MKNOD: i32 = 11;
pub(crate) const PROC_REMOVE: i32 = 12;
pub(crate) const PROC_RMDIR: i32 = 13;
pub(crate) const PROC_RENAME: i32 = 14;
pub(crate) const PROC_LINK: i32 = 15;
pub(crate) const PROC_READDIR: i32 = 16;
pub(crate) const PROC_READDIRPLUS: i32 = 17;
pub(crate) const PROC_FSSTAT: i32 = 18;
pub(crate) const PROC_FSINFO: i32 = 19;
pub(crate) const PROC_PATHCONF: i32 = 20;
pub(crate) const PROC_COMMIT: i32 = 21;

pub(crate) const MNT_VERSION: i32 = 3;