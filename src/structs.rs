
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevSnapshotInternal{
                pub device:String,pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ImageInfoSpecificQCow2{
                pub corrupt:String,pub lazy_refcounts:String,pub compat:String,pub refcount_bits:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ImageInfoSpecificVmdk{
                pub cid:String,pub create_type:String,pub extents:Vec<String>,pub parent_cid:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevCacheInfo{
                pub direct:String,pub no_flush:String,pub writeback:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockDeviceInfo{
                pub backing_file:String,pub bps_max:String,pub bps_rd_max:String,pub bps_wr_max:String,pub iops_max:String,pub iops_rd_max:String,pub iops_size:String,pub iops_wr_max:String,pub node_name:String,pub backing_file_depth:String,pub bps:String,pub bps_rd:String,pub bps_wr:String,pub cache:String,pub detect_zeroes:String,pub drv:String,pub encrypted:String,pub encryption_key_missing:String,pub file:String,pub image:String,pub iops:String,pub iops_rd:String,pub iops_wr:String,pub ro:String,pub write_threshold:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockDeviceMapEntry{
                pub offset:String,pub data:String,pub depth:String,pub length:String,pub start:String,pub zero:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockDirtyInfo{
                pub name:String,pub count:String,pub frozen:String,pub granularity:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockInfo{
                pub dirty_bitmaps:Vec<String>,pub inserted:String,pub io_status:String,pub tray_open:String,pub device:String,pub locked:String,pub removable:String,pub qemu_type:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockDeviceStats{
                pub flush_operations:String,pub flush_total_time_ns:String,pub rd_bytes:String,pub rd_merged:String,pub rd_operations:String,pub rd_total_time_ns:String,pub wr_bytes:String,pub wr_highest_offset:String,pub wr_merged:String,pub wr_operations:String,pub wr_total_time_ns:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockStats{
                pub backing:String,pub device:String,pub node_name:String,pub parent:String,pub stats:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockJobInfo{
                pub busy:String,pub device:String,pub io_status:String,pub len:String,pub offset:String,pub paused:String,pub ready:String,pub speed:String,pub qemu_type:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevSnapshot{
                pub device:String,pub format:String,pub mode:String,pub node_name:String,pub snapshot_node_name:String,pub snapshot_file:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct DriveBackup{
                pub bitmap:String,pub format:String,pub mode:String,pub on_source_error:String,pub on_target_error:String,pub speed:String,pub device:String,pub sync:String,pub target:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevBackup{
                pub on_source_error:String,pub on_target_error:String,pub speed:String,pub device:String,pub sync:String,pub target:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockDirtyBitmap{
                pub name:String,pub node:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockDirtyBitmapAdd{
                pub granularity:String,pub name:String,pub node:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevCacheOptions{
                pub direct:String,pub no_flush:String,pub writeback:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsBase{
                pub aio:String,pub cache:String,pub detect_zeroes:String,pub discard:String,pub id:String,pub node_name:String,pub read_only:String,pub rerror:String,pub werror:String,pub driver:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsFile{
                pub filename:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsNull{
                pub latency_ns:String,pub size:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsVVFAT{
                pub fat_type:String,pub floppy:String,pub rw:String,pub dir:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsGenericFormat{
                pub file:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsGenericCOWFormat{
                base: BlockdevOptionsGenericFormat,pub backing:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct Qcow2OverlapCheckFlags{
                pub active_l1:String,pub active_l2:String,pub inactive_l1:String,pub inactive_l2:String,pub main_header:String,pub refcount_block:String,pub refcount_table:String,pub snapshot_table:String,pub template:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsQcow2{
                base: BlockdevOptionsGenericCOWFormat,pub cache_size:String,pub l2_cache_size:String,pub lazy_refcounts:String,pub overlap_check:String,pub pass_discard_other:String,pub pass_discard_request:String,pub pass_discard_snapshot:String,pub refcount_cache_size:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsArchipelago{
                pub mport:String,pub segment:String,pub vport:String,pub volume:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlkdebugInjectErrorOptions{
                pub errno:String,pub immediately:String,pub once:String,pub sector:String,pub state:String,pub event:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlkdebugSetStateOptions{
                pub state:String,pub event:String,pub new_state:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsBlkdebug{
                pub align:String,pub config:String,pub inject_error:Vec<String>,pub set_state:Vec<String>,pub image:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsBlkverify{
                pub raw:String,pub test:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BlockdevOptionsQuorum{
                pub blkverify:String,pub read_pattern:String,pub rewrite_corrupted:String,pub children:Vec<String>,pub vote_threshold:String
            }
            
            #[derive(Debug)]
            pub struct BlockdevOptions{
                pub archipelago:String,pub blkdebug:String,pub blkverify:String,pub bochs:String,pub cloop:String,pub dmg:String,pub file:String,pub ftp:String,pub ftps:String,pub host_cdrom:String,pub host_device:String,pub host_floppy:String,pub http:String,pub https:String,pub null_aio:String,pub null_co:String,pub parallels:String,pub qcow:String,pub qcow2:String,pub qed:String,pub quorum:String,pub raw:String,pub tftp:String,pub vdi:String,pub vhdx:String,pub vmdk:String,pub vpc:String,pub vvfat:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct VersionTriple{
                pub major:String,pub micro:String,pub minor:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct VersionInfo{
                pub package:String,pub qemu:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct CommandInfo{
                pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct TraceEventInfo{
                pub name:String,pub state:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NameInfo{
                pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct KvmInfo{
                pub enabled:String,pub present:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct StatusInfo{
                pub running:String,pub singlestep:String,pub status:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct UuidInfo{
                pub UUID:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevInfo{
                pub filename:String,pub frontend_open:String,pub label:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevBackendInfo{
                pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct EventInfo{
                pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct MigrationStats{
                pub dirty_pages_rate:String,pub dirty_sync_count:String,pub duplicate:String,pub mbps:String,pub normal:String,pub normal_bytes:String,pub remaining:String,pub skipped:String,pub total:String,pub transferred:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct XBZRLECacheStats{
                pub bytes:String,pub cache_miss:String,pub cache_miss_rate:String,pub cache_size:String,pub overflow:String,pub pages:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct MigrationInfo{
                pub disk:String,pub downtime:String,pub expected_downtime:String,pub ram:String,pub setup_time:String,pub status:String,pub total_time:String,pub xbzrle_cache:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct MigrationCapabilityStatus{
                pub capability:String,pub state:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct MigrationParameters{
                pub compress_level:String,pub compress_threads:String,pub decompress_threads:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct MouseInfo{
                pub absolute:String,pub current:String,pub index:String,pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct CpuInfo{
                pub PC:String,pub nip:String,pub npc:String,pub pc:String,pub CPU:String,pub current:String,pub halted:String,pub qom_path:String,pub thread_id:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct IOThreadInfo{
                pub id:String,pub thread_id:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct VncBasicInfo{
                pub family:String,pub host:String,pub service:String,pub websocket:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct VncServerInfo{
                base: VncBasicInfo,pub auth:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct VncClientInfo{
                base: VncBasicInfo,pub sasl_username:String,pub x509_dname:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct VncInfo{
                pub auth:String,pub clients:Vec<String>,pub family:String,pub host:String,pub service:String,pub enabled:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct VncInfo2{
                pub display:String,pub vencrypt:String,pub auth:String,pub clients:Vec<String>,pub id:String,pub server:Vec<String>
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct SpiceBasicInfo{
                pub family:String,pub host:String,pub port:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct SpiceServerInfo{
                base: SpiceBasicInfo,pub auth:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct SpiceChannel{
                base: SpiceBasicInfo,pub channel_id:String,pub channel_type:String,pub connection_id:String,pub tls:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct SpiceInfo{
                pub auth:String,pub channels:Vec<String>,pub compiled_version:String,pub host:String,pub port:String,pub tls_port:String,pub enabled:String,pub migrated:String,pub mouse_mode:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct BalloonInfo{
                pub actual:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PciMemoryRange{
                pub base:String,pub limit:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PciMemoryRegion{
                pub mem_type_64:String,pub prefetch:String,pub address:String,pub bar:String,pub size:String,pub qemu_type:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PciBusInfo{
                pub io_range:String,pub memory_range:String,pub number:String,pub prefetchable_range:String,pub secondary:String,pub subordinate:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PciBridgeInfo{
                pub devices:Vec<String>,pub bus:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PciDeviceClass{
                pub desc:String,pub class:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PciDeviceId{
                pub device:String,pub vendor:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PciDeviceInfo{
                pub irq:String,pub pci_bridge:String,pub bus:String,pub class_info:String,pub function:String,pub id:String,pub qdev_id:String,pub regions:Vec<String>,pub slot:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PciInfo{
                pub bus:String,pub devices:Vec<String>
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct Abort{
                
            }
            
            #[derive(Debug)]
            pub struct TransactionAction{
                pub abort:String,pub blockdev_backup:String,pub blockdev_snapshot_internal_sync:String,pub blockdev_snapshot_sync:String,pub drive_backup:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ObjectPropertyInfo{
                pub name:String,pub qemu_type:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ObjectTypeInfo{
                pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct DevicePropertyInfo{
                pub description:String,pub name:String,pub qemu_type:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct DumpGuestMemoryCapability{
                pub formats:Vec<String>
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevNoneOptions{
                
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetLegacyNicOptions{
                pub addr:String,pub macaddr:String,pub model:String,pub netdev:String,pub vectors:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct qemu_string{
                pub str:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevUserOptions{
                pub bootfile:String,pub dhcpstart:String,pub dns:String,pub dnssearch:Vec<String>,pub guestfwd:Vec<String>,pub host:String,pub hostfwd:Vec<String>,pub hostname:String,pub ip:String,pub net:String,pub restrict:String,pub smb:String,pub smbserver:String,pub tftp:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevTapOptions{
                pub downscript:String,pub fd:String,pub fds:String,pub helper:String,pub ifname:String,pub queues:String,pub script:String,pub sndbuf:String,pub vhost:String,pub vhostfd:String,pub vhostfds:String,pub vhostforce:String,pub vnet_hdr:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevSocketOptions{
                pub connect:String,pub fd:String,pub listen:String,pub localaddr:String,pub mcast:String,pub udp:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevL2TPv3Options{
                pub cookie64:String,pub counter:String,pub dstport:String,pub ipv6:String,pub offset:String,pub pincounter:String,pub rxcookie:String,pub rxsession:String,pub srcport:String,pub txcookie:String,pub udp:String,pub dst:String,pub src:String,pub txsession:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevVdeOptions{
                pub group:String,pub mode:String,pub port:String,pub sock:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevDumpOptions{
                pub file:String,pub len:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevBridgeOptions{
                pub br:String,pub helper:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevHubPortOptions{
                pub hubid:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevNetmapOptions{
                pub devname:String,pub ifname:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetdevVhostUserOptions{
                pub vhostforce:String,pub chardev:String
            }
            
            #[derive(Debug)]
            pub struct NetClientOptions{
                pub bridge:String,pub dump:String,pub hubport:String,pub l2tpv3:String,pub netmap:String,pub nic:String,pub none:String,pub socket:String,pub tap:String,pub user:String,pub vde:String,pub vhost_user:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NetLegacy{
                pub id:String,pub name:String,pub vlan:String,pub opts:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct Netdev{
                pub id:String,pub opts:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct InetSocketAddress{
                pub ipv4:String,pub ipv6:String,pub to:String,pub host:String,pub port:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct UnixSocketAddress{
                pub path:String
            }
            
            #[derive(Debug)]
            pub struct SocketAddress{
                pub fd:String,pub inet:String,pub unix:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct MachineInfo{
                pub alias:String,pub is_default:String,pub cpu_max:String,pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct CpuDefinitionInfo{
                pub name:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct AddfdInfo{
                pub fd:String,pub fdset_id:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct FdsetFdInfo{
                pub opaque:String,pub fd:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct FdsetInfo{
                pub fds:Vec<String>,pub fdset_id:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct TargetInfo{
                pub arch:String
            }
            
            #[derive(Debug)]
            pub struct KeyValue{
                pub number:String,pub qcode:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevFile{
                pub qemu_in:String,pub out:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevHostdev{
                pub device:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevSocket{
                pub nodelay:String,pub reconnect:String,pub server:String,pub telnet:String,pub wait:String,pub addr:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevUdp{
                pub local:String,pub remote:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevMux{
                pub chardev:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevStdio{
                pub signal:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevSpiceChannel{
                pub qemu_type:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevSpicePort{
                pub fqdn:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevVC{
                pub cols:String,pub height:String,pub rows:String,pub width:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevRingbuf{
                pub size:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevDummy{
                
            }
            
            #[derive(Debug)]
            pub struct ChardevBackend{
                pub braille:String,pub console:String,pub file:String,pub memory:String,pub msmouse:String,pub mux:String,pub null:String,pub parallel:String,pub pipe:String,pub pty:String,pub ringbuf:String,pub serial:String,pub socket:String,pub spiceport:String,pub spicevmc:String,pub stdio:String,pub testdev:String,pub udp:String,pub vc:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ChardevReturn{
                pub pty:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct TPMPassthroughOptions{
                pub cancel_path:String,pub path:String
            }
            
            #[derive(Debug)]
            pub struct TpmTypeOptions{
                pub passthrough:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct TPMInfo{
                pub id:String,pub model:String,pub options:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct AcpiTableOptions{
                pub asl_compiler_id:String,pub asl_compiler_rev:String,pub data:String,pub file:String,pub oem_id:String,pub oem_rev:String,pub oem_table_id:String,pub rev:String,pub sig:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct CommandLineParameterInfo{
                pub default:String,pub help:String,pub name:String,pub qemu_type:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct CommandLineOptionInfo{
                pub option:String,pub parameters:Vec<String>
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct X86CPUFeatureWordInfo{
                pub cpuid_input_ecx:String,pub cpuid_input_eax:String,pub cpuid_register:String,pub features:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct InputKeyEvent{
                pub down:String,pub key:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct InputBtnEvent{
                pub button:String,pub down:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct InputMoveEvent{
                pub axis:String,pub value:String
            }
            
            #[derive(Debug)]
            pub struct InputEvent{
                pub abs:String,pub btn:String,pub key:String,pub rel:String
            }
            
            #[derive(Debug)]
            pub struct NumaOptions{
                pub node:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct NumaNodeOptions{
                pub cpus:Vec<String>,pub mem:String,pub memdev:String,pub nodeid:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct PCDIMMDeviceInfo{
                pub id:String,pub addr:String,pub hotpluggable:String,pub hotplugged:String,pub memdev:String,pub node:String,pub size:String,pub slot:String
            }
            
            #[derive(Debug)]
            pub struct MemoryDeviceInfo{
                pub dimm:String
            }
            
            #[derive(Debug, RustcDecodable)]
            pub struct ACPIOSTInfo{
                pub device:String,pub slot:String,pub slot_type:String,pub source:String,pub status:String
            }
            