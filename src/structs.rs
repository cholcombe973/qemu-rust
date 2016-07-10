
#[derive(Debug, RustcDecodable)]
pub struct BlockdevSnapshotInternal {
    pub device: String,
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct SnapshotInfo {
    pub date_nsec: f64,
    pub date_sec: f64,
    pub id: String,
    pub name: String,
    pub vm_clock_nsec: f64,
    pub vm_clock_sec: f64,
    pub vm_state_size: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct ImageInfoSpecificQCow2 {
    pub corrupt: bool,
    pub lazy_refcounts: bool,
    pub compat: String,
    pub refcount_bits: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct ImageInfoSpecificVmdk {
    pub cid: f64,
    pub create_type: String,
    pub extents: Vec<String>,
    pub parent_cid: f64,
}

#[derive(Debug)]
pub struct ImageInfoSpecific {
    pub qcow2: String,
    pub vmdk: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ImageInfo {
    pub actual_size: f64,
    pub backing_filename: String,
    pub backing_filename_format: String,
    pub backing_image: String,
    pub cluster_size: f64,
    pub compressed: bool,
    pub dirty_flag: bool,
    pub encrypted: bool,
    pub format_specific: String,
    pub full_backing_filename: String,
    pub snapshots: Vec<String>,
    pub filename: String,
    pub format: String,
    pub virtual_size: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct ImageCheck {
    pub allocated_clusters: f64,
    pub compressed_clusters: f64,
    pub corruptions: f64,
    pub corruptions_fixed: f64,
    pub fragmented_clusters: f64,
    pub image_end_offset: f64,
    pub leaks: f64,
    pub leaks_fixed: f64,
    pub total_clusters: f64,
    pub check_errors: f64,
    pub filename: String,
    pub format: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevCacheInfo {
    pub direct: bool,
    pub no_flush: bool,
    pub writeback: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockDeviceInfo {
    pub backing_file: String,
    pub bps_max: f64,
    pub bps_rd_max: f64,
    pub bps_wr_max: f64,
    pub iops_max: f64,
    pub iops_rd_max: f64,
    pub iops_size: f64,
    pub iops_wr_max: f64,
    pub node_name: String,
    pub backing_file_depth: f64,
    pub bps: f64,
    pub bps_rd: f64,
    pub bps_wr: f64,
    pub cache: String,
    pub detect_zeroes: String,
    pub drv: String,
    pub encrypted: bool,
    pub encryption_key_missing: bool,
    pub file: String,
    pub image: String,
    pub iops: f64,
    pub iops_rd: f64,
    pub iops_wr: f64,
    pub ro: bool,
    pub write_threshold: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockDeviceMapEntry {
    pub offset: f64,
    pub data: bool,
    pub depth: f64,
    pub length: f64,
    pub start: f64,
    pub zero: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockDirtyInfo {
    pub name: String,
    pub count: f64,
    pub frozen: bool,
    pub granularity: u32,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockInfo {
    pub dirty_bitmaps: Vec<String>,
    pub inserted: String,
    pub io_status: String,
    pub tray_open: bool,
    pub device: String,
    pub locked: bool,
    pub removable: bool,
    pub qemu_type: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockDeviceStats {
    pub flush_operations: f64,
    pub flush_total_time_ns: f64,
    pub rd_bytes: f64,
    pub rd_merged: f64,
    pub rd_operations: f64,
    pub rd_total_time_ns: f64,
    pub wr_bytes: f64,
    pub wr_highest_offset: f64,
    pub wr_merged: f64,
    pub wr_operations: f64,
    pub wr_total_time_ns: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockStats {
    pub backing: String,
    pub device: String,
    pub node_name: String,
    pub parent: String,
    pub stats: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockJobInfo {
    pub busy: bool,
    pub device: String,
    pub io_status: String,
    pub len: f64,
    pub offset: f64,
    pub paused: bool,
    pub ready: bool,
    pub speed: f64,
    pub qemu_type: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevSnapshot {
    pub device: String,
    pub format: String,
    pub mode: String,
    pub node_name: String,
    pub snapshot_node_name: String,
    pub snapshot_file: String,
}

#[derive(Debug, RustcDecodable)]
pub struct DriveBackup {
    pub bitmap: String,
    pub format: String,
    pub mode: String,
    pub on_source_error: String,
    pub on_target_error: String,
    pub speed: f64,
    pub device: String,
    pub sync: String,
    pub target: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevBackup {
    pub on_source_error: String,
    pub on_target_error: String,
    pub speed: f64,
    pub device: String,
    pub sync: String,
    pub target: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockDirtyBitmap {
    pub name: String,
    pub node: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockDirtyBitmapAdd {
    pub granularity: u32,
    pub name: String,
    pub node: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevCacheOptions {
    pub direct: bool,
    pub no_flush: bool,
    pub writeback: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsBase {
    pub aio: String,
    pub cache: String,
    pub detect_zeroes: String,
    pub discard: String,
    pub id: String,
    pub node_name: String,
    pub read_only: bool,
    pub rerror: String,
    pub werror: String,
    pub driver: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsFile {
    pub filename: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsNull {
    pub latency_ns: u64,
    pub size: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsVVFAT {
    pub fat_type: f64,
    pub floppy: bool,
    pub rw: bool,
    pub dir: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsGenericFormat {
    pub file: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsGenericCOWFormat {
    base: BlockdevOptionsGenericFormat,
    pub backing: String,
}

#[derive(Debug, RustcDecodable)]
pub struct Qcow2OverlapCheckFlags {
    pub active_l1: bool,
    pub active_l2: bool,
    pub inactive_l1: bool,
    pub inactive_l2: bool,
    pub main_header: bool,
    pub refcount_block: bool,
    pub refcount_table: bool,
    pub snapshot_table: bool,
    pub template: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsQcow2 {
    base: BlockdevOptionsGenericCOWFormat,
    pub cache_size: f64,
    pub l2_cache_size: f64,
    pub lazy_refcounts: bool,
    pub overlap_check: String,
    pub pass_discard_other: bool,
    pub pass_discard_request: bool,
    pub pass_discard_snapshot: bool,
    pub refcount_cache_size: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsArchipelago {
    pub mport: f64,
    pub segment: String,
    pub vport: f64,
    pub volume: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlkdebugInjectErrorOptions {
    pub errno: f64,
    pub immediately: bool,
    pub once: bool,
    pub sector: f64,
    pub state: f64,
    pub event: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlkdebugSetStateOptions {
    pub state: f64,
    pub event: String,
    pub new_state: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsBlkdebug {
    pub align: f64,
    pub config: String,
    pub inject_error: Vec<String>,
    pub set_state: Vec<String>,
    pub image: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsBlkverify {
    pub raw: String,
    pub test: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsQuorum {
    pub blkverify: bool,
    pub read_pattern: String,
    pub rewrite_corrupted: bool,
    pub children: Vec<String>,
    pub vote_threshold: f64,
}

#[derive(Debug)]
pub struct BlockdevOptions {
    pub archipelago: String,
    pub blkdebug: String,
    pub blkverify: String,
    pub bochs: String,
    pub cloop: String,
    pub dmg: String,
    pub file: String,
    pub ftp: String,
    pub ftps: String,
    pub host_cdrom: String,
    pub host_device: String,
    pub host_floppy: String,
    pub http: String,
    pub https: String,
    pub null_aio: String,
    pub null_co: String,
    pub parallels: String,
    pub qcow: String,
    pub qcow2: String,
    pub qed: String,
    pub quorum: String,
    pub raw: String,
    pub tftp: String,
    pub vdi: String,
    pub vhdx: String,
    pub vmdk: String,
    pub vpc: String,
    pub vvfat: String,
}

#[derive(Debug, RustcDecodable)]
pub struct VersionTriple {
    pub major: f64,
    pub micro: f64,
    pub minor: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct VersionInfo {
    pub package: String,
    pub qemu: String,
}

#[derive(Debug, RustcDecodable)]
pub struct CommandInfo {
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct TraceEventInfo {
    pub name: String,
    pub state: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NameInfo {
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct KvmInfo {
    pub enabled: bool,
    pub present: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct StatusInfo {
    pub running: bool,
    pub singlestep: bool,
    pub status: String,
}

#[derive(Debug, RustcDecodable)]
pub struct UuidInfo {
    pub UUID: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevInfo {
    pub filename: String,
    pub frontend_open: bool,
    pub label: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevBackendInfo {
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct EventInfo {
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct MigrationStats {
    pub dirty_pages_rate: f64,
    pub dirty_sync_count: f64,
    pub duplicate: f64,
    pub mbps: String,
    pub normal: f64,
    pub normal_bytes: f64,
    pub remaining: f64,
    pub skipped: f64,
    pub total: f64,
    pub transferred: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct XBZRLECacheStats {
    pub bytes: f64,
    pub cache_miss: f64,
    pub cache_miss_rate: String,
    pub cache_size: f64,
    pub overflow: f64,
    pub pages: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct MigrationInfo {
    pub disk: String,
    pub downtime: f64,
    pub expected_downtime: f64,
    pub ram: String,
    pub setup_time: f64,
    pub status: String,
    pub total_time: f64,
    pub xbzrle_cache: String,
}

#[derive(Debug, RustcDecodable)]
pub struct MigrationCapabilityStatus {
    pub capability: String,
    pub state: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct MigrationParameters {
    pub compress_level: f64,
    pub compress_threads: f64,
    pub decompress_threads: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct MouseInfo {
    pub absolute: bool,
    pub current: bool,
    pub index: f64,
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct CpuInfo {
    pub PC: f64,
    pub nip: f64,
    pub npc: f64,
    pub pc: f64,
    pub CPU: f64,
    pub current: bool,
    pub halted: bool,
    pub qom_path: String,
    pub thread_id: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct IOThreadInfo {
    pub id: String,
    pub thread_id: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct VncBasicInfo {
    pub family: String,
    pub host: String,
    pub service: String,
    pub websocket: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct VncServerInfo {
    base: VncBasicInfo,
    pub auth: String,
}

#[derive(Debug, RustcDecodable)]
pub struct VncClientInfo {
    base: VncBasicInfo,
    pub sasl_username: String,
    pub x509_dname: String,
}

#[derive(Debug, RustcDecodable)]
pub struct VncInfo {
    pub auth: String,
    pub clients: Vec<String>,
    pub family: String,
    pub host: String,
    pub service: String,
    pub enabled: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct VncInfo2 {
    pub display: String,
    pub vencrypt: String,
    pub auth: String,
    pub clients: Vec<String>,
    pub id: String,
    pub server: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct SpiceBasicInfo {
    pub family: String,
    pub host: String,
    pub port: String,
}

#[derive(Debug, RustcDecodable)]
pub struct SpiceServerInfo {
    base: SpiceBasicInfo,
    pub auth: String,
}

#[derive(Debug, RustcDecodable)]
pub struct SpiceChannel {
    base: SpiceBasicInfo,
    pub channel_id: f64,
    pub channel_type: f64,
    pub connection_id: f64,
    pub tls: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct SpiceInfo {
    pub auth: String,
    pub channels: Vec<String>,
    pub compiled_version: String,
    pub host: String,
    pub port: f64,
    pub tls_port: f64,
    pub enabled: bool,
    pub migrated: bool,
    pub mouse_mode: String,
}

#[derive(Debug, RustcDecodable)]
pub struct BalloonInfo {
    pub actual: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct PciMemoryRange {
    pub base: f64,
    pub limit: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct PciMemoryRegion {
    pub mem_type_64: bool,
    pub prefetch: bool,
    pub address: f64,
    pub bar: f64,
    pub size: f64,
    pub qemu_type: String,
}

#[derive(Debug, RustcDecodable)]
pub struct PciBusInfo {
    pub io_range: String,
    pub memory_range: String,
    pub number: f64,
    pub prefetchable_range: String,
    pub secondary: f64,
    pub subordinate: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct PciBridgeInfo {
    pub devices: Vec<String>,
    pub bus: String,
}

#[derive(Debug, RustcDecodable)]
pub struct PciDeviceClass {
    pub desc: String,
    pub class: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct PciDeviceId {
    pub device: f64,
    pub vendor: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct PciDeviceInfo {
    pub irq: f64,
    pub pci_bridge: String,
    pub bus: f64,
    pub class_info: String,
    pub function: f64,
    pub id: String,
    pub qdev_id: String,
    pub regions: Vec<String>,
    pub slot: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct PciInfo {
    pub bus: f64,
    pub devices: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct Abort {

            }

#[derive(Debug)]
pub struct TransactionAction {
    pub abort: String,
    pub blockdev_backup: String,
    pub blockdev_snapshot_internal_sync: String,
    pub blockdev_snapshot_sync: String,
    pub drive_backup: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ObjectPropertyInfo {
    pub name: String,
    pub qemu_type: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ObjectTypeInfo {
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct DevicePropertyInfo {
    pub description: String,
    pub name: String,
    pub qemu_type: String,
}

#[derive(Debug, RustcDecodable)]
pub struct DumpGuestMemoryCapability {
    pub formats: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevNoneOptions {

            }

#[derive(Debug, RustcDecodable)]
pub struct NetLegacyNicOptions {
    pub addr: String,
    pub macaddr: String,
    pub model: String,
    pub netdev: String,
    pub vectors: u32,
}

#[derive(Debug, RustcDecodable)]
pub struct qemu_string {
    pub String: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevUserOptions {
    pub bootfile: String,
    pub dhcpstart: String,
    pub dns: String,
    pub dnssearch: Vec<String>,
    pub guestfwd: Vec<String>,
    pub host: String,
    pub hostfwd: Vec<String>,
    pub hostname: String,
    pub ip: String,
    pub net: String,
    pub restrict: bool,
    pub smb: String,
    pub smbserver: String,
    pub tftp: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevTapOptions {
    pub downscript: String,
    pub fd: String,
    pub fds: String,
    pub helper: String,
    pub ifname: String,
    pub queues: u32,
    pub script: String,
    pub sndbuf: String,
    pub vhost: bool,
    pub vhostfd: String,
    pub vhostfds: String,
    pub vhostforce: bool,
    pub vnet_hdr: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevSocketOptions {
    pub connect: String,
    pub fd: String,
    pub listen: String,
    pub localaddr: String,
    pub mcast: String,
    pub udp: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevL2TPv3Options {
    pub cookie64: bool,
    pub counter: bool,
    pub dstport: String,
    pub ipv6: bool,
    pub offset: u32,
    pub pincounter: bool,
    pub rxcookie: u64,
    pub rxsession: u32,
    pub srcport: String,
    pub txcookie: u64,
    pub udp: bool,
    pub dst: String,
    pub src: String,
    pub txsession: u32,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevVdeOptions {
    pub group: String,
    pub mode: String,
    pub port: String,
    pub sock: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevDumpOptions {
    pub file: String,
    pub len: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevBridgeOptions {
    pub br: String,
    pub helper: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevHubPortOptions {
    pub hubid: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevNetmapOptions {
    pub devname: String,
    pub ifname: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetdevVhostUserOptions {
    pub vhostforce: bool,
    pub chardev: String,
}

#[derive(Debug)]
pub struct NetClientOptions {
    pub bridge: String,
    pub dump: String,
    pub hubport: String,
    pub l2tpv3: String,
    pub netmap: String,
    pub nic: String,
    pub none: String,
    pub socket: String,
    pub tap: String,
    pub user: String,
    pub vde: String,
    pub vhost_user: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NetLegacy {
    pub id: String,
    pub name: String,
    pub vlan: String,
    pub opts: String,
}

#[derive(Debug, RustcDecodable)]
pub struct Netdev {
    pub id: String,
    pub opts: String,
}

#[derive(Debug, RustcDecodable)]
pub struct InetSocketAddress {
    pub ipv4: bool,
    pub ipv6: bool,
    pub to: String,
    pub host: String,
    pub port: String,
}

#[derive(Debug, RustcDecodable)]
pub struct UnixSocketAddress {
    pub path: String,
}

#[derive(Debug)]
pub struct SocketAddress {
    pub fd: String,
    pub inet: String,
    pub unix: String,
}

#[derive(Debug, RustcDecodable)]
pub struct MachineInfo {
    pub alias: String,
    pub is_default: bool,
    pub cpu_max: f64,
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct CpuDefinitionInfo {
    pub name: String,
}

#[derive(Debug, RustcDecodable)]
pub struct AddfdInfo {
    pub fd: f64,
    pub fdset_id: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct FdsetFdInfo {
    pub opaque: String,
    pub fd: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct FdsetInfo {
    pub fds: Vec<String>,
    pub fdset_id: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct TargetInfo {
    pub arch: String,
}

#[derive(Debug)]
pub struct KeyValue {
    pub number: f64,
    pub qcode: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevFile {
    pub qemu_in: String,
    pub out: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevHostdev {
    pub device: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevSocket {
    pub nodelay: bool,
    pub reconnect: f64,
    pub server: bool,
    pub telnet: bool,
    pub wait: bool,
    pub addr: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevUdp {
    pub local: String,
    pub remote: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevMux {
    pub chardev: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevStdio {
    pub signal: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevSpiceChannel {
    pub qemu_type: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevSpicePort {
    pub fqdn: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevVC {
    pub cols: f64,
    pub height: f64,
    pub rows: f64,
    pub width: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevRingbuf {
    pub size: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevDummy {

            }

#[derive(Debug)]
pub struct ChardevBackend {
    pub braille: String,
    pub console: String,
    pub file: String,
    pub memory: String,
    pub msmouse: String,
    pub mux: String,
    pub null: String,
    pub parallel: String,
    pub pipe: String,
    pub pty: String,
    pub ringbuf: String,
    pub serial: String,
    pub socket: String,
    pub spiceport: String,
    pub spicevmc: String,
    pub stdio: String,
    pub testdev: String,
    pub udp: String,
    pub vc: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ChardevReturn {
    pub pty: String,
}

#[derive(Debug, RustcDecodable)]
pub struct TPMPassthroughOptions {
    pub cancel_path: String,
    pub path: String,
}

#[derive(Debug)]
pub struct TpmTypeOptions {
    pub passthrough: String,
}

#[derive(Debug, RustcDecodable)]
pub struct TPMInfo {
    pub id: String,
    pub model: String,
    pub options: String,
}

#[derive(Debug, RustcDecodable)]
pub struct AcpiTableOptions {
    pub asl_compiler_id: String,
    pub asl_compiler_rev: u32,
    pub data: String,
    pub file: String,
    pub oem_id: String,
    pub oem_rev: u32,
    pub oem_table_id: String,
    pub rev: String,
    pub sig: String,
}

#[derive(Debug, RustcDecodable)]
pub struct CommandLineParameterInfo {
    pub default: String,
    pub help: String,
    pub name: String,
    pub qemu_type: String,
}

#[derive(Debug, RustcDecodable)]
pub struct CommandLineOptionInfo {
    pub option: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct X86CPUFeatureWordInfo {
    pub cpuid_input_ecx: f64,
    pub cpuid_input_eax: f64,
    pub cpuid_register: String,
    pub features: f64,
}

#[derive(Debug, RustcDecodable)]
pub struct RxFilterInfo {
    pub broadcast_allowed: bool,
    pub main_mac: String,
    pub multicast: String,
    pub multicast_overflow: bool,
    pub multicast_table: Vec<String>,
    pub name: String,
    pub promiscuous: bool,
    pub unicast: String,
    pub unicast_overflow: bool,
    pub unicast_table: Vec<String>,
    pub vlan: String,
    pub vlan_table: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct InputKeyEvent {
    pub down: bool,
    pub key: String,
}

#[derive(Debug, RustcDecodable)]
pub struct InputBtnEvent {
    pub button: String,
    pub down: bool,
}

#[derive(Debug, RustcDecodable)]
pub struct InputMoveEvent {
    pub axis: String,
    pub value: f64,
}

#[derive(Debug)]
pub struct InputEvent {
    pub abs: String,
    pub btn: String,
    pub key: String,
    pub rel: String,
}

#[derive(Debug)]
pub struct NumaOptions {
    pub node: String,
}

#[derive(Debug, RustcDecodable)]
pub struct NumaNodeOptions {
    pub cpus: Vec<String>,
    pub mem: String,
    pub memdev: String,
    pub nodeid: String,
}

#[derive(Debug, RustcDecodable)]
pub struct Memdev {
    pub dump: bool,
    pub host_nodes: Vec<String>,
    pub merge: bool,
    pub policy: String,
    pub prealloc: bool,
    pub size: String,
}

#[derive(Debug, RustcDecodable)]
pub struct PCDIMMDeviceInfo {
    pub id: String,
    pub addr: f64,
    pub hotpluggable: bool,
    pub hotplugged: bool,
    pub memdev: String,
    pub node: f64,
    pub size: f64,
    pub slot: f64,
}

#[derive(Debug)]
pub struct MemoryDeviceInfo {
    pub dimm: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ACPIOSTInfo {
    pub device: String,
    pub slot: String,
    pub slot_type: String,
    pub source: f64,
    pub status: f64,
}
