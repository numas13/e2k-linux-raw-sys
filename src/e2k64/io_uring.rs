/* automatically generated by rust-bindgen 0.66.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type e2k_addr_t = crate::ctypes::c_ulong;
pub type e2k_size_t = crate::ctypes::c_ulong;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = crate::ctypes::c_uint;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_ipc_pid_t = crate::ctypes::c_int;
pub type __kernel_uid_t = crate::ctypes::c_uint;
pub type __kernel_gid_t = crate::ctypes::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = crate::ctypes::c_int;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_old_dev_t = crate::ctypes::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = crate::ctypes::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = crate::ctypes::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = crate::ctypes::c_int;
pub type __kernel_clockid_t = crate::ctypes::c_int;
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = crate::ctypes::c_uint;
pub type __kernel_rwf_t = crate::ctypes::c_int;
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fscrypt_policy_v1 {
pub version: __u8,
pub contents_encryption_mode: __u8,
pub filenames_encryption_mode: __u8,
pub flags: __u8,
pub master_key_descriptor: [__u8; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fscrypt_key {
pub mode: __u32,
pub raw: [__u8; 64usize],
pub size: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fscrypt_policy_v2 {
pub version: __u8,
pub contents_encryption_mode: __u8,
pub filenames_encryption_mode: __u8,
pub flags: __u8,
pub __reserved: [__u8; 4usize],
pub master_key_identifier: [__u8; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_get_policy_ex_arg {
pub policy_size: __u64,
pub policy: fscrypt_get_policy_ex_arg__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_key_specifier {
pub type_: __u32,
pub __reserved: __u32,
pub u: fscrypt_key_specifier__bindgen_ty_1,
}
#[repr(C)]
pub struct fscrypt_add_key_arg {
pub key_spec: fscrypt_key_specifier,
pub raw_size: __u32,
pub __reserved: [__u32; 9usize],
pub raw: __IncompleteArrayField<__u8>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_remove_key_arg {
pub key_spec: fscrypt_key_specifier,
pub removal_status_flags: __u32,
pub __reserved: [__u32; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_get_key_status_arg {
pub key_spec: fscrypt_key_specifier,
pub __reserved: [__u32; 6usize],
pub status: __u32,
pub status_flags: __u32,
pub user_count: __u32,
pub __out_reserved: [__u32; 13usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct file_clone_range {
pub src_fd: __s64,
pub src_offset: __u64,
pub src_length: __u64,
pub dest_offset: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fstrim_range {
pub start: __u64,
pub len: __u64,
pub minlen: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct file_dedupe_range_info {
pub dest_fd: __s64,
pub dest_offset: __u64,
pub bytes_deduped: __u64,
pub status: __s32,
pub reserved: __u32,
}
#[repr(C)]
#[derive(Debug)]
pub struct file_dedupe_range {
pub src_offset: __u64,
pub src_length: __u64,
pub dest_count: __u16,
pub reserved1: __u16,
pub reserved2: __u32,
pub info: __IncompleteArrayField<file_dedupe_range_info>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct files_stat_struct {
pub nr_files: crate::ctypes::c_ulong,
pub nr_free_files: crate::ctypes::c_ulong,
pub max_files: crate::ctypes::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct inodes_stat_t {
pub nr_inodes: crate::ctypes::c_long,
pub nr_unused: crate::ctypes::c_long,
pub dummy: [crate::ctypes::c_long; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fsxattr {
pub fsx_xflags: __u32,
pub fsx_extsize: __u32,
pub fsx_nextents: __u32,
pub fsx_projid: __u32,
pub fsx_cowextsize: __u32,
pub fsx_pad: [crate::ctypes::c_uchar; 8usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe {
pub opcode: __u8,
pub flags: __u8,
pub ioprio: __u16,
pub fd: __s32,
pub off: __u64,
pub addr: __u64,
pub len: __u32,
pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_1,
pub user_data: __u64,
pub __bindgen_anon_2: io_uring_sqe__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_cqe {
pub user_data: __u64,
pub res: __s32,
pub flags: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_sqring_offsets {
pub head: __u32,
pub tail: __u32,
pub ring_mask: __u32,
pub ring_entries: __u32,
pub flags: __u32,
pub dropped: __u32,
pub array: __u32,
pub resv1: __u32,
pub resv2: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_cqring_offsets {
pub head: __u32,
pub tail: __u32,
pub ring_mask: __u32,
pub ring_entries: __u32,
pub overflow: __u32,
pub cqes: __u32,
pub resv: [__u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_params {
pub sq_entries: __u32,
pub cq_entries: __u32,
pub flags: __u32,
pub sq_thread_cpu: __u32,
pub sq_thread_idle: __u32,
pub features: __u32,
pub resv: [__u32; 4usize],
pub sq_off: io_sqring_offsets,
pub cq_off: io_cqring_offsets,
}
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;
pub const PATH_MAX: u32 = 4096;
pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;
pub const _IOC_NRMASK: u32 = 255;
pub const _IOC_TYPEMASK: u32 = 255;
pub const _IOC_SIZEMASK: u32 = 16383;
pub const _IOC_DIRMASK: u32 = 3;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;
pub const IOC_IN: u32 = 1073741824;
pub const IOC_OUT: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOCSIZE_MASK: u32 = 1073676288;
pub const IOCSIZE_SHIFT: u32 = 16;
pub const FSCRYPT_POLICY_FLAGS_PAD_4: u32 = 0;
pub const FSCRYPT_POLICY_FLAGS_PAD_8: u32 = 1;
pub const FSCRYPT_POLICY_FLAGS_PAD_16: u32 = 2;
pub const FSCRYPT_POLICY_FLAGS_PAD_32: u32 = 3;
pub const FSCRYPT_POLICY_FLAGS_PAD_MASK: u32 = 3;
pub const FSCRYPT_POLICY_FLAG_DIRECT_KEY: u32 = 4;
pub const FSCRYPT_MODE_AES_256_XTS: u32 = 1;
pub const FSCRYPT_MODE_AES_256_CTS: u32 = 4;
pub const FSCRYPT_MODE_AES_128_CBC: u32 = 5;
pub const FSCRYPT_MODE_AES_128_CTS: u32 = 6;
pub const FSCRYPT_MODE_ADIANTUM: u32 = 9;
pub const FSCRYPT_POLICY_V1: u32 = 0;
pub const FSCRYPT_KEY_DESCRIPTOR_SIZE: u32 = 8;
pub const FSCRYPT_KEY_DESC_PREFIX: &[u8; 9] = b"fscrypt:\0";
pub const FSCRYPT_KEY_DESC_PREFIX_SIZE: u32 = 8;
pub const FSCRYPT_MAX_KEY_SIZE: u32 = 64;
pub const FSCRYPT_POLICY_V2: u32 = 2;
pub const FSCRYPT_KEY_IDENTIFIER_SIZE: u32 = 16;
pub const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: u32 = 1;
pub const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: u32 = 2;
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: u32 = 1;
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: u32 = 2;
pub const FSCRYPT_KEY_STATUS_ABSENT: u32 = 1;
pub const FSCRYPT_KEY_STATUS_PRESENT: u32 = 2;
pub const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: u32 = 3;
pub const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: u32 = 1;
pub const FS_KEY_DESCRIPTOR_SIZE: u32 = 8;
pub const FS_POLICY_FLAGS_PAD_4: u32 = 0;
pub const FS_POLICY_FLAGS_PAD_8: u32 = 1;
pub const FS_POLICY_FLAGS_PAD_16: u32 = 2;
pub const FS_POLICY_FLAGS_PAD_32: u32 = 3;
pub const FS_POLICY_FLAGS_PAD_MASK: u32 = 3;
pub const FS_POLICY_FLAG_DIRECT_KEY: u32 = 4;
pub const FS_POLICY_FLAGS_VALID: u32 = 7;
pub const FS_ENCRYPTION_MODE_INVALID: u32 = 0;
pub const FS_ENCRYPTION_MODE_AES_256_XTS: u32 = 1;
pub const FS_ENCRYPTION_MODE_AES_256_GCM: u32 = 2;
pub const FS_ENCRYPTION_MODE_AES_256_CBC: u32 = 3;
pub const FS_ENCRYPTION_MODE_AES_256_CTS: u32 = 4;
pub const FS_ENCRYPTION_MODE_AES_128_CBC: u32 = 5;
pub const FS_ENCRYPTION_MODE_AES_128_CTS: u32 = 6;
pub const FS_ENCRYPTION_MODE_SPECK128_256_XTS: u32 = 7;
pub const FS_ENCRYPTION_MODE_SPECK128_256_CTS: u32 = 8;
pub const FS_ENCRYPTION_MODE_ADIANTUM: u32 = 9;
pub const FS_KEY_DESC_PREFIX: &[u8; 9] = b"fscrypt:\0";
pub const FS_KEY_DESC_PREFIX_SIZE: u32 = 8;
pub const FS_MAX_KEY_SIZE: u32 = 64;
pub const MS_RDONLY: u32 = 1;
pub const MS_NOSUID: u32 = 2;
pub const MS_NODEV: u32 = 4;
pub const MS_NOEXEC: u32 = 8;
pub const MS_SYNCHRONOUS: u32 = 16;
pub const MS_REMOUNT: u32 = 32;
pub const MS_MANDLOCK: u32 = 64;
pub const MS_DIRSYNC: u32 = 128;
pub const MS_NOATIME: u32 = 1024;
pub const MS_NODIRATIME: u32 = 2048;
pub const MS_BIND: u32 = 4096;
pub const MS_MOVE: u32 = 8192;
pub const MS_REC: u32 = 16384;
pub const MS_VERBOSE: u32 = 32768;
pub const MS_SILENT: u32 = 32768;
pub const MS_POSIXACL: u32 = 65536;
pub const MS_UNBINDABLE: u32 = 131072;
pub const MS_PRIVATE: u32 = 262144;
pub const MS_SLAVE: u32 = 524288;
pub const MS_SHARED: u32 = 1048576;
pub const MS_RELATIME: u32 = 2097152;
pub const MS_KERNMOUNT: u32 = 4194304;
pub const MS_I_VERSION: u32 = 8388608;
pub const MS_STRICTATIME: u32 = 16777216;
pub const MS_LAZYTIME: u32 = 33554432;
pub const MS_SUBMOUNT: u32 = 67108864;
pub const MS_NOREMOTELOCK: u32 = 134217728;
pub const MS_NOSEC: u32 = 268435456;
pub const MS_BORN: u32 = 536870912;
pub const MS_ACTIVE: u32 = 1073741824;
pub const MS_NOUSER: u32 = 2147483648;
pub const MS_RMT_MASK: u32 = 41943121;
pub const MS_MGC_VAL: u32 = 3236757504;
pub const MS_MGC_MSK: u32 = 4294901760;
pub const OPEN_TREE_CLONE: u32 = 1;
pub const MOVE_MOUNT_F_SYMLINKS: u32 = 1;
pub const MOVE_MOUNT_F_AUTOMOUNTS: u32 = 2;
pub const MOVE_MOUNT_F_EMPTY_PATH: u32 = 4;
pub const MOVE_MOUNT_T_SYMLINKS: u32 = 16;
pub const MOVE_MOUNT_T_AUTOMOUNTS: u32 = 32;
pub const MOVE_MOUNT_T_EMPTY_PATH: u32 = 64;
pub const MOVE_MOUNT__MASK: u32 = 119;
pub const FSOPEN_CLOEXEC: u32 = 1;
pub const FSPICK_CLOEXEC: u32 = 1;
pub const FSPICK_SYMLINK_NOFOLLOW: u32 = 2;
pub const FSPICK_NO_AUTOMOUNT: u32 = 4;
pub const FSPICK_EMPTY_PATH: u32 = 8;
pub const FSMOUNT_CLOEXEC: u32 = 1;
pub const MOUNT_ATTR_RDONLY: u32 = 1;
pub const MOUNT_ATTR_NOSUID: u32 = 2;
pub const MOUNT_ATTR_NODEV: u32 = 4;
pub const MOUNT_ATTR_NOEXEC: u32 = 8;
pub const MOUNT_ATTR__ATIME: u32 = 112;
pub const MOUNT_ATTR_RELATIME: u32 = 0;
pub const MOUNT_ATTR_NOATIME: u32 = 16;
pub const MOUNT_ATTR_STRICTATIME: u32 = 32;
pub const MOUNT_ATTR_NODIRATIME: u32 = 128;
pub const INR_OPEN_CUR: u32 = 1024;
pub const INR_OPEN_MAX: u32 = 4096;
pub const BLOCK_SIZE_BITS: u32 = 10;
pub const BLOCK_SIZE: u32 = 1024;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_DATA: u32 = 3;
pub const SEEK_HOLE: u32 = 4;
pub const SEEK_MAX: u32 = 4;
pub const RENAME_NOREPLACE: u32 = 1;
pub const RENAME_EXCHANGE: u32 = 2;
pub const RENAME_WHITEOUT: u32 = 4;
pub const FILE_DEDUPE_RANGE_SAME: u32 = 0;
pub const FILE_DEDUPE_RANGE_DIFFERS: u32 = 1;
pub const NR_FILE: u32 = 8192;
pub const FS_XFLAG_REALTIME: u32 = 1;
pub const FS_XFLAG_PREALLOC: u32 = 2;
pub const FS_XFLAG_IMMUTABLE: u32 = 8;
pub const FS_XFLAG_APPEND: u32 = 16;
pub const FS_XFLAG_SYNC: u32 = 32;
pub const FS_XFLAG_NOATIME: u32 = 64;
pub const FS_XFLAG_NODUMP: u32 = 128;
pub const FS_XFLAG_RTINHERIT: u32 = 256;
pub const FS_XFLAG_PROJINHERIT: u32 = 512;
pub const FS_XFLAG_NOSYMLINKS: u32 = 1024;
pub const FS_XFLAG_EXTSIZE: u32 = 2048;
pub const FS_XFLAG_EXTSZINHERIT: u32 = 4096;
pub const FS_XFLAG_NODEFRAG: u32 = 8192;
pub const FS_XFLAG_FILESTREAM: u32 = 16384;
pub const FS_XFLAG_DAX: u32 = 32768;
pub const FS_XFLAG_COWEXTSIZE: u32 = 65536;
pub const FS_XFLAG_HASATTR: u32 = 2147483648;
pub const BMAP_IOCTL: u32 = 1;
pub const FSLABEL_MAX: u32 = 256;
pub const FS_SECRM_FL: u32 = 1;
pub const FS_UNRM_FL: u32 = 2;
pub const FS_COMPR_FL: u32 = 4;
pub const FS_SYNC_FL: u32 = 8;
pub const FS_IMMUTABLE_FL: u32 = 16;
pub const FS_APPEND_FL: u32 = 32;
pub const FS_NODUMP_FL: u32 = 64;
pub const FS_NOATIME_FL: u32 = 128;
pub const FS_DIRTY_FL: u32 = 256;
pub const FS_COMPRBLK_FL: u32 = 512;
pub const FS_NOCOMP_FL: u32 = 1024;
pub const FS_ENCRYPT_FL: u32 = 2048;
pub const FS_BTREE_FL: u32 = 4096;
pub const FS_INDEX_FL: u32 = 4096;
pub const FS_IMAGIC_FL: u32 = 8192;
pub const FS_JOURNAL_DATA_FL: u32 = 16384;
pub const FS_NOTAIL_FL: u32 = 32768;
pub const FS_DIRSYNC_FL: u32 = 65536;
pub const FS_TOPDIR_FL: u32 = 131072;
pub const FS_HUGE_FILE_FL: u32 = 262144;
pub const FS_EXTENT_FL: u32 = 524288;
pub const FS_VERITY_FL: u32 = 1048576;
pub const FS_EA_INODE_FL: u32 = 2097152;
pub const FS_EOFBLOCKS_FL: u32 = 4194304;
pub const FS_NOCOW_FL: u32 = 8388608;
pub const FS_INLINE_DATA_FL: u32 = 268435456;
pub const FS_PROJINHERIT_FL: u32 = 536870912;
pub const FS_CASEFOLD_FL: u32 = 1073741824;
pub const FS_RESERVED_FL: u32 = 2147483648;
pub const FS_FL_USER_VISIBLE: u32 = 253951;
pub const FS_FL_USER_MODIFIABLE: u32 = 229631;
pub const SYNC_FILE_RANGE_WAIT_BEFORE: u32 = 1;
pub const SYNC_FILE_RANGE_WRITE: u32 = 2;
pub const SYNC_FILE_RANGE_WAIT_AFTER: u32 = 4;
pub const SYNC_FILE_RANGE_WRITE_AND_WAIT: u32 = 7;
pub const IOSQE_FIXED_FILE: u32 = 1;
pub const IOSQE_IO_DRAIN: u32 = 2;
pub const IOSQE_IO_LINK: u32 = 4;
pub const IORING_SETUP_IOPOLL: u32 = 1;
pub const IORING_SETUP_SQPOLL: u32 = 2;
pub const IORING_SETUP_SQ_AFF: u32 = 4;
pub const IORING_OP_NOP: u32 = 0;
pub const IORING_OP_READV: u32 = 1;
pub const IORING_OP_WRITEV: u32 = 2;
pub const IORING_OP_FSYNC: u32 = 3;
pub const IORING_OP_READ_FIXED: u32 = 4;
pub const IORING_OP_WRITE_FIXED: u32 = 5;
pub const IORING_OP_POLL_ADD: u32 = 6;
pub const IORING_OP_POLL_REMOVE: u32 = 7;
pub const IORING_OP_SYNC_FILE_RANGE: u32 = 8;
pub const IORING_OP_SENDMSG: u32 = 9;
pub const IORING_OP_RECVMSG: u32 = 10;
pub const IORING_OP_TIMEOUT: u32 = 11;
pub const IORING_FSYNC_DATASYNC: u32 = 1;
pub const IORING_OFF_SQ_RING: u32 = 0;
pub const IORING_OFF_CQ_RING: u32 = 134217728;
pub const IORING_OFF_SQES: u32 = 268435456;
pub const IORING_SQ_NEED_WAKEUP: u32 = 1;
pub const IORING_ENTER_GETEVENTS: u32 = 1;
pub const IORING_ENTER_SQ_WAKEUP: u32 = 2;
pub const IORING_FEAT_SINGLE_MMAP: u32 = 1;
pub const IORING_REGISTER_BUFFERS: u32 = 0;
pub const IORING_UNREGISTER_BUFFERS: u32 = 1;
pub const IORING_REGISTER_FILES: u32 = 2;
pub const IORING_UNREGISTER_FILES: u32 = 3;
pub const IORING_REGISTER_EVENTFD: u32 = 4;
pub const IORING_UNREGISTER_EVENTFD: u32 = 5;
pub const IORING_SETUP_NO_MMAP: u32 = 16384;
pub const IORING_SETUP_REGISTERED_FD_ONLY: u32 = 32768;
pub const IORING_SETUP_NO_SQARRAY: u32 = 65536;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum fsconfig_command {
FSCONFIG_SET_FLAG = 0,
FSCONFIG_SET_STRING = 1,
FSCONFIG_SET_BINARY = 2,
FSCONFIG_SET_PATH = 3,
FSCONFIG_SET_PATH_EMPTY = 4,
FSCONFIG_SET_FD = 5,
FSCONFIG_CMD_CREATE = 6,
FSCONFIG_CMD_RECONFIGURE = 7,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union fscrypt_get_policy_ex_arg__bindgen_ty_1 {
pub version: __u8,
pub v1: fscrypt_policy_v1,
pub v2: fscrypt_policy_v2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union fscrypt_key_specifier__bindgen_ty_1 {
pub __reserved: [__u8; 32usize],
pub descriptor: [__u8; 8usize],
pub identifier: [__u8; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_1 {
pub rw_flags: __kernel_rwf_t,
pub fsync_flags: __u32,
pub poll_events: __u16,
pub sync_range_flags: __u32,
pub msg_flags: __u32,
pub timeout_flags: __u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_2 {
pub buf_index: __u16,
pub __pad2: [__u64; 3usize],
}
impl<T> __IncompleteArrayField<T> {
#[inline]
pub const fn new() -> Self {
__IncompleteArrayField(::core::marker::PhantomData, [])
}
#[inline]
pub fn as_ptr(&self) -> *const T {
self as *const _ as *const T
}
#[inline]
pub fn as_mut_ptr(&mut self) -> *mut T {
self as *mut _ as *mut T
}
#[inline]
pub unsafe fn as_slice(&self, len: usize) -> &[T] {
::core::slice::from_raw_parts(self.as_ptr(), len)
}
#[inline]
pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
}
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
fmt.write_str("__IncompleteArrayField")
}
}