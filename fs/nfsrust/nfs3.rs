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

const PORT: i32 = 2049;
const MAXDATA: i32 = 32768;
const MAXPATHLEN: u32 = PATH_MAX; 
const MAXNAMLEN: u32 = NAME_MAX;
const MAXGROUPS: i32 = 16;
const FHSIZE: i32 = 64;
const COOKIESIZE: i32 = 4;
const CREATEVERFSIZE: i32 = 8;
const COOKIEVERFSIZE: i32 = 8;
const WRITEVERFSIZE: i32 = 8;
const FIFO_DEV: i32 = -1;
const NFS3MODE_FMT: i32 = 0170000;
const NFS3MODE_DIR: i32 = 0040000;
const NFS3MODE_CHR: i32 = 0020000;
const NFS3MODE_BLK: i32 = 0060000;
const NFS3MODE_REG: i32 = 0100000;
const NFS3MODE_LNK: i32 = 0120000;
const NFS3MODE_SOCK: i32 = 0140000;
const NFS3MODE_FIFO: i32 = 0010000;

/* Flags for access() call */
const ACCESS_READ: i32 = 0x0001;
const ACCESS_LOOKUP: i32 = 0x0002;
const ACCESS_MODIFY: i32 = 0x0004;
const ACCESS_EXTEND: i32 = 0x0008;
const ACCESS_DELETE: i32 = 0x0010;
const ACCESS_EXECUTE: i32 = 0x0020;
const ACCESS_FULL: i32 = 0x003f;

/* Flags for create mode */
enum CreateMode {
    UNCHECKED = 0,
    GUARDED = 1,
    EXCLUSIVE = 2,
}

/* NFSv3 file system properties */
const FSF_LINK: i32 = 0x0001;
const FSF_SYMLINK: i32 = 0x0002;
const FSF_HOMOGENEOUS: i32 = 0x0008;
const FSF_CANSETTIME: i32 = 0x0010;

/* Some shorthands. See fs/nfsd/nfs3proc.c */
const FSF_DEFAULT: i32 = 0x001B;
const FSF_BILLYBOY: i32 = 0x0018;
const FSF_READONLY: i32 = 0x0008;

enum FType {
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

enum TimeHow {
    DONT_CHANGE = 0,
    SET_TO_SERVER_TIME = 1,
    SET_TO_CLIENT_TIME = 2,
}

struct FH {
    size: u16,
    data: [u8; FHSIZE as usize],
}

const VERSION: i32 = 3;
const PROC_NULL: i32 = 0;
const PROC_GETATTR: i32 = 1;
const PROC_SETATTR: i32 = 2;
const PROC_LOOKUP: i32 = 3;
const PROC_ACCESS: i32 = 4;
const PROC_READLINK: i32 = 5;
const PROC_READ: i32 = 6;
const PROC_WRITE: i32 = 7;
const PROC_CREATE: i32 = 8;
const PROC_MKDIR: i32 = 9;
const PROC_SYMLINK: i32 = 10;
const PROC_MKNOD: i32 = 11;
const PROC_REMOVE: i32 = 12;
const PROC_RMDIR: i32 = 13;
const PROC_RENAME: i32 = 14;
const PROC_LINK: i32 = 15;
const PROC_READDIR: i32 = 16;
const PROC_READDIRPLUS: i32 = 17;
const PROC_FSSTAT: i32 = 18;
const PROC_FSINFO: i32 = 19;
const PROC_PATHCONF: i32 = 20;
const PROC_COMMIT: i32 = 21;

const MNT_VERSION: i32 = 3;