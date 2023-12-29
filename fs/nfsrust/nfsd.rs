// SPDX-License-Identifier: GPL-2.0

//! Run tests with this:
//! ./tools/testing/kunit/kunit.py run --make_options LLVM=1 --arch x86_64 --kconfig_add CONFIG_RUST=y --kconfig_add CONFIG_NFS_RUST=y --kconfig_add CONFIG_NFS_SAMPLE_RUST_MINIMAL=y

use kernel::prelude::*;

// use super::nfs3;

mod nfs3;

const a: i32 = nfs3::PORT;