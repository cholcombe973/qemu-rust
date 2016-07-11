
#[derive(Debug,RustcDecodable)]
pub enum BiosAtaTranslation {
    auto,
    none,
    lba,
    large,
    rechs,
}

#[derive(Debug,RustcDecodable)]
pub enum BlockDeviceIoStatus {
    ok,
    failed,
    nospace,
}

#[derive(Debug,RustcDecodable)]
pub enum BlockdevOnError {
    report,
    ignore,
    enospc,
    stop,
}

#[derive(Debug,RustcDecodable)]
pub enum MirrorSyncMode {
    top,
    full,
    none,
    dirty_bitmap,
}

#[derive(Debug,RustcDecodable)]
pub enum BlockJobType {
    commit,
    stream,
    mirror,
    backup,
}

#[derive(Debug,RustcDecodable)]
pub enum NewImageMode {
    existing,
    absolute_paths,
}

#[derive(Debug,RustcDecodable)]
pub enum BlockdevDiscardOptions {
    ignore,
    unmap,
}

#[derive(Debug,RustcDecodable)]
pub enum BlockdevDetectZeroesOptions {
    off,
    on,
    unmap,
}

#[derive(Debug,RustcDecodable)]
pub enum BlockdevAioOptions {
    threads,
    native,
}

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

#[derive(Debug,RustcDecodable)]
pub enum Qcow2OverlapCheckMode {
    none,
    constant,
    cached,
    all,
}

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

#[derive(Debug,RustcDecodable)]
pub enum QuorumReadPattern {
    quorum,
    fifo,
}

#[derive(Debug,RustcDecodable)]
pub enum BlockErrorAction {
    ignore,
    report,
    stop,
}

#[derive(Debug,RustcDecodable)]
pub enum PreallocMode {
    off,
    metadata,
    falloc,
    full,
}

#[derive(Debug,RustcDecodable)]
pub enum ErrorClass {
    GenericError,
    CommandNotFound,
    DeviceEncrypted,
    DeviceNotActive,
    DeviceNotFound,
    KVMMissingCap,
}

#[derive(Debug,RustcDecodable)]
pub enum OnOffAuto {
    auto,
    on,
    off,
}

#[derive(Debug,RustcDecodable)]
pub enum TraceEventState {
    unavailable,
    disabled,
    enabled,
}

#[derive(Debug,RustcDecodable)]
pub enum LostTickPolicy {
    discard,
    delay,
    merge,
    slew,
}

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

#[derive(Debug,RustcDecodable)]
pub enum DataFormat {
    utf8,
    base64,
}

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

#[derive(Debug,RustcDecodable)]
pub enum MigrationCapability {
    xbzrle,
    rdma_pin_all,
    auto_converge,
    zero_blocks,
    compress,
}

#[derive(Debug,RustcDecodable)]
pub enum MigrationParameter {
    compress_level,
    compress_threads,
    decompress_threads,
}

#[derive(Debug,RustcDecodable)]
pub enum NetworkAddressFamily {
    ipv4,
    ipv6,
    unix,
    unknown,
}

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

#[derive(Debug,RustcDecodable)]
pub enum SpiceQueryMouseMode {
    client,
    server,
    unknown,
}

#[derive(Debug,RustcDecodable)]
pub enum DumpGuestMemoryFormat {
    elf,
    kdump_zlib,
    kdump_lzo,
    kdump_snappy,
}

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

#[derive(Debug,RustcDecodable)]
pub enum TpmModel {
    tpm_tis,
}

#[derive(Debug,RustcDecodable)]
pub enum TpmType {
    passthrough,
}

#[derive(Debug,RustcDecodable)]
pub enum CommandLineParameterType {
    string,
    boolean,
    number,
    size,
}

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

#[derive(Debug,RustcDecodable)]
pub enum RxState {
    normal,
    none,
    all,
}

#[derive(Debug,RustcDecodable)]
pub enum InputButton {
    Left,
    Middle,
    Right,
    WheelUp,
    WheelDown,
}

#[derive(Debug,RustcDecodable)]
pub enum InputAxis {
    X,
    Y,
}

#[derive(Debug,RustcDecodable)]
pub enum HostMemPolicy {
    default,
    preferred,
    bind,
    interleave,
}

#[derive(Debug,RustcDecodable)]
pub enum ACPISlotType {
    DIMM,
}

#[derive(Debug,RustcDecodable)]
pub enum WatchdogExpirationAction {
    reset,
    shutdown,
    poweroff,
    pause,
    debug,
    none,
}

#[derive(Debug,RustcDecodable)]
pub enum IoOperationType {
    read,
    write,
}

#[derive(Debug,RustcDecodable)]
pub enum GuestPanicAction {
    pause,
}
