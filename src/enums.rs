/// #
/// BiosAtaTranslation:
///
/// Policy that BIOS should use to interpret cylinder/head/sector
/// addresses.  Note that Bochs BIOS and SeaBIOS will not actually
/// translate logical CHS to physical; instead, they will use logical
/// block addressing.
///
/// @auto: If cylinder/heads/sizes are passed, choose between none and LBA
///        depending on the size of the disk.  If they are not passed,
///        choose none if QEMU can guess that the disk had 16 or fewer
///        heads, large if QEMU can guess that the disk had 131072 or
///        fewer tracks across all heads (i.e. cylinders*heads<131072),
///        otherwise LBA.
///
/// @none: The physical disk geometry is equal to the logical geometry.
///
/// @lba: Assume 63 sectors per track and one of 16, 32, 64, 128 or 255
///       heads (if fewer than 255 are enough to cover the whole disk
///       with 1024 cylinders/head).  The number of cylinders/head is
///       then computed based on the number of sectors and heads.
///
/// @large: The number of cylinders per head is scaled down to 1024
///         by correspondingly scaling up the number of heads.
///
/// @rechs: Same as @large, but first convert a 16-head geometry to
///         15-head, by proportionally scaling up the number of
///         cylinders/head.
///
/// Since: 2.0
/// #
#[derive(Debug,RustcDecodable)]
pub enum BiosAtaTranslation {
    auto,
    none,
    lba,
    large,
    rechs,
}
/// #
/// @BlockDeviceIoStatus:
///
/// An enumeration of block device I/O status.
///
/// @ok: The last I/O operation has succeeded
///
/// @failed: The last I/O operation has failed
///
/// @nospace: The last I/O operation has failed due to a no-space condition
///
/// Since: 1.0
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlockDeviceIoStatus {
    ok,
    failed,
    nospace,
}
/// #
/// @BlockdevOnError:
///
/// An enumeration of possible behaviors for errors on I/O operations.
/// The exact meaning depends on whether the I/O was initiated by a guest
/// or by a block job
///
/// @report: for guest operations, report the error to the guest;
///          for jobs, cancel the job
///
/// @ignore: ignore the error, only report a QMP event (BLOCK_IO_ERROR
///          or BLOCK_JOB_ERROR)
///
/// @enospc: same as @stop on ENOSPC, same as @report otherwise.
///
/// @stop: for guest operations, stop the virtual machine;
///        for jobs, pause the job
///
/// Since: 1.3
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlockdevOnError {
    report,
    ignore,
    enospc,
    stop,
}
/// #
/// @MirrorSyncMode:
///
/// An enumeration of possible behaviors for the initial synchronization
/// phase of storage mirroring.
///
/// @top: copies data in the topmost image to the destination
///
/// @full: copies data from all images to the destination
///
/// @none: only copy data written from now on
///
/// @dirty-bitmap: only copy data described by the dirty bitmap. Since: 2.4
///
/// Since: 1.3
/// #
#[derive(Debug,RustcDecodable)]
pub enum MirrorSyncMode {
    top,
    full,
    none,
    dirty_bitmap,
}
/// #
/// @BlockJobType:
///
/// Type of a block job.
///
/// @commit: block commit job type, see "block-commit"
///
/// @stream: block stream job type, see "block-stream"
///
/// @mirror: drive mirror job type, see "drive-mirror"
///
/// @backup: drive backup job type, see "drive-backup"
///
/// Since: 1.7
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlockJobType {
    commit,
    stream,
    mirror,
    backup,
}
/// #
/// @NewImageMode
///
/// An enumeration that tells QEMU how to set the backing file path in
/// a new image file.
///
/// @existing: QEMU should look for an existing image file.
///
/// @absolute-paths: QEMU should create a new image with absolute paths
/// for the backing file. If there is no backing file available, the new
/// image will not be backed either.
///
/// Since: 1.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum NewImageMode {
    existing,
    absolute_paths,
}
/// #
/// @BlockdevDiscardOptions
///
/// Determines how to handle discard requests.
///
/// @ignore:      Ignore the request
/// @unmap:       Forward as an unmap request
///
/// Since: 1.7
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlockdevDiscardOptions {
    ignore,
    unmap,
}
/// #
/// @BlockdevDetectZeroesOptions
///
/// Describes the operation mode for the automatic conversion of plain
/// zero writes by the OS to driver specific optimized zero write commands.
///
/// @off:      Disabled (default)
/// @on:       Enabled
/// @unmap:    Enabled and even try to unmap blocks if possible. This requires
///            also that @BlockdevDiscardOptions is set to unmap for this device.
///
/// Since: 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlockdevDetectZeroesOptions {
    off,
    on,
    unmap,
}
/// #
/// @BlockdevAioOptions
///
/// Selects the AIO backend to handle I/O requests
///
/// @threads:     Use qemu"s thread pool
/// @native:      Use native AIO backend (only Linux and Windows)
///
/// Since: 1.7
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlockdevAioOptions {
    threads,
    native,
}
/// #
/// @BlockdevDriver
///
/// Drivers that are supported in block device operations.
///
/// @host_device, @host_cdrom, @host_floppy: Since 2.1
/// @host_floppy: deprecated since 2.3
///
/// Since: 2.0
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlockdevDriver {
    archipelago,
    blkdebug,
    blkverify,
    bochs,
    cloop,
    dmg,
    file,
    ftp,
    ftps,
    host_cdrom,
    host_device,
    host_floppy,
    http,
    https,
    null_aio,
    null_co,
    parallels,
    qcow,
    qcow2,
    qed,
    quorum,
    raw,
    tftp,
    vdi,
    vhdx,
    vmdk,
    vpc,
    vvfat,
}
/// #
/// @Qcow2OverlapCheckMode
///
/// General overlap check modes.
///
/// @none:        Do not perform any checks
///
/// @constant:    Perform only checks which can be done in constant time and
///               without reading anything from disk
///
/// @cached:      Perform only checks which can be done without reading anything
///               from disk
///
/// @all:         Perform all available overlap checks
///
/// Since: 2.2
/// #
#[derive(Debug,RustcDecodable)]
pub enum Qcow2OverlapCheckMode {
    none,
    constant,
    cached,
    all,
}
/// #
/// @BlkdebugEvent
///
/// Trigger events supported by blkdebug.
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlkdebugEvent {
    l1_update,
    l1_grow_alloc_table,
    l1_grow_write_table,
    l1_grow_activate_table,
    l2_load,
    l2_update,
    l2_update_compressed,
    l2_alloc_cow_read,
    l2_alloc_write,
    read_aio,
    read_backing_aio,
    read_compressed,
    write_aio,
    write_compressed,
    vmstate_load,
    vmstate_save,
    cow_read,
    cow_write,
    reftable_load,
    reftable_grow,
    reftable_update,
    refblock_load,
    refblock_update,
    refblock_update_part,
    refblock_alloc,
    refblock_alloc_hookup,
    refblock_alloc_write,
    refblock_alloc_write_blocks,
    refblock_alloc_write_table,
    refblock_alloc_switch_table,
    cluster_alloc,
    cluster_alloc_bytes,
    cluster_free,
    flush_to_os,
    flush_to_disk,
    pwritev_rmw_head,
    pwritev_rmw_after_head,
    pwritev_rmw_tail,
    pwritev_rmw_after_tail,
    pwritev,
    pwritev_zero,
    pwritev_done,
    empty_image_prepare,
}
/// #
/// @QuorumReadPattern
///
/// An enumeration of quorum read patterns.
///
/// @quorum: read all the children and do a quorum vote on reads
///
/// @fifo: read only from the first child that has not failed
///
/// Since: 2.2
/// #
#[derive(Debug,RustcDecodable)]
pub enum QuorumReadPattern {
    quorum,
    fifo,
}
/// #
/// @BlockErrorAction
///
/// An enumeration of action that has been taken when a DISK I/O occurs
///
/// @ignore: error has been ignored
///
/// @report: error has been reported to the device
///
/// @stop: error caused VM to be stopped
///
/// Since: 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum BlockErrorAction {
    ignore,
    report,
    stop,
}
/// @PreallocMode
///
/// Preallocation mode of QEMU image file
///
/// @off: no preallocation
/// @metadata: preallocate only for metadata
/// @falloc: like @full preallocation but allocate disk space by
///          posix_fallocate() rather than writing zeros.
/// @full: preallocate all data by writing zeros to device to ensure disk
///        space is really available. @full preallocation also sets up
///        metadata correctly.
///
/// Since 2.2
/// #
#[derive(Debug,RustcDecodable)]
pub enum PreallocMode {
    off,
    metadata,
    falloc,
    full,
}
/// -*- Mode: Python -*-
///
/// QAPI common definitions
#[derive(Debug,RustcDecodable)]
pub enum ErrorClass {
    GenericError,
    CommandNotFound,
    DeviceEncrypted,
    DeviceNotActive,
    DeviceNotFound,
    KVMMissingCap,
}
/// #
/// @OnOffAuto
///
/// An enumeration of three options: on, off, and auto
///
/// @auto: QEMU selects the value between on and off
///
/// @on: Enabled
///
/// @off: Disabled
///
/// Since: 2.2
/// #
#[derive(Debug,RustcDecodable)]
pub enum OnOffAuto {
    auto,
    on,
    off,
}
/// -*- mode: python -*-
///
/// Copyright (C) 2011-2014 Lluís Vilanova <vilanova@ac.upc.edu>
///
/// This work is licensed under the terms of the GNU GPL, version 2 or later.
/// See the COPYING file in the top-level directory.
#[derive(Debug,RustcDecodable)]
pub enum TraceEventState {
    unavailable,
    disabled,
    enabled,
}
/// #
/// LostTickPolicy:
///
/// Policy for handling lost ticks in timer devices.
///
/// @discard: throw away the missed tick(s) and continue with future injection
///           normally.  Guest time may be delayed, unless the OS has explicit
///           handling of lost ticks
///
/// @delay: continue to deliver ticks at the normal rate.  Guest time will be
///         delayed due to the late tick
///
/// @merge: merge the missed tick(s) into one tick and inject.  Guest time
///         may be delayed, depending on how the OS reacts to the merging
///         of ticks
///
/// @slew: deliver ticks at a higher rate to catch up with the missed tick. The
///        guest time should not be delayed once catchup is complete.
///
/// Since: 2.0
/// #
#[derive(Debug,RustcDecodable)]
pub enum LostTickPolicy {
    discard,
    delay,
    merge,
    slew,
}
/// #
/// @RunState
///
/// An enumeration of VM run states.
///
/// @debug: QEMU is running on a debugger
///
/// @finish-migrate: guest is paused to finish the migration process
///
/// @inmigrate: guest is paused waiting for an incoming migration.  Note
/// that this state does not tell whether the machine will start at the
/// end of the migration.  This depends on the command-line -S option and
/// any invocation of "stop" or "cont" that has happened since QEMU was
/// started.
///
/// @internal-error: An internal error that prevents further guest execution
/// has occurred
///
/// @io-error: the last IOP has failed and the device is configured to pause
/// on I/O errors
///
/// @paused: guest has been paused via the "stop" command
///
/// @postmigrate: guest is paused following a successful "migrate"
///
/// @prelaunch: QEMU was started with -S and guest has not started
///
/// @restore-vm: guest is paused to restore VM state
///
/// @running: guest is actively running
///
/// @save-vm: guest is paused to save the VM state
///
/// @shutdown: guest is shut down (and -no-shutdown is in use)
///
/// @suspended: guest is suspended (ACPI S3)
///
/// @watchdog: the watchdog action is configured to pause and has been triggered
///
/// @guest-panicked: guest has been panicked as a result of guest OS panic
/// #
#[derive(Debug,RustcDecodable)]
pub enum RunState {
    debug,
    inmigrate,
    internal_error,
    io_error,
    paused,
    postmigrate,
    prelaunch,
    finish_migrate,
    restore_vm,
    running,
    save_vm,
    shutdown,
    suspended,
    watchdog,
    guest_panicked,
}
/// #
/// @DataFormat:
///
/// An enumeration of data format.
///
/// @utf8: Data is a UTF-8 string (RFC 3629)
///
/// @base64: Data is Base64 encoded binary (RFC 3548)
///
/// Since: 1.4
/// #
#[derive(Debug,RustcDecodable)]
pub enum DataFormat {
    utf8,
    base64,
}
/// @MigrationStatus:
///
/// An enumeration of migration status.
///
/// @none: no migration has ever happened.
///
/// @setup: migration process has been initiated.
///
/// @cancelling: in the process of cancelling migration.
///
/// @cancelled: cancelling migration is finished.
///
/// @active: in the process of doing migration.
///
/// @completed: migration is finished.
///
/// @failed: some error occurred during migration process.
///
/// Since: 2.3
///
/// #
#[derive(Debug,RustcDecodable)]
pub enum MigrationStatus {
    none,
    setup,
    cancelling,
    cancelled,
    active,
    completed,
    failed,
}
/// #
/// @MigrationCapability
///
/// Migration capabilities enumeration
///
/// @xbzrle: Migration supports xbzrle (Xor Based Zero Run Length Encoding).
///          This feature allows us to minimize migration traffic for certain work
///          loads, by sending compressed difference of the pages
///
/// @rdma-pin-all: Controls whether or not the entire VM memory footprint is
///          mlock()"d on demand or all at once. Refer to docs/rdma.txt for usage.
///          Disabled by default. (since 2.0)
///
/// @zero-blocks: During storage migration encode blocks of zeroes efficiently. This
///          essentially saves 1MB of zeroes per block on the wire. Enabling requires
///          source and target VM to support this feature. To enable it is sufficient
///          to enable the capability on the source VM. The feature is disabled by
///          default. (since 1.6)
///
/// @compress: Use multiple compression threads to accelerate live migration.
///          This feature can help to reduce the migration traffic, by sending
///          compressed pages. Please note that if compress and xbzrle are both
///          on, compress only takes effect in the ram bulk stage, after that,
///          it will be disabled and only xbzrle takes effect, this can help to
///          minimize migration traffic. The feature is disabled by default.
///          (since 2.4 )
///
/// @auto-converge: If enabled, QEMU will automatically throttle down the guest
///          to speed up convergence of RAM migration. (since 1.6)
///
/// Since: 1.2
/// #
#[derive(Debug,RustcDecodable)]
pub enum MigrationCapability {
    xbzrle,
    rdma_pin_all,
    auto_converge,
    zero_blocks,
    compress,
}
/// @MigrationParameter
///
/// Migration parameters enumeration
///
/// @compress-level: Set the compression level to be used in live migration,
///          the compression level is an integer between 0 and 9, where 0 means
///          no compression, 1 means the best compression speed, and 9 means best
///          compression ratio which will consume more CPU.
///
/// @compress-threads: Set compression thread count to be used in live migration,
///          the compression thread count is an integer between 1 and 255.
///
/// @decompress-threads: Set decompression thread count to be used in live
///          migration, the decompression thread count is an integer between 1
///          and 255. Usually, decompression is at least 4 times as fast as
///          compression, so set the decompress-threads to the number about 1/4
///          of compress-threads is adequate.
///
/// Since: 2.4
/// #
#[derive(Debug,RustcDecodable)]
pub enum MigrationParameter {
    compress_level,
    compress_threads,
    decompress_threads,
}
/// #
/// @NetworkAddressFamily
///
/// The network address family
///
/// @ipv4: IPV4 family
///
/// @ipv6: IPV6 family
///
/// @unix: unix socket
///
/// @unknown: otherwise
///
/// Since: 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum NetworkAddressFamily {
    ipv4,
    ipv6,
    unix,
    unknown,
}
/// #
/// @VncPriAuth:
///
/// vnc primary authentication method.
///
/// Since: 2.3
/// #
#[derive(Debug,RustcDecodable)]
pub enum VncPrimaryAuth {
    none,
    vnc,
    ra2,
    ra2ne,
    tight,
    ultra,
    tls,
    vencrypt,
    sasl,
}
/// #
/// @VncVencryptSubAuth:
///
/// vnc sub authentication method with vencrypt.
///
/// Since: 2.3
/// #
#[derive(Debug,RustcDecodable)]
pub enum VncVencryptSubAuth {
    plain,
    tls_none,
    x509_none,
    tls_vnc,
    x509_vnc,
    tls_plain,
    x509_plain,
    tls_sasl,
    x509_sasl,
}
/// #
/// @SpiceQueryMouseMode
///
/// An enumeration of Spice mouse states.
///
/// @client: Mouse cursor position is determined by the client.
///
/// @server: Mouse cursor position is determined by the server.
///
/// @unknown: No information is available about mouse mode used by
///           the spice server.
///
/// Note: spice/enums.h has a SpiceMouseMode already, hence the name.
///
/// Since: 1.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum SpiceQueryMouseMode {
    client,
    server,
    unknown,
}
/// #
/// @DumpGuestMemoryFormat:
///
/// An enumeration of guest-memory-dump"s format.
///
/// @elf: elf format
///
/// @kdump-zlib: kdump-compressed format with zlib-compressed
///
/// @kdump-lzo: kdump-compressed format with lzo-compressed
///
/// @kdump-snappy: kdump-compressed format with snappy-compressed
///
/// Since: 2.0
/// #
#[derive(Debug,RustcDecodable)]
pub enum DumpGuestMemoryFormat {
    elf,
    kdump_zlib,
    kdump_lzo,
    kdump_snappy,
}
/// #
/// @QKeyCode:
///
/// An enumeration of key name.
///
/// This is used by the send-key command.
///
/// Since: 1.3.0
///
/// "unmapped" and "pause" since 2.0
/// #
#[derive(Debug,RustcDecodable)]
pub enum QKeyCode {
    unmapped,
    shift,
    shift_r,
    alt,
    alt_r,
    altgr,
    altgr_r,
    ctrl,
    ctrl_r,
    menu,
    esc,
    qemu_1,
    qemu_2,
    qemu_3,
    qemu_4,
    qemu_5,
    qemu_6,
    qemu_7,
    qemu_8,
    qemu_9,
    qemu_0,
    minus,
    equal,
    backspace,
    tab,
    q,
    w,
    e,
    r,
    t,
    y,
    u,
    i,
    o,
    p,
    bracket_left,
    bracket_right,
    ret,
    a,
    s,
    d,
    f,
    g,
    h,
    j,
    k,
    l,
    semicolon,
    apostrophe,
    grave_accent,
    backslash,
    z,
    x,
    c,
    v,
    b,
    n,
    m,
    comma,
    dot,
    slash,
    asterisk,
    spc,
    caps_lock,
    f1,
    f2,
    f3,
    f4,
    f5,
    f6,
    f7,
    f8,
    f9,
    f10,
    num_lock,
    scroll_lock,
    kp_divide,
    kp_multiply,
    kp_subtract,
    kp_add,
    kp_enter,
    kp_decimal,
    sysrq,
    kp_0,
    kp_1,
    kp_2,
    kp_3,
    kp_4,
    kp_5,
    kp_6,
    kp_7,
    kp_8,
    kp_9,
    less,
    f11,
    f12,
    print,
    home,
    pgup,
    pgdn,
    end,
    left,
    up,
    down,
    right,
    insert,
    delete,
    stop,
    again,
    props,
    undo,
    front,
    copy,
    open,
    paste,
    find,
    cut,
    lf,
    help,
    meta_l,
    meta_r,
    compose,
    pause,
}
/// #
/// @TpmModel:
///
/// An enumeration of TPM models
///
/// @tpm-tis: TPM TIS model
///
/// Since: 1.5
/// #
#[derive(Debug,RustcDecodable)]
pub enum TpmModel {
    tpm_tis,
}
/// #
/// @TpmType:
///
/// An enumeration of TPM types
///
/// @passthrough: TPM passthrough type
///
/// Since: 1.5
/// #
#[derive(Debug,RustcDecodable)]
pub enum TpmType {
    passthrough,
}
/// #
/// @CommandLineParameterType:
///
/// Possible types for an option parameter.
///
/// @string: accepts a character string
///
/// @boolean: accepts "on" or "off"
///
/// @number: accepts a number
///
/// @size: accepts a number followed by an optional suffix (K)ilo,
///        (M)ega, (G)iga, (T)era
///
/// Since 1.5
/// #
#[derive(Debug,RustcDecodable)]
pub enum CommandLineParameterType {
    string,
    boolean,
    number,
    size,
}
/// #
/// @X86CPURegister32
///
/// A X86 32-bit register
///
/// Since: 1.5
/// #
#[derive(Debug,RustcDecodable)]
pub enum X86CPURegister32 {
    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,
}
/// #
/// @RxState:
///
/// Packets receiving state
///
/// @normal: filter assigned packets according to the mac-table
///
/// @none: don"t receive any assigned packet
///
/// @all: receive all assigned packets
///
/// Since: 1.6
/// #
#[derive(Debug,RustcDecodable)]
pub enum RxState {
    normal,
    none,
    all,
}
/// #
/// @InputButton
///
/// Button of a pointer input device (mouse, tablet).
///
/// Since: 2.0
/// #
#[derive(Debug,RustcDecodable)]
pub enum InputButton {
    Left,
    Middle,
    Right,
    WheelUp,
    WheelDown,
}
/// #
/// @InputButton
///
/// Position axis of a pointer input device (mouse, tablet).
///
/// Since: 2.0
/// #
#[derive(Debug,RustcDecodable)]
pub enum InputAxis {
    X,
    Y,
}
/// #
/// @HostMemPolicy
///
/// Host memory policy types
///
/// @default: restore default policy, remove any nondefault policy
///
/// @preferred: set the preferred host nodes for allocation
///
/// @bind: a strict policy that restricts memory allocation to the
///        host nodes specified
///
/// @interleave: memory allocations are interleaved across the set
///              of host nodes specified
///
/// Since 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum HostMemPolicy {
    default,
    preferred,
    bind,
    interleave,
}
/// # @ACPISlotType
///
/// @DIMM: memory slot
///
#[derive(Debug,RustcDecodable)]
pub enum ACPISlotType {
    DIMM,
}
/// #
/// @WatchdogExpirationAction
///
/// An enumeration of the actions taken when the watchdog device"s timer is
/// expired
///
/// @reset: system resets
///
/// @shutdown: system shutdown, note that it is similar to @powerdown, which
///            tries to set to system status and notify guest
///
/// @poweroff: system poweroff, the emulator program exits
///
/// @pause: system pauses, similar to @stop
///
/// @debug: system enters debug state
///
/// @none: nothing is done
///
/// Since: 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum WatchdogExpirationAction {
    reset,
    shutdown,
    poweroff,
    pause,
    debug,
    none,
}
/// #
/// @IoOperationType
///
/// An enumeration of the I/O operation types
///
/// @read: read operation
///
/// @write: write operation
///
/// Since: 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum IoOperationType {
    read,
    write,
}
/// #
/// @GuestPanicAction
///
/// An enumeration of the actions taken when guest OS panic is detected
///
/// @pause: system pauses
///
/// Since: 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub enum GuestPanicAction {
    pause,
}
