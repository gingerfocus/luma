pub const __builtin_bswap16 = @import("std").zig.c_builtins.__builtin_bswap16;
pub const __builtin_bswap32 = @import("std").zig.c_builtins.__builtin_bswap32;
pub const __builtin_bswap64 = @import("std").zig.c_builtins.__builtin_bswap64;
pub const __builtin_signbit = @import("std").zig.c_builtins.__builtin_signbit;
pub const __builtin_signbitf = @import("std").zig.c_builtins.__builtin_signbitf;
pub const __builtin_popcount = @import("std").zig.c_builtins.__builtin_popcount;
pub const __builtin_ctz = @import("std").zig.c_builtins.__builtin_ctz;
pub const __builtin_clz = @import("std").zig.c_builtins.__builtin_clz;
pub const __builtin_sqrt = @import("std").zig.c_builtins.__builtin_sqrt;
pub const __builtin_sqrtf = @import("std").zig.c_builtins.__builtin_sqrtf;
pub const __builtin_sin = @import("std").zig.c_builtins.__builtin_sin;
pub const __builtin_sinf = @import("std").zig.c_builtins.__builtin_sinf;
pub const __builtin_cos = @import("std").zig.c_builtins.__builtin_cos;
pub const __builtin_cosf = @import("std").zig.c_builtins.__builtin_cosf;
pub const __builtin_exp = @import("std").zig.c_builtins.__builtin_exp;
pub const __builtin_expf = @import("std").zig.c_builtins.__builtin_expf;
pub const __builtin_exp2 = @import("std").zig.c_builtins.__builtin_exp2;
pub const __builtin_exp2f = @import("std").zig.c_builtins.__builtin_exp2f;
pub const __builtin_log = @import("std").zig.c_builtins.__builtin_log;
pub const __builtin_logf = @import("std").zig.c_builtins.__builtin_logf;
pub const __builtin_log2 = @import("std").zig.c_builtins.__builtin_log2;
pub const __builtin_log2f = @import("std").zig.c_builtins.__builtin_log2f;
pub const __builtin_log10 = @import("std").zig.c_builtins.__builtin_log10;
pub const __builtin_log10f = @import("std").zig.c_builtins.__builtin_log10f;
pub const __builtin_abs = @import("std").zig.c_builtins.__builtin_abs;
pub const __builtin_fabs = @import("std").zig.c_builtins.__builtin_fabs;
pub const __builtin_fabsf = @import("std").zig.c_builtins.__builtin_fabsf;
pub const __builtin_floor = @import("std").zig.c_builtins.__builtin_floor;
pub const __builtin_floorf = @import("std").zig.c_builtins.__builtin_floorf;
pub const __builtin_ceil = @import("std").zig.c_builtins.__builtin_ceil;
pub const __builtin_ceilf = @import("std").zig.c_builtins.__builtin_ceilf;
pub const __builtin_trunc = @import("std").zig.c_builtins.__builtin_trunc;
pub const __builtin_truncf = @import("std").zig.c_builtins.__builtin_truncf;
pub const __builtin_round = @import("std").zig.c_builtins.__builtin_round;
pub const __builtin_roundf = @import("std").zig.c_builtins.__builtin_roundf;
pub const __builtin_strlen = @import("std").zig.c_builtins.__builtin_strlen;
pub const __builtin_strcmp = @import("std").zig.c_builtins.__builtin_strcmp;
pub const __builtin_object_size = @import("std").zig.c_builtins.__builtin_object_size;
pub const __builtin___memset_chk = @import("std").zig.c_builtins.__builtin___memset_chk;
pub const __builtin_memset = @import("std").zig.c_builtins.__builtin_memset;
pub const __builtin___memcpy_chk = @import("std").zig.c_builtins.__builtin___memcpy_chk;
pub const __builtin_memcpy = @import("std").zig.c_builtins.__builtin_memcpy;
pub const __builtin_expect = @import("std").zig.c_builtins.__builtin_expect;
pub const __builtin_nanf = @import("std").zig.c_builtins.__builtin_nanf;
pub const __builtin_huge_valf = @import("std").zig.c_builtins.__builtin_huge_valf;
pub const __builtin_inff = @import("std").zig.c_builtins.__builtin_inff;
pub const __builtin_isnan = @import("std").zig.c_builtins.__builtin_isnan;
pub const __builtin_isinf = @import("std").zig.c_builtins.__builtin_isinf;
pub const __builtin_isinf_sign = @import("std").zig.c_builtins.__builtin_isinf_sign;
pub const __has_builtin = @import("std").zig.c_builtins.__has_builtin;
pub const __builtin_assume = @import("std").zig.c_builtins.__builtin_assume;
pub const __builtin_unreachable = @import("std").zig.c_builtins.__builtin_unreachable;
pub const __builtin_constant_p = @import("std").zig.c_builtins.__builtin_constant_p;
pub const __builtin_mul_overflow = @import("std").zig.c_builtins.__builtin_mul_overflow;
pub extern fn __assert_fail(__assertion: [*c]const u8, __file: [*c]const u8, __line: c_uint, __function: [*c]const u8) noreturn;
pub extern fn __assert_perror_fail(__errnum: c_int, __file: [*c]const u8, __line: c_uint, __function: [*c]const u8) noreturn;
pub extern fn __assert(__assertion: [*c]const u8, __file: [*c]const u8, __line: c_int) noreturn;
pub const __u_char = u8;
pub const __u_short = c_ushort;
pub const __u_int = c_uint;
pub const __u_long = c_ulong;
pub const __int8_t = i8;
pub const __uint8_t = u8;
pub const __int16_t = c_short;
pub const __uint16_t = c_ushort;
pub const __int32_t = c_int;
pub const __uint32_t = c_uint;
pub const __int64_t = c_long;
pub const __uint64_t = c_ulong;
pub const __int_least8_t = __int8_t;
pub const __uint_least8_t = __uint8_t;
pub const __int_least16_t = __int16_t;
pub const __uint_least16_t = __uint16_t;
pub const __int_least32_t = __int32_t;
pub const __uint_least32_t = __uint32_t;
pub const __int_least64_t = __int64_t;
pub const __uint_least64_t = __uint64_t;
pub const __quad_t = c_long;
pub const __u_quad_t = c_ulong;
pub const __intmax_t = c_long;
pub const __uintmax_t = c_ulong;
pub const __dev_t = c_ulong;
pub const __uid_t = c_uint;
pub const __gid_t = c_uint;
pub const __ino_t = c_ulong;
pub const __ino64_t = c_ulong;
pub const __mode_t = c_uint;
pub const __nlink_t = c_ulong;
pub const __off_t = c_long;
pub const __off64_t = c_long;
pub const __pid_t = c_int;
pub const __fsid_t = extern struct {
    __val: [2]c_int,
};
pub const __clock_t = c_long;
pub const __rlim_t = c_ulong;
pub const __rlim64_t = c_ulong;
pub const __id_t = c_uint;
pub const __time_t = c_long;
pub const __useconds_t = c_uint;
pub const __suseconds_t = c_long;
pub const __suseconds64_t = c_long;
pub const __daddr_t = c_int;
pub const __key_t = c_int;
pub const __clockid_t = c_int;
pub const __timer_t = ?*anyopaque;
pub const __blksize_t = c_long;
pub const __blkcnt_t = c_long;
pub const __blkcnt64_t = c_long;
pub const __fsblkcnt_t = c_ulong;
pub const __fsblkcnt64_t = c_ulong;
pub const __fsfilcnt_t = c_ulong;
pub const __fsfilcnt64_t = c_ulong;
pub const __fsword_t = c_long;
pub const __ssize_t = c_long;
pub const __syscall_slong_t = c_long;
pub const __syscall_ulong_t = c_ulong;
pub const __loff_t = __off64_t;
pub const __caddr_t = [*c]u8;
pub const __intptr_t = c_long;
pub const __socklen_t = c_uint;
pub const __sig_atomic_t = c_int;
pub const struct_flock = extern struct {
    l_type: c_short,
    l_whence: c_short,
    l_start: __off_t,
    l_len: __off_t,
    l_pid: __pid_t,
};
pub const mode_t = __mode_t;
pub const off_t = __off_t;
pub const pid_t = __pid_t;
pub const time_t = __time_t;
pub const struct_timespec = extern struct {
    tv_sec: __time_t,
    tv_nsec: __syscall_slong_t,
};
pub const struct_stat = extern struct {
    st_dev: __dev_t,
    st_ino: __ino_t,
    st_nlink: __nlink_t,
    st_mode: __mode_t,
    st_uid: __uid_t,
    st_gid: __gid_t,
    __pad0: c_int,
    st_rdev: __dev_t,
    st_size: __off_t,
    st_blksize: __blksize_t,
    st_blocks: __blkcnt_t,
    st_atim: struct_timespec,
    st_mtim: struct_timespec,
    st_ctim: struct_timespec,
    __glibc_reserved: [3]__syscall_slong_t,
};
pub extern fn fcntl(__fd: c_int, __cmd: c_int, ...) c_int;
pub extern fn open(__file: [*c]const u8, __oflag: c_int, ...) c_int;
pub extern fn openat(__fd: c_int, __file: [*c]const u8, __oflag: c_int, ...) c_int;
pub extern fn creat(__file: [*c]const u8, __mode: mode_t) c_int;
pub extern fn lockf(__fd: c_int, __cmd: c_int, __len: off_t) c_int;
pub extern fn posix_fadvise(__fd: c_int, __offset: off_t, __len: off_t, __advise: c_int) c_int;
pub extern fn posix_fallocate(__fd: c_int, __offset: off_t, __len: off_t) c_int;
pub const struct___va_list_tag = extern struct {
    gp_offset: c_uint,
    fp_offset: c_uint,
    overflow_arg_area: ?*anyopaque,
    reg_save_area: ?*anyopaque,
};
pub const __builtin_va_list = [1]struct___va_list_tag;
pub const __gnuc_va_list = __builtin_va_list;
const union_unnamed_1 = extern union {
    __wch: c_uint,
    __wchb: [4]u8,
};
pub const __mbstate_t = extern struct {
    __count: c_int,
    __value: union_unnamed_1,
};
pub const struct__G_fpos_t = extern struct {
    __pos: __off_t,
    __state: __mbstate_t,
};
pub const __fpos_t = struct__G_fpos_t;
pub const struct__G_fpos64_t = extern struct {
    __pos: __off64_t,
    __state: __mbstate_t,
};
pub const __fpos64_t = struct__G_fpos64_t;
pub const struct__IO_marker = opaque {};
pub const _IO_lock_t = anyopaque;
pub const struct__IO_codecvt = opaque {};
pub const struct__IO_wide_data = opaque {};
pub const struct__IO_FILE = extern struct {
    _flags: c_int,
    _IO_read_ptr: [*c]u8,
    _IO_read_end: [*c]u8,
    _IO_read_base: [*c]u8,
    _IO_write_base: [*c]u8,
    _IO_write_ptr: [*c]u8,
    _IO_write_end: [*c]u8,
    _IO_buf_base: [*c]u8,
    _IO_buf_end: [*c]u8,
    _IO_save_base: [*c]u8,
    _IO_backup_base: [*c]u8,
    _IO_save_end: [*c]u8,
    _markers: ?*struct__IO_marker,
    _chain: [*c]struct__IO_FILE,
    _fileno: c_int,
    _flags2: c_int,
    _old_offset: __off_t,
    _cur_column: c_ushort,
    _vtable_offset: i8,
    _shortbuf: [1]u8,
    _lock: ?*_IO_lock_t,
    _offset: __off64_t,
    _codecvt: ?*struct__IO_codecvt,
    _wide_data: ?*struct__IO_wide_data,
    _freeres_list: [*c]struct__IO_FILE,
    _freeres_buf: ?*anyopaque,
    __pad5: usize,
    _mode: c_int,
    _unused2: [20]u8,
};
pub const __FILE = struct__IO_FILE;
pub const FILE = struct__IO_FILE;
pub const cookie_read_function_t = fn (?*anyopaque, [*c]u8, usize) callconv(.C) __ssize_t;
pub const cookie_write_function_t = fn (?*anyopaque, [*c]const u8, usize) callconv(.C) __ssize_t;
pub const cookie_seek_function_t = fn (?*anyopaque, [*c]__off64_t, c_int) callconv(.C) c_int;
pub const cookie_close_function_t = fn (?*anyopaque) callconv(.C) c_int;
pub const struct__IO_cookie_io_functions_t = extern struct {
    read: ?*const cookie_read_function_t,
    write: ?*const cookie_write_function_t,
    seek: ?*const cookie_seek_function_t,
    close: ?*const cookie_close_function_t,
};
pub const cookie_io_functions_t = struct__IO_cookie_io_functions_t;
pub const va_list = __gnuc_va_list;
pub const fpos_t = __fpos_t;
pub extern var stdin: [*c]FILE;
pub extern var stdout: [*c]FILE;
pub extern var stderr: [*c]FILE;
pub extern fn remove(__filename: [*c]const u8) c_int;
pub extern fn rename(__old: [*c]const u8, __new: [*c]const u8) c_int;
pub extern fn renameat(__oldfd: c_int, __old: [*c]const u8, __newfd: c_int, __new: [*c]const u8) c_int;
pub extern fn fclose(__stream: [*c]FILE) c_int;
pub extern fn tmpfile() [*c]FILE;
pub extern fn tmpnam([*c]u8) [*c]u8;
pub extern fn tmpnam_r(__s: [*c]u8) [*c]u8;
pub extern fn tempnam(__dir: [*c]const u8, __pfx: [*c]const u8) [*c]u8;
pub extern fn fflush(__stream: [*c]FILE) c_int;
pub extern fn fflush_unlocked(__stream: [*c]FILE) c_int;
pub extern fn fopen(__filename: [*c]const u8, __modes: [*c]const u8) [*c]FILE;
pub extern fn freopen(noalias __filename: [*c]const u8, noalias __modes: [*c]const u8, noalias __stream: [*c]FILE) [*c]FILE;
pub extern fn fdopen(__fd: c_int, __modes: [*c]const u8) [*c]FILE;
pub extern fn fopencookie(noalias __magic_cookie: ?*anyopaque, noalias __modes: [*c]const u8, __io_funcs: cookie_io_functions_t) [*c]FILE;
pub extern fn fmemopen(__s: ?*anyopaque, __len: usize, __modes: [*c]const u8) [*c]FILE;
pub extern fn open_memstream(__bufloc: [*c][*c]u8, __sizeloc: [*c]usize) [*c]FILE;
pub extern fn setbuf(noalias __stream: [*c]FILE, noalias __buf: [*c]u8) void;
pub extern fn setvbuf(noalias __stream: [*c]FILE, noalias __buf: [*c]u8, __modes: c_int, __n: usize) c_int;
pub extern fn setbuffer(noalias __stream: [*c]FILE, noalias __buf: [*c]u8, __size: usize) void;
pub extern fn setlinebuf(__stream: [*c]FILE) void;
pub extern fn fprintf(__stream: [*c]FILE, __format: [*c]const u8, ...) c_int;
pub extern fn printf(__format: [*c]const u8, ...) c_int;
pub extern fn sprintf(__s: [*c]u8, __format: [*c]const u8, ...) c_int;
pub extern fn vfprintf(__s: [*c]FILE, __format: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn vprintf(__format: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn vsprintf(__s: [*c]u8, __format: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn snprintf(__s: [*c]u8, __maxlen: c_ulong, __format: [*c]const u8, ...) c_int;
pub extern fn vsnprintf(__s: [*c]u8, __maxlen: c_ulong, __format: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn vasprintf(noalias __ptr: [*c][*c]u8, noalias __f: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn __asprintf(noalias __ptr: [*c][*c]u8, noalias __fmt: [*c]const u8, ...) c_int;
pub extern fn asprintf(noalias __ptr: [*c][*c]u8, noalias __fmt: [*c]const u8, ...) c_int;
pub extern fn vdprintf(__fd: c_int, noalias __fmt: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn dprintf(__fd: c_int, noalias __fmt: [*c]const u8, ...) c_int;
pub extern fn fscanf(noalias __stream: [*c]FILE, noalias __format: [*c]const u8, ...) c_int;
pub extern fn scanf(noalias __format: [*c]const u8, ...) c_int;
pub extern fn sscanf(noalias __s: [*c]const u8, noalias __format: [*c]const u8, ...) c_int;
pub const _Float32 = f32;
pub const _Float64 = f64;
pub const _Float32x = f64;
pub const _Float64x = c_longdouble;
pub extern fn vfscanf(noalias __s: [*c]FILE, noalias __format: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn vscanf(noalias __format: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn vsscanf(noalias __s: [*c]const u8, noalias __format: [*c]const u8, __arg: [*c]struct___va_list_tag) c_int;
pub extern fn fgetc(__stream: [*c]FILE) c_int;
pub extern fn getc(__stream: [*c]FILE) c_int;
pub extern fn getchar() c_int;
pub extern fn getc_unlocked(__stream: [*c]FILE) c_int;
pub extern fn getchar_unlocked() c_int;
pub extern fn fgetc_unlocked(__stream: [*c]FILE) c_int;
pub extern fn fputc(__c: c_int, __stream: [*c]FILE) c_int;
pub extern fn putc(__c: c_int, __stream: [*c]FILE) c_int;
pub extern fn putchar(__c: c_int) c_int;
pub extern fn fputc_unlocked(__c: c_int, __stream: [*c]FILE) c_int;
pub extern fn putc_unlocked(__c: c_int, __stream: [*c]FILE) c_int;
pub extern fn putchar_unlocked(__c: c_int) c_int;
pub extern fn getw(__stream: [*c]FILE) c_int;
pub extern fn putw(__w: c_int, __stream: [*c]FILE) c_int;
pub extern fn fgets(noalias __s: [*c]u8, __n: c_int, noalias __stream: [*c]FILE) [*c]u8;
pub extern fn __getdelim(noalias __lineptr: [*c][*c]u8, noalias __n: [*c]usize, __delimiter: c_int, noalias __stream: [*c]FILE) __ssize_t;
pub extern fn getdelim(noalias __lineptr: [*c][*c]u8, noalias __n: [*c]usize, __delimiter: c_int, noalias __stream: [*c]FILE) __ssize_t;
pub extern fn getline(noalias __lineptr: [*c][*c]u8, noalias __n: [*c]usize, noalias __stream: [*c]FILE) __ssize_t;
pub extern fn fputs(noalias __s: [*c]const u8, noalias __stream: [*c]FILE) c_int;
pub extern fn puts(__s: [*c]const u8) c_int;
pub extern fn ungetc(__c: c_int, __stream: [*c]FILE) c_int;
pub extern fn fread(__ptr: ?*anyopaque, __size: c_ulong, __n: c_ulong, __stream: [*c]FILE) c_ulong;
pub extern fn fwrite(__ptr: ?*const anyopaque, __size: c_ulong, __n: c_ulong, __s: [*c]FILE) c_ulong;
pub extern fn fread_unlocked(noalias __ptr: ?*anyopaque, __size: usize, __n: usize, noalias __stream: [*c]FILE) usize;
pub extern fn fwrite_unlocked(noalias __ptr: ?*const anyopaque, __size: usize, __n: usize, noalias __stream: [*c]FILE) usize;
pub extern fn fseek(__stream: [*c]FILE, __off: c_long, __whence: c_int) c_int;
pub extern fn ftell(__stream: [*c]FILE) c_long;
pub extern fn rewind(__stream: [*c]FILE) void;
pub extern fn fseeko(__stream: [*c]FILE, __off: __off_t, __whence: c_int) c_int;
pub extern fn ftello(__stream: [*c]FILE) __off_t;
pub extern fn fgetpos(noalias __stream: [*c]FILE, noalias __pos: [*c]fpos_t) c_int;
pub extern fn fsetpos(__stream: [*c]FILE, __pos: [*c]const fpos_t) c_int;
pub extern fn clearerr(__stream: [*c]FILE) void;
pub extern fn feof(__stream: [*c]FILE) c_int;
pub extern fn ferror(__stream: [*c]FILE) c_int;
pub extern fn clearerr_unlocked(__stream: [*c]FILE) void;
pub extern fn feof_unlocked(__stream: [*c]FILE) c_int;
pub extern fn ferror_unlocked(__stream: [*c]FILE) c_int;
pub extern fn perror(__s: [*c]const u8) void;
pub extern fn fileno(__stream: [*c]FILE) c_int;
pub extern fn fileno_unlocked(__stream: [*c]FILE) c_int;
pub extern fn pclose(__stream: [*c]FILE) c_int;
pub extern fn popen(__command: [*c]const u8, __modes: [*c]const u8) [*c]FILE;
pub extern fn ctermid(__s: [*c]u8) [*c]u8;
pub extern fn flockfile(__stream: [*c]FILE) void;
pub extern fn ftrylockfile(__stream: [*c]FILE) c_int;
pub extern fn funlockfile(__stream: [*c]FILE) void;
pub extern fn __uflow([*c]FILE) c_int;
pub extern fn __overflow([*c]FILE, c_int) c_int;
pub const wchar_t = c_int;
pub const div_t = extern struct {
    quot: c_int,
    rem: c_int,
};
pub const ldiv_t = extern struct {
    quot: c_long,
    rem: c_long,
};
pub const lldiv_t = extern struct {
    quot: c_longlong,
    rem: c_longlong,
};
pub extern fn __ctype_get_mb_cur_max() usize;
pub extern fn atof(__nptr: [*c]const u8) f64;
pub extern fn atoi(__nptr: [*c]const u8) c_int;
pub extern fn atol(__nptr: [*c]const u8) c_long;
pub extern fn atoll(__nptr: [*c]const u8) c_longlong;
pub extern fn strtod(__nptr: [*c]const u8, __endptr: [*c][*c]u8) f64;
pub extern fn strtof(__nptr: [*c]const u8, __endptr: [*c][*c]u8) f32;
pub extern fn strtold(__nptr: [*c]const u8, __endptr: [*c][*c]u8) c_longdouble;
pub extern fn strtol(__nptr: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_long;
pub extern fn strtoul(__nptr: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_ulong;
pub extern fn strtoq(noalias __nptr: [*c]const u8, noalias __endptr: [*c][*c]u8, __base: c_int) c_longlong;
pub extern fn strtouq(noalias __nptr: [*c]const u8, noalias __endptr: [*c][*c]u8, __base: c_int) c_ulonglong;
pub extern fn strtoll(__nptr: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_longlong;
pub extern fn strtoull(__nptr: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_ulonglong;
pub extern fn l64a(__n: c_long) [*c]u8;
pub extern fn a64l(__s: [*c]const u8) c_long;
pub const u_char = __u_char;
pub const u_short = __u_short;
pub const u_int = __u_int;
pub const u_long = __u_long;
pub const quad_t = __quad_t;
pub const u_quad_t = __u_quad_t;
pub const fsid_t = __fsid_t;
pub const loff_t = __loff_t;
pub const ino_t = __ino_t;
pub const dev_t = __dev_t;
pub const gid_t = __gid_t;
pub const nlink_t = __nlink_t;
pub const uid_t = __uid_t;
pub const id_t = __id_t;
pub const daddr_t = __daddr_t;
pub const caddr_t = __caddr_t;
pub const key_t = __key_t;
pub const clock_t = __clock_t;
pub const clockid_t = __clockid_t;
pub const timer_t = __timer_t;
pub const ulong = c_ulong;
pub const ushort = c_ushort;
pub const uint = c_uint;
pub const u_int8_t = __uint8_t;
pub const u_int16_t = __uint16_t;
pub const u_int32_t = __uint32_t;
pub const u_int64_t = __uint64_t;
pub const register_t = c_long;
pub fn __bswap_16(arg___bsx: __uint16_t) callconv(.C) __uint16_t {
    var __bsx = arg___bsx;
    return @as(__uint16_t, @bitCast(@as(c_short, @truncate(((@as(c_int, @bitCast(@as(c_uint, __bsx))) >> @intCast(8)) & @as(c_int, 255)) | ((@as(c_int, @bitCast(@as(c_uint, __bsx))) & @as(c_int, 255)) << @intCast(8))))));
}
pub fn __bswap_32(arg___bsx: __uint32_t) callconv(.C) __uint32_t {
    var __bsx = arg___bsx;
    return ((((__bsx & @as(c_uint, 4278190080)) >> @intCast(24)) | ((__bsx & @as(c_uint, 16711680)) >> @intCast(8))) | ((__bsx & @as(c_uint, 65280)) << @intCast(8))) | ((__bsx & @as(c_uint, 255)) << @intCast(24));
}
pub fn __bswap_64(arg___bsx: __uint64_t) callconv(.C) __uint64_t {
    var __bsx = arg___bsx;
    return @as(__uint64_t, @bitCast(@as(c_ulong, @truncate(((((((((@as(c_ulonglong, @bitCast(@as(c_ulonglong, __bsx))) & @as(c_ulonglong, 18374686479671623680)) >> @intCast(56)) | ((@as(c_ulonglong, @bitCast(@as(c_ulonglong, __bsx))) & @as(c_ulonglong, 71776119061217280)) >> @intCast(40))) | ((@as(c_ulonglong, @bitCast(@as(c_ulonglong, __bsx))) & @as(c_ulonglong, 280375465082880)) >> @intCast(24))) | ((@as(c_ulonglong, @bitCast(@as(c_ulonglong, __bsx))) & @as(c_ulonglong, 1095216660480)) >> @intCast(8))) | ((@as(c_ulonglong, @bitCast(@as(c_ulonglong, __bsx))) & @as(c_ulonglong, 4278190080)) << @intCast(8))) | ((@as(c_ulonglong, @bitCast(@as(c_ulonglong, __bsx))) & @as(c_ulonglong, 16711680)) << @intCast(24))) | ((@as(c_ulonglong, @bitCast(@as(c_ulonglong, __bsx))) & @as(c_ulonglong, 65280)) << @intCast(40))) | ((@as(c_ulonglong, @bitCast(@as(c_ulonglong, __bsx))) & @as(c_ulonglong, 255)) << @intCast(56))))));
}
pub fn __uint16_identity(arg___x: __uint16_t) callconv(.C) __uint16_t {
    var __x = arg___x;
    return __x;
}
pub fn __uint32_identity(arg___x: __uint32_t) callconv(.C) __uint32_t {
    var __x = arg___x;
    return __x;
}
pub fn __uint64_identity(arg___x: __uint64_t) callconv(.C) __uint64_t {
    var __x = arg___x;
    return __x;
}
pub const __sigset_t = extern struct {
    __val: [16]c_ulong,
};
pub const sigset_t = __sigset_t;
pub const struct_timeval = extern struct {
    tv_sec: __time_t,
    tv_usec: __suseconds_t,
};
pub const suseconds_t = __suseconds_t;
pub const __fd_mask = c_long;
pub const fd_set = extern struct {
    __fds_bits: [16]__fd_mask,
};
pub const fd_mask = __fd_mask;
pub extern fn select(__nfds: c_int, noalias __readfds: [*c]fd_set, noalias __writefds: [*c]fd_set, noalias __exceptfds: [*c]fd_set, noalias __timeout: [*c]struct_timeval) c_int;
pub extern fn pselect(__nfds: c_int, noalias __readfds: [*c]fd_set, noalias __writefds: [*c]fd_set, noalias __exceptfds: [*c]fd_set, noalias __timeout: [*c]const struct_timespec, noalias __sigmask: [*c]const __sigset_t) c_int;
pub const blksize_t = __blksize_t;
pub const blkcnt_t = __blkcnt_t;
pub const fsblkcnt_t = __fsblkcnt_t;
pub const fsfilcnt_t = __fsfilcnt_t;
const struct_unnamed_2 = extern struct {
    __low: c_uint,
    __high: c_uint,
};
pub const __atomic_wide_counter = extern union {
    __value64: c_ulonglong,
    __value32: struct_unnamed_2,
};
pub const struct___pthread_internal_list = extern struct {
    __prev: [*c]struct___pthread_internal_list,
    __next: [*c]struct___pthread_internal_list,
};
pub const __pthread_list_t = struct___pthread_internal_list;
pub const struct___pthread_internal_slist = extern struct {
    __next: [*c]struct___pthread_internal_slist,
};
pub const __pthread_slist_t = struct___pthread_internal_slist;
pub const struct___pthread_mutex_s = extern struct {
    __lock: c_int,
    __count: c_uint,
    __owner: c_int,
    __nusers: c_uint,
    __kind: c_int,
    __spins: c_short,
    __elision: c_short,
    __list: __pthread_list_t,
};
pub const struct___pthread_rwlock_arch_t = extern struct {
    __readers: c_uint,
    __writers: c_uint,
    __wrphase_futex: c_uint,
    __writers_futex: c_uint,
    __pad3: c_uint,
    __pad4: c_uint,
    __cur_writer: c_int,
    __shared: c_int,
    __rwelision: i8,
    __pad1: [7]u8,
    __pad2: c_ulong,
    __flags: c_uint,
};
pub const struct___pthread_cond_s = extern struct {
    __wseq: __atomic_wide_counter,
    __g1_start: __atomic_wide_counter,
    __g_refs: [2]c_uint,
    __g_size: [2]c_uint,
    __g1_orig_size: c_uint,
    __wrefs: c_uint,
    __g_signals: [2]c_uint,
};
pub const __tss_t = c_uint;
pub const __thrd_t = c_ulong;
pub const __once_flag = extern struct {
    __data: c_int,
};
pub const pthread_t = c_ulong;
pub const pthread_mutexattr_t = extern union {
    __size: [4]u8,
    __align: c_int,
};
pub const pthread_condattr_t = extern union {
    __size: [4]u8,
    __align: c_int,
};
pub const pthread_key_t = c_uint;
pub const pthread_once_t = c_int;
pub const union_pthread_attr_t = extern union {
    __size: [56]u8,
    __align: c_long,
};
pub const pthread_attr_t = union_pthread_attr_t;
pub const pthread_mutex_t = extern union {
    __data: struct___pthread_mutex_s,
    __size: [40]u8,
    __align: c_long,
};
pub const pthread_cond_t = extern union {
    __data: struct___pthread_cond_s,
    __size: [48]u8,
    __align: c_longlong,
};
pub const pthread_rwlock_t = extern union {
    __data: struct___pthread_rwlock_arch_t,
    __size: [56]u8,
    __align: c_long,
};
pub const pthread_rwlockattr_t = extern union {
    __size: [8]u8,
    __align: c_long,
};
pub const pthread_spinlock_t = c_int;
pub const pthread_barrier_t = extern union {
    __size: [32]u8,
    __align: c_long,
};
pub const pthread_barrierattr_t = extern union {
    __size: [4]u8,
    __align: c_int,
};
pub extern fn random() c_long;
pub extern fn srandom(__seed: c_uint) void;
pub extern fn initstate(__seed: c_uint, __statebuf: [*c]u8, __statelen: usize) [*c]u8;
pub extern fn setstate(__statebuf: [*c]u8) [*c]u8;
pub const struct_random_data = extern struct {
    fptr: [*c]i32,
    rptr: [*c]i32,
    state: [*c]i32,
    rand_type: c_int,
    rand_deg: c_int,
    rand_sep: c_int,
    end_ptr: [*c]i32,
};
pub extern fn random_r(noalias __buf: [*c]struct_random_data, noalias __result: [*c]i32) c_int;
pub extern fn srandom_r(__seed: c_uint, __buf: [*c]struct_random_data) c_int;
pub extern fn initstate_r(__seed: c_uint, noalias __statebuf: [*c]u8, __statelen: usize, noalias __buf: [*c]struct_random_data) c_int;
pub extern fn setstate_r(noalias __statebuf: [*c]u8, noalias __buf: [*c]struct_random_data) c_int;
pub extern fn rand() c_int;
pub extern fn srand(__seed: c_uint) void;
pub extern fn rand_r(__seed: [*c]c_uint) c_int;
pub extern fn drand48() f64;
pub extern fn erand48(__xsubi: [*c]c_ushort) f64;
pub extern fn lrand48() c_long;
pub extern fn nrand48(__xsubi: [*c]c_ushort) c_long;
pub extern fn mrand48() c_long;
pub extern fn jrand48(__xsubi: [*c]c_ushort) c_long;
pub extern fn srand48(__seedval: c_long) void;
pub extern fn seed48(__seed16v: [*c]c_ushort) [*c]c_ushort;
pub extern fn lcong48(__param: [*c]c_ushort) void;
pub const struct_drand48_data = extern struct {
    __x: [3]c_ushort,
    __old_x: [3]c_ushort,
    __c: c_ushort,
    __init: c_ushort,
    __a: c_ulonglong,
};
pub extern fn drand48_r(noalias __buffer: [*c]struct_drand48_data, noalias __result: [*c]f64) c_int;
pub extern fn erand48_r(__xsubi: [*c]c_ushort, noalias __buffer: [*c]struct_drand48_data, noalias __result: [*c]f64) c_int;
pub extern fn lrand48_r(noalias __buffer: [*c]struct_drand48_data, noalias __result: [*c]c_long) c_int;
pub extern fn nrand48_r(__xsubi: [*c]c_ushort, noalias __buffer: [*c]struct_drand48_data, noalias __result: [*c]c_long) c_int;
pub extern fn mrand48_r(noalias __buffer: [*c]struct_drand48_data, noalias __result: [*c]c_long) c_int;
pub extern fn jrand48_r(__xsubi: [*c]c_ushort, noalias __buffer: [*c]struct_drand48_data, noalias __result: [*c]c_long) c_int;
pub extern fn srand48_r(__seedval: c_long, __buffer: [*c]struct_drand48_data) c_int;
pub extern fn seed48_r(__seed16v: [*c]c_ushort, __buffer: [*c]struct_drand48_data) c_int;
pub extern fn lcong48_r(__param: [*c]c_ushort, __buffer: [*c]struct_drand48_data) c_int;
pub extern fn arc4random() __uint32_t;
pub extern fn arc4random_buf(__buf: ?*anyopaque, __size: usize) void;
pub extern fn arc4random_uniform(__upper_bound: __uint32_t) __uint32_t;
pub extern fn malloc(__size: c_ulong) ?*anyopaque;
pub extern fn calloc(__nmemb: c_ulong, __size: c_ulong) ?*anyopaque;
pub extern fn realloc(__ptr: ?*anyopaque, __size: c_ulong) ?*anyopaque;
pub extern fn free(__ptr: ?*anyopaque) void;
pub extern fn reallocarray(__ptr: ?*anyopaque, __nmemb: usize, __size: usize) ?*anyopaque;
pub extern fn alloca(__size: c_ulong) ?*anyopaque;
pub extern fn valloc(__size: usize) ?*anyopaque;
pub extern fn posix_memalign(__memptr: [*c]?*anyopaque, __alignment: usize, __size: usize) c_int;
pub extern fn aligned_alloc(__alignment: c_ulong, __size: c_ulong) ?*anyopaque;
pub extern fn abort() noreturn;
pub extern fn atexit(__func: ?*const fn () callconv(.C) void) c_int;
pub extern fn at_quick_exit(__func: ?*const fn () callconv(.C) void) c_int;
pub extern fn on_exit(__func: ?*const fn (c_int, ?*anyopaque) callconv(.C) void, __arg: ?*anyopaque) c_int;
pub extern fn exit(__status: c_int) noreturn;
pub extern fn quick_exit(__status: c_int) noreturn;
pub extern fn _Exit(__status: c_int) noreturn;
pub extern fn getenv(__name: [*c]const u8) [*c]u8;
pub extern fn putenv(__string: [*c]u8) c_int;
pub extern fn setenv(__name: [*c]const u8, __value: [*c]const u8, __replace: c_int) c_int;
pub extern fn unsetenv(__name: [*c]const u8) c_int;
pub extern fn clearenv() c_int;
pub extern fn mktemp(__template: [*c]u8) [*c]u8;
pub extern fn mkstemp(__template: [*c]u8) c_int;
pub extern fn mkstemps(__template: [*c]u8, __suffixlen: c_int) c_int;
pub extern fn mkdtemp(__template: [*c]u8) [*c]u8;
pub extern fn system(__command: [*c]const u8) c_int;
pub extern fn realpath(noalias __name: [*c]const u8, noalias __resolved: [*c]u8) [*c]u8;
pub const __compar_fn_t = ?*const fn (?*const anyopaque, ?*const anyopaque) callconv(.C) c_int;
pub extern fn bsearch(__key: ?*const anyopaque, __base: ?*const anyopaque, __nmemb: usize, __size: usize, __compar: __compar_fn_t) ?*anyopaque;
pub extern fn qsort(__base: ?*anyopaque, __nmemb: usize, __size: usize, __compar: __compar_fn_t) void;
pub extern fn abs(__x: c_int) c_int;
pub extern fn labs(__x: c_long) c_long;
pub extern fn llabs(__x: c_longlong) c_longlong;
pub extern fn div(__numer: c_int, __denom: c_int) div_t;
pub extern fn ldiv(__numer: c_long, __denom: c_long) ldiv_t;
pub extern fn lldiv(__numer: c_longlong, __denom: c_longlong) lldiv_t;
pub extern fn ecvt(__value: f64, __ndigit: c_int, noalias __decpt: [*c]c_int, noalias __sign: [*c]c_int) [*c]u8;
pub extern fn fcvt(__value: f64, __ndigit: c_int, noalias __decpt: [*c]c_int, noalias __sign: [*c]c_int) [*c]u8;
pub extern fn gcvt(__value: f64, __ndigit: c_int, __buf: [*c]u8) [*c]u8;
pub extern fn qecvt(__value: c_longdouble, __ndigit: c_int, noalias __decpt: [*c]c_int, noalias __sign: [*c]c_int) [*c]u8;
pub extern fn qfcvt(__value: c_longdouble, __ndigit: c_int, noalias __decpt: [*c]c_int, noalias __sign: [*c]c_int) [*c]u8;
pub extern fn qgcvt(__value: c_longdouble, __ndigit: c_int, __buf: [*c]u8) [*c]u8;
pub extern fn ecvt_r(__value: f64, __ndigit: c_int, noalias __decpt: [*c]c_int, noalias __sign: [*c]c_int, noalias __buf: [*c]u8, __len: usize) c_int;
pub extern fn fcvt_r(__value: f64, __ndigit: c_int, noalias __decpt: [*c]c_int, noalias __sign: [*c]c_int, noalias __buf: [*c]u8, __len: usize) c_int;
pub extern fn qecvt_r(__value: c_longdouble, __ndigit: c_int, noalias __decpt: [*c]c_int, noalias __sign: [*c]c_int, noalias __buf: [*c]u8, __len: usize) c_int;
pub extern fn qfcvt_r(__value: c_longdouble, __ndigit: c_int, noalias __decpt: [*c]c_int, noalias __sign: [*c]c_int, noalias __buf: [*c]u8, __len: usize) c_int;
pub extern fn mblen(__s: [*c]const u8, __n: usize) c_int;
pub extern fn mbtowc(noalias __pwc: [*c]wchar_t, noalias __s: [*c]const u8, __n: usize) c_int;
pub extern fn wctomb(__s: [*c]u8, __wchar: wchar_t) c_int;
pub extern fn mbstowcs(noalias __pwcs: [*c]wchar_t, noalias __s: [*c]const u8, __n: usize) usize;
pub extern fn wcstombs(noalias __s: [*c]u8, noalias __pwcs: [*c]const wchar_t, __n: usize) usize;
pub extern fn rpmatch(__response: [*c]const u8) c_int;
pub extern fn getsubopt(noalias __optionp: [*c][*c]u8, noalias __tokens: [*c]const [*c]u8, noalias __valuep: [*c][*c]u8) c_int;
pub extern fn getloadavg(__loadavg: [*c]f64, __nelem: c_int) c_int;
pub extern fn memcpy(__dest: ?*anyopaque, __src: ?*const anyopaque, __n: c_ulong) ?*anyopaque;
pub extern fn memmove(__dest: ?*anyopaque, __src: ?*const anyopaque, __n: c_ulong) ?*anyopaque;
pub extern fn memccpy(__dest: ?*anyopaque, __src: ?*const anyopaque, __c: c_int, __n: c_ulong) ?*anyopaque;
pub extern fn memset(__s: ?*anyopaque, __c: c_int, __n: c_ulong) ?*anyopaque;
pub extern fn memcmp(__s1: ?*const anyopaque, __s2: ?*const anyopaque, __n: c_ulong) c_int;
pub extern fn __memcmpeq(__s1: ?*const anyopaque, __s2: ?*const anyopaque, __n: usize) c_int;
pub extern fn memchr(__s: ?*const anyopaque, __c: c_int, __n: c_ulong) ?*anyopaque;
pub extern fn strcpy(__dest: [*c]u8, __src: [*c]const u8) [*c]u8;
pub extern fn strncpy(__dest: [*c]u8, __src: [*c]const u8, __n: c_ulong) [*c]u8;
pub extern fn strcat(__dest: [*c]u8, __src: [*c]const u8) [*c]u8;
pub extern fn strncat(__dest: [*c]u8, __src: [*c]const u8, __n: c_ulong) [*c]u8;
pub extern fn strcmp(__s1: [*c]const u8, __s2: [*c]const u8) c_int;
pub extern fn strncmp(__s1: [*c]const u8, __s2: [*c]const u8, __n: c_ulong) c_int;
pub extern fn strcoll(__s1: [*c]const u8, __s2: [*c]const u8) c_int;
pub extern fn strxfrm(__dest: [*c]u8, __src: [*c]const u8, __n: c_ulong) c_ulong;
pub const struct___locale_data = opaque {};
pub const struct___locale_struct = extern struct {
    __locales: [13]?*struct___locale_data,
    __ctype_b: [*c]const c_ushort,
    __ctype_tolower: [*c]const c_int,
    __ctype_toupper: [*c]const c_int,
    __names: [13][*c]const u8,
};
pub const __locale_t = [*c]struct___locale_struct;
pub const locale_t = __locale_t;
pub extern fn strcoll_l(__s1: [*c]const u8, __s2: [*c]const u8, __l: locale_t) c_int;
pub extern fn strxfrm_l(__dest: [*c]u8, __src: [*c]const u8, __n: usize, __l: locale_t) usize;
pub extern fn strdup(__s: [*c]const u8) [*c]u8;
pub extern fn strndup(__string: [*c]const u8, __n: c_ulong) [*c]u8;
pub extern fn strchr(__s: [*c]const u8, __c: c_int) [*c]u8;
pub extern fn strrchr(__s: [*c]const u8, __c: c_int) [*c]u8;
pub extern fn strchrnul(__s: [*c]const u8, __c: c_int) [*c]u8;
pub extern fn strcspn(__s: [*c]const u8, __reject: [*c]const u8) c_ulong;
pub extern fn strspn(__s: [*c]const u8, __accept: [*c]const u8) c_ulong;
pub extern fn strpbrk(__s: [*c]const u8, __accept: [*c]const u8) [*c]u8;
pub extern fn strstr(__haystack: [*c]const u8, __needle: [*c]const u8) [*c]u8;
pub extern fn strtok(__s: [*c]u8, __delim: [*c]const u8) [*c]u8;
pub extern fn __strtok_r(noalias __s: [*c]u8, noalias __delim: [*c]const u8, noalias __save_ptr: [*c][*c]u8) [*c]u8;
pub extern fn strtok_r(noalias __s: [*c]u8, noalias __delim: [*c]const u8, noalias __save_ptr: [*c][*c]u8) [*c]u8;
pub extern fn strcasestr(__haystack: [*c]const u8, __needle: [*c]const u8) [*c]u8;
pub extern fn memmem(__haystack: ?*const anyopaque, __haystacklen: usize, __needle: ?*const anyopaque, __needlelen: usize) ?*anyopaque;
pub extern fn __mempcpy(noalias __dest: ?*anyopaque, noalias __src: ?*const anyopaque, __n: usize) ?*anyopaque;
pub extern fn mempcpy(__dest: ?*anyopaque, __src: ?*const anyopaque, __n: c_ulong) ?*anyopaque;
pub extern fn strlen(__s: [*c]const u8) c_ulong;
pub extern fn strnlen(__string: [*c]const u8, __maxlen: usize) usize;
pub extern fn strerror(__errnum: c_int) [*c]u8;
pub extern fn strerror_r(__errnum: c_int, __buf: [*c]u8, __buflen: usize) c_int;
pub extern fn strerror_l(__errnum: c_int, __l: locale_t) [*c]u8;
pub extern fn bcmp(__s1: ?*const anyopaque, __s2: ?*const anyopaque, __n: c_ulong) c_int;
pub extern fn bcopy(__src: ?*const anyopaque, __dest: ?*anyopaque, __n: usize) void;
pub extern fn bzero(__s: ?*anyopaque, __n: c_ulong) void;
pub extern fn index(__s: [*c]const u8, __c: c_int) [*c]u8;
pub extern fn rindex(__s: [*c]const u8, __c: c_int) [*c]u8;
pub extern fn ffs(__i: c_int) c_int;
pub extern fn ffsl(__l: c_long) c_int;
pub extern fn ffsll(__ll: c_longlong) c_int;
pub extern fn strcasecmp(__s1: [*c]const u8, __s2: [*c]const u8) c_int;
pub extern fn strncasecmp(__s1: [*c]const u8, __s2: [*c]const u8, __n: c_ulong) c_int;
pub extern fn strcasecmp_l(__s1: [*c]const u8, __s2: [*c]const u8, __loc: locale_t) c_int;
pub extern fn strncasecmp_l(__s1: [*c]const u8, __s2: [*c]const u8, __n: usize, __loc: locale_t) c_int;
pub extern fn explicit_bzero(__s: ?*anyopaque, __n: usize) void;
pub extern fn strsep(noalias __stringp: [*c][*c]u8, noalias __delim: [*c]const u8) [*c]u8;
pub extern fn strsignal(__sig: c_int) [*c]u8;
pub extern fn __stpcpy(noalias __dest: [*c]u8, noalias __src: [*c]const u8) [*c]u8;
pub extern fn stpcpy(__dest: [*c]u8, __src: [*c]const u8) [*c]u8;
pub extern fn __stpncpy(noalias __dest: [*c]u8, noalias __src: [*c]const u8, __n: usize) [*c]u8;
pub extern fn stpncpy(__dest: [*c]u8, __src: [*c]const u8, __n: c_ulong) [*c]u8;
pub extern fn strlcpy(__dest: [*c]u8, __src: [*c]const u8, __n: c_ulong) c_ulong;
pub extern fn strlcat(__dest: [*c]u8, __src: [*c]const u8, __n: c_ulong) c_ulong;
pub const int_least8_t = __int_least8_t;
pub const int_least16_t = __int_least16_t;
pub const int_least32_t = __int_least32_t;
pub const int_least64_t = __int_least64_t;
pub const uint_least8_t = __uint_least8_t;
pub const uint_least16_t = __uint_least16_t;
pub const uint_least32_t = __uint_least32_t;
pub const uint_least64_t = __uint_least64_t;
pub const int_fast8_t = i8;
pub const int_fast16_t = c_long;
pub const int_fast32_t = c_long;
pub const int_fast64_t = c_long;
pub const uint_fast8_t = u8;
pub const uint_fast16_t = c_ulong;
pub const uint_fast32_t = c_ulong;
pub const uint_fast64_t = c_ulong;
pub const intmax_t = __intmax_t;
pub const uintmax_t = __uintmax_t;
pub const __s8 = i8;
pub const __u8 = u8;
pub const __s16 = c_short;
pub const __u16 = c_ushort;
pub const __s32 = c_int;
pub const __u32 = c_uint;
pub const __s64 = c_longlong;
pub const __u64 = c_ulonglong;
pub const __kernel_fd_set = extern struct {
    fds_bits: [16]c_ulong,
};
pub const __kernel_sighandler_t = ?*const fn (c_int) callconv(.C) void;
pub const __kernel_key_t = c_int;
pub const __kernel_mqd_t = c_int;
pub const __kernel_old_uid_t = c_ushort;
pub const __kernel_old_gid_t = c_ushort;
pub const __kernel_old_dev_t = c_ulong;
pub const __kernel_long_t = c_long;
pub const __kernel_ulong_t = c_ulong;
pub const __kernel_ino_t = __kernel_ulong_t;
pub const __kernel_mode_t = c_uint;
pub const __kernel_pid_t = c_int;
pub const __kernel_ipc_pid_t = c_int;
pub const __kernel_uid_t = c_uint;
pub const __kernel_gid_t = c_uint;
pub const __kernel_suseconds_t = __kernel_long_t;
pub const __kernel_daddr_t = c_int;
pub const __kernel_uid32_t = c_uint;
pub const __kernel_gid32_t = c_uint;
pub const __kernel_size_t = __kernel_ulong_t;
pub const __kernel_ssize_t = __kernel_long_t;
pub const __kernel_ptrdiff_t = __kernel_long_t;
pub const __kernel_fsid_t = extern struct {
    val: [2]c_int,
};
pub const __kernel_off_t = __kernel_long_t;
pub const __kernel_loff_t = c_longlong;
pub const __kernel_old_time_t = __kernel_long_t;
pub const __kernel_time_t = __kernel_long_t;
pub const __kernel_time64_t = c_longlong;
pub const __kernel_clock_t = __kernel_long_t;
pub const __kernel_timer_t = c_int;
pub const __kernel_clockid_t = c_int;
pub const __kernel_caddr_t = [*c]u8;
pub const __kernel_uid16_t = c_ushort;
pub const __kernel_gid16_t = c_ushort;
pub const __s128 = i128;
pub const __u128 = u128;
pub const __le16 = __u16;
pub const __be16 = __u16;
pub const __le32 = __u32;
pub const __be32 = __u32;
pub const __le64 = __u64;
pub const __be64 = __u64;
pub const __sum16 = __u16;
pub const __wsum = __u32;
pub const __poll_t = c_uint;
pub const drm_handle_t = c_uint;
pub const drm_context_t = c_uint;
pub const drm_drawable_t = c_uint;
pub const drm_magic_t = c_uint;
pub const struct_drm_clip_rect = extern struct {
    x1: c_ushort,
    y1: c_ushort,
    x2: c_ushort,
    y2: c_ushort,
};
pub const struct_drm_drawable_info = extern struct {
    num_rects: c_uint,
    rects: [*c]struct_drm_clip_rect,
};
pub const struct_drm_tex_region = extern struct {
    next: u8,
    prev: u8,
    in_use: u8,
    padding: u8,
    age: c_uint,
};
pub const struct_drm_hw_lock = extern struct {
    lock: c_uint,
    padding: [60]u8,
};
pub const struct_drm_version = extern struct {
    version_major: c_int,
    version_minor: c_int,
    version_patchlevel: c_int,
    name_len: __kernel_size_t,
    name: [*c]u8,
    date_len: __kernel_size_t,
    date: [*c]u8,
    desc_len: __kernel_size_t,
    desc: [*c]u8,
};
pub const struct_drm_unique = extern struct {
    unique_len: __kernel_size_t,
    unique: [*c]u8,
};
pub const struct_drm_list = extern struct {
    count: c_int,
    version: [*c]struct_drm_version,
};
pub const struct_drm_block = extern struct {
    unused: c_int,
};
pub const DRM_ADD_COMMAND: c_int = 0;
pub const DRM_RM_COMMAND: c_int = 1;
pub const DRM_INST_HANDLER: c_int = 2;
pub const DRM_UNINST_HANDLER: c_int = 3;
const enum_unnamed_3 = c_uint;
pub const struct_drm_control = extern struct {
    func: enum_unnamed_3,
    irq: c_int,
};
pub const _DRM_FRAME_BUFFER: c_int = 0;
pub const _DRM_REGISTERS: c_int = 1;
pub const _DRM_SHM: c_int = 2;
pub const _DRM_AGP: c_int = 3;
pub const _DRM_SCATTER_GATHER: c_int = 4;
pub const _DRM_CONSISTENT: c_int = 5;
pub const enum_drm_map_type = c_uint;
pub const _DRM_RESTRICTED: c_int = 1;
pub const _DRM_READ_ONLY: c_int = 2;
pub const _DRM_LOCKED: c_int = 4;
pub const _DRM_KERNEL: c_int = 8;
pub const _DRM_WRITE_COMBINING: c_int = 16;
pub const _DRM_CONTAINS_LOCK: c_int = 32;
pub const _DRM_REMOVABLE: c_int = 64;
pub const _DRM_DRIVER: c_int = 128;
pub const enum_drm_map_flags = c_uint;
pub const struct_drm_ctx_priv_map = extern struct {
    ctx_id: c_uint,
    handle: ?*anyopaque,
};
pub const struct_drm_map = extern struct {
    offset: c_ulong,
    size: c_ulong,
    type: enum_drm_map_type,
    flags: enum_drm_map_flags,
    handle: ?*anyopaque,
    mtrr: c_int,
};
pub const struct_drm_client = extern struct {
    idx: c_int,
    auth: c_int,
    pid: c_ulong,
    uid: c_ulong,
    magic: c_ulong,
    iocs: c_ulong,
};
pub const _DRM_STAT_LOCK: c_int = 0;
pub const _DRM_STAT_OPENS: c_int = 1;
pub const _DRM_STAT_CLOSES: c_int = 2;
pub const _DRM_STAT_IOCTLS: c_int = 3;
pub const _DRM_STAT_LOCKS: c_int = 4;
pub const _DRM_STAT_UNLOCKS: c_int = 5;
pub const _DRM_STAT_VALUE: c_int = 6;
pub const _DRM_STAT_BYTE: c_int = 7;
pub const _DRM_STAT_COUNT: c_int = 8;
pub const _DRM_STAT_IRQ: c_int = 9;
pub const _DRM_STAT_PRIMARY: c_int = 10;
pub const _DRM_STAT_SECONDARY: c_int = 11;
pub const _DRM_STAT_DMA: c_int = 12;
pub const _DRM_STAT_SPECIAL: c_int = 13;
pub const _DRM_STAT_MISSED: c_int = 14;
pub const enum_drm_stat_type = c_uint;
const struct_unnamed_4 = extern struct {
    value: c_ulong,
    type: enum_drm_stat_type,
};
pub const struct_drm_stats = extern struct {
    count: c_ulong,
    data: [15]struct_unnamed_4,
};
pub const _DRM_LOCK_READY: c_int = 1;
pub const _DRM_LOCK_QUIESCENT: c_int = 2;
pub const _DRM_LOCK_FLUSH: c_int = 4;
pub const _DRM_LOCK_FLUSH_ALL: c_int = 8;
pub const _DRM_HALT_ALL_QUEUES: c_int = 16;
pub const _DRM_HALT_CUR_QUEUES: c_int = 32;
pub const enum_drm_lock_flags = c_uint;
pub const struct_drm_lock = extern struct {
    context: c_int,
    flags: enum_drm_lock_flags,
};
pub const _DRM_DMA_BLOCK: c_int = 1;
pub const _DRM_DMA_WHILE_LOCKED: c_int = 2;
pub const _DRM_DMA_PRIORITY: c_int = 4;
pub const _DRM_DMA_WAIT: c_int = 16;
pub const _DRM_DMA_SMALLER_OK: c_int = 32;
pub const _DRM_DMA_LARGER_OK: c_int = 64;
pub const enum_drm_dma_flags = c_uint;
pub const _DRM_PAGE_ALIGN: c_int = 1;
pub const _DRM_AGP_BUFFER: c_int = 2;
pub const _DRM_SG_BUFFER: c_int = 4;
pub const _DRM_FB_BUFFER: c_int = 8;
pub const _DRM_PCI_BUFFER_RO: c_int = 16;
const enum_unnamed_5 = c_uint;
pub const struct_drm_buf_desc = extern struct {
    count: c_int,
    size: c_int,
    low_mark: c_int,
    high_mark: c_int,
    flags: enum_unnamed_5,
    agp_start: c_ulong,
};
pub const struct_drm_buf_info = extern struct {
    count: c_int,
    list: [*c]struct_drm_buf_desc,
};
pub const struct_drm_buf_free = extern struct {
    count: c_int,
    list: [*c]c_int,
};
pub const struct_drm_buf_pub = extern struct {
    idx: c_int,
    total: c_int,
    used: c_int,
    address: ?*anyopaque,
};
pub const struct_drm_buf_map = extern struct {
    count: c_int,
    virtual: ?*anyopaque,
    list: [*c]struct_drm_buf_pub,
};
pub const struct_drm_dma = extern struct {
    context: c_int,
    send_count: c_int,
    send_indices: [*c]c_int,
    send_sizes: [*c]c_int,
    flags: enum_drm_dma_flags,
    request_count: c_int,
    request_size: c_int,
    request_indices: [*c]c_int,
    request_sizes: [*c]c_int,
    granted_count: c_int,
};
pub const _DRM_CONTEXT_PRESERVED: c_int = 1;
pub const _DRM_CONTEXT_2DONLY: c_int = 2;
pub const enum_drm_ctx_flags = c_uint;
pub const struct_drm_ctx = extern struct {
    handle: drm_context_t,
    flags: enum_drm_ctx_flags,
};
pub const struct_drm_ctx_res = extern struct {
    count: c_int,
    contexts: [*c]struct_drm_ctx,
};
pub const struct_drm_draw = extern struct {
    handle: drm_drawable_t,
};
pub const DRM_DRAWABLE_CLIPRECTS: c_int = 0;
pub const drm_drawable_info_type_t = c_uint;
pub const struct_drm_update_draw = extern struct {
    handle: drm_drawable_t,
    type: c_uint,
    num: c_uint,
    data: c_ulonglong,
};
pub const struct_drm_auth = extern struct {
    magic: drm_magic_t,
};
pub const struct_drm_irq_busid = extern struct {
    irq: c_int,
    busnum: c_int,
    devnum: c_int,
    funcnum: c_int,
};
pub const _DRM_VBLANK_ABSOLUTE: c_int = 0;
pub const _DRM_VBLANK_RELATIVE: c_int = 1;
pub const _DRM_VBLANK_HIGH_CRTC_MASK: c_int = 62;
pub const _DRM_VBLANK_EVENT: c_int = 67108864;
pub const _DRM_VBLANK_FLIP: c_int = 134217728;
pub const _DRM_VBLANK_NEXTONMISS: c_int = 268435456;
pub const _DRM_VBLANK_SECONDARY: c_int = 536870912;
pub const _DRM_VBLANK_SIGNAL: c_int = 1073741824;
pub const enum_drm_vblank_seq_type = c_uint;
pub const struct_drm_wait_vblank_request = extern struct {
    type: enum_drm_vblank_seq_type,
    sequence: c_uint,
    signal: c_ulong,
};
pub const struct_drm_wait_vblank_reply = extern struct {
    type: enum_drm_vblank_seq_type,
    sequence: c_uint,
    tval_sec: c_long,
    tval_usec: c_long,
};
pub const union_drm_wait_vblank = extern union {
    request: struct_drm_wait_vblank_request,
    reply: struct_drm_wait_vblank_reply,
};
pub const struct_drm_modeset_ctl = extern struct {
    crtc: __u32,
    cmd: __u32,
};
pub const struct_drm_agp_mode = extern struct {
    mode: c_ulong,
};
pub const struct_drm_agp_buffer = extern struct {
    size: c_ulong,
    handle: c_ulong,
    type: c_ulong,
    physical: c_ulong,
};
pub const struct_drm_agp_binding = extern struct {
    handle: c_ulong,
    offset: c_ulong,
};
pub const struct_drm_agp_info = extern struct {
    agp_version_major: c_int,
    agp_version_minor: c_int,
    mode: c_ulong,
    aperture_base: c_ulong,
    aperture_size: c_ulong,
    memory_allowed: c_ulong,
    memory_used: c_ulong,
    id_vendor: c_ushort,
    id_device: c_ushort,
};
pub const struct_drm_scatter_gather = extern struct {
    size: c_ulong,
    handle: c_ulong,
};
pub const struct_drm_set_version = extern struct {
    drm_di_major: c_int,
    drm_di_minor: c_int,
    drm_dd_major: c_int,
    drm_dd_minor: c_int,
};
pub const struct_drm_gem_close = extern struct {
    handle: __u32,
    pad: __u32,
};
pub const struct_drm_gem_flink = extern struct {
    handle: __u32,
    name: __u32,
};
pub const struct_drm_gem_open = extern struct {
    name: __u32,
    handle: __u32,
    size: __u64,
};
pub const struct_drm_get_cap = extern struct {
    capability: __u64,
    value: __u64,
};
pub const struct_drm_set_client_cap = extern struct {
    capability: __u64,
    value: __u64,
};
pub const struct_drm_prime_handle = extern struct {
    handle: __u32,
    flags: __u32,
    fd: __s32,
};
pub const struct_drm_syncobj_create = extern struct {
    handle: __u32,
    flags: __u32,
};
pub const struct_drm_syncobj_destroy = extern struct {
    handle: __u32,
    pad: __u32,
};
pub const struct_drm_syncobj_handle = extern struct {
    handle: __u32,
    flags: __u32,
    fd: __s32,
    pad: __u32,
};
pub const struct_drm_syncobj_transfer = extern struct {
    src_handle: __u32,
    dst_handle: __u32,
    src_point: __u64,
    dst_point: __u64,
    flags: __u32,
    pad: __u32,
};
pub const struct_drm_syncobj_wait = extern struct {
    handles: __u64,
    timeout_nsec: __s64,
    count_handles: __u32,
    flags: __u32,
    first_signaled: __u32,
    pad: __u32,
};
pub const struct_drm_syncobj_timeline_wait = extern struct {
    handles: __u64,
    points: __u64,
    timeout_nsec: __s64,
    count_handles: __u32,
    flags: __u32,
    first_signaled: __u32,
    pad: __u32,
};
pub const struct_drm_syncobj_array = extern struct {
    handles: __u64,
    count_handles: __u32,
    pad: __u32,
};
pub const struct_drm_syncobj_timeline_array = extern struct {
    handles: __u64,
    points: __u64,
    count_handles: __u32,
    flags: __u32,
};
pub const struct_drm_crtc_get_sequence = extern struct {
    crtc_id: __u32,
    active: __u32,
    sequence: __u64,
    sequence_ns: __s64,
};
pub const struct_drm_crtc_queue_sequence = extern struct {
    crtc_id: __u32,
    flags: __u32,
    sequence: __u64,
    user_data: __u64,
};
pub const struct_drm_mode_modeinfo = extern struct {
    clock: __u32,
    hdisplay: __u16,
    hsync_start: __u16,
    hsync_end: __u16,
    htotal: __u16,
    hskew: __u16,
    vdisplay: __u16,
    vsync_start: __u16,
    vsync_end: __u16,
    vtotal: __u16,
    vscan: __u16,
    vrefresh: __u32,
    flags: __u32,
    type: __u32,
    name: [32]u8,
};
pub const struct_drm_mode_card_res = extern struct {
    fb_id_ptr: __u64,
    crtc_id_ptr: __u64,
    connector_id_ptr: __u64,
    encoder_id_ptr: __u64,
    count_fbs: __u32,
    count_crtcs: __u32,
    count_connectors: __u32,
    count_encoders: __u32,
    min_width: __u32,
    max_width: __u32,
    min_height: __u32,
    max_height: __u32,
};
pub const struct_drm_mode_crtc = extern struct {
    set_connectors_ptr: __u64,
    count_connectors: __u32,
    crtc_id: __u32,
    fb_id: __u32,
    x: __u32,
    y: __u32,
    gamma_size: __u32,
    mode_valid: __u32,
    mode: struct_drm_mode_modeinfo,
};
pub const struct_drm_mode_set_plane = extern struct {
    plane_id: __u32,
    crtc_id: __u32,
    fb_id: __u32,
    flags: __u32,
    crtc_x: __s32,
    crtc_y: __s32,
    crtc_w: __u32,
    crtc_h: __u32,
    src_x: __u32,
    src_y: __u32,
    src_h: __u32,
    src_w: __u32,
};
pub const struct_drm_mode_get_plane = extern struct {
    plane_id: __u32,
    crtc_id: __u32,
    fb_id: __u32,
    possible_crtcs: __u32,
    gamma_size: __u32,
    count_format_types: __u32,
    format_type_ptr: __u64,
};
pub const struct_drm_mode_get_plane_res = extern struct {
    plane_id_ptr: __u64,
    count_planes: __u32,
};
pub const struct_drm_mode_get_encoder = extern struct {
    encoder_id: __u32,
    encoder_type: __u32,
    crtc_id: __u32,
    possible_crtcs: __u32,
    possible_clones: __u32,
};
pub const DRM_MODE_SUBCONNECTOR_Automatic: c_int = 0;
pub const DRM_MODE_SUBCONNECTOR_Unknown: c_int = 0;
pub const DRM_MODE_SUBCONNECTOR_VGA: c_int = 1;
pub const DRM_MODE_SUBCONNECTOR_DVID: c_int = 3;
pub const DRM_MODE_SUBCONNECTOR_DVIA: c_int = 4;
pub const DRM_MODE_SUBCONNECTOR_Composite: c_int = 5;
pub const DRM_MODE_SUBCONNECTOR_SVIDEO: c_int = 6;
pub const DRM_MODE_SUBCONNECTOR_Component: c_int = 8;
pub const DRM_MODE_SUBCONNECTOR_SCART: c_int = 9;
pub const DRM_MODE_SUBCONNECTOR_DisplayPort: c_int = 10;
pub const DRM_MODE_SUBCONNECTOR_HDMIA: c_int = 11;
pub const DRM_MODE_SUBCONNECTOR_Native: c_int = 15;
pub const DRM_MODE_SUBCONNECTOR_Wireless: c_int = 18;
pub const enum_drm_mode_subconnector = c_uint;
pub const struct_drm_mode_get_connector = extern struct {
    encoders_ptr: __u64,
    modes_ptr: __u64,
    props_ptr: __u64,
    prop_values_ptr: __u64,
    count_modes: __u32,
    count_props: __u32,
    count_encoders: __u32,
    encoder_id: __u32,
    connector_id: __u32,
    connector_type: __u32,
    connector_type_id: __u32,
    connection: __u32,
    mm_width: __u32,
    mm_height: __u32,
    subpixel: __u32,
    pad: __u32,
};
pub const struct_drm_mode_property_enum = extern struct {
    value: __u64,
    name: [32]u8,
};
pub const struct_drm_mode_get_property = extern struct {
    values_ptr: __u64,
    enum_blob_ptr: __u64,
    prop_id: __u32,
    flags: __u32,
    name: [32]u8,
    count_values: __u32,
    count_enum_blobs: __u32,
};
pub const struct_drm_mode_connector_set_property = extern struct {
    value: __u64,
    prop_id: __u32,
    connector_id: __u32,
};
pub const struct_drm_mode_obj_get_properties = extern struct {
    props_ptr: __u64,
    prop_values_ptr: __u64,
    count_props: __u32,
    obj_id: __u32,
    obj_type: __u32,
};
pub const struct_drm_mode_obj_set_property = extern struct {
    value: __u64,
    prop_id: __u32,
    obj_id: __u32,
    obj_type: __u32,
};
pub const struct_drm_mode_get_blob = extern struct {
    blob_id: __u32,
    length: __u32,
    data: __u64,
};
pub const struct_drm_mode_fb_cmd = extern struct {
    fb_id: __u32,
    width: __u32,
    height: __u32,
    pitch: __u32,
    bpp: __u32,
    depth: __u32,
    handle: __u32,
};
pub const struct_drm_mode_fb_cmd2 = extern struct {
    fb_id: __u32,
    width: __u32,
    height: __u32,
    pixel_format: __u32,
    flags: __u32,
    handles: [4]__u32,
    pitches: [4]__u32,
    offsets: [4]__u32,
    modifier: [4]__u64,
};
pub const struct_drm_mode_fb_dirty_cmd = extern struct {
    fb_id: __u32,
    flags: __u32,
    color: __u32,
    num_clips: __u32,
    clips_ptr: __u64,
};
pub const struct_drm_mode_mode_cmd = extern struct {
    connector_id: __u32,
    mode: struct_drm_mode_modeinfo,
};
pub const struct_drm_mode_cursor = extern struct {
    flags: __u32,
    crtc_id: __u32,
    x: __s32,
    y: __s32,
    width: __u32,
    height: __u32,
    handle: __u32,
};
pub const struct_drm_mode_cursor2 = extern struct {
    flags: __u32,
    crtc_id: __u32,
    x: __s32,
    y: __s32,
    width: __u32,
    height: __u32,
    handle: __u32,
    hot_x: __s32,
    hot_y: __s32,
};
pub const struct_drm_mode_crtc_lut = extern struct {
    crtc_id: __u32,
    gamma_size: __u32,
    red: __u64,
    green: __u64,
    blue: __u64,
};
pub const struct_drm_color_ctm = extern struct {
    matrix: [9]__u64,
};
pub const struct_drm_color_lut = extern struct {
    red: __u16,
    green: __u16,
    blue: __u16,
    reserved: __u16,
};
const struct_unnamed_6 = extern struct {
    x: __u16,
    y: __u16,
};
const struct_unnamed_7 = extern struct {
    x: __u16,
    y: __u16,
};
pub const struct_hdr_metadata_infoframe = extern struct {
    eotf: __u8,
    metadata_type: __u8,
    display_primaries: [3]struct_unnamed_6,
    white_point: struct_unnamed_7,
    max_display_mastering_luminance: __u16,
    min_display_mastering_luminance: __u16,
    max_cll: __u16,
    max_fall: __u16,
};
const union_unnamed_8 = extern union {
    hdmi_metadata_type1: struct_hdr_metadata_infoframe,
};
pub const struct_hdr_output_metadata = extern struct {
    metadata_type: __u32,
    unnamed_0: union_unnamed_8,
};
pub const struct_drm_mode_crtc_page_flip = extern struct {
    crtc_id: __u32,
    fb_id: __u32,
    flags: __u32,
    reserved: __u32,
    user_data: __u64,
};
pub const struct_drm_mode_crtc_page_flip_target = extern struct {
    crtc_id: __u32,
    fb_id: __u32,
    flags: __u32,
    sequence: __u32,
    user_data: __u64,
};
pub const struct_drm_mode_create_dumb = extern struct {
    height: __u32,
    width: __u32,
    bpp: __u32,
    flags: __u32,
    handle: __u32,
    pitch: __u32,
    size: __u64,
};
pub const struct_drm_mode_map_dumb = extern struct {
    handle: __u32,
    pad: __u32,
    offset: __u64,
};
pub const struct_drm_mode_destroy_dumb = extern struct {
    handle: __u32,
};
pub const struct_drm_mode_atomic = extern struct {
    flags: __u32,
    count_objs: __u32,
    objs_ptr: __u64,
    count_props_ptr: __u64,
    props_ptr: __u64,
    prop_values_ptr: __u64,
    reserved: __u64,
    user_data: __u64,
};
pub const struct_drm_format_modifier_blob = extern struct {
    version: __u32,
    flags: __u32,
    count_formats: __u32,
    formats_offset: __u32,
    count_modifiers: __u32,
    modifiers_offset: __u32,
};
pub const struct_drm_format_modifier = extern struct {
    formats: __u64,
    offset: __u32,
    pad: __u32,
    modifier: __u64,
};
pub const struct_drm_mode_create_blob = extern struct {
    data: __u64,
    length: __u32,
    blob_id: __u32,
};
pub const struct_drm_mode_destroy_blob = extern struct {
    blob_id: __u32,
};
pub const struct_drm_mode_create_lease = extern struct {
    object_ids: __u64,
    object_count: __u32,
    flags: __u32,
    lessee_id: __u32,
    fd: __u32,
};
pub const struct_drm_mode_list_lessees = extern struct {
    count_lessees: __u32,
    pad: __u32,
    lessees_ptr: __u64,
};
pub const struct_drm_mode_get_lease = extern struct {
    count_objects: __u32,
    pad: __u32,
    objects_ptr: __u64,
};
pub const struct_drm_mode_revoke_lease = extern struct {
    lessee_id: __u32,
};
pub const struct_drm_mode_rect = extern struct {
    x1: __s32,
    y1: __s32,
    x2: __s32,
    y2: __s32,
};
pub const struct_drm_event = extern struct {
    type: __u32,
    length: __u32,
};
pub const struct_drm_event_vblank = extern struct {
    base: struct_drm_event,
    user_data: __u64,
    tv_sec: __u32,
    tv_usec: __u32,
    sequence: __u32,
    crtc_id: __u32,
};
pub const struct_drm_event_crtc_sequence = extern struct {
    base: struct_drm_event,
    user_data: __u64,
    time_ns: __s64,
    sequence: __u64,
};
pub const drm_clip_rect_t = struct_drm_clip_rect;
pub const drm_drawable_info_t = struct_drm_drawable_info;
pub const drm_tex_region_t = struct_drm_tex_region;
pub const drm_hw_lock_t = struct_drm_hw_lock;
pub const drm_version_t = struct_drm_version;
pub const drm_unique_t = struct_drm_unique;
pub const drm_list_t = struct_drm_list;
pub const drm_block_t = struct_drm_block;
pub const drm_control_t = struct_drm_control;
pub const drm_map_type_t = enum_drm_map_type;
pub const drm_map_flags_t = enum_drm_map_flags;
pub const drm_ctx_priv_map_t = struct_drm_ctx_priv_map;
pub const drm_map_t = struct_drm_map;
pub const drm_client_t = struct_drm_client;
pub const drm_stat_type_t = enum_drm_stat_type;
pub const drm_stats_t = struct_drm_stats;
pub const drm_lock_flags_t = enum_drm_lock_flags;
pub const drm_lock_t = struct_drm_lock;
pub const drm_dma_flags_t = enum_drm_dma_flags;
pub const drm_buf_desc_t = struct_drm_buf_desc;
pub const drm_buf_info_t = struct_drm_buf_info;
pub const drm_buf_free_t = struct_drm_buf_free;
pub const drm_buf_pub_t = struct_drm_buf_pub;
pub const drm_buf_map_t = struct_drm_buf_map;
pub const drm_dma_t = struct_drm_dma;
pub const drm_wait_vblank_t = union_drm_wait_vblank;
pub const drm_agp_mode_t = struct_drm_agp_mode;
pub const drm_ctx_flags_t = enum_drm_ctx_flags;
pub const drm_ctx_t = struct_drm_ctx;
pub const drm_ctx_res_t = struct_drm_ctx_res;
pub const drm_draw_t = struct_drm_draw;
pub const drm_update_draw_t = struct_drm_update_draw;
pub const drm_auth_t = struct_drm_auth;
pub const drm_irq_busid_t = struct_drm_irq_busid;
pub const drm_vblank_seq_type_t = enum_drm_vblank_seq_type;
pub const drm_agp_buffer_t = struct_drm_agp_buffer;
pub const drm_agp_binding_t = struct_drm_agp_binding;
pub const drm_agp_info_t = struct_drm_agp_info;
pub const drm_scatter_gather_t = struct_drm_scatter_gather;
pub const drm_set_version_t = struct_drm_set_version;
pub const drmSize = c_uint;
pub const drmSizePtr = [*c]c_uint;
pub const drmAddress = ?*anyopaque;
pub const drmAddressPtr = [*c]?*anyopaque;
pub const struct__drmServerInfo = extern struct {
    debug_print: ?*const fn ([*c]const u8, [*c]struct___va_list_tag) callconv(.C) c_int,
    load_module: ?*const fn ([*c]const u8) callconv(.C) c_int,
    get_perms: ?*const fn ([*c]gid_t, [*c]mode_t) callconv(.C) void,
};
pub const drmServerInfo = struct__drmServerInfo;
pub const drmServerInfoPtr = [*c]struct__drmServerInfo;
pub const struct_drmHashEntry = extern struct {
    fd: c_int,
    f: ?*const fn (c_int, ?*anyopaque, ?*anyopaque) callconv(.C) void,
    tagTable: ?*anyopaque,
};
pub const drmHashEntry = struct_drmHashEntry;
pub extern fn drmIoctl(fd: c_int, request: c_ulong, arg: ?*anyopaque) c_int;
pub extern fn drmGetHashTable() ?*anyopaque;
pub extern fn drmGetEntry(fd: c_int) [*c]drmHashEntry;
pub const struct__drmVersion = extern struct {
    version_major: c_int,
    version_minor: c_int,
    version_patchlevel: c_int,
    name_len: c_int,
    name: [*c]u8,
    date_len: c_int,
    date: [*c]u8,
    desc_len: c_int,
    desc: [*c]u8,
};
pub const drmVersion = struct__drmVersion;
pub const drmVersionPtr = [*c]struct__drmVersion;
const struct_unnamed_9 = extern struct {
    value: c_ulong,
    long_format: [*c]const u8,
    long_name: [*c]const u8,
    rate_format: [*c]const u8,
    rate_name: [*c]const u8,
    isvalue: c_int,
    mult_names: [*c]const u8,
    mult: c_int,
    verbose: c_int,
};
pub const struct__drmStats = extern struct {
    count: c_ulong,
    data: [15]struct_unnamed_9,
};
pub const drmStatsT = struct__drmStats;
pub const DRM_FRAME_BUFFER: c_int = 0;
pub const DRM_REGISTERS: c_int = 1;
pub const DRM_SHM: c_int = 2;
pub const DRM_AGP: c_int = 3;
pub const DRM_SCATTER_GATHER: c_int = 4;
pub const DRM_CONSISTENT: c_int = 5;
pub const drmMapType = c_uint;
pub const DRM_RESTRICTED: c_int = 1;
pub const DRM_READ_ONLY: c_int = 2;
pub const DRM_LOCKED: c_int = 4;
pub const DRM_KERNEL: c_int = 8;
pub const DRM_WRITE_COMBINING: c_int = 16;
pub const DRM_CONTAINS_LOCK: c_int = 32;
pub const DRM_REMOVABLE: c_int = 64;
pub const drmMapFlags = c_uint;
pub const DRM_DMA_BLOCK: c_int = 1;
pub const DRM_DMA_WHILE_LOCKED: c_int = 2;
pub const DRM_DMA_PRIORITY: c_int = 4;
pub const DRM_DMA_WAIT: c_int = 16;
pub const DRM_DMA_SMALLER_OK: c_int = 32;
pub const DRM_DMA_LARGER_OK: c_int = 64;
pub const drmDMAFlags = c_uint;
pub const DRM_PAGE_ALIGN: c_int = 1;
pub const DRM_AGP_BUFFER: c_int = 2;
pub const DRM_SG_BUFFER: c_int = 4;
pub const DRM_FB_BUFFER: c_int = 8;
pub const DRM_PCI_BUFFER_RO: c_int = 16;
pub const drmBufDescFlags = c_uint;
pub const DRM_LOCK_READY: c_int = 1;
pub const DRM_LOCK_QUIESCENT: c_int = 2;
pub const DRM_LOCK_FLUSH: c_int = 4;
pub const DRM_LOCK_FLUSH_ALL: c_int = 8;
pub const DRM_HALT_ALL_QUEUES: c_int = 16;
pub const DRM_HALT_CUR_QUEUES: c_int = 32;
pub const drmLockFlags = c_uint;
pub const DRM_CONTEXT_PRESERVED: c_int = 1;
pub const DRM_CONTEXT_2DONLY: c_int = 2;
pub const drm_context_tFlags = c_uint;
pub const drm_context_tFlagsPtr = [*c]drm_context_tFlags;
pub const struct__drmBufDesc = extern struct {
    count: c_int,
    size: c_int,
    low_mark: c_int,
    high_mark: c_int,
};
pub const drmBufDesc = struct__drmBufDesc;
pub const drmBufDescPtr = [*c]struct__drmBufDesc;
pub const struct__drmBufInfo = extern struct {
    count: c_int,
    list: drmBufDescPtr,
};
pub const drmBufInfo = struct__drmBufInfo;
pub const drmBufInfoPtr = [*c]struct__drmBufInfo;
pub const struct__drmBuf = extern struct {
    idx: c_int,
    total: c_int,
    used: c_int,
    address: drmAddress,
};
pub const drmBuf = struct__drmBuf;
pub const drmBufPtr = [*c]struct__drmBuf;
pub const struct__drmBufMap = extern struct {
    count: c_int,
    list: drmBufPtr,
};
pub const drmBufMap = struct__drmBufMap;
pub const drmBufMapPtr = [*c]struct__drmBufMap;
pub const struct__drmLock = extern struct {
    lock: c_uint,
    padding: [60]u8,
};
pub const drmLock = struct__drmLock;
pub const drmLockPtr = [*c]struct__drmLock;
pub const struct__drmDMAReq = extern struct {
    context: drm_context_t,
    send_count: c_int,
    send_list: [*c]c_int,
    send_sizes: [*c]c_int,
    flags: drmDMAFlags,
    request_count: c_int,
    request_size: c_int,
    request_list: [*c]c_int,
    request_sizes: [*c]c_int,
    granted_count: c_int,
};
pub const drmDMAReq = struct__drmDMAReq;
pub const drmDMAReqPtr = [*c]struct__drmDMAReq;
pub const struct__drmRegion = extern struct {
    handle: drm_handle_t,
    offset: c_uint,
    size: drmSize,
    map: drmAddress,
};
pub const drmRegion = struct__drmRegion;
pub const drmRegionPtr = [*c]struct__drmRegion;
pub const struct__drmTextureRegion = extern struct {
    next: u8,
    prev: u8,
    in_use: u8,
    padding: u8,
    age: c_uint,
};
pub const drmTextureRegion = struct__drmTextureRegion;
pub const drmTextureRegionPtr = [*c]struct__drmTextureRegion;
pub const DRM_VBLANK_ABSOLUTE: c_int = 0;
pub const DRM_VBLANK_RELATIVE: c_int = 1;
pub const DRM_VBLANK_HIGH_CRTC_MASK: c_int = 62;
pub const DRM_VBLANK_EVENT: c_int = 67108864;
pub const DRM_VBLANK_FLIP: c_int = 134217728;
pub const DRM_VBLANK_NEXTONMISS: c_int = 268435456;
pub const DRM_VBLANK_SECONDARY: c_int = 536870912;
pub const DRM_VBLANK_SIGNAL: c_int = 1073741824;
pub const drmVBlankSeqType = c_uint;
pub const struct__drmVBlankReq = extern struct {
    type: drmVBlankSeqType,
    sequence: c_uint,
    signal: c_ulong,
};
pub const drmVBlankReq = struct__drmVBlankReq;
pub const drmVBlankReqPtr = [*c]struct__drmVBlankReq;
pub const struct__drmVBlankReply = extern struct {
    type: drmVBlankSeqType,
    sequence: c_uint,
    tval_sec: c_long,
    tval_usec: c_long,
};
pub const drmVBlankReply = struct__drmVBlankReply;
pub const drmVBlankReplyPtr = [*c]struct__drmVBlankReply;
pub const union__drmVBlank = extern union {
    request: drmVBlankReq,
    reply: drmVBlankReply,
};
pub const drmVBlank = union__drmVBlank;
pub const drmVBlankPtr = [*c]union__drmVBlank;
pub const struct__drmSetVersion = extern struct {
    drm_di_major: c_int,
    drm_di_minor: c_int,
    drm_dd_major: c_int,
    drm_dd_minor: c_int,
};
pub const drmSetVersion = struct__drmSetVersion;
pub const drmSetVersionPtr = [*c]struct__drmSetVersion;
pub extern fn drmAvailable() c_int;
pub extern fn drmOpen(name: [*c]const u8, busid: [*c]const u8) c_int;
pub extern fn drmOpenWithType(name: [*c]const u8, busid: [*c]const u8, @"type": c_int) c_int;
pub extern fn drmOpenControl(minor: c_int) c_int;
pub extern fn drmOpenRender(minor: c_int) c_int;
pub extern fn drmClose(fd: c_int) c_int;
pub extern fn drmGetVersion(fd: c_int) drmVersionPtr;
pub extern fn drmGetLibVersion(fd: c_int) drmVersionPtr;
pub extern fn drmGetCap(fd: c_int, capability: u64, value: [*c]u64) c_int;
pub extern fn drmFreeVersion(drmVersionPtr) void;
pub extern fn drmGetMagic(fd: c_int, magic: [*c]drm_magic_t) c_int;
pub extern fn drmGetBusid(fd: c_int) [*c]u8;
pub extern fn drmGetInterruptFromBusID(fd: c_int, busnum: c_int, devnum: c_int, funcnum: c_int) c_int;
pub extern fn drmGetMap(fd: c_int, idx: c_int, offset: [*c]drm_handle_t, size: [*c]drmSize, @"type": [*c]drmMapType, flags: [*c]drmMapFlags, handle: [*c]drm_handle_t, mtrr: [*c]c_int) c_int;
pub extern fn drmGetClient(fd: c_int, idx: c_int, auth: [*c]c_int, pid: [*c]c_int, uid: [*c]c_int, magic: [*c]c_ulong, iocs: [*c]c_ulong) c_int;
pub extern fn drmGetStats(fd: c_int, stats: [*c]drmStatsT) c_int;
pub extern fn drmSetInterfaceVersion(fd: c_int, version: [*c]drmSetVersion) c_int;
pub extern fn drmCommandNone(fd: c_int, drmCommandIndex: c_ulong) c_int;
pub extern fn drmCommandRead(fd: c_int, drmCommandIndex: c_ulong, data: ?*anyopaque, size: c_ulong) c_int;
pub extern fn drmCommandWrite(fd: c_int, drmCommandIndex: c_ulong, data: ?*anyopaque, size: c_ulong) c_int;
pub extern fn drmCommandWriteRead(fd: c_int, drmCommandIndex: c_ulong, data: ?*anyopaque, size: c_ulong) c_int;
pub extern fn drmFreeBusid(busid: [*c]const u8) void;
pub extern fn drmSetBusid(fd: c_int, busid: [*c]const u8) c_int;
pub extern fn drmAuthMagic(fd: c_int, magic: drm_magic_t) c_int;
pub extern fn drmAddMap(fd: c_int, offset: drm_handle_t, size: drmSize, @"type": drmMapType, flags: drmMapFlags, handle: [*c]drm_handle_t) c_int;
pub extern fn drmRmMap(fd: c_int, handle: drm_handle_t) c_int;
pub extern fn drmAddContextPrivateMapping(fd: c_int, ctx_id: drm_context_t, handle: drm_handle_t) c_int;
pub extern fn drmAddBufs(fd: c_int, count: c_int, size: c_int, flags: drmBufDescFlags, agp_offset: c_int) c_int;
pub extern fn drmMarkBufs(fd: c_int, low: f64, high: f64) c_int;
pub extern fn drmCreateContext(fd: c_int, handle: [*c]drm_context_t) c_int;
pub extern fn drmSetContextFlags(fd: c_int, context: drm_context_t, flags: drm_context_tFlags) c_int;
pub extern fn drmGetContextFlags(fd: c_int, context: drm_context_t, flags: drm_context_tFlagsPtr) c_int;
pub extern fn drmAddContextTag(fd: c_int, context: drm_context_t, tag: ?*anyopaque) c_int;
pub extern fn drmDelContextTag(fd: c_int, context: drm_context_t) c_int;
pub extern fn drmGetContextTag(fd: c_int, context: drm_context_t) ?*anyopaque;
pub extern fn drmGetReservedContextList(fd: c_int, count: [*c]c_int) [*c]drm_context_t;
pub extern fn drmFreeReservedContextList([*c]drm_context_t) void;
pub extern fn drmSwitchToContext(fd: c_int, context: drm_context_t) c_int;
pub extern fn drmDestroyContext(fd: c_int, handle: drm_context_t) c_int;
pub extern fn drmCreateDrawable(fd: c_int, handle: [*c]drm_drawable_t) c_int;
pub extern fn drmDestroyDrawable(fd: c_int, handle: drm_drawable_t) c_int;
pub extern fn drmUpdateDrawableInfo(fd: c_int, handle: drm_drawable_t, @"type": drm_drawable_info_type_t, num: c_uint, data: ?*anyopaque) c_int;
pub extern fn drmCtlInstHandler(fd: c_int, irq: c_int) c_int;
pub extern fn drmCtlUninstHandler(fd: c_int) c_int;
pub extern fn drmSetClientCap(fd: c_int, capability: u64, value: u64) c_int;
pub extern fn drmCrtcGetSequence(fd: c_int, crtcId: u32, sequence: [*c]u64, ns: [*c]u64) c_int;
pub extern fn drmCrtcQueueSequence(fd: c_int, crtcId: u32, flags: u32, sequence: u64, sequence_queued: [*c]u64, user_data: u64) c_int;
pub extern fn drmMap(fd: c_int, handle: drm_handle_t, size: drmSize, address: drmAddressPtr) c_int;
pub extern fn drmUnmap(address: drmAddress, size: drmSize) c_int;
pub extern fn drmGetBufInfo(fd: c_int) drmBufInfoPtr;
pub extern fn drmMapBufs(fd: c_int) drmBufMapPtr;
pub extern fn drmUnmapBufs(bufs: drmBufMapPtr) c_int;
pub extern fn drmDMA(fd: c_int, request: drmDMAReqPtr) c_int;
pub extern fn drmFreeBufs(fd: c_int, count: c_int, list: [*c]c_int) c_int;
pub extern fn drmGetLock(fd: c_int, context: drm_context_t, flags: drmLockFlags) c_int;
pub extern fn drmUnlock(fd: c_int, context: drm_context_t) c_int;
pub extern fn drmFinish(fd: c_int, context: c_int, flags: drmLockFlags) c_int;
pub extern fn drmGetContextPrivateMapping(fd: c_int, ctx_id: drm_context_t, handle: [*c]drm_handle_t) c_int;
pub extern fn drmAgpAcquire(fd: c_int) c_int;
pub extern fn drmAgpRelease(fd: c_int) c_int;
pub extern fn drmAgpEnable(fd: c_int, mode: c_ulong) c_int;
pub extern fn drmAgpAlloc(fd: c_int, size: c_ulong, @"type": c_ulong, address: [*c]c_ulong, handle: [*c]drm_handle_t) c_int;
pub extern fn drmAgpFree(fd: c_int, handle: drm_handle_t) c_int;
pub extern fn drmAgpBind(fd: c_int, handle: drm_handle_t, offset: c_ulong) c_int;
pub extern fn drmAgpUnbind(fd: c_int, handle: drm_handle_t) c_int;
pub extern fn drmAgpVersionMajor(fd: c_int) c_int;
pub extern fn drmAgpVersionMinor(fd: c_int) c_int;
pub extern fn drmAgpGetMode(fd: c_int) c_ulong;
pub extern fn drmAgpBase(fd: c_int) c_ulong;
pub extern fn drmAgpSize(fd: c_int) c_ulong;
pub extern fn drmAgpMemoryUsed(fd: c_int) c_ulong;
pub extern fn drmAgpMemoryAvail(fd: c_int) c_ulong;
pub extern fn drmAgpVendorId(fd: c_int) c_uint;
pub extern fn drmAgpDeviceId(fd: c_int) c_uint;
pub extern fn drmScatterGatherAlloc(fd: c_int, size: c_ulong, handle: [*c]drm_handle_t) c_int;
pub extern fn drmScatterGatherFree(fd: c_int, handle: drm_handle_t) c_int;
pub extern fn drmWaitVBlank(fd: c_int, vbl: drmVBlankPtr) c_int;
pub extern fn drmSetServerInfo(info: drmServerInfoPtr) void;
pub extern fn drmError(err: c_int, label: [*c]const u8) c_int;
pub extern fn drmMalloc(size: c_int) ?*anyopaque;
pub extern fn drmFree(pt: ?*anyopaque) void;
pub extern fn drmHashCreate() ?*anyopaque;
pub extern fn drmHashDestroy(t: ?*anyopaque) c_int;
pub extern fn drmHashLookup(t: ?*anyopaque, key: c_ulong, value: [*c]?*anyopaque) c_int;
pub extern fn drmHashInsert(t: ?*anyopaque, key: c_ulong, value: ?*anyopaque) c_int;
pub extern fn drmHashDelete(t: ?*anyopaque, key: c_ulong) c_int;
pub extern fn drmHashFirst(t: ?*anyopaque, key: [*c]c_ulong, value: [*c]?*anyopaque) c_int;
pub extern fn drmHashNext(t: ?*anyopaque, key: [*c]c_ulong, value: [*c]?*anyopaque) c_int;
pub extern fn drmRandomCreate(seed: c_ulong) ?*anyopaque;
pub extern fn drmRandomDestroy(state: ?*anyopaque) c_int;
pub extern fn drmRandom(state: ?*anyopaque) c_ulong;
pub extern fn drmRandomDouble(state: ?*anyopaque) f64;
pub extern fn drmSLCreate() ?*anyopaque;
pub extern fn drmSLDestroy(l: ?*anyopaque) c_int;
pub extern fn drmSLLookup(l: ?*anyopaque, key: c_ulong, value: [*c]?*anyopaque) c_int;
pub extern fn drmSLInsert(l: ?*anyopaque, key: c_ulong, value: ?*anyopaque) c_int;
pub extern fn drmSLDelete(l: ?*anyopaque, key: c_ulong) c_int;
pub extern fn drmSLNext(l: ?*anyopaque, key: [*c]c_ulong, value: [*c]?*anyopaque) c_int;
pub extern fn drmSLFirst(l: ?*anyopaque, key: [*c]c_ulong, value: [*c]?*anyopaque) c_int;
pub extern fn drmSLDump(l: ?*anyopaque) void;
pub extern fn drmSLLookupNeighbors(l: ?*anyopaque, key: c_ulong, prev_key: [*c]c_ulong, prev_value: [*c]?*anyopaque, next_key: [*c]c_ulong, next_value: [*c]?*anyopaque) c_int;
pub extern fn drmOpenOnce(unused: ?*anyopaque, BusID: [*c]const u8, newlyopened: [*c]c_int) c_int;
pub extern fn drmOpenOnceWithType(BusID: [*c]const u8, newlyopened: [*c]c_int, @"type": c_int) c_int;
pub extern fn drmCloseOnce(fd: c_int) void;
pub extern fn drmMsg(format: [*c]const u8, ...) void;
pub extern fn drmSetMaster(fd: c_int) c_int;
pub extern fn drmDropMaster(fd: c_int) c_int;
pub extern fn drmIsMaster(fd: c_int) c_int;
pub const struct__drmEventContext = extern struct {
    version: c_int,
    vblank_handler: ?*const fn (c_int, c_uint, c_uint, c_uint, ?*anyopaque) callconv(.C) void,
    page_flip_handler: ?*const fn (c_int, c_uint, c_uint, c_uint, ?*anyopaque) callconv(.C) void,
    page_flip_handler2: ?*const fn (c_int, c_uint, c_uint, c_uint, c_uint, ?*anyopaque) callconv(.C) void,
    sequence_handler: ?*const fn (c_int, u64, u64, u64) callconv(.C) void,
};
pub const drmEventContext = struct__drmEventContext;
pub const drmEventContextPtr = [*c]struct__drmEventContext;
pub extern fn drmHandleEvent(fd: c_int, evctx: drmEventContextPtr) c_int;
pub extern fn drmGetDeviceNameFromFd(fd: c_int) [*c]u8;
pub extern fn drmGetDeviceNameFromFd2(fd: c_int) [*c]u8;
pub extern fn drmGetNodeTypeFromFd(fd: c_int) c_int;
pub extern fn drmPrimeHandleToFD(fd: c_int, handle: u32, flags: u32, prime_fd: [*c]c_int) c_int;
pub extern fn drmPrimeFDToHandle(fd: c_int, prime_fd: c_int, handle: [*c]u32) c_int;
pub extern fn drmCloseBufferHandle(fd: c_int, handle: u32) c_int;
pub extern fn drmGetPrimaryDeviceNameFromFd(fd: c_int) [*c]u8;
pub extern fn drmGetRenderDeviceNameFromFd(fd: c_int) [*c]u8;
pub const struct__drmPciBusInfo = extern struct {
    domain: u16,
    bus: u8,
    dev: u8,
    func: u8,
};
pub const drmPciBusInfo = struct__drmPciBusInfo;
pub const drmPciBusInfoPtr = [*c]struct__drmPciBusInfo;
pub const struct__drmPciDeviceInfo = extern struct {
    vendor_id: u16,
    device_id: u16,
    subvendor_id: u16,
    subdevice_id: u16,
    revision_id: u8,
};
pub const drmPciDeviceInfo = struct__drmPciDeviceInfo;
pub const drmPciDeviceInfoPtr = [*c]struct__drmPciDeviceInfo;
pub const struct__drmUsbBusInfo = extern struct {
    bus: u8,
    dev: u8,
};
pub const drmUsbBusInfo = struct__drmUsbBusInfo;
pub const drmUsbBusInfoPtr = [*c]struct__drmUsbBusInfo;
pub const struct__drmUsbDeviceInfo = extern struct {
    vendor: u16,
    product: u16,
};
pub const drmUsbDeviceInfo = struct__drmUsbDeviceInfo;
pub const drmUsbDeviceInfoPtr = [*c]struct__drmUsbDeviceInfo;
pub const struct__drmPlatformBusInfo = extern struct {
    fullname: [512]u8,
};
pub const drmPlatformBusInfo = struct__drmPlatformBusInfo;
pub const drmPlatformBusInfoPtr = [*c]struct__drmPlatformBusInfo;
pub const struct__drmPlatformDeviceInfo = extern struct {
    compatible: [*c][*c]u8,
};
pub const drmPlatformDeviceInfo = struct__drmPlatformDeviceInfo;
pub const drmPlatformDeviceInfoPtr = [*c]struct__drmPlatformDeviceInfo;
pub const struct__drmHost1xBusInfo = extern struct {
    fullname: [512]u8,
};
pub const drmHost1xBusInfo = struct__drmHost1xBusInfo;
pub const drmHost1xBusInfoPtr = [*c]struct__drmHost1xBusInfo;
pub const struct__drmHost1xDeviceInfo = extern struct {
    compatible: [*c][*c]u8,
};
pub const drmHost1xDeviceInfo = struct__drmHost1xDeviceInfo;
pub const drmHost1xDeviceInfoPtr = [*c]struct__drmHost1xDeviceInfo;
const union_unnamed_10 = extern union {
    pci: drmPciBusInfoPtr,
    usb: drmUsbBusInfoPtr,
    platform: drmPlatformBusInfoPtr,
    host1x: drmHost1xBusInfoPtr,
};
const union_unnamed_11 = extern union {
    pci: drmPciDeviceInfoPtr,
    usb: drmUsbDeviceInfoPtr,
    platform: drmPlatformDeviceInfoPtr,
    host1x: drmHost1xDeviceInfoPtr,
};
pub const struct__drmDevice = extern struct {
    nodes: [*c][*c]u8,
    available_nodes: c_int,
    bustype: c_int,
    businfo: union_unnamed_10,
    deviceinfo: union_unnamed_11,
};
pub const drmDevice = struct__drmDevice;
pub const drmDevicePtr = [*c]struct__drmDevice;
pub extern fn drmGetDevice(fd: c_int, device: [*c]drmDevicePtr) c_int;
pub extern fn drmFreeDevice(device: [*c]drmDevicePtr) void;
pub extern fn drmGetDevices(devices: [*c]drmDevicePtr, max_devices: c_int) c_int;
pub extern fn drmFreeDevices(devices: [*c]drmDevicePtr, count: c_int) void;
pub extern fn drmGetDevice2(fd: c_int, flags: u32, device: [*c]drmDevicePtr) c_int;
pub extern fn drmGetDevices2(flags: u32, devices: [*c]drmDevicePtr, max_devices: c_int) c_int;
pub extern fn drmGetDeviceFromDevId(dev_id: dev_t, flags: u32, device: [*c]drmDevicePtr) c_int;
pub extern fn drmGetNodeTypeFromDevId(devid: dev_t) c_int;
pub extern fn drmDevicesEqual(a: drmDevicePtr, b: drmDevicePtr) c_int;
pub extern fn drmSyncobjCreate(fd: c_int, flags: u32, handle: [*c]u32) c_int;
pub extern fn drmSyncobjDestroy(fd: c_int, handle: u32) c_int;
pub extern fn drmSyncobjHandleToFD(fd: c_int, handle: u32, obj_fd: [*c]c_int) c_int;
pub extern fn drmSyncobjFDToHandle(fd: c_int, obj_fd: c_int, handle: [*c]u32) c_int;
pub extern fn drmSyncobjImportSyncFile(fd: c_int, handle: u32, sync_file_fd: c_int) c_int;
pub extern fn drmSyncobjExportSyncFile(fd: c_int, handle: u32, sync_file_fd: [*c]c_int) c_int;
pub extern fn drmSyncobjWait(fd: c_int, handles: [*c]u32, num_handles: c_uint, timeout_nsec: i64, flags: c_uint, first_signaled: [*c]u32) c_int;
pub extern fn drmSyncobjReset(fd: c_int, handles: [*c]const u32, handle_count: u32) c_int;
pub extern fn drmSyncobjSignal(fd: c_int, handles: [*c]const u32, handle_count: u32) c_int;
pub extern fn drmSyncobjTimelineSignal(fd: c_int, handles: [*c]const u32, points: [*c]u64, handle_count: u32) c_int;
pub extern fn drmSyncobjTimelineWait(fd: c_int, handles: [*c]u32, points: [*c]u64, num_handles: c_uint, timeout_nsec: i64, flags: c_uint, first_signaled: [*c]u32) c_int;
pub extern fn drmSyncobjQuery(fd: c_int, handles: [*c]u32, points: [*c]u64, handle_count: u32) c_int;
pub extern fn drmSyncobjQuery2(fd: c_int, handles: [*c]u32, points: [*c]u64, handle_count: u32, flags: u32) c_int;
pub extern fn drmSyncobjTransfer(fd: c_int, dst_handle: u32, dst_point: u64, src_handle: u32, src_point: u64, flags: u32) c_int;
pub extern fn drmSyncobjEventfd(fd: c_int, handle: u32, point: u64, ev_fd: c_int, flags: u32) c_int;
pub extern fn drmGetFormatModifierVendor(modifier: u64) [*c]u8;
pub extern fn drmGetFormatModifierName(modifier: u64) [*c]u8;
pub extern fn drmGetFormatName(format: u32) [*c]u8;
pub const ptrdiff_t = c_long;
pub const max_align_t = extern struct {
    __clang_max_align_nonce1: c_longlong align(8),
    __clang_max_align_nonce2: c_longdouble align(16),
};
pub const struct__drmModeRes = extern struct {
    count_fbs: c_int,
    fbs: [*c]u32,
    count_crtcs: c_int,
    crtcs: [*c]u32,
    count_connectors: c_int,
    connectors: [*c]u32,
    count_encoders: c_int,
    encoders: [*c]u32,
    min_width: u32,
    max_width: u32,
    min_height: u32,
    max_height: u32,
};
pub const drmModeRes = struct__drmModeRes;
pub const drmModeResPtr = [*c]struct__drmModeRes;
pub const struct__drmModeModeInfo = extern struct {
    clock: u32,
    hdisplay: u16,
    hsync_start: u16,
    hsync_end: u16,
    htotal: u16,
    hskew: u16,
    vdisplay: u16,
    vsync_start: u16,
    vsync_end: u16,
    vtotal: u16,
    vscan: u16,
    vrefresh: u32,
    flags: u32,
    type: u32,
    name: [32]u8,
};
pub const drmModeModeInfo = struct__drmModeModeInfo;
pub const drmModeModeInfoPtr = [*c]struct__drmModeModeInfo;
pub const struct__drmModeFB = extern struct {
    fb_id: u32,
    width: u32,
    height: u32,
    pitch: u32,
    bpp: u32,
    depth: u32,
    handle: u32,
};
pub const drmModeFB = struct__drmModeFB;
pub const drmModeFBPtr = [*c]struct__drmModeFB;
pub const struct__drmModeFB2 = extern struct {
    fb_id: u32,
    width: u32,
    height: u32,
    pixel_format: u32,
    modifier: u64,
    flags: u32,
    handles: [4]u32,
    pitches: [4]u32,
    offsets: [4]u32,
};
pub const drmModeFB2 = struct__drmModeFB2;
pub const drmModeFB2Ptr = [*c]struct__drmModeFB2;
pub const drmModeClip = struct_drm_clip_rect;
pub const drmModeClipPtr = [*c]struct_drm_clip_rect;
pub const struct__drmModePropertyBlob = extern struct {
    id: u32,
    length: u32,
    data: ?*anyopaque,
};
pub const drmModePropertyBlobRes = struct__drmModePropertyBlob;
pub const drmModePropertyBlobPtr = [*c]struct__drmModePropertyBlob;
pub const struct__drmModeProperty = extern struct {
    prop_id: u32,
    flags: u32,
    name: [32]u8,
    count_values: c_int,
    values: [*c]u64,
    count_enums: c_int,
    enums: [*c]struct_drm_mode_property_enum,
    count_blobs: c_int,
    blob_ids: [*c]u32,
};
pub const drmModePropertyRes = struct__drmModeProperty;
pub const drmModePropertyPtr = [*c]struct__drmModeProperty;
pub fn drmModeGetPropertyType(arg_prop: [*c]const drmModePropertyRes) callconv(.C) u32 {
    var prop = arg_prop;
    return prop.*.flags & @as(u32, @bitCast(((((@as(c_int, 1) << @intCast(1)) | (@as(c_int, 1) << @intCast(3))) | (@as(c_int, 1) << @intCast(4))) | (@as(c_int, 1) << @intCast(5))) | @as(c_int, 65472)));
}
pub fn drm_property_type_is(property: drmModePropertyPtr, arg_type: u32) callconv(.C) c_int {
    var @"type" = arg_type;
    return @intFromBool(drmModeGetPropertyType(property) == @"type");
}
pub const struct__drmModeCrtc = extern struct {
    crtc_id: u32,
    buffer_id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    mode_valid: c_int,
    mode: drmModeModeInfo,
    gamma_size: c_int,
};
pub const drmModeCrtc = struct__drmModeCrtc;
pub const drmModeCrtcPtr = [*c]struct__drmModeCrtc;
pub const struct__drmModeEncoder = extern struct {
    encoder_id: u32,
    encoder_type: u32,
    crtc_id: u32,
    possible_crtcs: u32,
    possible_clones: u32,
};
pub const drmModeEncoder = struct__drmModeEncoder;
pub const drmModeEncoderPtr = [*c]struct__drmModeEncoder;
pub const DRM_MODE_CONNECTED: c_int = 1;
pub const DRM_MODE_DISCONNECTED: c_int = 2;
pub const DRM_MODE_UNKNOWNCONNECTION: c_int = 3;
pub const drmModeConnection = c_uint;
pub const DRM_MODE_SUBPIXEL_UNKNOWN: c_int = 1;
pub const DRM_MODE_SUBPIXEL_HORIZONTAL_RGB: c_int = 2;
pub const DRM_MODE_SUBPIXEL_HORIZONTAL_BGR: c_int = 3;
pub const DRM_MODE_SUBPIXEL_VERTICAL_RGB: c_int = 4;
pub const DRM_MODE_SUBPIXEL_VERTICAL_BGR: c_int = 5;
pub const DRM_MODE_SUBPIXEL_NONE: c_int = 6;
pub const drmModeSubPixel = c_uint;
pub const struct__drmModeConnector = extern struct {
    connector_id: u32,
    encoder_id: u32,
    connector_type: u32,
    connector_type_id: u32,
    connection: drmModeConnection,
    mmWidth: u32,
    mmHeight: u32,
    subpixel: drmModeSubPixel,
    count_modes: c_int,
    modes: drmModeModeInfoPtr,
    count_props: c_int,
    props: [*c]u32,
    prop_values: [*c]u64,
    count_encoders: c_int,
    encoders: [*c]u32,
};
pub const drmModeConnector = struct__drmModeConnector;
pub const drmModeConnectorPtr = [*c]struct__drmModeConnector;
pub const struct__drmModeObjectProperties = extern struct {
    count_props: u32,
    props: [*c]u32,
    prop_values: [*c]u64,
};
pub const drmModeObjectProperties = struct__drmModeObjectProperties;
pub const drmModeObjectPropertiesPtr = [*c]struct__drmModeObjectProperties;
pub const struct__drmModeFormatModifierIterator = extern struct {
    fmt_idx: u32,
    mod_idx: u32,
    fmt: u32,
    mod: u64,
};
pub const drmModeFormatModifierIterator = struct__drmModeFormatModifierIterator;
pub const struct__drmModePlane = extern struct {
    count_formats: u32,
    formats: [*c]u32,
    plane_id: u32,
    crtc_id: u32,
    fb_id: u32,
    crtc_x: u32,
    crtc_y: u32,
    x: u32,
    y: u32,
    possible_crtcs: u32,
    gamma_size: u32,
};
pub const drmModePlane = struct__drmModePlane;
pub const drmModePlanePtr = [*c]struct__drmModePlane;
pub const struct__drmModePlaneRes = extern struct {
    count_planes: u32,
    planes: [*c]u32,
};
pub const drmModePlaneRes = struct__drmModePlaneRes;
pub const drmModePlaneResPtr = [*c]struct__drmModePlaneRes;
pub extern fn drmModeFreeModeInfo(ptr: drmModeModeInfoPtr) void;
pub extern fn drmModeFreeResources(ptr: drmModeResPtr) void;
pub extern fn drmModeFreeFB(ptr: drmModeFBPtr) void;
pub extern fn drmModeFreeFB2(ptr: drmModeFB2Ptr) void;
pub extern fn drmModeFreeCrtc(ptr: drmModeCrtcPtr) void;
pub extern fn drmModeFreeConnector(ptr: drmModeConnectorPtr) void;
pub extern fn drmModeFreeEncoder(ptr: drmModeEncoderPtr) void;
pub extern fn drmModeFreePlane(ptr: drmModePlanePtr) void;
pub extern fn drmModeFreePlaneResources(ptr: drmModePlaneResPtr) void;
pub extern fn drmIsKMS(fd: c_int) c_int;
pub extern fn drmModeGetResources(fd: c_int) drmModeResPtr;
pub extern fn drmModeGetFB(fd: c_int, bufferId: u32) drmModeFBPtr;
pub extern fn drmModeGetFB2(fd: c_int, bufferId: u32) drmModeFB2Ptr;
pub extern fn drmModeAddFB(fd: c_int, width: u32, height: u32, depth: u8, bpp: u8, pitch: u32, bo_handle: u32, buf_id: [*c]u32) c_int;
pub extern fn drmModeAddFB2(fd: c_int, width: u32, height: u32, pixel_format: u32, bo_handles: [*c]const u32, pitches: [*c]const u32, offsets: [*c]const u32, buf_id: [*c]u32, flags: u32) c_int;
pub extern fn drmModeAddFB2WithModifiers(fd: c_int, width: u32, height: u32, pixel_format: u32, bo_handles: [*c]const u32, pitches: [*c]const u32, offsets: [*c]const u32, modifier: [*c]const u64, buf_id: [*c]u32, flags: u32) c_int;
pub extern fn drmModeRmFB(fd: c_int, bufferId: u32) c_int;
pub extern fn drmModeCloseFB(fd: c_int, buffer_id: u32) c_int;
pub extern fn drmModeDirtyFB(fd: c_int, bufferId: u32, clips: drmModeClipPtr, num_clips: u32) c_int;
pub extern fn drmModeGetCrtc(fd: c_int, crtcId: u32) drmModeCrtcPtr;
pub extern fn drmModeSetCrtc(fd: c_int, crtcId: u32, bufferId: u32, x: u32, y: u32, connectors: [*c]u32, count: c_int, mode: drmModeModeInfoPtr) c_int;
pub extern fn drmModeSetCursor(fd: c_int, crtcId: u32, bo_handle: u32, width: u32, height: u32) c_int;
pub extern fn drmModeSetCursor2(fd: c_int, crtcId: u32, bo_handle: u32, width: u32, height: u32, hot_x: i32, hot_y: i32) c_int;
pub extern fn drmModeMoveCursor(fd: c_int, crtcId: u32, x: c_int, y: c_int) c_int;
pub extern fn drmModeGetEncoder(fd: c_int, encoder_id: u32) drmModeEncoderPtr;
pub extern fn drmModeGetConnector(fd: c_int, connectorId: u32) drmModeConnectorPtr;
pub extern fn drmModeGetConnectorCurrent(fd: c_int, connector_id: u32) drmModeConnectorPtr;
pub extern fn drmModeConnectorGetPossibleCrtcs(fd: c_int, connector: [*c]const drmModeConnector) u32;
pub extern fn drmModeAttachMode(fd: c_int, connectorId: u32, mode_info: drmModeModeInfoPtr) c_int;
pub extern fn drmModeDetachMode(fd: c_int, connectorId: u32, mode_info: drmModeModeInfoPtr) c_int;
pub extern fn drmModeGetProperty(fd: c_int, propertyId: u32) drmModePropertyPtr;
pub extern fn drmModeFreeProperty(ptr: drmModePropertyPtr) void;
pub extern fn drmModeGetPropertyBlob(fd: c_int, blob_id: u32) drmModePropertyBlobPtr;
pub extern fn drmModeFormatModifierBlobIterNext(blob: [*c]const drmModePropertyBlobRes, iter: [*c]drmModeFormatModifierIterator) bool;
pub extern fn drmModeFreePropertyBlob(ptr: drmModePropertyBlobPtr) void;
pub extern fn drmModeConnectorSetProperty(fd: c_int, connector_id: u32, property_id: u32, value: u64) c_int;
pub extern fn drmCheckModesettingSupported(busid: [*c]const u8) c_int;
pub extern fn drmModeCrtcSetGamma(fd: c_int, crtc_id: u32, size: u32, red: [*c]const u16, green: [*c]const u16, blue: [*c]const u16) c_int;
pub extern fn drmModeCrtcGetGamma(fd: c_int, crtc_id: u32, size: u32, red: [*c]u16, green: [*c]u16, blue: [*c]u16) c_int;
pub extern fn drmModePageFlip(fd: c_int, crtc_id: u32, fb_id: u32, flags: u32, user_data: ?*anyopaque) c_int;
pub extern fn drmModePageFlipTarget(fd: c_int, crtc_id: u32, fb_id: u32, flags: u32, user_data: ?*anyopaque, target_vblank: u32) c_int;
pub extern fn drmModeGetPlaneResources(fd: c_int) drmModePlaneResPtr;
pub extern fn drmModeGetPlane(fd: c_int, plane_id: u32) drmModePlanePtr;
pub extern fn drmModeSetPlane(fd: c_int, plane_id: u32, crtc_id: u32, fb_id: u32, flags: u32, crtc_x: i32, crtc_y: i32, crtc_w: u32, crtc_h: u32, src_x: u32, src_y: u32, src_w: u32, src_h: u32) c_int;
pub extern fn drmModeObjectGetProperties(fd: c_int, object_id: u32, object_type: u32) drmModeObjectPropertiesPtr;
pub extern fn drmModeFreeObjectProperties(ptr: drmModeObjectPropertiesPtr) void;
pub extern fn drmModeObjectSetProperty(fd: c_int, object_id: u32, object_type: u32, property_id: u32, value: u64) c_int;
pub const struct__drmModeAtomicReq = opaque {};
pub const drmModeAtomicReq = struct__drmModeAtomicReq;
pub const drmModeAtomicReqPtr = ?*struct__drmModeAtomicReq;
pub extern fn drmModeAtomicAlloc() drmModeAtomicReqPtr;
pub extern fn drmModeAtomicDuplicate(req: drmModeAtomicReqPtr) drmModeAtomicReqPtr;
pub extern fn drmModeAtomicMerge(base: drmModeAtomicReqPtr, augment: drmModeAtomicReqPtr) c_int;
pub extern fn drmModeAtomicFree(req: drmModeAtomicReqPtr) void;
pub extern fn drmModeAtomicGetCursor(req: drmModeAtomicReqPtr) c_int;
pub extern fn drmModeAtomicSetCursor(req: drmModeAtomicReqPtr, cursor: c_int) void;
pub extern fn drmModeAtomicAddProperty(req: drmModeAtomicReqPtr, object_id: u32, property_id: u32, value: u64) c_int;
pub extern fn drmModeAtomicCommit(fd: c_int, req: drmModeAtomicReqPtr, flags: u32, user_data: ?*anyopaque) c_int;
pub extern fn drmModeCreatePropertyBlob(fd: c_int, data: ?*const anyopaque, size: usize, id: [*c]u32) c_int;
pub extern fn drmModeDestroyPropertyBlob(fd: c_int, id: u32) c_int;
pub extern fn drmModeCreateLease(fd: c_int, objects: [*c]const u32, num_objects: c_int, flags: c_int, lessee_id: [*c]u32) c_int;
pub const struct_drmModeLesseeList = extern struct {
    count: u32 align(4),
    pub fn lessees(self: anytype) @import("std").zig.c_translation.FlexibleArrayType(@TypeOf(self), u32) {
        const Intermediate = @import("std").zig.c_translation.FlexibleArrayType(@TypeOf(self), u8);
        const ReturnType = @import("std").zig.c_translation.FlexibleArrayType(@TypeOf(self), u32);
        return @as(ReturnType, @ptrCast(@alignCast(@as(Intermediate, @ptrCast(self)) + 4)));
    }
};
pub const drmModeLesseeListRes = struct_drmModeLesseeList;
pub const drmModeLesseeListPtr = [*c]struct_drmModeLesseeList;
pub extern fn drmModeListLessees(fd: c_int) drmModeLesseeListPtr;
pub const struct_drmModeObjectList = extern struct {
    count: u32 align(4),
    pub fn objects(self: anytype) @import("std").zig.c_translation.FlexibleArrayType(@TypeOf(self), u32) {
        const Intermediate = @import("std").zig.c_translation.FlexibleArrayType(@TypeOf(self), u8);
        const ReturnType = @import("std").zig.c_translation.FlexibleArrayType(@TypeOf(self), u32);
        return @as(ReturnType, @ptrCast(@alignCast(@as(Intermediate, @ptrCast(self)) + 4)));
    }
};
pub const drmModeObjectListRes = struct_drmModeObjectList;
pub const drmModeObjectListPtr = [*c]struct_drmModeObjectList;
pub extern fn drmModeGetLease(fd: c_int) drmModeObjectListPtr;
pub extern fn drmModeRevokeLease(fd: c_int, lessee_id: u32) c_int;
pub extern fn drmModeGetConnectorTypeName(connector_type: u32) [*c]const u8;
pub extern fn drmModeCreateDumbBuffer(fd: c_int, width: u32, height: u32, bpp: u32, flags: u32, handle: [*c]u32, pitch: [*c]u32, size: [*c]u64) c_int;
pub extern fn drmModeDestroyDumbBuffer(fd: c_int, handle: u32) c_int;
pub extern fn drmModeMapDumbBuffer(fd: c_int, handle: u32, offset: [*c]u64) c_int;
pub export var crtc: [*c]drmModeCrtc = null;
pub export var mode: drmModeModeInfo = drmModeModeInfo{
    .clock = @as(u32, @bitCast(@as(c_int, 0))),
    .hdisplay = @import("std").mem.zeroes(u16),
    .hsync_start = @import("std").mem.zeroes(u16),
    .hsync_end = @import("std").mem.zeroes(u16),
    .htotal = @import("std").mem.zeroes(u16),
    .hskew = @import("std").mem.zeroes(u16),
    .vdisplay = @import("std").mem.zeroes(u16),
    .vsync_start = @import("std").mem.zeroes(u16),
    .vsync_end = @import("std").mem.zeroes(u16),
    .vtotal = @import("std").mem.zeroes(u16),
    .vscan = @import("std").mem.zeroes(u16),
    .vrefresh = @import("std").mem.zeroes(u32),
    .flags = @import("std").mem.zeroes(u32),
    .type = @import("std").mem.zeroes(u32),
    .name = @import("std").mem.zeroes([32]u8),
};
pub export var plane: [*c]drmModePlane = null;
pub export fn get_property_value(arg_drm_fd: c_int, arg_object_id: u32, arg_object_type: u32, arg_prop_name: [*c]const u8) u64 {
    var drm_fd = arg_drm_fd;
    var object_id = arg_object_id;
    var object_type = arg_object_type;
    var prop_name = arg_prop_name;
    var props: [*c]drmModeObjectProperties = drmModeObjectGetProperties(drm_fd, object_id, object_type);
    {
        var i: u32 = 0;
        while (i < props.*.count_props) : (i +%= 1) {
            var prop: [*c]drmModePropertyRes = drmModeGetProperty(drm_fd, props.*.props[i]);
            var val: u64 = props.*.prop_values[i];
            if (strcmp(@as([*c]u8, @ptrCast(@alignCast(&prop.*.name))), prop_name) == @as(c_int, 0)) {
                drmModeFreeProperty(prop);
                drmModeFreeObjectProperties(props);
                return val;
            }
            drmModeFreeProperty(prop);
        }
    }
    abort();
    return @import("std").mem.zeroes(u64);
}
pub export fn main(arg_argc: c_int, arg_argv: [*c][*c]u8) c_int {
    var argc = arg_argc;
    _ = @TypeOf(argc);
    var argv = arg_argv;
    _ = @TypeOf(argv);
    var drm_fd: c_int = open("/dev/dri/card0", @as(c_int, 2) | @as(c_int, 2048));
    if (drm_fd < @as(c_int, 0)) {
        perror("open failed");
        return 1;
    }
    if (drmSetClientCap(drm_fd, @as(u64, @bitCast(@as(c_long, @as(c_int, 2)))), @as(u64, @bitCast(@as(c_long, @as(c_int, 1))))) != @as(c_int, 0)) {
        perror("drmSetClientCap(UNIVERSAL_PLANES) failed");
        return 1;
    }
    var resources: [*c]drmModeRes = drmModeGetResources(drm_fd);
    {
        var i: c_int = 0;
        while (i < resources.*.count_crtcs) : (i += 1) {
            var crtc_id: u32 = (blk: {
                const tmp = i;
                if (tmp >= 0) break :blk resources.*.crtcs + @as(usize, @intCast(tmp)) else break :blk resources.*.crtcs - ~@as(usize, @bitCast(@as(isize, @intCast(tmp)) +% -1));
            }).*;
            crtc = drmModeGetCrtc(drm_fd, crtc_id);
            if (crtc.*.mode_valid != 0) {
                break;
            }
            drmModeFreeCrtc(crtc);
            crtc = null;
        }
    }
    _ = blk: {
        _ = @sizeOf(c_int);
        break :blk blk_1: {
            break :blk_1 if (crtc != @as([*c]drmModeCrtc, @ptrCast(@alignCast(@as(?*anyopaque, @ptrFromInt(@as(c_int, 0))))))) {} else {
                __assert_fail("crtc != NULL", "02-selectres.c", @as(c_uint, @bitCast(@as(c_int, 63))), "int main(int, char **)");
            };
        };
    };
    _ = printf("Using CRTC %u\n", crtc.*.crtc_id);
    mode = crtc.*.mode;
    _ = printf("Using mode %dx%d %dHz\n", @as(c_int, @bitCast(@as(c_uint, mode.hdisplay))), @as(c_int, @bitCast(@as(c_uint, mode.vdisplay))), mode.vrefresh);
    var planes: [*c]drmModePlaneRes = drmModeGetPlaneResources(drm_fd);
    {
        var i: u32 = 0;
        while (i < planes.*.count_planes) : (i +%= 1) {
            var plane_id: u32 = planes.*.planes[i];
            plane = drmModeGetPlane(drm_fd, plane_id);
            var plane_type: u64 = get_property_value(drm_fd, plane_id, @as(c_uint, 4008636142), "type");
            if ((plane.*.crtc_id == crtc.*.crtc_id) and (plane_type == @as(u64, @bitCast(@as(c_long, @as(c_int, 1)))))) {
                break;
            }
            drmModeFreePlane(plane);
            plane = null;
        }
    }
    _ = blk: {
        _ = @sizeOf(c_int);
        break :blk blk_1: {
            break :blk_1 if (plane != @as([*c]drmModePlane, @ptrCast(@alignCast(@as(?*anyopaque, @ptrFromInt(@as(c_int, 0))))))) {} else {
                __assert_fail("plane != NULL", "02-selectres.c", @as(c_uint, @bitCast(@as(c_int, 83))), "int main(int, char **)");
            };
        };
    };
    _ = printf("Using plane %u\n", plane.*.plane_id);
    drmModeFreePlaneResources(planes);
    drmModeFreeResources(resources);
    return 0;
}
pub const __INTMAX_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `L`"); // (no file):80:9
pub const __UINTMAX_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `UL`"); // (no file):86:9
pub const __FLT16_DENORM_MIN__ = @compileError("unable to translate C expr: unexpected token 'IntegerLiteral'"); // (no file):109:9
pub const __FLT16_EPSILON__ = @compileError("unable to translate C expr: unexpected token 'IntegerLiteral'"); // (no file):113:9
pub const __FLT16_MAX__ = @compileError("unable to translate C expr: unexpected token 'IntegerLiteral'"); // (no file):119:9
pub const __FLT16_MIN__ = @compileError("unable to translate C expr: unexpected token 'IntegerLiteral'"); // (no file):122:9
pub const __INT64_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `L`"); // (no file):183:9
pub const __UINT32_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `U`"); // (no file):205:9
pub const __UINT64_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `UL`"); // (no file):213:9
pub const __seg_gs = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // (no file):342:9
pub const __seg_fs = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // (no file):343:9
pub const __GLIBC_USE = @compileError("unable to translate macro: undefined identifier `__GLIBC_USE_`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/features.h:188:9
pub const __glibc_has_attribute = @compileError("unable to translate macro: undefined identifier `__has_attribute`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:45:10
pub const __glibc_has_extension = @compileError("unable to translate macro: undefined identifier `__has_extension`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:55:10
pub const __THROW = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:79:11
pub const __THROWNL = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:80:11
pub const __NTH = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:81:11
pub const __NTHNL = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:82:11
pub const __COLD = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:102:11
pub const __CONCAT = @compileError("unable to translate C expr: unexpected token '##'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:131:9
pub const __STRING = @compileError("unable to translate C expr: unexpected token '#'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:132:9
pub const __warnattr = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:216:10
pub const __errordecl = @compileError("unable to translate C expr: unexpected token 'extern'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:217:10
pub const __flexarr = @compileError("unable to translate C expr: unexpected token '['"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:225:10
pub const __REDIRECT = @compileError("unable to translate macro: undefined identifier `__asm__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:256:10
pub const __REDIRECT_NTH = @compileError("unable to translate macro: undefined identifier `__asm__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:263:11
pub const __REDIRECT_NTHNL = @compileError("unable to translate macro: undefined identifier `__asm__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:265:11
pub const __ASMNAME2 = @compileError("unable to translate C expr: unexpected token 'Identifier'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:269:10
pub const __attribute_malloc__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:298:10
pub const __attribute_alloc_size__ = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:309:10
pub const __attribute_alloc_align__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:315:10
pub const __attribute_pure__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:325:10
pub const __attribute_const__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:332:10
pub const __attribute_maybe_unused__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:338:10
pub const __attribute_used__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:347:10
pub const __attribute_noinline__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:348:10
pub const __attribute_deprecated__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:356:10
pub const __attribute_deprecated_msg__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:366:10
pub const __attribute_format_arg__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:379:10
pub const __attribute_format_strfmon__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:389:10
pub const __attribute_nonnull__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:401:11
pub const __returns_nonnull = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:414:10
pub const __attribute_warn_unused_result__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:423:10
pub const __always_inline = @compileError("unable to translate macro: undefined identifier `__inline`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:441:10
pub const __attribute_artificial__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:450:10
pub const __extern_inline = @compileError("unable to translate macro: undefined identifier `__inline`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:468:11
pub const __extern_always_inline = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:469:11
pub const __restrict_arr = @compileError("unable to translate macro: undefined identifier `__restrict`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:512:10
pub const __attribute_copy__ = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:561:10
pub const __LDBL_REDIR2_DECL = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:638:10
pub const __LDBL_REDIR_DECL = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:639:10
pub const __glibc_macro_warning1 = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:653:10
pub const __glibc_macro_warning = @compileError("unable to translate macro: undefined identifier `GCC`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:654:10
pub const __fortified_attr_access = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:699:11
pub const __attr_access = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:700:11
pub const __attr_access_none = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:701:11
pub const __attr_dealloc = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:711:10
pub const __attribute_returns_twice__ = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/sys/cdefs.h:718:10
pub const __ASSERT_VOID_CAST = @compileError("unable to translate C expr: unexpected token 'Eof'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/assert.h:40:10
pub const assert = @compileError("unable to translate macro: undefined identifier `__extension__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/assert.h:115:11
pub const __ASSERT_FUNCTION = @compileError("unable to translate macro: undefined identifier `__extension__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/assert.h:137:12
pub const static_assert = @compileError("unable to translate C expr: unexpected token '_Static_assert'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/assert.h:155:10
pub const __STD_TYPE = @compileError("unable to translate C expr: unexpected token 'typedef'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/types.h:137:10
pub const __FSID_T_TYPE = @compileError("unable to translate macro: undefined identifier `__val`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/typesizes.h:73:9
pub const st_atime = @compileError("unable to translate macro: undefined identifier `st_atim`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/struct_stat.h:77:11
pub const st_mtime = @compileError("unable to translate macro: undefined identifier `st_mtim`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/struct_stat.h:78:11
pub const st_ctime = @compileError("unable to translate macro: undefined identifier `st_ctim`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/struct_stat.h:79:11
pub const __getc_unlocked_body = @compileError("TODO postfix inc/dec expr"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/types/struct_FILE.h:102:9
pub const __putc_unlocked_body = @compileError("TODO postfix inc/dec expr"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/types/struct_FILE.h:106:9
pub const __CFLOAT32 = @compileError("unable to translate: TODO _Complex"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:149:12
pub const __CFLOAT64 = @compileError("unable to translate: TODO _Complex"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:160:13
pub const __CFLOAT32X = @compileError("unable to translate: TODO _Complex"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:169:12
pub const __CFLOAT64X = @compileError("unable to translate: TODO _Complex"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:178:13
pub const __builtin_nansf32 = @compileError("unable to translate macro: undefined identifier `__builtin_nansf`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:221:12
pub const __builtin_huge_valf64 = @compileError("unable to translate macro: undefined identifier `__builtin_huge_val`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:255:13
pub const __builtin_inff64 = @compileError("unable to translate macro: undefined identifier `__builtin_inf`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:256:13
pub const __builtin_nanf64 = @compileError("unable to translate macro: undefined identifier `__builtin_nan`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:257:13
pub const __builtin_nansf64 = @compileError("unable to translate macro: undefined identifier `__builtin_nans`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:258:13
pub const __builtin_huge_valf32x = @compileError("unable to translate macro: undefined identifier `__builtin_huge_val`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:272:12
pub const __builtin_inff32x = @compileError("unable to translate macro: undefined identifier `__builtin_inf`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:273:12
pub const __builtin_nanf32x = @compileError("unable to translate macro: undefined identifier `__builtin_nan`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:274:12
pub const __builtin_nansf32x = @compileError("unable to translate macro: undefined identifier `__builtin_nans`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:275:12
pub const __builtin_huge_valf64x = @compileError("unable to translate macro: undefined identifier `__builtin_huge_vall`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:289:13
pub const __builtin_inff64x = @compileError("unable to translate macro: undefined identifier `__builtin_infl`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:290:13
pub const __builtin_nanf64x = @compileError("unable to translate macro: undefined identifier `__builtin_nanl`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:291:13
pub const __builtin_nansf64x = @compileError("unable to translate macro: undefined identifier `__builtin_nansl`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/floatn-common.h:292:13
pub const __FD_ZERO = @compileError("unable to translate macro: undefined identifier `__i`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/select.h:25:9
pub const __FD_SET = @compileError("unable to translate C expr: expected ')' instead got '|='"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/select.h:32:9
pub const __FD_CLR = @compileError("unable to translate C expr: expected ')' instead got '&='"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/select.h:34:9
pub const __PTHREAD_MUTEX_INITIALIZER = @compileError("unable to translate C expr: unexpected token '{'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/struct_mutex.h:56:10
pub const __PTHREAD_RWLOCK_ELISION_EXTRA = @compileError("unable to translate C expr: unexpected token '{'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/struct_rwlock.h:40:11
pub const __ONCE_FLAG_INIT = @compileError("unable to translate C expr: unexpected token '{'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/bits/thread-shared-types.h:113:9
pub const va_start = @compileError("unable to translate macro: undefined identifier `__builtin_va_start`"); // /nix/store/19sirdxr263wp244wgp51vb291br0f9k-zig-0.11.0/lib/zig/include/stdarg.h:33:9
pub const va_end = @compileError("unable to translate macro: undefined identifier `__builtin_va_end`"); // /nix/store/19sirdxr263wp244wgp51vb291br0f9k-zig-0.11.0/lib/zig/include/stdarg.h:35:9
pub const va_arg = @compileError("unable to translate macro: undefined identifier `__builtin_va_arg`"); // /nix/store/19sirdxr263wp244wgp51vb291br0f9k-zig-0.11.0/lib/zig/include/stdarg.h:36:9
pub const __va_copy = @compileError("unable to translate macro: undefined identifier `__builtin_va_copy`"); // /nix/store/19sirdxr263wp244wgp51vb291br0f9k-zig-0.11.0/lib/zig/include/stdarg.h:41:9
pub const va_copy = @compileError("unable to translate macro: undefined identifier `__builtin_va_copy`"); // /nix/store/19sirdxr263wp244wgp51vb291br0f9k-zig-0.11.0/lib/zig/include/stdarg.h:46:9
pub const __struct_group = @compileError("unable to translate C expr: expected ')' instead got '...'"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/linux/stddef.h:26:9
pub const __DECLARE_FLEX_ARRAY = @compileError("unable to translate macro: undefined identifier `__empty_`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/linux/stddef.h:42:9
pub const __aligned_u64 = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/linux/types.h:50:9
pub const __aligned_be64 = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/linux/types.h:51:9
pub const __aligned_le64 = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/zzwrkd2n3blr2m5ydm6236kbisvldj73-glibc-2.38-44-dev/include/linux/types.h:52:9
pub const DRM_NODE_NAME_MAX = @compileError("unable to translate macro: undefined identifier `MAX3`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:96:9
pub const DRM_PRINTFLIKE = @compileError("unable to translate macro: undefined identifier `__attribute__`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:116:9
pub const __drm_dummy_lock = @compileError("unable to translate macro: undefined identifier `__volatile__`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:364:9
pub const DRM_CAS = @compileError("unable to translate macro: undefined identifier `__dummy`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:372:9
pub const DRM_LIGHT_LOCK = @compileError("unable to translate macro: undefined identifier `__ret`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:522:9
pub const DRM_LIGHT_LOCK_COUNT = @compileError("unable to translate macro: undefined identifier `__ret`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:531:9
pub const DRM_LOCK = @compileError("unable to translate C expr: unexpected token 'do'"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:539:9
pub const DRM_UNLOCK = @compileError("unable to translate macro: undefined identifier `__ret`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:545:9
pub const DRM_SPINLOCK = @compileError("unable to translate macro: undefined identifier `__ret`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:553:9
pub const DRM_SPINLOCK_TAKE = @compileError("unable to translate macro: undefined identifier `__ret`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:562:9
pub const DRM_SPINLOCK_COUNT = @compileError("unable to translate macro: undefined identifier `__i`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:572:9
pub const DRM_SPINUNLOCK = @compileError("unable to translate macro: undefined identifier `__ret`"); // /nix/store/shnla8vp4dx7ams238w32pkxpi702r84-libdrm-2.4.118-dev/include/xf86drm.h:582:9
pub const offsetof = @compileError("unable to translate macro: undefined identifier `__builtin_offsetof`"); // /nix/store/19sirdxr263wp244wgp51vb291br0f9k-zig-0.11.0/lib/zig/include/stddef.h:111:9
pub const __llvm__ = @as(c_int, 1);
pub const __clang__ = @as(c_int, 1);
pub const __clang_major__ = @as(c_int, 16);
pub const __clang_minor__ = @as(c_int, 0);
pub const __clang_patchlevel__ = @as(c_int, 6);
pub const __clang_version__ = "16.0.6 ";
pub const __GNUC__ = @as(c_int, 4);
pub const __GNUC_MINOR__ = @as(c_int, 2);
pub const __GNUC_PATCHLEVEL__ = @as(c_int, 1);
pub const __GXX_ABI_VERSION = @as(c_int, 1002);
pub const __ATOMIC_RELAXED = @as(c_int, 0);
pub const __ATOMIC_CONSUME = @as(c_int, 1);
pub const __ATOMIC_ACQUIRE = @as(c_int, 2);
pub const __ATOMIC_RELEASE = @as(c_int, 3);
pub const __ATOMIC_ACQ_REL = @as(c_int, 4);
pub const __ATOMIC_SEQ_CST = @as(c_int, 5);
pub const __OPENCL_MEMORY_SCOPE_WORK_ITEM = @as(c_int, 0);
pub const __OPENCL_MEMORY_SCOPE_WORK_GROUP = @as(c_int, 1);
pub const __OPENCL_MEMORY_SCOPE_DEVICE = @as(c_int, 2);
pub const __OPENCL_MEMORY_SCOPE_ALL_SVM_DEVICES = @as(c_int, 3);
pub const __OPENCL_MEMORY_SCOPE_SUB_GROUP = @as(c_int, 4);
pub const __PRAGMA_REDEFINE_EXTNAME = @as(c_int, 1);
pub const __VERSION__ = "Clang 16.0.6";
pub const __OBJC_BOOL_IS_BOOL = @as(c_int, 0);
pub const __CONSTANT_CFSTRINGS__ = @as(c_int, 1);
pub const __clang_literal_encoding__ = "UTF-8";
pub const __clang_wide_literal_encoding__ = "UTF-32";
pub const __ORDER_LITTLE_ENDIAN__ = @as(c_int, 1234);
pub const __ORDER_BIG_ENDIAN__ = @as(c_int, 4321);
pub const __ORDER_PDP_ENDIAN__ = @as(c_int, 3412);
pub const __BYTE_ORDER__ = __ORDER_LITTLE_ENDIAN__;
pub const __LITTLE_ENDIAN__ = @as(c_int, 1);
pub const _LP64 = @as(c_int, 1);
pub const __LP64__ = @as(c_int, 1);
pub const __CHAR_BIT__ = @as(c_int, 8);
pub const __BOOL_WIDTH__ = @as(c_int, 8);
pub const __SHRT_WIDTH__ = @as(c_int, 16);
pub const __INT_WIDTH__ = @as(c_int, 32);
pub const __LONG_WIDTH__ = @as(c_int, 64);
pub const __LLONG_WIDTH__ = @as(c_int, 64);
pub const __BITINT_MAXWIDTH__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 8388608, .decimal);
pub const __SCHAR_MAX__ = @as(c_int, 127);
pub const __SHRT_MAX__ = @as(c_int, 32767);
pub const __INT_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __LONG_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __LONG_LONG_MAX__ = @as(c_longlong, 9223372036854775807);
pub const __WCHAR_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __WCHAR_WIDTH__ = @as(c_int, 32);
pub const __WINT_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const __WINT_WIDTH__ = @as(c_int, 32);
pub const __INTMAX_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __INTMAX_WIDTH__ = @as(c_int, 64);
pub const __SIZE_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __SIZE_WIDTH__ = @as(c_int, 64);
pub const __UINTMAX_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __UINTMAX_WIDTH__ = @as(c_int, 64);
pub const __PTRDIFF_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __PTRDIFF_WIDTH__ = @as(c_int, 64);
pub const __INTPTR_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __INTPTR_WIDTH__ = @as(c_int, 64);
pub const __UINTPTR_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __UINTPTR_WIDTH__ = @as(c_int, 64);
pub const __SIZEOF_DOUBLE__ = @as(c_int, 8);
pub const __SIZEOF_FLOAT__ = @as(c_int, 4);
pub const __SIZEOF_INT__ = @as(c_int, 4);
pub const __SIZEOF_LONG__ = @as(c_int, 8);
pub const __SIZEOF_LONG_DOUBLE__ = @as(c_int, 16);
pub const __SIZEOF_LONG_LONG__ = @as(c_int, 8);
pub const __SIZEOF_POINTER__ = @as(c_int, 8);
pub const __SIZEOF_SHORT__ = @as(c_int, 2);
pub const __SIZEOF_PTRDIFF_T__ = @as(c_int, 8);
pub const __SIZEOF_SIZE_T__ = @as(c_int, 8);
pub const __SIZEOF_WCHAR_T__ = @as(c_int, 4);
pub const __SIZEOF_WINT_T__ = @as(c_int, 4);
pub const __SIZEOF_INT128__ = @as(c_int, 16);
pub const __INTMAX_TYPE__ = c_long;
pub const __INTMAX_FMTd__ = "ld";
pub const __INTMAX_FMTi__ = "li";
pub const __UINTMAX_TYPE__ = c_ulong;
pub const __UINTMAX_FMTo__ = "lo";
pub const __UINTMAX_FMTu__ = "lu";
pub const __UINTMAX_FMTx__ = "lx";
pub const __UINTMAX_FMTX__ = "lX";
pub const __PTRDIFF_TYPE__ = c_long;
pub const __PTRDIFF_FMTd__ = "ld";
pub const __PTRDIFF_FMTi__ = "li";
pub const __INTPTR_TYPE__ = c_long;
pub const __INTPTR_FMTd__ = "ld";
pub const __INTPTR_FMTi__ = "li";
pub const __SIZE_TYPE__ = c_ulong;
pub const __SIZE_FMTo__ = "lo";
pub const __SIZE_FMTu__ = "lu";
pub const __SIZE_FMTx__ = "lx";
pub const __SIZE_FMTX__ = "lX";
pub const __WCHAR_TYPE__ = c_int;
pub const __WINT_TYPE__ = c_uint;
pub const __SIG_ATOMIC_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __SIG_ATOMIC_WIDTH__ = @as(c_int, 32);
pub const __CHAR16_TYPE__ = c_ushort;
pub const __CHAR32_TYPE__ = c_uint;
pub const __UINTPTR_TYPE__ = c_ulong;
pub const __UINTPTR_FMTo__ = "lo";
pub const __UINTPTR_FMTu__ = "lu";
pub const __UINTPTR_FMTx__ = "lx";
pub const __UINTPTR_FMTX__ = "lX";
pub const __FLT16_HAS_DENORM__ = @as(c_int, 1);
pub const __FLT16_DIG__ = @as(c_int, 3);
pub const __FLT16_DECIMAL_DIG__ = @as(c_int, 5);
pub const __FLT16_HAS_INFINITY__ = @as(c_int, 1);
pub const __FLT16_HAS_QUIET_NAN__ = @as(c_int, 1);
pub const __FLT16_MANT_DIG__ = @as(c_int, 11);
pub const __FLT16_MAX_10_EXP__ = @as(c_int, 4);
pub const __FLT16_MAX_EXP__ = @as(c_int, 16);
pub const __FLT16_MIN_10_EXP__ = -@as(c_int, 4);
pub const __FLT16_MIN_EXP__ = -@as(c_int, 13);
pub const __FLT_DENORM_MIN__ = @as(f32, 1.40129846e-45);
pub const __FLT_HAS_DENORM__ = @as(c_int, 1);
pub const __FLT_DIG__ = @as(c_int, 6);
pub const __FLT_DECIMAL_DIG__ = @as(c_int, 9);
pub const __FLT_EPSILON__ = @as(f32, 1.19209290e-7);
pub const __FLT_HAS_INFINITY__ = @as(c_int, 1);
pub const __FLT_HAS_QUIET_NAN__ = @as(c_int, 1);
pub const __FLT_MANT_DIG__ = @as(c_int, 24);
pub const __FLT_MAX_10_EXP__ = @as(c_int, 38);
pub const __FLT_MAX_EXP__ = @as(c_int, 128);
pub const __FLT_MAX__ = @as(f32, 3.40282347e+38);
pub const __FLT_MIN_10_EXP__ = -@as(c_int, 37);
pub const __FLT_MIN_EXP__ = -@as(c_int, 125);
pub const __FLT_MIN__ = @as(f32, 1.17549435e-38);
pub const __DBL_DENORM_MIN__ = @as(f64, 4.9406564584124654e-324);
pub const __DBL_HAS_DENORM__ = @as(c_int, 1);
pub const __DBL_DIG__ = @as(c_int, 15);
pub const __DBL_DECIMAL_DIG__ = @as(c_int, 17);
pub const __DBL_EPSILON__ = @as(f64, 2.2204460492503131e-16);
pub const __DBL_HAS_INFINITY__ = @as(c_int, 1);
pub const __DBL_HAS_QUIET_NAN__ = @as(c_int, 1);
pub const __DBL_MANT_DIG__ = @as(c_int, 53);
pub const __DBL_MAX_10_EXP__ = @as(c_int, 308);
pub const __DBL_MAX_EXP__ = @as(c_int, 1024);
pub const __DBL_MAX__ = @as(f64, 1.7976931348623157e+308);
pub const __DBL_MIN_10_EXP__ = -@as(c_int, 307);
pub const __DBL_MIN_EXP__ = -@as(c_int, 1021);
pub const __DBL_MIN__ = @as(f64, 2.2250738585072014e-308);
pub const __LDBL_DENORM_MIN__ = @as(c_longdouble, 3.64519953188247460253e-4951);
pub const __LDBL_HAS_DENORM__ = @as(c_int, 1);
pub const __LDBL_DIG__ = @as(c_int, 18);
pub const __LDBL_DECIMAL_DIG__ = @as(c_int, 21);
pub const __LDBL_EPSILON__ = @as(c_longdouble, 1.08420217248550443401e-19);
pub const __LDBL_HAS_INFINITY__ = @as(c_int, 1);
pub const __LDBL_HAS_QUIET_NAN__ = @as(c_int, 1);
pub const __LDBL_MANT_DIG__ = @as(c_int, 64);
pub const __LDBL_MAX_10_EXP__ = @as(c_int, 4932);
pub const __LDBL_MAX_EXP__ = @as(c_int, 16384);
pub const __LDBL_MAX__ = @as(c_longdouble, 1.18973149535723176502e+4932);
pub const __LDBL_MIN_10_EXP__ = -@as(c_int, 4931);
pub const __LDBL_MIN_EXP__ = -@as(c_int, 16381);
pub const __LDBL_MIN__ = @as(c_longdouble, 3.36210314311209350626e-4932);
pub const __POINTER_WIDTH__ = @as(c_int, 64);
pub const __BIGGEST_ALIGNMENT__ = @as(c_int, 16);
pub const __WINT_UNSIGNED__ = @as(c_int, 1);
pub const __INT8_TYPE__ = i8;
pub const __INT8_FMTd__ = "hhd";
pub const __INT8_FMTi__ = "hhi";
pub const __INT8_C_SUFFIX__ = "";
pub const __INT16_TYPE__ = c_short;
pub const __INT16_FMTd__ = "hd";
pub const __INT16_FMTi__ = "hi";
pub const __INT16_C_SUFFIX__ = "";
pub const __INT32_TYPE__ = c_int;
pub const __INT32_FMTd__ = "d";
pub const __INT32_FMTi__ = "i";
pub const __INT32_C_SUFFIX__ = "";
pub const __INT64_TYPE__ = c_long;
pub const __INT64_FMTd__ = "ld";
pub const __INT64_FMTi__ = "li";
pub const __UINT8_TYPE__ = u8;
pub const __UINT8_FMTo__ = "hho";
pub const __UINT8_FMTu__ = "hhu";
pub const __UINT8_FMTx__ = "hhx";
pub const __UINT8_FMTX__ = "hhX";
pub const __UINT8_C_SUFFIX__ = "";
pub const __UINT8_MAX__ = @as(c_int, 255);
pub const __INT8_MAX__ = @as(c_int, 127);
pub const __UINT16_TYPE__ = c_ushort;
pub const __UINT16_FMTo__ = "ho";
pub const __UINT16_FMTu__ = "hu";
pub const __UINT16_FMTx__ = "hx";
pub const __UINT16_FMTX__ = "hX";
pub const __UINT16_C_SUFFIX__ = "";
pub const __UINT16_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 65535, .decimal);
pub const __INT16_MAX__ = @as(c_int, 32767);
pub const __UINT32_TYPE__ = c_uint;
pub const __UINT32_FMTo__ = "o";
pub const __UINT32_FMTu__ = "u";
pub const __UINT32_FMTx__ = "x";
pub const __UINT32_FMTX__ = "X";
pub const __UINT32_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const __INT32_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __UINT64_TYPE__ = c_ulong;
pub const __UINT64_FMTo__ = "lo";
pub const __UINT64_FMTu__ = "lu";
pub const __UINT64_FMTx__ = "lx";
pub const __UINT64_FMTX__ = "lX";
pub const __UINT64_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __INT64_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __INT_LEAST8_TYPE__ = i8;
pub const __INT_LEAST8_MAX__ = @as(c_int, 127);
pub const __INT_LEAST8_WIDTH__ = @as(c_int, 8);
pub const __INT_LEAST8_FMTd__ = "hhd";
pub const __INT_LEAST8_FMTi__ = "hhi";
pub const __UINT_LEAST8_TYPE__ = u8;
pub const __UINT_LEAST8_MAX__ = @as(c_int, 255);
pub const __UINT_LEAST8_FMTo__ = "hho";
pub const __UINT_LEAST8_FMTu__ = "hhu";
pub const __UINT_LEAST8_FMTx__ = "hhx";
pub const __UINT_LEAST8_FMTX__ = "hhX";
pub const __INT_LEAST16_TYPE__ = c_short;
pub const __INT_LEAST16_MAX__ = @as(c_int, 32767);
pub const __INT_LEAST16_WIDTH__ = @as(c_int, 16);
pub const __INT_LEAST16_FMTd__ = "hd";
pub const __INT_LEAST16_FMTi__ = "hi";
pub const __UINT_LEAST16_TYPE__ = c_ushort;
pub const __UINT_LEAST16_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 65535, .decimal);
pub const __UINT_LEAST16_FMTo__ = "ho";
pub const __UINT_LEAST16_FMTu__ = "hu";
pub const __UINT_LEAST16_FMTx__ = "hx";
pub const __UINT_LEAST16_FMTX__ = "hX";
pub const __INT_LEAST32_TYPE__ = c_int;
pub const __INT_LEAST32_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __INT_LEAST32_WIDTH__ = @as(c_int, 32);
pub const __INT_LEAST32_FMTd__ = "d";
pub const __INT_LEAST32_FMTi__ = "i";
pub const __UINT_LEAST32_TYPE__ = c_uint;
pub const __UINT_LEAST32_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const __UINT_LEAST32_FMTo__ = "o";
pub const __UINT_LEAST32_FMTu__ = "u";
pub const __UINT_LEAST32_FMTx__ = "x";
pub const __UINT_LEAST32_FMTX__ = "X";
pub const __INT_LEAST64_TYPE__ = c_long;
pub const __INT_LEAST64_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __INT_LEAST64_WIDTH__ = @as(c_int, 64);
pub const __INT_LEAST64_FMTd__ = "ld";
pub const __INT_LEAST64_FMTi__ = "li";
pub const __UINT_LEAST64_TYPE__ = c_ulong;
pub const __UINT_LEAST64_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __UINT_LEAST64_FMTo__ = "lo";
pub const __UINT_LEAST64_FMTu__ = "lu";
pub const __UINT_LEAST64_FMTx__ = "lx";
pub const __UINT_LEAST64_FMTX__ = "lX";
pub const __INT_FAST8_TYPE__ = i8;
pub const __INT_FAST8_MAX__ = @as(c_int, 127);
pub const __INT_FAST8_WIDTH__ = @as(c_int, 8);
pub const __INT_FAST8_FMTd__ = "hhd";
pub const __INT_FAST8_FMTi__ = "hhi";
pub const __UINT_FAST8_TYPE__ = u8;
pub const __UINT_FAST8_MAX__ = @as(c_int, 255);
pub const __UINT_FAST8_FMTo__ = "hho";
pub const __UINT_FAST8_FMTu__ = "hhu";
pub const __UINT_FAST8_FMTx__ = "hhx";
pub const __UINT_FAST8_FMTX__ = "hhX";
pub const __INT_FAST16_TYPE__ = c_short;
pub const __INT_FAST16_MAX__ = @as(c_int, 32767);
pub const __INT_FAST16_WIDTH__ = @as(c_int, 16);
pub const __INT_FAST16_FMTd__ = "hd";
pub const __INT_FAST16_FMTi__ = "hi";
pub const __UINT_FAST16_TYPE__ = c_ushort;
pub const __UINT_FAST16_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 65535, .decimal);
pub const __UINT_FAST16_FMTo__ = "ho";
pub const __UINT_FAST16_FMTu__ = "hu";
pub const __UINT_FAST16_FMTx__ = "hx";
pub const __UINT_FAST16_FMTX__ = "hX";
pub const __INT_FAST32_TYPE__ = c_int;
pub const __INT_FAST32_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __INT_FAST32_WIDTH__ = @as(c_int, 32);
pub const __INT_FAST32_FMTd__ = "d";
pub const __INT_FAST32_FMTi__ = "i";
pub const __UINT_FAST32_TYPE__ = c_uint;
pub const __UINT_FAST32_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const __UINT_FAST32_FMTo__ = "o";
pub const __UINT_FAST32_FMTu__ = "u";
pub const __UINT_FAST32_FMTx__ = "x";
pub const __UINT_FAST32_FMTX__ = "X";
pub const __INT_FAST64_TYPE__ = c_long;
pub const __INT_FAST64_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __INT_FAST64_WIDTH__ = @as(c_int, 64);
pub const __INT_FAST64_FMTd__ = "ld";
pub const __INT_FAST64_FMTi__ = "li";
pub const __UINT_FAST64_TYPE__ = c_ulong;
pub const __UINT_FAST64_MAX__ = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __UINT_FAST64_FMTo__ = "lo";
pub const __UINT_FAST64_FMTu__ = "lu";
pub const __UINT_FAST64_FMTx__ = "lx";
pub const __UINT_FAST64_FMTX__ = "lX";
pub const __USER_LABEL_PREFIX__ = "";
pub const __FINITE_MATH_ONLY__ = @as(c_int, 0);
pub const __GNUC_STDC_INLINE__ = @as(c_int, 1);
pub const __GCC_ATOMIC_TEST_AND_SET_TRUEVAL = @as(c_int, 1);
pub const __CLANG_ATOMIC_BOOL_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_CHAR_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_CHAR16_T_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_CHAR32_T_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_WCHAR_T_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_SHORT_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_INT_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_LONG_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_LLONG_LOCK_FREE = @as(c_int, 2);
pub const __CLANG_ATOMIC_POINTER_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_BOOL_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_CHAR_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_CHAR16_T_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_CHAR32_T_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_WCHAR_T_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_SHORT_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_INT_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_LONG_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_LLONG_LOCK_FREE = @as(c_int, 2);
pub const __GCC_ATOMIC_POINTER_LOCK_FREE = @as(c_int, 2);
pub const __NO_INLINE__ = @as(c_int, 1);
pub const __PIC__ = @as(c_int, 2);
pub const __pic__ = @as(c_int, 2);
pub const __FLT_RADIX__ = @as(c_int, 2);
pub const __DECIMAL_DIG__ = __LDBL_DECIMAL_DIG__;
pub const __SSP_STRONG__ = @as(c_int, 2);
pub const __GCC_ASM_FLAG_OUTPUTS__ = @as(c_int, 1);
pub const __code_model_small__ = @as(c_int, 1);
pub const __amd64__ = @as(c_int, 1);
pub const __amd64 = @as(c_int, 1);
pub const __x86_64 = @as(c_int, 1);
pub const __x86_64__ = @as(c_int, 1);
pub const __SEG_GS = @as(c_int, 1);
pub const __SEG_FS = @as(c_int, 1);
pub const __k8 = @as(c_int, 1);
pub const __k8__ = @as(c_int, 1);
pub const __tune_k8__ = @as(c_int, 1);
pub const __REGISTER_PREFIX__ = "";
pub const __NO_MATH_INLINES = @as(c_int, 1);
pub const __AES__ = @as(c_int, 1);
pub const __VAES__ = @as(c_int, 1);
pub const __PCLMUL__ = @as(c_int, 1);
pub const __VPCLMULQDQ__ = @as(c_int, 1);
pub const __LAHF_SAHF__ = @as(c_int, 1);
pub const __LZCNT__ = @as(c_int, 1);
pub const __RDRND__ = @as(c_int, 1);
pub const __FSGSBASE__ = @as(c_int, 1);
pub const __BMI__ = @as(c_int, 1);
pub const __BMI2__ = @as(c_int, 1);
pub const __POPCNT__ = @as(c_int, 1);
pub const __PRFCHW__ = @as(c_int, 1);
pub const __RDSEED__ = @as(c_int, 1);
pub const __ADX__ = @as(c_int, 1);
pub const __MOVBE__ = @as(c_int, 1);
pub const __FMA__ = @as(c_int, 1);
pub const __F16C__ = @as(c_int, 1);
pub const __GFNI__ = @as(c_int, 1);
pub const __SHA__ = @as(c_int, 1);
pub const __FXSR__ = @as(c_int, 1);
pub const __XSAVE__ = @as(c_int, 1);
pub const __XSAVEOPT__ = @as(c_int, 1);
pub const __XSAVEC__ = @as(c_int, 1);
pub const __XSAVES__ = @as(c_int, 1);
pub const __PKU__ = @as(c_int, 1);
pub const __CLFLUSHOPT__ = @as(c_int, 1);
pub const __CLWB__ = @as(c_int, 1);
pub const __SHSTK__ = @as(c_int, 1);
pub const __RDPID__ = @as(c_int, 1);
pub const __WAITPKG__ = @as(c_int, 1);
pub const __MOVDIRI__ = @as(c_int, 1);
pub const __MOVDIR64B__ = @as(c_int, 1);
pub const __PTWRITE__ = @as(c_int, 1);
pub const __INVPCID__ = @as(c_int, 1);
pub const __AVX2__ = @as(c_int, 1);
pub const __AVX__ = @as(c_int, 1);
pub const __SSE4_2__ = @as(c_int, 1);
pub const __SSE4_1__ = @as(c_int, 1);
pub const __SSSE3__ = @as(c_int, 1);
pub const __SSE3__ = @as(c_int, 1);
pub const __SSE2__ = @as(c_int, 1);
pub const __SSE2_MATH__ = @as(c_int, 1);
pub const __SSE__ = @as(c_int, 1);
pub const __SSE_MATH__ = @as(c_int, 1);
pub const __MMX__ = @as(c_int, 1);
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_1 = @as(c_int, 1);
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_2 = @as(c_int, 1);
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_4 = @as(c_int, 1);
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_8 = @as(c_int, 1);
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_16 = @as(c_int, 1);
pub const __SIZEOF_FLOAT128__ = @as(c_int, 16);
pub const unix = @as(c_int, 1);
pub const __unix = @as(c_int, 1);
pub const __unix__ = @as(c_int, 1);
pub const linux = @as(c_int, 1);
pub const __linux = @as(c_int, 1);
pub const __linux__ = @as(c_int, 1);
pub const __ELF__ = @as(c_int, 1);
pub const __gnu_linux__ = @as(c_int, 1);
pub const __FLOAT128__ = @as(c_int, 1);
pub const __STDC__ = @as(c_int, 1);
pub const __STDC_HOSTED__ = @as(c_int, 1);
pub const __STDC_VERSION__ = @as(c_long, 201710);
pub const __STDC_UTF_16__ = @as(c_int, 1);
pub const __STDC_UTF_32__ = @as(c_int, 1);
pub const __GLIBC_MINOR__ = @as(c_int, 19);
pub const _DEBUG = @as(c_int, 1);
pub const __GCC_HAVE_DWARF2_CFI_ASM = @as(c_int, 1);
pub const _ASSERT_H = @as(c_int, 1);
pub const _FEATURES_H = @as(c_int, 1);
pub const __KERNEL_STRICT_NAMES = "";
pub inline fn __GNUC_PREREQ(maj: anytype, min: anytype) @TypeOf(((__GNUC__ << @as(c_int, 16)) + __GNUC_MINOR__) >= ((maj << @as(c_int, 16)) + min)) {
    return ((__GNUC__ << @as(c_int, 16)) + __GNUC_MINOR__) >= ((maj << @as(c_int, 16)) + min);
}
pub inline fn __glibc_clang_prereq(maj: anytype, min: anytype) @TypeOf(((__clang_major__ << @as(c_int, 16)) + __clang_minor__) >= ((maj << @as(c_int, 16)) + min)) {
    return ((__clang_major__ << @as(c_int, 16)) + __clang_minor__) >= ((maj << @as(c_int, 16)) + min);
}
pub const _DEFAULT_SOURCE = @as(c_int, 1);
pub const __GLIBC_USE_ISOC2X = @as(c_int, 0);
pub const __USE_ISOC11 = @as(c_int, 1);
pub const __USE_ISOC99 = @as(c_int, 1);
pub const __USE_ISOC95 = @as(c_int, 1);
pub const __USE_POSIX_IMPLICITLY = @as(c_int, 1);
pub const _POSIX_SOURCE = @as(c_int, 1);
pub const _POSIX_C_SOURCE = @as(c_long, 200809);
pub const __USE_POSIX = @as(c_int, 1);
pub const __USE_POSIX2 = @as(c_int, 1);
pub const __USE_POSIX199309 = @as(c_int, 1);
pub const __USE_POSIX199506 = @as(c_int, 1);
pub const __USE_XOPEN2K = @as(c_int, 1);
pub const __USE_XOPEN2K8 = @as(c_int, 1);
pub const _ATFILE_SOURCE = @as(c_int, 1);
pub const __WORDSIZE = @as(c_int, 64);
pub const __WORDSIZE_TIME64_COMPAT32 = @as(c_int, 1);
pub const __SYSCALL_WORDSIZE = @as(c_int, 64);
pub const __TIMESIZE = __WORDSIZE;
pub const __USE_MISC = @as(c_int, 1);
pub const __USE_ATFILE = @as(c_int, 1);
pub const __USE_FORTIFY_LEVEL = @as(c_int, 0);
pub const __GLIBC_USE_DEPRECATED_GETS = @as(c_int, 0);
pub const __GLIBC_USE_DEPRECATED_SCANF = @as(c_int, 0);
pub const __GLIBC_USE_C2X_STRTOL = @as(c_int, 0);
pub const _STDC_PREDEF_H = @as(c_int, 1);
pub const __STDC_IEC_559__ = @as(c_int, 1);
pub const __STDC_IEC_60559_BFP__ = @as(c_long, 201404);
pub const __STDC_IEC_559_COMPLEX__ = @as(c_int, 1);
pub const __STDC_IEC_60559_COMPLEX__ = @as(c_long, 201404);
pub const __STDC_ISO_10646__ = @as(c_long, 201706);
pub const __GNU_LIBRARY__ = @as(c_int, 6);
pub const __GLIBC__ = @as(c_int, 2);
pub inline fn __GLIBC_PREREQ(maj: anytype, min: anytype) @TypeOf(((__GLIBC__ << @as(c_int, 16)) + __GLIBC_MINOR__) >= ((maj << @as(c_int, 16)) + min)) {
    return ((__GLIBC__ << @as(c_int, 16)) + __GLIBC_MINOR__) >= ((maj << @as(c_int, 16)) + min);
}
pub const _SYS_CDEFS_H = @as(c_int, 1);
pub inline fn __glibc_has_builtin(name: anytype) @TypeOf(__has_builtin(name)) {
    return __has_builtin(name);
}
pub const __LEAF = "";
pub const __LEAF_ATTR = "";
pub inline fn __P(args: anytype) @TypeOf(args) {
    return args;
}
pub inline fn __PMT(args: anytype) @TypeOf(args) {
    return args;
}
pub const __ptr_t = ?*anyopaque;
pub const __BEGIN_DECLS = "";
pub const __END_DECLS = "";
pub inline fn __bos(ptr: anytype) @TypeOf(__builtin_object_size(ptr, __USE_FORTIFY_LEVEL > @as(c_int, 1))) {
    return __builtin_object_size(ptr, __USE_FORTIFY_LEVEL > @as(c_int, 1));
}
pub inline fn __bos0(ptr: anytype) @TypeOf(__builtin_object_size(ptr, @as(c_int, 0))) {
    return __builtin_object_size(ptr, @as(c_int, 0));
}
pub inline fn __glibc_objsize0(__o: anytype) @TypeOf(__bos0(__o)) {
    return __bos0(__o);
}
pub inline fn __glibc_objsize(__o: anytype) @TypeOf(__bos(__o)) {
    return __bos(__o);
}
pub const __glibc_c99_flexarr_available = @as(c_int, 1);
pub inline fn __ASMNAME(cname: anytype) @TypeOf(__ASMNAME2(__USER_LABEL_PREFIX__, cname)) {
    return __ASMNAME2(__USER_LABEL_PREFIX__, cname);
}
pub const __REDIRECT_FORTIFY = __REDIRECT;
pub const __REDIRECT_FORTIFY_NTH = __REDIRECT_NTH;
pub inline fn __nonnull(params: anytype) @TypeOf(__attribute_nonnull__(params)) {
    return __attribute_nonnull__(params);
}
pub const __wur = "";
pub const __fortify_function = __extern_always_inline ++ __attribute_artificial__;
pub inline fn __glibc_unlikely(cond: anytype) @TypeOf(__builtin_expect(cond, @as(c_int, 0))) {
    return __builtin_expect(cond, @as(c_int, 0));
}
pub inline fn __glibc_likely(cond: anytype) @TypeOf(__builtin_expect(cond, @as(c_int, 1))) {
    return __builtin_expect(cond, @as(c_int, 1));
}
pub const __attribute_nonstring__ = "";
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI = @as(c_int, 0);
pub inline fn __LDBL_REDIR1(name: anytype, proto: anytype, alias: anytype) @TypeOf(name ++ proto) {
    _ = @TypeOf(alias);
    return name ++ proto;
}
pub inline fn __LDBL_REDIR(name: anytype, proto: anytype) @TypeOf(name ++ proto) {
    return name ++ proto;
}
pub inline fn __LDBL_REDIR1_NTH(name: anytype, proto: anytype, alias: anytype) @TypeOf(name ++ proto ++ __THROW) {
    _ = @TypeOf(alias);
    return name ++ proto ++ __THROW;
}
pub inline fn __LDBL_REDIR_NTH(name: anytype, proto: anytype) @TypeOf(name ++ proto ++ __THROW) {
    return name ++ proto ++ __THROW;
}
pub inline fn __REDIRECT_LDBL(name: anytype, proto: anytype, alias: anytype) @TypeOf(__REDIRECT(name, proto, alias)) {
    return __REDIRECT(name, proto, alias);
}
pub inline fn __REDIRECT_NTH_LDBL(name: anytype, proto: anytype, alias: anytype) @TypeOf(__REDIRECT_NTH(name, proto, alias)) {
    return __REDIRECT_NTH(name, proto, alias);
}
pub const __HAVE_GENERIC_SELECTION = @as(c_int, 1);
pub const __attr_dealloc_free = "";
pub const __stub___compat_bdflush = "";
pub const __stub_chflags = "";
pub const __stub_fchflags = "";
pub const __stub_gtty = "";
pub const __stub_revoke = "";
pub const __stub_setlogin = "";
pub const __stub_sigreturn = "";
pub const __stub_stty = "";
pub const _FCNTL_H = @as(c_int, 1);
pub const _BITS_TYPES_H = @as(c_int, 1);
pub const __S16_TYPE = c_short;
pub const __U16_TYPE = c_ushort;
pub const __S32_TYPE = c_int;
pub const __U32_TYPE = c_uint;
pub const __SLONGWORD_TYPE = c_long;
pub const __ULONGWORD_TYPE = c_ulong;
pub const __SQUAD_TYPE = c_long;
pub const __UQUAD_TYPE = c_ulong;
pub const __SWORD_TYPE = c_long;
pub const __UWORD_TYPE = c_ulong;
pub const __SLONG32_TYPE = c_int;
pub const __ULONG32_TYPE = c_uint;
pub const __S64_TYPE = c_long;
pub const __U64_TYPE = c_ulong;
pub const _BITS_TYPESIZES_H = @as(c_int, 1);
pub const __SYSCALL_SLONG_TYPE = __SLONGWORD_TYPE;
pub const __SYSCALL_ULONG_TYPE = __ULONGWORD_TYPE;
pub const __DEV_T_TYPE = __UQUAD_TYPE;
pub const __UID_T_TYPE = __U32_TYPE;
pub const __GID_T_TYPE = __U32_TYPE;
pub const __INO_T_TYPE = __SYSCALL_ULONG_TYPE;
pub const __INO64_T_TYPE = __UQUAD_TYPE;
pub const __MODE_T_TYPE = __U32_TYPE;
pub const __NLINK_T_TYPE = __SYSCALL_ULONG_TYPE;
pub const __FSWORD_T_TYPE = __SYSCALL_SLONG_TYPE;
pub const __OFF_T_TYPE = __SYSCALL_SLONG_TYPE;
pub const __OFF64_T_TYPE = __SQUAD_TYPE;
pub const __PID_T_TYPE = __S32_TYPE;
pub const __RLIM_T_TYPE = __SYSCALL_ULONG_TYPE;
pub const __RLIM64_T_TYPE = __UQUAD_TYPE;
pub const __BLKCNT_T_TYPE = __SYSCALL_SLONG_TYPE;
pub const __BLKCNT64_T_TYPE = __SQUAD_TYPE;
pub const __FSBLKCNT_T_TYPE = __SYSCALL_ULONG_TYPE;
pub const __FSBLKCNT64_T_TYPE = __UQUAD_TYPE;
pub const __FSFILCNT_T_TYPE = __SYSCALL_ULONG_TYPE;
pub const __FSFILCNT64_T_TYPE = __UQUAD_TYPE;
pub const __ID_T_TYPE = __U32_TYPE;
pub const __CLOCK_T_TYPE = __SYSCALL_SLONG_TYPE;
pub const __TIME_T_TYPE = __SYSCALL_SLONG_TYPE;
pub const __USECONDS_T_TYPE = __U32_TYPE;
pub const __SUSECONDS_T_TYPE = __SYSCALL_SLONG_TYPE;
pub const __SUSECONDS64_T_TYPE = __SQUAD_TYPE;
pub const __DADDR_T_TYPE = __S32_TYPE;
pub const __KEY_T_TYPE = __S32_TYPE;
pub const __CLOCKID_T_TYPE = __S32_TYPE;
pub const __TIMER_T_TYPE = ?*anyopaque;
pub const __BLKSIZE_T_TYPE = __SYSCALL_SLONG_TYPE;
pub const __SSIZE_T_TYPE = __SWORD_TYPE;
pub const __CPU_MASK_TYPE = __SYSCALL_ULONG_TYPE;
pub const __OFF_T_MATCHES_OFF64_T = @as(c_int, 1);
pub const __INO_T_MATCHES_INO64_T = @as(c_int, 1);
pub const __RLIM_T_MATCHES_RLIM64_T = @as(c_int, 1);
pub const __STATFS_MATCHES_STATFS64 = @as(c_int, 1);
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64 = @as(c_int, 1);
pub const __FD_SETSIZE = @as(c_int, 1024);
pub const _BITS_TIME64_H = @as(c_int, 1);
pub const __TIME64_T_TYPE = __TIME_T_TYPE;
pub const __O_LARGEFILE = @as(c_int, 0);
pub const F_GETLK64 = @as(c_int, 5);
pub const F_SETLK64 = @as(c_int, 6);
pub const F_SETLKW64 = @as(c_int, 7);
pub const O_ACCMODE = @as(c_int, 0o003);
pub const O_RDONLY = @as(c_int, 0o0);
pub const O_WRONLY = @as(c_int, 0o1);
pub const O_RDWR = @as(c_int, 0o2);
pub const O_CREAT = @as(c_int, 0o100);
pub const O_EXCL = @as(c_int, 0o200);
pub const O_NOCTTY = @as(c_int, 0o400);
pub const O_TRUNC = @as(c_int, 0o1000);
pub const O_APPEND = @as(c_int, 0o2000);
pub const O_NONBLOCK = @as(c_int, 0o4000);
pub const O_NDELAY = O_NONBLOCK;
pub const O_SYNC = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o4010000, .octal);
pub const O_FSYNC = O_SYNC;
pub const O_ASYNC = @as(c_int, 0o20000);
pub const __O_DIRECTORY = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o200000, .octal);
pub const __O_NOFOLLOW = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o400000, .octal);
pub const __O_CLOEXEC = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o2000000, .octal);
pub const __O_DIRECT = @as(c_int, 0o40000);
pub const __O_NOATIME = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o1000000, .octal);
pub const __O_PATH = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o10000000, .octal);
pub const __O_DSYNC = @as(c_int, 0o10000);
pub const __O_TMPFILE = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o20000000, .octal) | __O_DIRECTORY;
pub const F_GETLK = F_GETLK64;
pub const F_SETLK = F_SETLK64;
pub const F_SETLKW = F_SETLKW64;
pub const O_DIRECTORY = __O_DIRECTORY;
pub const O_NOFOLLOW = __O_NOFOLLOW;
pub const O_CLOEXEC = __O_CLOEXEC;
pub const O_DSYNC = __O_DSYNC;
pub const O_RSYNC = O_SYNC;
pub const F_DUPFD = @as(c_int, 0);
pub const F_GETFD = @as(c_int, 1);
pub const F_SETFD = @as(c_int, 2);
pub const F_GETFL = @as(c_int, 3);
pub const F_SETFL = @as(c_int, 4);
pub const __F_SETOWN = @as(c_int, 8);
pub const __F_GETOWN = @as(c_int, 9);
pub const F_SETOWN = __F_SETOWN;
pub const F_GETOWN = __F_GETOWN;
pub const __F_SETSIG = @as(c_int, 10);
pub const __F_GETSIG = @as(c_int, 11);
pub const __F_SETOWN_EX = @as(c_int, 15);
pub const __F_GETOWN_EX = @as(c_int, 16);
pub const F_DUPFD_CLOEXEC = @as(c_int, 1030);
pub const FD_CLOEXEC = @as(c_int, 1);
pub const F_RDLCK = @as(c_int, 0);
pub const F_WRLCK = @as(c_int, 1);
pub const F_UNLCK = @as(c_int, 2);
pub const F_EXLCK = @as(c_int, 4);
pub const F_SHLCK = @as(c_int, 8);
pub const LOCK_SH = @as(c_int, 1);
pub const LOCK_EX = @as(c_int, 2);
pub const LOCK_NB = @as(c_int, 4);
pub const LOCK_UN = @as(c_int, 8);
pub const FAPPEND = O_APPEND;
pub const FFSYNC = O_FSYNC;
pub const FASYNC = O_ASYNC;
pub const FNONBLOCK = O_NONBLOCK;
pub const FNDELAY = O_NDELAY;
pub const __POSIX_FADV_DONTNEED = @as(c_int, 4);
pub const __POSIX_FADV_NOREUSE = @as(c_int, 5);
pub const POSIX_FADV_NORMAL = @as(c_int, 0);
pub const POSIX_FADV_RANDOM = @as(c_int, 1);
pub const POSIX_FADV_SEQUENTIAL = @as(c_int, 2);
pub const POSIX_FADV_WILLNEED = @as(c_int, 3);
pub const POSIX_FADV_DONTNEED = __POSIX_FADV_DONTNEED;
pub const POSIX_FADV_NOREUSE = __POSIX_FADV_NOREUSE;
pub inline fn __OPEN_NEEDS_MODE(oflag: anytype) @TypeOf(((oflag & O_CREAT) != @as(c_int, 0)) or ((oflag & __O_TMPFILE) == __O_TMPFILE)) {
    return ((oflag & O_CREAT) != @as(c_int, 0)) or ((oflag & __O_TMPFILE) == __O_TMPFILE);
}
pub const __mode_t_defined = "";
pub const __off_t_defined = "";
pub const __pid_t_defined = "";
pub const _STRUCT_TIMESPEC = @as(c_int, 1);
pub const _BITS_ENDIAN_H = @as(c_int, 1);
pub const __LITTLE_ENDIAN = @as(c_int, 1234);
pub const __BIG_ENDIAN = @as(c_int, 4321);
pub const __PDP_ENDIAN = @as(c_int, 3412);
pub const _BITS_ENDIANNESS_H = @as(c_int, 1);
pub const __BYTE_ORDER = __LITTLE_ENDIAN;
pub const __FLOAT_WORD_ORDER = __BYTE_ORDER;
pub inline fn __LONG_LONG_PAIR(HI: anytype, LO: anytype) @TypeOf(HI) {
    return blk: {
        _ = @TypeOf(LO);
        break :blk HI;
    };
}
pub const __time_t_defined = @as(c_int, 1);
pub const _BITS_STAT_H = @as(c_int, 1);
pub const _BITS_STRUCT_STAT_H = @as(c_int, 1);
pub const _STATBUF_ST_BLKSIZE = "";
pub const _STATBUF_ST_RDEV = "";
pub const _STATBUF_ST_NSEC = "";
pub const __S_IFMT = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o170000, .octal);
pub const __S_IFDIR = @as(c_int, 0o040000);
pub const __S_IFCHR = @as(c_int, 0o020000);
pub const __S_IFBLK = @as(c_int, 0o060000);
pub const __S_IFREG = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o100000, .octal);
pub const __S_IFIFO = @as(c_int, 0o010000);
pub const __S_IFLNK = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o120000, .octal);
pub const __S_IFSOCK = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0o140000, .octal);
pub inline fn __S_TYPEISMQ(buf: anytype) @TypeOf(buf.*.st_mode - buf.*.st_mode) {
    return buf.*.st_mode - buf.*.st_mode;
}
pub inline fn __S_TYPEISSEM(buf: anytype) @TypeOf(buf.*.st_mode - buf.*.st_mode) {
    return buf.*.st_mode - buf.*.st_mode;
}
pub inline fn __S_TYPEISSHM(buf: anytype) @TypeOf(buf.*.st_mode - buf.*.st_mode) {
    return buf.*.st_mode - buf.*.st_mode;
}
pub const __S_ISUID = @as(c_int, 0o4000);
pub const __S_ISGID = @as(c_int, 0o2000);
pub const __S_ISVTX = @as(c_int, 0o1000);
pub const __S_IREAD = @as(c_int, 0o400);
pub const __S_IWRITE = @as(c_int, 0o200);
pub const __S_IEXEC = @as(c_int, 0o100);
pub const UTIME_NOW = (@as(c_long, 1) << @as(c_int, 30)) - @as(c_long, 1);
pub const UTIME_OMIT = (@as(c_long, 1) << @as(c_int, 30)) - @as(c_long, 2);
pub const S_IFMT = __S_IFMT;
pub const S_IFDIR = __S_IFDIR;
pub const S_IFCHR = __S_IFCHR;
pub const S_IFBLK = __S_IFBLK;
pub const S_IFREG = __S_IFREG;
pub const S_IFIFO = __S_IFIFO;
pub const S_IFLNK = __S_IFLNK;
pub const S_IFSOCK = __S_IFSOCK;
pub const S_ISUID = __S_ISUID;
pub const S_ISGID = __S_ISGID;
pub const S_ISVTX = __S_ISVTX;
pub const S_IRUSR = __S_IREAD;
pub const S_IWUSR = __S_IWRITE;
pub const S_IXUSR = __S_IEXEC;
pub const S_IRWXU = (__S_IREAD | __S_IWRITE) | __S_IEXEC;
pub const S_IRGRP = S_IRUSR >> @as(c_int, 3);
pub const S_IWGRP = S_IWUSR >> @as(c_int, 3);
pub const S_IXGRP = S_IXUSR >> @as(c_int, 3);
pub const S_IRWXG = S_IRWXU >> @as(c_int, 3);
pub const S_IROTH = S_IRGRP >> @as(c_int, 3);
pub const S_IWOTH = S_IWGRP >> @as(c_int, 3);
pub const S_IXOTH = S_IXGRP >> @as(c_int, 3);
pub const S_IRWXO = S_IRWXG >> @as(c_int, 3);
pub const R_OK = @as(c_int, 4);
pub const W_OK = @as(c_int, 2);
pub const X_OK = @as(c_int, 1);
pub const F_OK = @as(c_int, 0);
pub const SEEK_SET = @as(c_int, 0);
pub const SEEK_CUR = @as(c_int, 1);
pub const SEEK_END = @as(c_int, 2);
pub const AT_FDCWD = -@as(c_int, 100);
pub const AT_SYMLINK_NOFOLLOW = @as(c_int, 0x100);
pub const AT_REMOVEDIR = @as(c_int, 0x200);
pub const AT_SYMLINK_FOLLOW = @as(c_int, 0x400);
pub const AT_EACCESS = @as(c_int, 0x200);
pub const F_ULOCK = @as(c_int, 0);
pub const F_LOCK = @as(c_int, 1);
pub const F_TLOCK = @as(c_int, 2);
pub const F_TEST = @as(c_int, 3);
pub const _STDIO_H = @as(c_int, 1);
pub const __GLIBC_INTERNAL_STARTING_HEADER_IMPLEMENTATION = "";
pub const __GLIBC_USE_LIB_EXT2 = @as(c_int, 0);
pub const __GLIBC_USE_IEC_60559_BFP_EXT = @as(c_int, 0);
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X = @as(c_int, 0);
pub const __GLIBC_USE_IEC_60559_EXT = @as(c_int, 0);
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT = @as(c_int, 0);
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X = @as(c_int, 0);
pub const __GLIBC_USE_IEC_60559_TYPES_EXT = @as(c_int, 0);
pub const __need_size_t = "";
pub const __need_NULL = "";
pub const _SIZE_T = "";
pub const NULL = @import("std").zig.c_translation.cast(?*anyopaque, @as(c_int, 0));
pub const __need___va_list = "";
pub const __GNUC_VA_LIST = "";
pub const _____fpos_t_defined = @as(c_int, 1);
pub const ____mbstate_t_defined = @as(c_int, 1);
pub const _____fpos64_t_defined = @as(c_int, 1);
pub const ____FILE_defined = @as(c_int, 1);
pub const __FILE_defined = @as(c_int, 1);
pub const __struct_FILE_defined = @as(c_int, 1);
pub const _IO_EOF_SEEN = @as(c_int, 0x0010);
pub inline fn __feof_unlocked_body(_fp: anytype) @TypeOf((_fp.*._flags & _IO_EOF_SEEN) != @as(c_int, 0)) {
    return (_fp.*._flags & _IO_EOF_SEEN) != @as(c_int, 0);
}
pub const _IO_ERR_SEEN = @as(c_int, 0x0020);
pub inline fn __ferror_unlocked_body(_fp: anytype) @TypeOf((_fp.*._flags & _IO_ERR_SEEN) != @as(c_int, 0)) {
    return (_fp.*._flags & _IO_ERR_SEEN) != @as(c_int, 0);
}
pub const _IO_USER_LOCK = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0x8000, .hexadecimal);
pub const __cookie_io_functions_t_defined = @as(c_int, 1);
pub const _VA_LIST_DEFINED = "";
pub const __ssize_t_defined = "";
pub const _IOFBF = @as(c_int, 0);
pub const _IOLBF = @as(c_int, 1);
pub const _IONBF = @as(c_int, 2);
pub const BUFSIZ = @as(c_int, 8192);
pub const EOF = -@as(c_int, 1);
pub const P_tmpdir = "/tmp";
pub const L_tmpnam = @as(c_int, 20);
pub const TMP_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_int, 238328, .decimal);
pub const _BITS_STDIO_LIM_H = @as(c_int, 1);
pub const FILENAME_MAX = @as(c_int, 4096);
pub const L_ctermid = @as(c_int, 9);
pub const FOPEN_MAX = @as(c_int, 16);
pub const __attr_dealloc_fclose = __attr_dealloc(fclose, @as(c_int, 1));
pub const _BITS_FLOATN_H = "";
pub const __HAVE_FLOAT128 = @as(c_int, 0);
pub const __HAVE_DISTINCT_FLOAT128 = @as(c_int, 0);
pub const __HAVE_FLOAT64X = @as(c_int, 1);
pub const __HAVE_FLOAT64X_LONG_DOUBLE = @as(c_int, 1);
pub const _BITS_FLOATN_COMMON_H = "";
pub const __HAVE_FLOAT16 = @as(c_int, 0);
pub const __HAVE_FLOAT32 = @as(c_int, 1);
pub const __HAVE_FLOAT64 = @as(c_int, 1);
pub const __HAVE_FLOAT32X = @as(c_int, 1);
pub const __HAVE_FLOAT128X = @as(c_int, 0);
pub const __HAVE_DISTINCT_FLOAT16 = __HAVE_FLOAT16;
pub const __HAVE_DISTINCT_FLOAT32 = @as(c_int, 0);
pub const __HAVE_DISTINCT_FLOAT64 = @as(c_int, 0);
pub const __HAVE_DISTINCT_FLOAT32X = @as(c_int, 0);
pub const __HAVE_DISTINCT_FLOAT64X = @as(c_int, 0);
pub const __HAVE_DISTINCT_FLOAT128X = __HAVE_FLOAT128X;
pub const __HAVE_FLOAT128_UNLIKE_LDBL = (__HAVE_DISTINCT_FLOAT128 != 0) and (__LDBL_MANT_DIG__ != @as(c_int, 113));
pub const __HAVE_FLOATN_NOT_TYPEDEF = @as(c_int, 0);
pub const __f32 = @import("std").zig.c_translation.Macros.F_SUFFIX;
pub inline fn __f64(x: anytype) @TypeOf(x) {
    return x;
}
pub inline fn __f32x(x: anytype) @TypeOf(x) {
    return x;
}
pub const __f64x = @import("std").zig.c_translation.Macros.L_SUFFIX;
pub inline fn __builtin_huge_valf32() @TypeOf(__builtin_huge_valf()) {
    return __builtin_huge_valf();
}
pub inline fn __builtin_inff32() @TypeOf(__builtin_inff()) {
    return __builtin_inff();
}
pub inline fn __builtin_nanf32(x: anytype) @TypeOf(__builtin_nanf(x)) {
    return __builtin_nanf(x);
}
pub const __need_wchar_t = "";
pub const _WCHAR_T = "";
pub const _STDLIB_H = @as(c_int, 1);
pub const WNOHANG = @as(c_int, 1);
pub const WUNTRACED = @as(c_int, 2);
pub const WSTOPPED = @as(c_int, 2);
pub const WEXITED = @as(c_int, 4);
pub const WCONTINUED = @as(c_int, 8);
pub const WNOWAIT = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0x01000000, .hexadecimal);
pub const __WNOTHREAD = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0x20000000, .hexadecimal);
pub const __WALL = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0x40000000, .hexadecimal);
pub const __WCLONE = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0x80000000, .hexadecimal);
pub inline fn __WEXITSTATUS(status: anytype) @TypeOf((status & @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xff00, .hexadecimal)) >> @as(c_int, 8)) {
    return (status & @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xff00, .hexadecimal)) >> @as(c_int, 8);
}
pub inline fn __WTERMSIG(status: anytype) @TypeOf(status & @as(c_int, 0x7f)) {
    return status & @as(c_int, 0x7f);
}
pub inline fn __WSTOPSIG(status: anytype) @TypeOf(__WEXITSTATUS(status)) {
    return __WEXITSTATUS(status);
}
pub inline fn __WIFEXITED(status: anytype) @TypeOf(__WTERMSIG(status) == @as(c_int, 0)) {
    return __WTERMSIG(status) == @as(c_int, 0);
}
pub inline fn __WIFSIGNALED(status: anytype) @TypeOf((@import("std").zig.c_translation.cast(i8, (status & @as(c_int, 0x7f)) + @as(c_int, 1)) >> @as(c_int, 1)) > @as(c_int, 0)) {
    return (@import("std").zig.c_translation.cast(i8, (status & @as(c_int, 0x7f)) + @as(c_int, 1)) >> @as(c_int, 1)) > @as(c_int, 0);
}
pub inline fn __WIFSTOPPED(status: anytype) @TypeOf((status & @as(c_int, 0xff)) == @as(c_int, 0x7f)) {
    return (status & @as(c_int, 0xff)) == @as(c_int, 0x7f);
}
pub inline fn __WIFCONTINUED(status: anytype) @TypeOf(status == __W_CONTINUED) {
    return status == __W_CONTINUED;
}
pub inline fn __WCOREDUMP(status: anytype) @TypeOf(status & __WCOREFLAG) {
    return status & __WCOREFLAG;
}
pub inline fn __W_EXITCODE(ret: anytype, sig: anytype) @TypeOf((ret << @as(c_int, 8)) | sig) {
    return (ret << @as(c_int, 8)) | sig;
}
pub inline fn __W_STOPCODE(sig: anytype) @TypeOf((sig << @as(c_int, 8)) | @as(c_int, 0x7f)) {
    return (sig << @as(c_int, 8)) | @as(c_int, 0x7f);
}
pub const __W_CONTINUED = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xffff, .hexadecimal);
pub const __WCOREFLAG = @as(c_int, 0x80);
pub inline fn WEXITSTATUS(status: anytype) @TypeOf(__WEXITSTATUS(status)) {
    return __WEXITSTATUS(status);
}
pub inline fn WTERMSIG(status: anytype) @TypeOf(__WTERMSIG(status)) {
    return __WTERMSIG(status);
}
pub inline fn WSTOPSIG(status: anytype) @TypeOf(__WSTOPSIG(status)) {
    return __WSTOPSIG(status);
}
pub inline fn WIFEXITED(status: anytype) @TypeOf(__WIFEXITED(status)) {
    return __WIFEXITED(status);
}
pub inline fn WIFSIGNALED(status: anytype) @TypeOf(__WIFSIGNALED(status)) {
    return __WIFSIGNALED(status);
}
pub inline fn WIFSTOPPED(status: anytype) @TypeOf(__WIFSTOPPED(status)) {
    return __WIFSTOPPED(status);
}
pub inline fn WIFCONTINUED(status: anytype) @TypeOf(__WIFCONTINUED(status)) {
    return __WIFCONTINUED(status);
}
pub const __ldiv_t_defined = @as(c_int, 1);
pub const __lldiv_t_defined = @as(c_int, 1);
pub const RAND_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const EXIT_FAILURE = @as(c_int, 1);
pub const EXIT_SUCCESS = @as(c_int, 0);
pub const MB_CUR_MAX = __ctype_get_mb_cur_max();
pub const _SYS_TYPES_H = @as(c_int, 1);
pub const __u_char_defined = "";
pub const __ino_t_defined = "";
pub const __dev_t_defined = "";
pub const __gid_t_defined = "";
pub const __nlink_t_defined = "";
pub const __uid_t_defined = "";
pub const __id_t_defined = "";
pub const __daddr_t_defined = "";
pub const __key_t_defined = "";
pub const __clock_t_defined = @as(c_int, 1);
pub const __clockid_t_defined = @as(c_int, 1);
pub const __timer_t_defined = @as(c_int, 1);
pub const _BITS_STDINT_INTN_H = @as(c_int, 1);
pub const __BIT_TYPES_DEFINED__ = @as(c_int, 1);
pub const _ENDIAN_H = @as(c_int, 1);
pub const LITTLE_ENDIAN = __LITTLE_ENDIAN;
pub const BIG_ENDIAN = __BIG_ENDIAN;
pub const PDP_ENDIAN = __PDP_ENDIAN;
pub const BYTE_ORDER = __BYTE_ORDER;
pub const _BITS_BYTESWAP_H = @as(c_int, 1);
pub inline fn __bswap_constant_16(x: anytype) __uint16_t {
    return @import("std").zig.c_translation.cast(__uint16_t, ((x >> @as(c_int, 8)) & @as(c_int, 0xff)) | ((x & @as(c_int, 0xff)) << @as(c_int, 8)));
}
pub inline fn __bswap_constant_32(x: anytype) @TypeOf(((((x & @import("std").zig.c_translation.promoteIntLiteral(c_uint, 0xff000000, .hexadecimal)) >> @as(c_int, 24)) | ((x & @import("std").zig.c_translation.promoteIntLiteral(c_uint, 0x00ff0000, .hexadecimal)) >> @as(c_int, 8))) | ((x & @as(c_uint, 0x0000ff00)) << @as(c_int, 8))) | ((x & @as(c_uint, 0x000000ff)) << @as(c_int, 24))) {
    return ((((x & @import("std").zig.c_translation.promoteIntLiteral(c_uint, 0xff000000, .hexadecimal)) >> @as(c_int, 24)) | ((x & @import("std").zig.c_translation.promoteIntLiteral(c_uint, 0x00ff0000, .hexadecimal)) >> @as(c_int, 8))) | ((x & @as(c_uint, 0x0000ff00)) << @as(c_int, 8))) | ((x & @as(c_uint, 0x000000ff)) << @as(c_int, 24));
}
pub inline fn __bswap_constant_64(x: anytype) @TypeOf(((((((((x & @as(c_ulonglong, 0xff00000000000000)) >> @as(c_int, 56)) | ((x & @as(c_ulonglong, 0x00ff000000000000)) >> @as(c_int, 40))) | ((x & @as(c_ulonglong, 0x0000ff0000000000)) >> @as(c_int, 24))) | ((x & @as(c_ulonglong, 0x000000ff00000000)) >> @as(c_int, 8))) | ((x & @as(c_ulonglong, 0x00000000ff000000)) << @as(c_int, 8))) | ((x & @as(c_ulonglong, 0x0000000000ff0000)) << @as(c_int, 24))) | ((x & @as(c_ulonglong, 0x000000000000ff00)) << @as(c_int, 40))) | ((x & @as(c_ulonglong, 0x00000000000000ff)) << @as(c_int, 56))) {
    return ((((((((x & @as(c_ulonglong, 0xff00000000000000)) >> @as(c_int, 56)) | ((x & @as(c_ulonglong, 0x00ff000000000000)) >> @as(c_int, 40))) | ((x & @as(c_ulonglong, 0x0000ff0000000000)) >> @as(c_int, 24))) | ((x & @as(c_ulonglong, 0x000000ff00000000)) >> @as(c_int, 8))) | ((x & @as(c_ulonglong, 0x00000000ff000000)) << @as(c_int, 8))) | ((x & @as(c_ulonglong, 0x0000000000ff0000)) << @as(c_int, 24))) | ((x & @as(c_ulonglong, 0x000000000000ff00)) << @as(c_int, 40))) | ((x & @as(c_ulonglong, 0x00000000000000ff)) << @as(c_int, 56));
}
pub const _BITS_UINTN_IDENTITY_H = @as(c_int, 1);
pub inline fn htobe16(x: anytype) @TypeOf(__bswap_16(x)) {
    return __bswap_16(x);
}
pub inline fn htole16(x: anytype) @TypeOf(__uint16_identity(x)) {
    return __uint16_identity(x);
}
pub inline fn be16toh(x: anytype) @TypeOf(__bswap_16(x)) {
    return __bswap_16(x);
}
pub inline fn le16toh(x: anytype) @TypeOf(__uint16_identity(x)) {
    return __uint16_identity(x);
}
pub inline fn htobe32(x: anytype) @TypeOf(__bswap_32(x)) {
    return __bswap_32(x);
}
pub inline fn htole32(x: anytype) @TypeOf(__uint32_identity(x)) {
    return __uint32_identity(x);
}
pub inline fn be32toh(x: anytype) @TypeOf(__bswap_32(x)) {
    return __bswap_32(x);
}
pub inline fn le32toh(x: anytype) @TypeOf(__uint32_identity(x)) {
    return __uint32_identity(x);
}
pub inline fn htobe64(x: anytype) @TypeOf(__bswap_64(x)) {
    return __bswap_64(x);
}
pub inline fn htole64(x: anytype) @TypeOf(__uint64_identity(x)) {
    return __uint64_identity(x);
}
pub inline fn be64toh(x: anytype) @TypeOf(__bswap_64(x)) {
    return __bswap_64(x);
}
pub inline fn le64toh(x: anytype) @TypeOf(__uint64_identity(x)) {
    return __uint64_identity(x);
}
pub const _SYS_SELECT_H = @as(c_int, 1);
pub inline fn __FD_ISSET(d: anytype, s: anytype) @TypeOf((__FDS_BITS(s)[@as(usize, @intCast(__FD_ELT(d)))] & __FD_MASK(d)) != @as(c_int, 0)) {
    return (__FDS_BITS(s)[@as(usize, @intCast(__FD_ELT(d)))] & __FD_MASK(d)) != @as(c_int, 0);
}
pub const __sigset_t_defined = @as(c_int, 1);
pub const ____sigset_t_defined = "";
pub const _SIGSET_NWORDS = @import("std").zig.c_translation.MacroArithmetic.div(@as(c_int, 1024), @as(c_int, 8) * @import("std").zig.c_translation.sizeof(c_ulong));
pub const __timeval_defined = @as(c_int, 1);
pub const __suseconds_t_defined = "";
pub const __NFDBITS = @as(c_int, 8) * @import("std").zig.c_translation.cast(c_int, @import("std").zig.c_translation.sizeof(__fd_mask));
pub inline fn __FD_ELT(d: anytype) @TypeOf(@import("std").zig.c_translation.MacroArithmetic.div(d, __NFDBITS)) {
    return @import("std").zig.c_translation.MacroArithmetic.div(d, __NFDBITS);
}
pub inline fn __FD_MASK(d: anytype) __fd_mask {
    return @import("std").zig.c_translation.cast(__fd_mask, @as(c_ulong, 1) << @import("std").zig.c_translation.MacroArithmetic.rem(d, __NFDBITS));
}
pub inline fn __FDS_BITS(set: anytype) @TypeOf(set.*.__fds_bits) {
    return set.*.__fds_bits;
}
pub const FD_SETSIZE = __FD_SETSIZE;
pub const NFDBITS = __NFDBITS;
pub inline fn FD_SET(fd: anytype, fdsetp: anytype) @TypeOf(__FD_SET(fd, fdsetp)) {
    return __FD_SET(fd, fdsetp);
}
pub inline fn FD_CLR(fd: anytype, fdsetp: anytype) @TypeOf(__FD_CLR(fd, fdsetp)) {
    return __FD_CLR(fd, fdsetp);
}
pub inline fn FD_ISSET(fd: anytype, fdsetp: anytype) @TypeOf(__FD_ISSET(fd, fdsetp)) {
    return __FD_ISSET(fd, fdsetp);
}
pub inline fn FD_ZERO(fdsetp: anytype) @TypeOf(__FD_ZERO(fdsetp)) {
    return __FD_ZERO(fdsetp);
}
pub const __blksize_t_defined = "";
pub const __blkcnt_t_defined = "";
pub const __fsblkcnt_t_defined = "";
pub const __fsfilcnt_t_defined = "";
pub const _BITS_PTHREADTYPES_COMMON_H = @as(c_int, 1);
pub const _THREAD_SHARED_TYPES_H = @as(c_int, 1);
pub const _BITS_PTHREADTYPES_ARCH_H = @as(c_int, 1);
pub const __SIZEOF_PTHREAD_MUTEX_T = @as(c_int, 40);
pub const __SIZEOF_PTHREAD_ATTR_T = @as(c_int, 56);
pub const __SIZEOF_PTHREAD_RWLOCK_T = @as(c_int, 56);
pub const __SIZEOF_PTHREAD_BARRIER_T = @as(c_int, 32);
pub const __SIZEOF_PTHREAD_MUTEXATTR_T = @as(c_int, 4);
pub const __SIZEOF_PTHREAD_COND_T = @as(c_int, 48);
pub const __SIZEOF_PTHREAD_CONDATTR_T = @as(c_int, 4);
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T = @as(c_int, 8);
pub const __SIZEOF_PTHREAD_BARRIERATTR_T = @as(c_int, 4);
pub const __LOCK_ALIGNMENT = "";
pub const __ONCE_ALIGNMENT = "";
pub const _BITS_ATOMIC_WIDE_COUNTER_H = "";
pub const _THREAD_MUTEX_INTERNAL_H = @as(c_int, 1);
pub const __PTHREAD_MUTEX_HAVE_PREV = @as(c_int, 1);
pub const _RWLOCK_INTERNAL_H = "";
pub inline fn __PTHREAD_RWLOCK_INITIALIZER(__flags: anytype) @TypeOf(__flags) {
    return blk: {
        _ = @as(c_int, 0);
        _ = @as(c_int, 0);
        _ = @as(c_int, 0);
        _ = @as(c_int, 0);
        _ = @as(c_int, 0);
        _ = @as(c_int, 0);
        _ = @as(c_int, 0);
        _ = @as(c_int, 0);
        _ = @TypeOf(__PTHREAD_RWLOCK_ELISION_EXTRA);
        _ = @as(c_int, 0);
        break :blk __flags;
    };
}
pub const __have_pthread_attr_t = @as(c_int, 1);
pub const _ALLOCA_H = @as(c_int, 1);
pub const __COMPAR_FN_T = "";
pub const _STRING_H = @as(c_int, 1);
pub const _BITS_TYPES_LOCALE_T_H = @as(c_int, 1);
pub const _BITS_TYPES___LOCALE_T_H = @as(c_int, 1);
pub const _STRINGS_H = @as(c_int, 1);
pub const _XF86DRM_H_ = "";
pub const __STDARG_H = "";
pub const _VA_LIST = "";
pub const __CLANG_STDINT_H = "";
pub const _STDINT_H = @as(c_int, 1);
pub const _BITS_WCHAR_H = @as(c_int, 1);
pub const __WCHAR_MAX = __WCHAR_MAX__;
pub const __WCHAR_MIN = -__WCHAR_MAX - @as(c_int, 1);
pub const _BITS_STDINT_UINTN_H = @as(c_int, 1);
pub const __intptr_t_defined = "";
pub const __INT64_C = @import("std").zig.c_translation.Macros.L_SUFFIX;
pub const __UINT64_C = @import("std").zig.c_translation.Macros.UL_SUFFIX;
pub const INT8_MIN = -@as(c_int, 128);
pub const INT16_MIN = -@as(c_int, 32767) - @as(c_int, 1);
pub const INT32_MIN = -@import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal) - @as(c_int, 1);
pub const INT64_MIN = -__INT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 9223372036854775807, .decimal)) - @as(c_int, 1);
pub const INT8_MAX = @as(c_int, 127);
pub const INT16_MAX = @as(c_int, 32767);
pub const INT32_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const INT64_MAX = __INT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 9223372036854775807, .decimal));
pub const UINT8_MAX = @as(c_int, 255);
pub const UINT16_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_int, 65535, .decimal);
pub const UINT32_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const UINT64_MAX = __UINT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 18446744073709551615, .decimal));
pub const INT_LEAST8_MIN = -@as(c_int, 128);
pub const INT_LEAST16_MIN = -@as(c_int, 32767) - @as(c_int, 1);
pub const INT_LEAST32_MIN = -@import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal) - @as(c_int, 1);
pub const INT_LEAST64_MIN = -__INT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 9223372036854775807, .decimal)) - @as(c_int, 1);
pub const INT_LEAST8_MAX = @as(c_int, 127);
pub const INT_LEAST16_MAX = @as(c_int, 32767);
pub const INT_LEAST32_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const INT_LEAST64_MAX = __INT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 9223372036854775807, .decimal));
pub const UINT_LEAST8_MAX = @as(c_int, 255);
pub const UINT_LEAST16_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_int, 65535, .decimal);
pub const UINT_LEAST32_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const UINT_LEAST64_MAX = __UINT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 18446744073709551615, .decimal));
pub const INT_FAST8_MIN = -@as(c_int, 128);
pub const INT_FAST16_MIN = -@import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal) - @as(c_int, 1);
pub const INT_FAST32_MIN = -@import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal) - @as(c_int, 1);
pub const INT_FAST64_MIN = -__INT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 9223372036854775807, .decimal)) - @as(c_int, 1);
pub const INT_FAST8_MAX = @as(c_int, 127);
pub const INT_FAST16_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const INT_FAST32_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const INT_FAST64_MAX = __INT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 9223372036854775807, .decimal));
pub const UINT_FAST8_MAX = @as(c_int, 255);
pub const UINT_FAST16_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const UINT_FAST32_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const UINT_FAST64_MAX = __UINT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 18446744073709551615, .decimal));
pub const INTPTR_MIN = -@import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal) - @as(c_int, 1);
pub const INTPTR_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const UINTPTR_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const INTMAX_MIN = -__INT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 9223372036854775807, .decimal)) - @as(c_int, 1);
pub const INTMAX_MAX = __INT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 9223372036854775807, .decimal));
pub const UINTMAX_MAX = __UINT64_C(@import("std").zig.c_translation.promoteIntLiteral(c_int, 18446744073709551615, .decimal));
pub const PTRDIFF_MIN = -@import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal) - @as(c_int, 1);
pub const PTRDIFF_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const SIG_ATOMIC_MIN = -@import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal) - @as(c_int, 1);
pub const SIG_ATOMIC_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const SIZE_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const WCHAR_MIN = __WCHAR_MIN;
pub const WCHAR_MAX = __WCHAR_MAX;
pub const WINT_MIN = @as(c_uint, 0);
pub const WINT_MAX = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub inline fn INT8_C(c: anytype) @TypeOf(c) {
    return c;
}
pub inline fn INT16_C(c: anytype) @TypeOf(c) {
    return c;
}
pub inline fn INT32_C(c: anytype) @TypeOf(c) {
    return c;
}
pub const INT64_C = @import("std").zig.c_translation.Macros.L_SUFFIX;
pub inline fn UINT8_C(c: anytype) @TypeOf(c) {
    return c;
}
pub inline fn UINT16_C(c: anytype) @TypeOf(c) {
    return c;
}
pub const UINT32_C = @import("std").zig.c_translation.Macros.U_SUFFIX;
pub const UINT64_C = @import("std").zig.c_translation.Macros.UL_SUFFIX;
pub const INTMAX_C = @import("std").zig.c_translation.Macros.L_SUFFIX;
pub const UINTMAX_C = @import("std").zig.c_translation.Macros.UL_SUFFIX;
pub const _DRM_H_ = "";
pub const _LINUX_TYPES_H = "";
pub const _ASM_GENERIC_TYPES_H = "";
pub const _ASM_GENERIC_INT_LL64_H = "";
pub const __ASM_X86_BITSPERLONG_H = "";
pub const __BITS_PER_LONG = @as(c_int, 64);
pub const __ASM_GENERIC_BITS_PER_LONG = "";
pub const _LINUX_POSIX_TYPES_H = "";
pub const _LINUX_STDDEF_H = "";
pub const _ASM_X86_POSIX_TYPES_64_H = "";
pub const __ASM_GENERIC_POSIX_TYPES_H = "";
pub const __bitwise = "";
pub const __bitwise__ = __bitwise;
pub const _ASM_GENERIC_IOCTL_H = "";
pub const _IOC_NRBITS = @as(c_int, 8);
pub const _IOC_TYPEBITS = @as(c_int, 8);
pub const _IOC_SIZEBITS = @as(c_int, 14);
pub const _IOC_DIRBITS = @as(c_int, 2);
pub const _IOC_NRMASK = (@as(c_int, 1) << _IOC_NRBITS) - @as(c_int, 1);
pub const _IOC_TYPEMASK = (@as(c_int, 1) << _IOC_TYPEBITS) - @as(c_int, 1);
pub const _IOC_SIZEMASK = (@as(c_int, 1) << _IOC_SIZEBITS) - @as(c_int, 1);
pub const _IOC_DIRMASK = (@as(c_int, 1) << _IOC_DIRBITS) - @as(c_int, 1);
pub const _IOC_NRSHIFT = @as(c_int, 0);
pub const _IOC_TYPESHIFT = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const _IOC_NONE = @as(c_uint, 0);
pub const _IOC_WRITE = @as(c_uint, 1);
pub const _IOC_READ = @as(c_uint, 2);
pub inline fn _IOC(dir: anytype, @"type": anytype, nr: anytype, size: anytype) @TypeOf((((dir << _IOC_DIRSHIFT) | (@"type" << _IOC_TYPESHIFT)) | (nr << _IOC_NRSHIFT)) | (size << _IOC_SIZESHIFT)) {
    return (((dir << _IOC_DIRSHIFT) | (@"type" << _IOC_TYPESHIFT)) | (nr << _IOC_NRSHIFT)) | (size << _IOC_SIZESHIFT);
}
pub inline fn _IOC_TYPECHECK(t: anytype) @TypeOf(@import("std").zig.c_translation.sizeof(t)) {
    _ = @TypeOf(t);
    return @import("std").zig.c_translation.sizeof(t);
}
pub inline fn _IO(@"type": anytype, nr: anytype) @TypeOf(_IOC(_IOC_NONE, @"type", nr, @as(c_int, 0))) {
    return _IOC(_IOC_NONE, @"type", nr, @as(c_int, 0));
}
pub inline fn _IOR(@"type": anytype, nr: anytype, size: anytype) @TypeOf(_IOC(_IOC_READ, @"type", nr, _IOC_TYPECHECK(size))) {
    return _IOC(_IOC_READ, @"type", nr, _IOC_TYPECHECK(size));
}
pub inline fn _IOW(@"type": anytype, nr: anytype, size: anytype) @TypeOf(_IOC(_IOC_WRITE, @"type", nr, _IOC_TYPECHECK(size))) {
    return _IOC(_IOC_WRITE, @"type", nr, _IOC_TYPECHECK(size));
}
pub inline fn _IOWR(@"type": anytype, nr: anytype, size: anytype) @TypeOf(_IOC(_IOC_READ | _IOC_WRITE, @"type", nr, _IOC_TYPECHECK(size))) {
    return _IOC(_IOC_READ | _IOC_WRITE, @"type", nr, _IOC_TYPECHECK(size));
}
pub inline fn _IOR_BAD(@"type": anytype, nr: anytype, size: anytype) @TypeOf(_IOC(_IOC_READ, @"type", nr, @import("std").zig.c_translation.sizeof(size))) {
    _ = @TypeOf(size);
    return _IOC(_IOC_READ, @"type", nr, @import("std").zig.c_translation.sizeof(size));
}
pub inline fn _IOW_BAD(@"type": anytype, nr: anytype, size: anytype) @TypeOf(_IOC(_IOC_WRITE, @"type", nr, @import("std").zig.c_translation.sizeof(size))) {
    _ = @TypeOf(size);
    return _IOC(_IOC_WRITE, @"type", nr, @import("std").zig.c_translation.sizeof(size));
}
pub inline fn _IOWR_BAD(@"type": anytype, nr: anytype, size: anytype) @TypeOf(_IOC(_IOC_READ | _IOC_WRITE, @"type", nr, @import("std").zig.c_translation.sizeof(size))) {
    _ = @TypeOf(size);
    return _IOC(_IOC_READ | _IOC_WRITE, @"type", nr, @import("std").zig.c_translation.sizeof(size));
}
pub inline fn _IOC_DIR(nr: anytype) @TypeOf((nr >> _IOC_DIRSHIFT) & _IOC_DIRMASK) {
    return (nr >> _IOC_DIRSHIFT) & _IOC_DIRMASK;
}
pub inline fn _IOC_TYPE(nr: anytype) @TypeOf((nr >> _IOC_TYPESHIFT) & _IOC_TYPEMASK) {
    return (nr >> _IOC_TYPESHIFT) & _IOC_TYPEMASK;
}
pub inline fn _IOC_NR(nr: anytype) @TypeOf((nr >> _IOC_NRSHIFT) & _IOC_NRMASK) {
    return (nr >> _IOC_NRSHIFT) & _IOC_NRMASK;
}
pub inline fn _IOC_SIZE(nr: anytype) @TypeOf((nr >> _IOC_SIZESHIFT) & _IOC_SIZEMASK) {
    return (nr >> _IOC_SIZESHIFT) & _IOC_SIZEMASK;
}
pub const IOC_IN = _IOC_WRITE << _IOC_DIRSHIFT;
pub const IOC_OUT = _IOC_READ << _IOC_DIRSHIFT;
pub const IOC_INOUT = (_IOC_WRITE | _IOC_READ) << _IOC_DIRSHIFT;
pub const IOCSIZE_MASK = _IOC_SIZEMASK << _IOC_SIZESHIFT;
pub const IOCSIZE_SHIFT = _IOC_SIZESHIFT;
pub const DRM_NAME = "drm";
pub const DRM_MIN_ORDER = @as(c_int, 5);
pub const DRM_MAX_ORDER = @as(c_int, 22);
pub const DRM_RAM_PERCENT = @as(c_int, 10);
pub const _DRM_LOCK_HELD = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 0x80000000, .hexadecimal);
pub const _DRM_LOCK_CONT = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 0x40000000, .hexadecimal);
pub inline fn _DRM_LOCK_IS_HELD(lock: anytype) @TypeOf(lock & _DRM_LOCK_HELD) {
    return lock & _DRM_LOCK_HELD;
}
pub inline fn _DRM_LOCK_IS_CONT(lock: anytype) @TypeOf(lock & _DRM_LOCK_CONT) {
    return lock & _DRM_LOCK_CONT;
}
pub inline fn _DRM_LOCKING_CONTEXT(lock: anytype) @TypeOf(lock & ~(_DRM_LOCK_HELD | _DRM_LOCK_CONT)) {
    return lock & ~(_DRM_LOCK_HELD | _DRM_LOCK_CONT);
}
pub const _DRM_VBLANK_HIGH_CRTC_SHIFT = @as(c_int, 1);
pub const _DRM_VBLANK_TYPES_MASK = _DRM_VBLANK_ABSOLUTE | _DRM_VBLANK_RELATIVE;
pub const _DRM_VBLANK_FLAGS_MASK = ((_DRM_VBLANK_EVENT | _DRM_VBLANK_SIGNAL) | _DRM_VBLANK_SECONDARY) | _DRM_VBLANK_NEXTONMISS;
pub const _DRM_PRE_MODESET = @as(c_int, 1);
pub const _DRM_POST_MODESET = @as(c_int, 2);
pub const DRM_CAP_DUMB_BUFFER = @as(c_int, 0x1);
pub const DRM_CAP_VBLANK_HIGH_CRTC = @as(c_int, 0x2);
pub const DRM_CAP_DUMB_PREFERRED_DEPTH = @as(c_int, 0x3);
pub const DRM_CAP_DUMB_PREFER_SHADOW = @as(c_int, 0x4);
pub const DRM_CAP_PRIME = @as(c_int, 0x5);
pub const DRM_PRIME_CAP_IMPORT = @as(c_int, 0x1);
pub const DRM_PRIME_CAP_EXPORT = @as(c_int, 0x2);
pub const DRM_CAP_TIMESTAMP_MONOTONIC = @as(c_int, 0x6);
pub const DRM_CAP_ASYNC_PAGE_FLIP = @as(c_int, 0x7);
pub const DRM_CAP_CURSOR_WIDTH = @as(c_int, 0x8);
pub const DRM_CAP_CURSOR_HEIGHT = @as(c_int, 0x9);
pub const DRM_CAP_ADDFB2_MODIFIERS = @as(c_int, 0x10);
pub const DRM_CAP_PAGE_FLIP_TARGET = @as(c_int, 0x11);
pub const DRM_CAP_CRTC_IN_VBLANK_EVENT = @as(c_int, 0x12);
pub const DRM_CAP_SYNCOBJ = @as(c_int, 0x13);
pub const DRM_CAP_SYNCOBJ_TIMELINE = @as(c_int, 0x14);
pub const DRM_CLIENT_CAP_STEREO_3D = @as(c_int, 1);
pub const DRM_CLIENT_CAP_UNIVERSAL_PLANES = @as(c_int, 2);
pub const DRM_CLIENT_CAP_ATOMIC = @as(c_int, 3);
pub const DRM_CLIENT_CAP_ASPECT_RATIO = @as(c_int, 4);
pub const DRM_CLIENT_CAP_WRITEBACK_CONNECTORS = @as(c_int, 5);
pub const DRM_RDWR = O_RDWR;
pub const DRM_CLOEXEC = O_CLOEXEC;
pub const DRM_SYNCOBJ_CREATE_SIGNALED = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_IMPORT_SYNC_FILE = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT = @as(c_int, 1) << @as(c_int, 1);
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE = @as(c_int, 1) << @as(c_int, 2);
pub const DRM_SYNCOBJ_QUERY_FLAGS_LAST_SUBMITTED = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_CRTC_SEQUENCE_RELATIVE = @as(c_int, 0x00000001);
pub const DRM_CRTC_SEQUENCE_NEXT_ON_MISS = @as(c_int, 0x00000002);
pub const _DRM_MODE_H = "";
pub const DRM_CONNECTOR_NAME_LEN = @as(c_int, 32);
pub const DRM_DISPLAY_MODE_LEN = @as(c_int, 32);
pub const DRM_PROP_NAME_LEN = @as(c_int, 32);
pub const DRM_MODE_TYPE_BUILTIN = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_MODE_TYPE_CLOCK_C = (@as(c_int, 1) << @as(c_int, 1)) | DRM_MODE_TYPE_BUILTIN;
pub const DRM_MODE_TYPE_CRTC_C = (@as(c_int, 1) << @as(c_int, 2)) | DRM_MODE_TYPE_BUILTIN;
pub const DRM_MODE_TYPE_PREFERRED = @as(c_int, 1) << @as(c_int, 3);
pub const DRM_MODE_TYPE_DEFAULT = @as(c_int, 1) << @as(c_int, 4);
pub const DRM_MODE_TYPE_USERDEF = @as(c_int, 1) << @as(c_int, 5);
pub const DRM_MODE_TYPE_DRIVER = @as(c_int, 1) << @as(c_int, 6);
pub const DRM_MODE_TYPE_ALL = (DRM_MODE_TYPE_PREFERRED | DRM_MODE_TYPE_USERDEF) | DRM_MODE_TYPE_DRIVER;
pub const DRM_MODE_FLAG_PHSYNC = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_MODE_FLAG_NHSYNC = @as(c_int, 1) << @as(c_int, 1);
pub const DRM_MODE_FLAG_PVSYNC = @as(c_int, 1) << @as(c_int, 2);
pub const DRM_MODE_FLAG_NVSYNC = @as(c_int, 1) << @as(c_int, 3);
pub const DRM_MODE_FLAG_INTERLACE = @as(c_int, 1) << @as(c_int, 4);
pub const DRM_MODE_FLAG_DBLSCAN = @as(c_int, 1) << @as(c_int, 5);
pub const DRM_MODE_FLAG_CSYNC = @as(c_int, 1) << @as(c_int, 6);
pub const DRM_MODE_FLAG_PCSYNC = @as(c_int, 1) << @as(c_int, 7);
pub const DRM_MODE_FLAG_NCSYNC = @as(c_int, 1) << @as(c_int, 8);
pub const DRM_MODE_FLAG_HSKEW = @as(c_int, 1) << @as(c_int, 9);
pub const DRM_MODE_FLAG_BCAST = @as(c_int, 1) << @as(c_int, 10);
pub const DRM_MODE_FLAG_PIXMUX = @as(c_int, 1) << @as(c_int, 11);
pub const DRM_MODE_FLAG_DBLCLK = @as(c_int, 1) << @as(c_int, 12);
pub const DRM_MODE_FLAG_CLKDIV2 = @as(c_int, 1) << @as(c_int, 13);
pub const DRM_MODE_FLAG_3D_MASK = @as(c_int, 0x1f) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_NONE = @as(c_int, 0) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_FRAME_PACKING = @as(c_int, 1) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_FIELD_ALTERNATIVE = @as(c_int, 2) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_LINE_ALTERNATIVE = @as(c_int, 3) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_FULL = @as(c_int, 4) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_L_DEPTH = @as(c_int, 5) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_L_DEPTH_GFX_GFX_DEPTH = @as(c_int, 6) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_TOP_AND_BOTTOM = @as(c_int, 7) << @as(c_int, 14);
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_HALF = @as(c_int, 8) << @as(c_int, 14);
pub const DRM_MODE_PICTURE_ASPECT_NONE = @as(c_int, 0);
pub const DRM_MODE_PICTURE_ASPECT_4_3 = @as(c_int, 1);
pub const DRM_MODE_PICTURE_ASPECT_16_9 = @as(c_int, 2);
pub const DRM_MODE_PICTURE_ASPECT_64_27 = @as(c_int, 3);
pub const DRM_MODE_PICTURE_ASPECT_256_135 = @as(c_int, 4);
pub const DRM_MODE_CONTENT_TYPE_NO_DATA = @as(c_int, 0);
pub const DRM_MODE_CONTENT_TYPE_GRAPHICS = @as(c_int, 1);
pub const DRM_MODE_CONTENT_TYPE_PHOTO = @as(c_int, 2);
pub const DRM_MODE_CONTENT_TYPE_CINEMA = @as(c_int, 3);
pub const DRM_MODE_CONTENT_TYPE_GAME = @as(c_int, 4);
pub const DRM_MODE_FLAG_PIC_AR_MASK = @as(c_int, 0x0F) << @as(c_int, 19);
pub const DRM_MODE_FLAG_PIC_AR_NONE = DRM_MODE_PICTURE_ASPECT_NONE << @as(c_int, 19);
pub const DRM_MODE_FLAG_PIC_AR_4_3 = DRM_MODE_PICTURE_ASPECT_4_3 << @as(c_int, 19);
pub const DRM_MODE_FLAG_PIC_AR_16_9 = DRM_MODE_PICTURE_ASPECT_16_9 << @as(c_int, 19);
pub const DRM_MODE_FLAG_PIC_AR_64_27 = DRM_MODE_PICTURE_ASPECT_64_27 << @as(c_int, 19);
pub const DRM_MODE_FLAG_PIC_AR_256_135 = DRM_MODE_PICTURE_ASPECT_256_135 << @as(c_int, 19);
pub const DRM_MODE_FLAG_ALL = (((((((((((DRM_MODE_FLAG_PHSYNC | DRM_MODE_FLAG_NHSYNC) | DRM_MODE_FLAG_PVSYNC) | DRM_MODE_FLAG_NVSYNC) | DRM_MODE_FLAG_INTERLACE) | DRM_MODE_FLAG_DBLSCAN) | DRM_MODE_FLAG_CSYNC) | DRM_MODE_FLAG_PCSYNC) | DRM_MODE_FLAG_NCSYNC) | DRM_MODE_FLAG_HSKEW) | DRM_MODE_FLAG_DBLCLK) | DRM_MODE_FLAG_CLKDIV2) | DRM_MODE_FLAG_3D_MASK;
pub const DRM_MODE_DPMS_ON = @as(c_int, 0);
pub const DRM_MODE_DPMS_STANDBY = @as(c_int, 1);
pub const DRM_MODE_DPMS_SUSPEND = @as(c_int, 2);
pub const DRM_MODE_DPMS_OFF = @as(c_int, 3);
pub const DRM_MODE_SCALE_NONE = @as(c_int, 0);
pub const DRM_MODE_SCALE_FULLSCREEN = @as(c_int, 1);
pub const DRM_MODE_SCALE_CENTER = @as(c_int, 2);
pub const DRM_MODE_SCALE_ASPECT = @as(c_int, 3);
pub const DRM_MODE_DITHERING_OFF = @as(c_int, 0);
pub const DRM_MODE_DITHERING_ON = @as(c_int, 1);
pub const DRM_MODE_DITHERING_AUTO = @as(c_int, 2);
pub const DRM_MODE_DIRTY_OFF = @as(c_int, 0);
pub const DRM_MODE_DIRTY_ON = @as(c_int, 1);
pub const DRM_MODE_DIRTY_ANNOTATE = @as(c_int, 2);
pub const DRM_MODE_LINK_STATUS_GOOD = @as(c_int, 0);
pub const DRM_MODE_LINK_STATUS_BAD = @as(c_int, 1);
pub const DRM_MODE_ROTATE_0 = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_MODE_ROTATE_90 = @as(c_int, 1) << @as(c_int, 1);
pub const DRM_MODE_ROTATE_180 = @as(c_int, 1) << @as(c_int, 2);
pub const DRM_MODE_ROTATE_270 = @as(c_int, 1) << @as(c_int, 3);
pub const DRM_MODE_ROTATE_MASK = ((DRM_MODE_ROTATE_0 | DRM_MODE_ROTATE_90) | DRM_MODE_ROTATE_180) | DRM_MODE_ROTATE_270;
pub const DRM_MODE_REFLECT_X = @as(c_int, 1) << @as(c_int, 4);
pub const DRM_MODE_REFLECT_Y = @as(c_int, 1) << @as(c_int, 5);
pub const DRM_MODE_REFLECT_MASK = DRM_MODE_REFLECT_X | DRM_MODE_REFLECT_Y;
pub const DRM_MODE_CONTENT_PROTECTION_UNDESIRED = @as(c_int, 0);
pub const DRM_MODE_CONTENT_PROTECTION_DESIRED = @as(c_int, 1);
pub const DRM_MODE_CONTENT_PROTECTION_ENABLED = @as(c_int, 2);
pub const DRM_MODE_PRESENT_TOP_FIELD = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_MODE_PRESENT_BOTTOM_FIELD = @as(c_int, 1) << @as(c_int, 1);
pub const DRM_MODE_ENCODER_NONE = @as(c_int, 0);
pub const DRM_MODE_ENCODER_DAC = @as(c_int, 1);
pub const DRM_MODE_ENCODER_TMDS = @as(c_int, 2);
pub const DRM_MODE_ENCODER_LVDS = @as(c_int, 3);
pub const DRM_MODE_ENCODER_TVDAC = @as(c_int, 4);
pub const DRM_MODE_ENCODER_VIRTUAL = @as(c_int, 5);
pub const DRM_MODE_ENCODER_DSI = @as(c_int, 6);
pub const DRM_MODE_ENCODER_DPMST = @as(c_int, 7);
pub const DRM_MODE_ENCODER_DPI = @as(c_int, 8);
pub const DRM_MODE_CONNECTOR_Unknown = @as(c_int, 0);
pub const DRM_MODE_CONNECTOR_VGA = @as(c_int, 1);
pub const DRM_MODE_CONNECTOR_DVII = @as(c_int, 2);
pub const DRM_MODE_CONNECTOR_DVID = @as(c_int, 3);
pub const DRM_MODE_CONNECTOR_DVIA = @as(c_int, 4);
pub const DRM_MODE_CONNECTOR_Composite = @as(c_int, 5);
pub const DRM_MODE_CONNECTOR_SVIDEO = @as(c_int, 6);
pub const DRM_MODE_CONNECTOR_LVDS = @as(c_int, 7);
pub const DRM_MODE_CONNECTOR_Component = @as(c_int, 8);
pub const DRM_MODE_CONNECTOR_9PinDIN = @as(c_int, 9);
pub const DRM_MODE_CONNECTOR_DisplayPort = @as(c_int, 10);
pub const DRM_MODE_CONNECTOR_HDMIA = @as(c_int, 11);
pub const DRM_MODE_CONNECTOR_HDMIB = @as(c_int, 12);
pub const DRM_MODE_CONNECTOR_TV = @as(c_int, 13);
pub const DRM_MODE_CONNECTOR_eDP = @as(c_int, 14);
pub const DRM_MODE_CONNECTOR_VIRTUAL = @as(c_int, 15);
pub const DRM_MODE_CONNECTOR_DSI = @as(c_int, 16);
pub const DRM_MODE_CONNECTOR_DPI = @as(c_int, 17);
pub const DRM_MODE_CONNECTOR_WRITEBACK = @as(c_int, 18);
pub const DRM_MODE_CONNECTOR_SPI = @as(c_int, 19);
pub const DRM_MODE_CONNECTOR_USB = @as(c_int, 20);
pub const DRM_MODE_PROP_PENDING = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_MODE_PROP_RANGE = @as(c_int, 1) << @as(c_int, 1);
pub const DRM_MODE_PROP_IMMUTABLE = @as(c_int, 1) << @as(c_int, 2);
pub const DRM_MODE_PROP_ENUM = @as(c_int, 1) << @as(c_int, 3);
pub const DRM_MODE_PROP_BLOB = @as(c_int, 1) << @as(c_int, 4);
pub const DRM_MODE_PROP_BITMASK = @as(c_int, 1) << @as(c_int, 5);
pub const DRM_MODE_PROP_LEGACY_TYPE = ((DRM_MODE_PROP_RANGE | DRM_MODE_PROP_ENUM) | DRM_MODE_PROP_BLOB) | DRM_MODE_PROP_BITMASK;
pub const DRM_MODE_PROP_EXTENDED_TYPE = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0x0000ffc0, .hexadecimal);
pub inline fn DRM_MODE_PROP_TYPE(n: anytype) @TypeOf(n << @as(c_int, 6)) {
    return n << @as(c_int, 6);
}
pub const DRM_MODE_PROP_OBJECT = DRM_MODE_PROP_TYPE(@as(c_int, 1));
pub const DRM_MODE_PROP_SIGNED_RANGE = DRM_MODE_PROP_TYPE(@as(c_int, 2));
pub const DRM_MODE_PROP_ATOMIC = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0x80000000, .hexadecimal);
pub const DRM_MODE_OBJECT_CRTC = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xcccccccc, .hexadecimal);
pub const DRM_MODE_OBJECT_CONNECTOR = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xc0c0c0c0, .hexadecimal);
pub const DRM_MODE_OBJECT_ENCODER = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xe0e0e0e0, .hexadecimal);
pub const DRM_MODE_OBJECT_MODE = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xdededede, .hexadecimal);
pub const DRM_MODE_OBJECT_PROPERTY = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xb0b0b0b0, .hexadecimal);
pub const DRM_MODE_OBJECT_FB = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xfbfbfbfb, .hexadecimal);
pub const DRM_MODE_OBJECT_BLOB = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xbbbbbbbb, .hexadecimal);
pub const DRM_MODE_OBJECT_PLANE = @import("std").zig.c_translation.promoteIntLiteral(c_int, 0xeeeeeeee, .hexadecimal);
pub const DRM_MODE_OBJECT_ANY = @as(c_int, 0);
pub const DRM_MODE_FB_INTERLACED = @as(c_int, 1) << @as(c_int, 0);
pub const DRM_MODE_FB_MODIFIERS = @as(c_int, 1) << @as(c_int, 1);
pub const DRM_MODE_FB_DIRTY_ANNOTATE_COPY = @as(c_int, 0x01);
pub const DRM_MODE_FB_DIRTY_ANNOTATE_FILL = @as(c_int, 0x02);
pub const DRM_MODE_FB_DIRTY_FLAGS = @as(c_int, 0x03);
pub const DRM_MODE_FB_DIRTY_MAX_CLIPS = @as(c_int, 256);
pub const DRM_MODE_CURSOR_BO = @as(c_int, 0x01);
pub const DRM_MODE_CURSOR_MOVE = @as(c_int, 0x02);
pub const DRM_MODE_CURSOR_FLAGS = @as(c_int, 0x03);
pub const DRM_MODE_PAGE_FLIP_EVENT = @as(c_int, 0x01);
pub const DRM_MODE_PAGE_FLIP_ASYNC = @as(c_int, 0x02);
pub const DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE = @as(c_int, 0x4);
pub const DRM_MODE_PAGE_FLIP_TARGET_RELATIVE = @as(c_int, 0x8);
pub const DRM_MODE_PAGE_FLIP_TARGET = DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE | DRM_MODE_PAGE_FLIP_TARGET_RELATIVE;
pub const DRM_MODE_PAGE_FLIP_FLAGS = (DRM_MODE_PAGE_FLIP_EVENT | DRM_MODE_PAGE_FLIP_ASYNC) | DRM_MODE_PAGE_FLIP_TARGET;
pub const DRM_MODE_ATOMIC_TEST_ONLY = @as(c_int, 0x0100);
pub const DRM_MODE_ATOMIC_NONBLOCK = @as(c_int, 0x0200);
pub const DRM_MODE_ATOMIC_ALLOW_MODESET = @as(c_int, 0x0400);
pub const DRM_MODE_ATOMIC_FLAGS = (((DRM_MODE_PAGE_FLIP_EVENT | DRM_MODE_PAGE_FLIP_ASYNC) | DRM_MODE_ATOMIC_TEST_ONLY) | DRM_MODE_ATOMIC_NONBLOCK) | DRM_MODE_ATOMIC_ALLOW_MODESET;
pub const FORMAT_BLOB_CURRENT = @as(c_int, 1);
pub const DRM_IOCTL_BASE = 'd';
pub inline fn DRM_IO(nr: anytype) @TypeOf(_IO(DRM_IOCTL_BASE, nr)) {
    return _IO(DRM_IOCTL_BASE, nr);
}
pub inline fn DRM_IOR(nr: anytype, @"type": anytype) @TypeOf(_IOR(DRM_IOCTL_BASE, nr, @"type")) {
    return _IOR(DRM_IOCTL_BASE, nr, @"type");
}
pub inline fn DRM_IOW(nr: anytype, @"type": anytype) @TypeOf(_IOW(DRM_IOCTL_BASE, nr, @"type")) {
    return _IOW(DRM_IOCTL_BASE, nr, @"type");
}
pub inline fn DRM_IOWR(nr: anytype, @"type": anytype) @TypeOf(_IOWR(DRM_IOCTL_BASE, nr, @"type")) {
    return _IOWR(DRM_IOCTL_BASE, nr, @"type");
}
pub const DRM_IOCTL_VERSION = DRM_IOWR(@as(c_int, 0x00), struct_drm_version);
pub const DRM_IOCTL_GET_UNIQUE = DRM_IOWR(@as(c_int, 0x01), struct_drm_unique);
pub const DRM_IOCTL_GET_MAGIC = DRM_IOR(@as(c_int, 0x02), struct_drm_auth);
pub const DRM_IOCTL_IRQ_BUSID = DRM_IOWR(@as(c_int, 0x03), struct_drm_irq_busid);
pub const DRM_IOCTL_GET_MAP = DRM_IOWR(@as(c_int, 0x04), struct_drm_map);
pub const DRM_IOCTL_GET_CLIENT = DRM_IOWR(@as(c_int, 0x05), struct_drm_client);
pub const DRM_IOCTL_GET_STATS = DRM_IOR(@as(c_int, 0x06), struct_drm_stats);
pub const DRM_IOCTL_SET_VERSION = DRM_IOWR(@as(c_int, 0x07), struct_drm_set_version);
pub const DRM_IOCTL_MODESET_CTL = DRM_IOW(@as(c_int, 0x08), struct_drm_modeset_ctl);
pub const DRM_IOCTL_GEM_CLOSE = DRM_IOW(@as(c_int, 0x09), struct_drm_gem_close);
pub const DRM_IOCTL_GEM_FLINK = DRM_IOWR(@as(c_int, 0x0a), struct_drm_gem_flink);
pub const DRM_IOCTL_GEM_OPEN = DRM_IOWR(@as(c_int, 0x0b), struct_drm_gem_open);
pub const DRM_IOCTL_GET_CAP = DRM_IOWR(@as(c_int, 0x0c), struct_drm_get_cap);
pub const DRM_IOCTL_SET_CLIENT_CAP = DRM_IOW(@as(c_int, 0x0d), struct_drm_set_client_cap);
pub const DRM_IOCTL_SET_UNIQUE = DRM_IOW(@as(c_int, 0x10), struct_drm_unique);
pub const DRM_IOCTL_AUTH_MAGIC = DRM_IOW(@as(c_int, 0x11), struct_drm_auth);
pub const DRM_IOCTL_BLOCK = DRM_IOWR(@as(c_int, 0x12), struct_drm_block);
pub const DRM_IOCTL_UNBLOCK = DRM_IOWR(@as(c_int, 0x13), struct_drm_block);
pub const DRM_IOCTL_CONTROL = DRM_IOW(@as(c_int, 0x14), struct_drm_control);
pub const DRM_IOCTL_ADD_MAP = DRM_IOWR(@as(c_int, 0x15), struct_drm_map);
pub const DRM_IOCTL_ADD_BUFS = DRM_IOWR(@as(c_int, 0x16), struct_drm_buf_desc);
pub const DRM_IOCTL_MARK_BUFS = DRM_IOW(@as(c_int, 0x17), struct_drm_buf_desc);
pub const DRM_IOCTL_INFO_BUFS = DRM_IOWR(@as(c_int, 0x18), struct_drm_buf_info);
pub const DRM_IOCTL_MAP_BUFS = DRM_IOWR(@as(c_int, 0x19), struct_drm_buf_map);
pub const DRM_IOCTL_FREE_BUFS = DRM_IOW(@as(c_int, 0x1a), struct_drm_buf_free);
pub const DRM_IOCTL_RM_MAP = DRM_IOW(@as(c_int, 0x1b), struct_drm_map);
pub const DRM_IOCTL_SET_SAREA_CTX = DRM_IOW(@as(c_int, 0x1c), struct_drm_ctx_priv_map);
pub const DRM_IOCTL_GET_SAREA_CTX = DRM_IOWR(@as(c_int, 0x1d), struct_drm_ctx_priv_map);
pub const DRM_IOCTL_SET_MASTER = DRM_IO(@as(c_int, 0x1e));
pub const DRM_IOCTL_DROP_MASTER = DRM_IO(@as(c_int, 0x1f));
pub const DRM_IOCTL_ADD_CTX = DRM_IOWR(@as(c_int, 0x20), struct_drm_ctx);
pub const DRM_IOCTL_RM_CTX = DRM_IOWR(@as(c_int, 0x21), struct_drm_ctx);
pub const DRM_IOCTL_MOD_CTX = DRM_IOW(@as(c_int, 0x22), struct_drm_ctx);
pub const DRM_IOCTL_GET_CTX = DRM_IOWR(@as(c_int, 0x23), struct_drm_ctx);
pub const DRM_IOCTL_SWITCH_CTX = DRM_IOW(@as(c_int, 0x24), struct_drm_ctx);
pub const DRM_IOCTL_NEW_CTX = DRM_IOW(@as(c_int, 0x25), struct_drm_ctx);
pub const DRM_IOCTL_RES_CTX = DRM_IOWR(@as(c_int, 0x26), struct_drm_ctx_res);
pub const DRM_IOCTL_ADD_DRAW = DRM_IOWR(@as(c_int, 0x27), struct_drm_draw);
pub const DRM_IOCTL_RM_DRAW = DRM_IOWR(@as(c_int, 0x28), struct_drm_draw);
pub const DRM_IOCTL_DMA = DRM_IOWR(@as(c_int, 0x29), struct_drm_dma);
pub const DRM_IOCTL_LOCK = DRM_IOW(@as(c_int, 0x2a), struct_drm_lock);
pub const DRM_IOCTL_UNLOCK = DRM_IOW(@as(c_int, 0x2b), struct_drm_lock);
pub const DRM_IOCTL_FINISH = DRM_IOW(@as(c_int, 0x2c), struct_drm_lock);
pub const DRM_IOCTL_PRIME_HANDLE_TO_FD = DRM_IOWR(@as(c_int, 0x2d), struct_drm_prime_handle);
pub const DRM_IOCTL_PRIME_FD_TO_HANDLE = DRM_IOWR(@as(c_int, 0x2e), struct_drm_prime_handle);
pub const DRM_IOCTL_AGP_ACQUIRE = DRM_IO(@as(c_int, 0x30));
pub const DRM_IOCTL_AGP_RELEASE = DRM_IO(@as(c_int, 0x31));
pub const DRM_IOCTL_AGP_ENABLE = DRM_IOW(@as(c_int, 0x32), struct_drm_agp_mode);
pub const DRM_IOCTL_AGP_INFO = DRM_IOR(@as(c_int, 0x33), struct_drm_agp_info);
pub const DRM_IOCTL_AGP_ALLOC = DRM_IOWR(@as(c_int, 0x34), struct_drm_agp_buffer);
pub const DRM_IOCTL_AGP_FREE = DRM_IOW(@as(c_int, 0x35), struct_drm_agp_buffer);
pub const DRM_IOCTL_AGP_BIND = DRM_IOW(@as(c_int, 0x36), struct_drm_agp_binding);
pub const DRM_IOCTL_AGP_UNBIND = DRM_IOW(@as(c_int, 0x37), struct_drm_agp_binding);
pub const DRM_IOCTL_SG_ALLOC = DRM_IOWR(@as(c_int, 0x38), struct_drm_scatter_gather);
pub const DRM_IOCTL_SG_FREE = DRM_IOW(@as(c_int, 0x39), struct_drm_scatter_gather);
pub const DRM_IOCTL_WAIT_VBLANK = DRM_IOWR(@as(c_int, 0x3a), union_drm_wait_vblank);
pub const DRM_IOCTL_CRTC_GET_SEQUENCE = DRM_IOWR(@as(c_int, 0x3b), struct_drm_crtc_get_sequence);
pub const DRM_IOCTL_CRTC_QUEUE_SEQUENCE = DRM_IOWR(@as(c_int, 0x3c), struct_drm_crtc_queue_sequence);
pub const DRM_IOCTL_UPDATE_DRAW = DRM_IOW(@as(c_int, 0x3f), struct_drm_update_draw);
pub const DRM_IOCTL_MODE_GETRESOURCES = DRM_IOWR(@as(c_int, 0xA0), struct_drm_mode_card_res);
pub const DRM_IOCTL_MODE_GETCRTC = DRM_IOWR(@as(c_int, 0xA1), struct_drm_mode_crtc);
pub const DRM_IOCTL_MODE_SETCRTC = DRM_IOWR(@as(c_int, 0xA2), struct_drm_mode_crtc);
pub const DRM_IOCTL_MODE_CURSOR = DRM_IOWR(@as(c_int, 0xA3), struct_drm_mode_cursor);
pub const DRM_IOCTL_MODE_GETGAMMA = DRM_IOWR(@as(c_int, 0xA4), struct_drm_mode_crtc_lut);
pub const DRM_IOCTL_MODE_SETGAMMA = DRM_IOWR(@as(c_int, 0xA5), struct_drm_mode_crtc_lut);
pub const DRM_IOCTL_MODE_GETENCODER = DRM_IOWR(@as(c_int, 0xA6), struct_drm_mode_get_encoder);
pub const DRM_IOCTL_MODE_GETCONNECTOR = DRM_IOWR(@as(c_int, 0xA7), struct_drm_mode_get_connector);
pub const DRM_IOCTL_MODE_ATTACHMODE = DRM_IOWR(@as(c_int, 0xA8), struct_drm_mode_mode_cmd);
pub const DRM_IOCTL_MODE_DETACHMODE = DRM_IOWR(@as(c_int, 0xA9), struct_drm_mode_mode_cmd);
pub const DRM_IOCTL_MODE_GETPROPERTY = DRM_IOWR(@as(c_int, 0xAA), struct_drm_mode_get_property);
pub const DRM_IOCTL_MODE_SETPROPERTY = DRM_IOWR(@as(c_int, 0xAB), struct_drm_mode_connector_set_property);
pub const DRM_IOCTL_MODE_GETPROPBLOB = DRM_IOWR(@as(c_int, 0xAC), struct_drm_mode_get_blob);
pub const DRM_IOCTL_MODE_GETFB = DRM_IOWR(@as(c_int, 0xAD), struct_drm_mode_fb_cmd);
pub const DRM_IOCTL_MODE_ADDFB = DRM_IOWR(@as(c_int, 0xAE), struct_drm_mode_fb_cmd);
pub const DRM_IOCTL_MODE_RMFB = DRM_IOWR(@as(c_int, 0xAF), c_uint);
pub const DRM_IOCTL_MODE_PAGE_FLIP = DRM_IOWR(@as(c_int, 0xB0), struct_drm_mode_crtc_page_flip);
pub const DRM_IOCTL_MODE_DIRTYFB = DRM_IOWR(@as(c_int, 0xB1), struct_drm_mode_fb_dirty_cmd);
pub const DRM_IOCTL_MODE_CREATE_DUMB = DRM_IOWR(@as(c_int, 0xB2), struct_drm_mode_create_dumb);
pub const DRM_IOCTL_MODE_MAP_DUMB = DRM_IOWR(@as(c_int, 0xB3), struct_drm_mode_map_dumb);
pub const DRM_IOCTL_MODE_DESTROY_DUMB = DRM_IOWR(@as(c_int, 0xB4), struct_drm_mode_destroy_dumb);
pub const DRM_IOCTL_MODE_GETPLANERESOURCES = DRM_IOWR(@as(c_int, 0xB5), struct_drm_mode_get_plane_res);
pub const DRM_IOCTL_MODE_GETPLANE = DRM_IOWR(@as(c_int, 0xB6), struct_drm_mode_get_plane);
pub const DRM_IOCTL_MODE_SETPLANE = DRM_IOWR(@as(c_int, 0xB7), struct_drm_mode_set_plane);
pub const DRM_IOCTL_MODE_ADDFB2 = DRM_IOWR(@as(c_int, 0xB8), struct_drm_mode_fb_cmd2);
pub const DRM_IOCTL_MODE_OBJ_GETPROPERTIES = DRM_IOWR(@as(c_int, 0xB9), struct_drm_mode_obj_get_properties);
pub const DRM_IOCTL_MODE_OBJ_SETPROPERTY = DRM_IOWR(@as(c_int, 0xBA), struct_drm_mode_obj_set_property);
pub const DRM_IOCTL_MODE_CURSOR2 = DRM_IOWR(@as(c_int, 0xBB), struct_drm_mode_cursor2);
pub const DRM_IOCTL_MODE_ATOMIC = DRM_IOWR(@as(c_int, 0xBC), struct_drm_mode_atomic);
pub const DRM_IOCTL_MODE_CREATEPROPBLOB = DRM_IOWR(@as(c_int, 0xBD), struct_drm_mode_create_blob);
pub const DRM_IOCTL_MODE_DESTROYPROPBLOB = DRM_IOWR(@as(c_int, 0xBE), struct_drm_mode_destroy_blob);
pub const DRM_IOCTL_SYNCOBJ_CREATE = DRM_IOWR(@as(c_int, 0xBF), struct_drm_syncobj_create);
pub const DRM_IOCTL_SYNCOBJ_DESTROY = DRM_IOWR(@as(c_int, 0xC0), struct_drm_syncobj_destroy);
pub const DRM_IOCTL_SYNCOBJ_HANDLE_TO_FD = DRM_IOWR(@as(c_int, 0xC1), struct_drm_syncobj_handle);
pub const DRM_IOCTL_SYNCOBJ_FD_TO_HANDLE = DRM_IOWR(@as(c_int, 0xC2), struct_drm_syncobj_handle);
pub const DRM_IOCTL_SYNCOBJ_WAIT = DRM_IOWR(@as(c_int, 0xC3), struct_drm_syncobj_wait);
pub const DRM_IOCTL_SYNCOBJ_RESET = DRM_IOWR(@as(c_int, 0xC4), struct_drm_syncobj_array);
pub const DRM_IOCTL_SYNCOBJ_SIGNAL = DRM_IOWR(@as(c_int, 0xC5), struct_drm_syncobj_array);
pub const DRM_IOCTL_MODE_CREATE_LEASE = DRM_IOWR(@as(c_int, 0xC6), struct_drm_mode_create_lease);
pub const DRM_IOCTL_MODE_LIST_LESSEES = DRM_IOWR(@as(c_int, 0xC7), struct_drm_mode_list_lessees);
pub const DRM_IOCTL_MODE_GET_LEASE = DRM_IOWR(@as(c_int, 0xC8), struct_drm_mode_get_lease);
pub const DRM_IOCTL_MODE_REVOKE_LEASE = DRM_IOWR(@as(c_int, 0xC9), struct_drm_mode_revoke_lease);
pub const DRM_IOCTL_SYNCOBJ_TIMELINE_WAIT = DRM_IOWR(@as(c_int, 0xCA), struct_drm_syncobj_timeline_wait);
pub const DRM_IOCTL_SYNCOBJ_QUERY = DRM_IOWR(@as(c_int, 0xCB), struct_drm_syncobj_timeline_array);
pub const DRM_IOCTL_SYNCOBJ_TRANSFER = DRM_IOWR(@as(c_int, 0xCC), struct_drm_syncobj_transfer);
pub const DRM_IOCTL_SYNCOBJ_TIMELINE_SIGNAL = DRM_IOWR(@as(c_int, 0xCD), struct_drm_syncobj_timeline_array);
pub const DRM_IOCTL_MODE_GETFB2 = DRM_IOWR(@as(c_int, 0xCE), struct_drm_mode_fb_cmd2);
pub const DRM_COMMAND_BASE = @as(c_int, 0x40);
pub const DRM_COMMAND_END = @as(c_int, 0xA0);
pub const DRM_EVENT_VBLANK = @as(c_int, 0x01);
pub const DRM_EVENT_FLIP_COMPLETE = @as(c_int, 0x02);
pub const DRM_EVENT_CRTC_SEQUENCE = @as(c_int, 0x03);
pub const DRM_MAX_MINOR = @as(c_int, 64);
pub inline fn DRM_IOCTL_NR(n: anytype) @TypeOf(_IOC_NR(n)) {
    return _IOC_NR(n);
}
pub const DRM_IOC_VOID = _IOC_NONE;
pub const DRM_IOC_READ = _IOC_READ;
pub const DRM_IOC_WRITE = _IOC_WRITE;
pub const DRM_IOC_READWRITE = _IOC_READ | _IOC_WRITE;
pub inline fn DRM_IOC(dir: anytype, group: anytype, nr: anytype, size: anytype) @TypeOf(_IOC(dir, group, nr, size)) {
    return _IOC(dir, group, nr, size);
}
pub const DRM_DEV_UID = @as(c_int, 0);
pub const DRM_DEV_GID = @as(c_int, 0);
pub const DRM_DEV_DIRMODE = (((((S_IRUSR | S_IWUSR) | S_IXUSR) | S_IRGRP) | S_IXGRP) | S_IROTH) | S_IXOTH;
pub const DRM_DEV_MODE = ((S_IRUSR | S_IWUSR) | S_IRGRP) | S_IWGRP;
pub const DRM_DIR_NAME = "/dev/dri";
pub const DRM_PRIMARY_MINOR_NAME = "card";
pub const DRM_CONTROL_MINOR_NAME = "controlD";
pub const DRM_RENDER_MINOR_NAME = "renderD";
pub const DRM_PROC_NAME = "/proc/dri/";
pub const DRM_DEV_NAME = "%s/" ++ DRM_PRIMARY_MINOR_NAME ++ "%d";
pub const DRM_CONTROL_DEV_NAME = "%s/" ++ DRM_CONTROL_MINOR_NAME ++ "%d";
pub const DRM_RENDER_DEV_NAME = "%s/" ++ DRM_RENDER_MINOR_NAME ++ "%d";
pub const DRM_ERR_NO_DEVICE = -@as(c_int, 1001);
pub const DRM_ERR_NO_ACCESS = -@as(c_int, 1002);
pub const DRM_ERR_NOT_ROOT = -@as(c_int, 1003);
pub const DRM_ERR_INVALID = -@as(c_int, 1004);
pub const DRM_ERR_NO_FD = -@as(c_int, 1005);
pub const DRM_AGP_NO_HANDLE = @as(c_int, 0);
pub const DRM_VBLANK_HIGH_CRTC_SHIFT = @as(c_int, 1);
pub const DRM_LOCK_HELD = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 0x80000000, .hexadecimal);
pub const DRM_LOCK_CONT = @import("std").zig.c_translation.promoteIntLiteral(c_uint, 0x40000000, .hexadecimal);
pub inline fn DRM_CAS_RESULT(_result: anytype) @TypeOf(u8 ++ _result) {
    return u8 ++ _result;
}
pub const DRM_NODE_PRIMARY = @as(c_int, 0);
pub const DRM_NODE_CONTROL = @as(c_int, 1);
pub const DRM_NODE_RENDER = @as(c_int, 2);
pub const DRM_NODE_MAX = @as(c_int, 3);
pub const DRM_EVENT_CONTEXT_VERSION = @as(c_int, 4);
pub const DRM_BUS_PCI = @as(c_int, 0);
pub const DRM_BUS_USB = @as(c_int, 1);
pub const DRM_BUS_PLATFORM = @as(c_int, 2);
pub const DRM_BUS_HOST1X = @as(c_int, 3);
pub const DRM_PLATFORM_DEVICE_NAME_LEN = @as(c_int, 512);
pub const DRM_HOST1X_DEVICE_NAME_LEN = @as(c_int, 512);
pub const DRM_DEVICE_GET_PCI_REVISION = @as(c_int, 1) << @as(c_int, 0);
pub inline fn fourcc_mod_get_vendor(modifier: anytype) @TypeOf((modifier >> @as(c_int, 56)) & @as(c_int, 0xff)) {
    return (modifier >> @as(c_int, 56)) & @as(c_int, 0xff);
}
pub const _XF86DRMMODE_H_ = "";
pub const __STDBOOL_H = "";
pub const __bool_true_false_are_defined = @as(c_int, 1);
pub const @"bool" = bool;
pub const @"true" = @as(c_int, 1);
pub const @"false" = @as(c_int, 0);
pub const __STDDEF_H = "";
pub const __need_ptrdiff_t = "";
pub const __need_STDDEF_H_misc = "";
pub const _PTRDIFF_T = "";
pub const __CLANG_MAX_ALIGN_T_DEFINED = "";
pub const DRM_MODE_FEATURE_KMS = @as(c_int, 1);
pub const DRM_MODE_FEATURE_DIRTYFB = @as(c_int, 1);
pub const DRM_PLANE_TYPE_OVERLAY = @as(c_int, 0);
pub const DRM_PLANE_TYPE_PRIMARY = @as(c_int, 1);
pub const DRM_PLANE_TYPE_CURSOR = @as(c_int, 2);
pub const flock = struct_flock;
pub const timespec = struct_timespec;
pub const stat = struct_stat;
pub const __va_list_tag = struct___va_list_tag;
pub const _G_fpos_t = struct__G_fpos_t;
pub const _G_fpos64_t = struct__G_fpos64_t;
pub const _IO_marker = struct__IO_marker;
pub const _IO_codecvt = struct__IO_codecvt;
pub const _IO_wide_data = struct__IO_wide_data;
pub const _IO_FILE = struct__IO_FILE;
pub const _IO_cookie_io_functions_t = struct__IO_cookie_io_functions_t;
pub const timeval = struct_timeval;
pub const __pthread_internal_list = struct___pthread_internal_list;
pub const __pthread_internal_slist = struct___pthread_internal_slist;
pub const __pthread_mutex_s = struct___pthread_mutex_s;
pub const __pthread_rwlock_arch_t = struct___pthread_rwlock_arch_t;
pub const __pthread_cond_s = struct___pthread_cond_s;
pub const random_data = struct_random_data;
pub const drand48_data = struct_drand48_data;
pub const __locale_data = struct___locale_data;
pub const __locale_struct = struct___locale_struct;
pub const drm_clip_rect = struct_drm_clip_rect;
pub const drm_drawable_info = struct_drm_drawable_info;
pub const drm_tex_region = struct_drm_tex_region;
pub const drm_hw_lock = struct_drm_hw_lock;
pub const drm_version = struct_drm_version;
pub const drm_unique = struct_drm_unique;
pub const drm_list = struct_drm_list;
pub const drm_block = struct_drm_block;
pub const drm_control = struct_drm_control;
pub const drm_map_type = enum_drm_map_type;
pub const drm_map_flags = enum_drm_map_flags;
pub const drm_ctx_priv_map = struct_drm_ctx_priv_map;
pub const drm_map = struct_drm_map;
pub const drm_client = struct_drm_client;
pub const drm_stat_type = enum_drm_stat_type;
pub const drm_stats = struct_drm_stats;
pub const drm_lock_flags = enum_drm_lock_flags;
pub const drm_lock = struct_drm_lock;
pub const drm_dma_flags = enum_drm_dma_flags;
pub const drm_buf_desc = struct_drm_buf_desc;
pub const drm_buf_info = struct_drm_buf_info;
pub const drm_buf_free = struct_drm_buf_free;
pub const drm_buf_pub = struct_drm_buf_pub;
pub const drm_buf_map = struct_drm_buf_map;
pub const drm_dma = struct_drm_dma;
pub const drm_ctx_flags = enum_drm_ctx_flags;
pub const drm_ctx = struct_drm_ctx;
pub const drm_ctx_res = struct_drm_ctx_res;
pub const drm_draw = struct_drm_draw;
pub const drm_update_draw = struct_drm_update_draw;
pub const drm_auth = struct_drm_auth;
pub const drm_irq_busid = struct_drm_irq_busid;
pub const drm_vblank_seq_type = enum_drm_vblank_seq_type;
pub const drm_wait_vblank_request = struct_drm_wait_vblank_request;
pub const drm_wait_vblank_reply = struct_drm_wait_vblank_reply;
pub const drm_wait_vblank = union_drm_wait_vblank;
pub const drm_modeset_ctl = struct_drm_modeset_ctl;
pub const drm_agp_mode = struct_drm_agp_mode;
pub const drm_agp_buffer = struct_drm_agp_buffer;
pub const drm_agp_binding = struct_drm_agp_binding;
pub const drm_agp_info = struct_drm_agp_info;
pub const drm_scatter_gather = struct_drm_scatter_gather;
pub const drm_set_version = struct_drm_set_version;
pub const drm_gem_close = struct_drm_gem_close;
pub const drm_gem_flink = struct_drm_gem_flink;
pub const drm_gem_open = struct_drm_gem_open;
pub const drm_get_cap = struct_drm_get_cap;
pub const drm_set_client_cap = struct_drm_set_client_cap;
pub const drm_prime_handle = struct_drm_prime_handle;
pub const drm_syncobj_create = struct_drm_syncobj_create;
pub const drm_syncobj_destroy = struct_drm_syncobj_destroy;
pub const drm_syncobj_handle = struct_drm_syncobj_handle;
pub const drm_syncobj_transfer = struct_drm_syncobj_transfer;
pub const drm_syncobj_wait = struct_drm_syncobj_wait;
pub const drm_syncobj_timeline_wait = struct_drm_syncobj_timeline_wait;
pub const drm_syncobj_array = struct_drm_syncobj_array;
pub const drm_syncobj_timeline_array = struct_drm_syncobj_timeline_array;
pub const drm_crtc_get_sequence = struct_drm_crtc_get_sequence;
pub const drm_crtc_queue_sequence = struct_drm_crtc_queue_sequence;
pub const drm_mode_modeinfo = struct_drm_mode_modeinfo;
pub const drm_mode_card_res = struct_drm_mode_card_res;
pub const drm_mode_crtc = struct_drm_mode_crtc;
pub const drm_mode_set_plane = struct_drm_mode_set_plane;
pub const drm_mode_get_plane = struct_drm_mode_get_plane;
pub const drm_mode_get_plane_res = struct_drm_mode_get_plane_res;
pub const drm_mode_get_encoder = struct_drm_mode_get_encoder;
pub const drm_mode_subconnector = enum_drm_mode_subconnector;
pub const drm_mode_get_connector = struct_drm_mode_get_connector;
pub const drm_mode_property_enum = struct_drm_mode_property_enum;
pub const drm_mode_get_property = struct_drm_mode_get_property;
pub const drm_mode_connector_set_property = struct_drm_mode_connector_set_property;
pub const drm_mode_obj_get_properties = struct_drm_mode_obj_get_properties;
pub const drm_mode_obj_set_property = struct_drm_mode_obj_set_property;
pub const drm_mode_get_blob = struct_drm_mode_get_blob;
pub const drm_mode_fb_cmd = struct_drm_mode_fb_cmd;
pub const drm_mode_fb_cmd2 = struct_drm_mode_fb_cmd2;
pub const drm_mode_fb_dirty_cmd = struct_drm_mode_fb_dirty_cmd;
pub const drm_mode_mode_cmd = struct_drm_mode_mode_cmd;
pub const drm_mode_cursor = struct_drm_mode_cursor;
pub const drm_mode_cursor2 = struct_drm_mode_cursor2;
pub const drm_mode_crtc_lut = struct_drm_mode_crtc_lut;
pub const drm_color_ctm = struct_drm_color_ctm;
pub const drm_color_lut = struct_drm_color_lut;
pub const hdr_metadata_infoframe = struct_hdr_metadata_infoframe;
pub const hdr_output_metadata = struct_hdr_output_metadata;
pub const drm_mode_crtc_page_flip = struct_drm_mode_crtc_page_flip;
pub const drm_mode_crtc_page_flip_target = struct_drm_mode_crtc_page_flip_target;
pub const drm_mode_create_dumb = struct_drm_mode_create_dumb;
pub const drm_mode_map_dumb = struct_drm_mode_map_dumb;
pub const drm_mode_destroy_dumb = struct_drm_mode_destroy_dumb;
pub const drm_mode_atomic = struct_drm_mode_atomic;
pub const drm_format_modifier_blob = struct_drm_format_modifier_blob;
pub const drm_format_modifier = struct_drm_format_modifier;
pub const drm_mode_create_blob = struct_drm_mode_create_blob;
pub const drm_mode_destroy_blob = struct_drm_mode_destroy_blob;
pub const drm_mode_create_lease = struct_drm_mode_create_lease;
pub const drm_mode_list_lessees = struct_drm_mode_list_lessees;
pub const drm_mode_get_lease = struct_drm_mode_get_lease;
pub const drm_mode_revoke_lease = struct_drm_mode_revoke_lease;
pub const drm_mode_rect = struct_drm_mode_rect;
pub const drm_event = struct_drm_event;
pub const drm_event_vblank = struct_drm_event_vblank;
pub const drm_event_crtc_sequence = struct_drm_event_crtc_sequence;
pub const _drmServerInfo = struct__drmServerInfo;
pub const _drmVersion = struct__drmVersion;
pub const _drmStats = struct__drmStats;
pub const _drmBufDesc = struct__drmBufDesc;
pub const _drmBufInfo = struct__drmBufInfo;
pub const _drmBuf = struct__drmBuf;
pub const _drmBufMap = struct__drmBufMap;
pub const _drmLock = struct__drmLock;
pub const _drmDMAReq = struct__drmDMAReq;
pub const _drmRegion = struct__drmRegion;
pub const _drmTextureRegion = struct__drmTextureRegion;
pub const _drmVBlankReq = struct__drmVBlankReq;
pub const _drmVBlankReply = struct__drmVBlankReply;
pub const _drmVBlank = union__drmVBlank;
pub const _drmSetVersion = struct__drmSetVersion;
pub const _drmEventContext = struct__drmEventContext;
pub const _drmPciBusInfo = struct__drmPciBusInfo;
pub const _drmPciDeviceInfo = struct__drmPciDeviceInfo;
pub const _drmUsbBusInfo = struct__drmUsbBusInfo;
pub const _drmUsbDeviceInfo = struct__drmUsbDeviceInfo;
pub const _drmPlatformBusInfo = struct__drmPlatformBusInfo;
pub const _drmPlatformDeviceInfo = struct__drmPlatformDeviceInfo;
pub const _drmHost1xBusInfo = struct__drmHost1xBusInfo;
pub const _drmHost1xDeviceInfo = struct__drmHost1xDeviceInfo;
pub const _drmDevice = struct__drmDevice;
pub const _drmModeRes = struct__drmModeRes;
pub const _drmModeModeInfo = struct__drmModeModeInfo;
pub const _drmModeFB = struct__drmModeFB;
pub const _drmModeFB2 = struct__drmModeFB2;
pub const _drmModePropertyBlob = struct__drmModePropertyBlob;
pub const _drmModeProperty = struct__drmModeProperty;
pub const _drmModeCrtc = struct__drmModeCrtc;
pub const _drmModeEncoder = struct__drmModeEncoder;
pub const _drmModeConnector = struct__drmModeConnector;
pub const _drmModeObjectProperties = struct__drmModeObjectProperties;
pub const _drmModeFormatModifierIterator = struct__drmModeFormatModifierIterator;
pub const _drmModePlane = struct__drmModePlane;
pub const _drmModePlaneRes = struct__drmModePlaneRes;
pub const _drmModeAtomicReq = struct__drmModeAtomicReq;
pub const drmModeLesseeList = struct_drmModeLesseeList;
pub const drmModeObjectList = struct_drmModeObjectList;
