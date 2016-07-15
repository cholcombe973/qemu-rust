/// #
/// @BlockdevSnapshotInternal
///
/// @device: the name of the device to generate the snapshot from
///
/// @name: the name of the internal snapshot to be created
///
/// Notes: In transaction, if @name is empty, or any snapshot matching @name
///        exists, the operation will fail. Only some image formats support it,
///        for example, qcow2, rbd, and sheepdog.
///
/// Since: 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevSnapshotInternal {
    pub device: String,
    pub name: String,
}
/// #
/// @SnapshotInfo
///
/// @id: unique snapshot id
///
/// @name: user chosen name
///
/// @vm-state-size: size of the VM state
///
/// @date-sec: UTC date of the snapshot in seconds
///
/// @date-nsec: fractional part in nano seconds to be used with date-sec
///
/// @vm-clock-sec: VM clock relative to boot in seconds
///
/// @vm-clock-nsec: fractional part in nano seconds to be used with vm-clock-sec
///
/// Since: 1.3
///
/// #
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
/// #
/// @ImageInfoSpecificQCow2:
///
/// @compat: compatibility level
///
/// @lazy-refcounts: #optional on or off; only valid for compat >= 1.1
///
/// @corrupt: #optional true if the image has been marked corrupt; only valid for
///           compat >= 1.1 (since 2.2)
///
/// @refcount-bits: width of a refcount entry in bits (since 2.3)
///
/// Since: 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct ImageInfoSpecificQCow2 {
    pub corrupt: bool,
    pub lazy_refcounts: bool,
    pub compat: String,
    pub refcount_bits: f64,
}
/// #
/// @ImageInfoSpecificVmdk:
///
/// @create-type: The create type of VMDK image
///
/// @cid: Content id of image
///
/// @parent-cid: Parent VMDK image"s cid
///
/// @extents: List of extent files
///
/// Since: 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct ImageInfoSpecificVmdk {
    pub cid: f64,
    pub create_type: String,
    pub extents: Vec<String>,
    pub parent_cid: f64,
}
/// #
/// @ImageInfoSpecific:
///
/// A discriminated record of image format specific information structures.
///
/// Since: 1.7
/// #
#[derive(Debug,RustcDecodable)]
pub struct ImageInfoSpecific {
    pub qcow2: String,
    pub vmdk: String,
}
/// #
/// @ImageInfo:
///
/// Information about a QEMU image file
///
/// @filename: name of the image file
///
/// @format: format of the image file
///
/// @virtual-size: maximum capacity in bytes of the image
///
/// @actual-size: #optional actual size on disk in bytes of the image
///
/// @dirty-flag: #optional true if image is not cleanly closed
///
/// @cluster-size: #optional size of a cluster in bytes
///
/// @encrypted: #optional true if the image is encrypted
///
/// @compressed: #optional true if the image is compressed (Since 1.7)
///
/// @backing-filename: #optional name of the backing file
///
/// @full-backing-filename: #optional full path of the backing file
///
/// @backing-filename-format: #optional the format of the backing file
///
/// @snapshots: #optional list of VM snapshots
///
/// @backing-image: #optional info of the backing image (since 1.6)
///
/// @format-specific: #optional structure supplying additional format-specific
/// information (since 1.7)
///
/// Since: 1.3
///
/// #
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
/// #
/// @ImageCheck:
///
/// Information about a QEMU image file check
///
/// @filename: name of the image file checked
///
/// @format: format of the image file checked
///
/// @check-errors: number of unexpected errors occurred during check
///
/// @image-end-offset: #optional offset (in bytes) where the image ends, this
///                    field is present if the driver for the image format
///                    supports it
///
/// @corruptions: #optional number of corruptions found during the check if any
///
/// @leaks: #optional number of leaks found during the check if any
///
/// @corruptions-fixed: #optional number of corruptions fixed during the check
///                     if any
///
/// @leaks-fixed: #optional number of leaks fixed during the check if any
///
/// @total-clusters: #optional total number of clusters, this field is present
///                  if the driver for the image format supports it
///
/// @allocated-clusters: #optional total number of allocated clusters, this
///                      field is present if the driver for the image format
///                      supports it
///
/// @fragmented-clusters: #optional total number of fragmented clusters, this
///                       field is present if the driver for the image format
///                       supports it
///
/// @compressed-clusters: #optional total number of compressed clusters, this
///                       field is present if the driver for the image format
///                       supports it
///
/// Since: 1.4
///
/// #
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
/// #
/// @BlockdevCacheInfo
///
/// Cache mode information for a block device
///
/// @writeback:   true if writeback mode is enabled
/// @direct:      true if the host page cache is bypassed (O_DIRECT)
/// @no-flush:    true if flush requests are ignored for the device
///
/// Since: 2.3
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevCacheInfo {
    pub direct: bool,
    pub no_flush: bool,
    pub writeback: bool,
}
/// #
/// @BlockDeviceInfo:
///
/// Information about the backing device for a block device.
///
/// @file: the filename of the backing device
///
/// @node-name: #optional the name of the block driver node (Since 2.0)
///
/// @ro: true if the backing device was open read-only
///
/// @drv: the name of the block format used to open the backing device. As of
///       0.14.0 this can be: "blkdebug", "bochs", "cloop", "cow", "dmg",
///       "file", "file", "ftp", "ftps", "host_cdrom", "host_device",
///       "host_floppy", "http", "https", "nbd", "parallels", "qcow",
///       "qcow2", "raw", "tftp", "vdi", "vmdk", "vpc", "vvfat"
///       2.2: "archipelago" added, "cow" dropped
///       2.3: "host_floppy" deprecated
///
/// @backing_file: #optional the name of the backing file (for copy-on-write)
///
/// @backing_file_depth: number of files in the backing file chain (since: 1.2)
///
/// @encrypted: true if the backing device is encrypted
///
/// @encryption_key_missing: true if the backing device is encrypted but an
///                          valid encryption key is missing
///
/// @detect_zeroes: detect and optimize zero writes (Since 2.1)
///
/// @bps: total throughput limit in bytes per second is specified
///
/// @bps_rd: read throughput limit in bytes per second is specified
///
/// @bps_wr: write throughput limit in bytes per second is specified
///
/// @iops: total I/O operations per second is specified
///
/// @iops_rd: read I/O operations per second is specified
///
/// @iops_wr: write I/O operations per second is specified
///
/// @image: the info of image used (since: 1.6)
///
/// @bps_max: #optional total max in bytes (Since 1.7)
///
/// @bps_rd_max: #optional read max in bytes (Since 1.7)
///
/// @bps_wr_max: #optional write max in bytes (Since 1.7)
///
/// @iops_max: #optional total I/O operations max (Since 1.7)
///
/// @iops_rd_max: #optional read I/O operations max (Since 1.7)
///
/// @iops_wr_max: #optional write I/O operations max (Since 1.7)
///
/// @iops_size: #optional an I/O size in bytes (Since 1.7)
///
/// @cache: the cache mode used for the block device (since: 2.3)
///
/// @write_threshold: configured write threshold for the device.
///                   0 if disabled. (Since 2.3)
///
/// Since: 0.14.0
///
/// #
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
/// #
/// @BlockDeviceMapEntry:
///
/// Entry in the metadata map of the device (returned by "qemu-img map")
///
/// @start: Offset in the image of the first byte described by this entry
///         (in bytes)
///
/// @length: Length of the range described by this entry (in bytes)
///
/// @depth: Number of layers (0 = top image, 1 = top image"s backing file, etc.)
///         before reaching one for which the range is allocated.  The value is
///         in the range 0 to the depth of the image chain - 1.
///
/// @zero: the sectors in this range read as zeros
///
/// @data: reading the image will actually read data from a file (in particular,
///        if @offset is present this means that the sectors are not simply
///        preallocated, but contain actual data in raw format)
///
/// @offset: if present, the image file stores the data for this range in
///          raw format at the given offset.
///
/// Since 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockDeviceMapEntry {
    pub offset: f64,
    pub data: bool,
    pub depth: f64,
    pub length: f64,
    pub start: f64,
    pub zero: bool,
}
/// #
/// @BlockDirtyInfo:
///
/// Block dirty bitmap information.
///
/// @name: #optional the name of the dirty bitmap (Since 2.4)
///
/// @count: number of dirty bytes according to the dirty bitmap
///
/// @granularity: granularity of the dirty bitmap in bytes (since 1.4)
///
/// @frozen: whether the dirty bitmap is frozen (Since 2.4)
///
/// Since: 1.3
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockDirtyInfo {
    pub name: String,
    pub count: f64,
    pub frozen: bool,
    pub granularity: u32,
}
/// #
/// @BlockInfo:
///
/// Block device information.  This structure describes a virtual device and
/// the backing device associated with it.
///
/// @device: The device name associated with the virtual device.
///
/// @type: This field is returned only for compatibility reasons, it should
///        not be used (always returns "unknown")
///
/// @removable: True if the device supports removable media.
///
/// @locked: True if the guest has locked this device from having its media
///          removed
///
/// @tray_open: #optional True if the device has a tray and it is open
///             (only present if removable is true)
///
/// @dirty-bitmaps: #optional dirty bitmaps information (only present if the
///                 driver has one or more dirty bitmaps) (Since 2.0)
///
/// @io-status: #optional @BlockDeviceIoStatus. Only present if the device
///             supports it and the VM is configured to stop on errors
///             (supported device models: virtio-blk, ide, scsi-disk)
///
/// @inserted: #optional @BlockDeviceInfo describing the device if media is
///            present
///
/// Since:  0.14.0
/// #
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
/// #
/// @BlockDeviceStats:
///
/// Statistics of a virtual block device or a block backing device.
///
/// @rd_bytes:      The number of bytes read by the device.
///
/// @wr_bytes:      The number of bytes written by the device.
///
/// @rd_operations: The number of read operations performed by the device.
///
/// @wr_operations: The number of write operations performed by the device.
///
/// @flush_operations: The number of cache flush operations performed by the
///                    device (since 0.15.0)
///
/// @flush_total_time_ns: Total time spend on cache flushes in nano-seconds
///                       (since 0.15.0).
///
/// @wr_total_time_ns: Total time spend on writes in nano-seconds (since 0.15.0).
///
/// @rd_total_time_ns: Total_time_spend on reads in nano-seconds (since 0.15.0).
///
/// @wr_highest_offset: The offset after the greatest byte written to the
///                     device.  The intended use of this information is for
///                     growable sparse files (like qcow2) that are used on top
///                     of a physical device.
///
/// @rd_merged: Number of read requests that have been merged into another
///             request (Since 2.3).
///
/// @wr_merged: Number of write requests that have been merged into another
///             request (Since 2.3).
///
/// Since: 0.14.0
/// #
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
/// #
/// @BlockStats:
///
/// Statistics of a virtual block device or a block backing device.
///
/// @device: #optional If the stats are for a virtual block device, the name
///          corresponding to the virtual block device.
///
/// @node-name: #optional The node name of the device. (Since 2.3)
///
/// @stats:  A @BlockDeviceStats for the device.
///
/// @parent: #optional This describes the file block device if it has one.
///
/// @backing: #optional This describes the backing block device if it has one.
///           (Since 2.0)
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockStats {
    pub backing: String,
    pub device: String,
    pub node_name: String,
    pub parent: String,
    pub stats: String,
}
/// #
/// @BlockJobInfo:
///
/// Information about a long-running block device operation.
///
/// @type: the job type ("stream" for image streaming)
///
/// @device: the block device name
///
/// @len: the maximum progress value
///
/// @busy: false if the job is known to be in a quiescent state, with
///        no pending I/O.  Since 1.3.
///
/// @paused: whether the job is paused or, if @busy is true, will
///          pause itself as soon as possible.  Since 1.3.
///
/// @offset: the current progress value
///
/// @speed: the rate limit, bytes per second
///
/// @io-status: the status of the job (since 1.3)
///
/// @ready: true if the job may be completed (since 2.2)
///
/// Since: 1.1
/// #
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
/// #
/// @BlockdevSnapshot
///
/// Either @device or @node-name must be set but not both.
///
/// @device: #optional the name of the device to generate the snapshot from.
///
/// @node-name: #optional graph node name to generate the snapshot from (Since 2.0)
///
/// @snapshot-file: the target of the new image. A new file will be created.
///
/// @snapshot-node-name: #optional the graph node name of the new image (Since 2.0)
///
/// @format: #optional the format of the snapshot image, default is "qcow2".
///
/// @mode: #optional whether and how QEMU should create a new image, default is
///        "absolute-paths".
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevSnapshot {
    pub device: String,
    pub format: String,
    pub mode: String,
    pub node_name: String,
    pub snapshot_node_name: String,
    pub snapshot_file: String,
}
/// #
/// @DriveBackup
///
/// @device: the name of the device which should be copied.
///
/// @target: the target of the new image. If the file exists, or if it
///          is a device, the existing file/device will be used as the new
///          destination.  If it does not exist, a new file will be created.
///
/// @format: #optional the format of the new destination, default is to
///          probe if @mode is "existing", else the format of the source
///
/// @sync: what parts of the disk image should be copied to the destination
///        (all the disk, only the sectors allocated in the topmost image, from a
///        dirty bitmap, or only new I/O).
///
/// @mode: #optional whether and how QEMU should create a new image, default is
///        "absolute-paths".
///
/// @speed: #optional the maximum speed, in bytes per second
///
/// @bitmap: #optional the name of dirty bitmap if sync is "dirty-bitmap".
///          Must be present if sync is "dirty-bitmap", must NOT be present
///          otherwise. (Since 2.4)
///
/// @on-source-error: #optional the action to take on an error on the source,
///                   default "report".  "stop" and "enospc" can only be used
///                   if the block device supports io-status (see BlockInfo).
///
/// @on-target-error: #optional the action to take on an error on the target,
///                   default "report" (no limitations, since this applies to
///                   a different block device than @device).
///
/// Note that @on-source-error and @on-target-error only affect background I/O.
/// If an error occurs during a guest write request, the device"s rerror/werror
/// actions will be used.
///
/// Since: 1.6
/// #
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
/// #
/// @BlockdevBackup
///
/// @device: the name of the device which should be copied.
///
/// @target: the name of the backup target device.
///
/// @sync: what parts of the disk image should be copied to the destination
///        (all the disk, only the sectors allocated in the topmost image, or
///        only new I/O).
///
/// @speed: #optional the maximum speed, in bytes per second. The default is 0,
///         for unlimited.
///
/// @on-source-error: #optional the action to take on an error on the source,
///                   default "report".  "stop" and "enospc" can only be used
///                   if the block device supports io-status (see BlockInfo).
///
/// @on-target-error: #optional the action to take on an error on the target,
///                   default "report" (no limitations, since this applies to
///                   a different block device than @device).
///
/// Note that @on-source-error and @on-target-error only affect background I/O.
/// If an error occurs during a guest write request, the device"s rerror/werror
/// actions will be used.
///
/// Since: 2.3
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevBackup {
    pub on_source_error: String,
    pub on_target_error: String,
    pub speed: f64,
    pub device: String,
    pub sync: String,
    pub target: String,
}
/// #
/// @BlockDirtyBitmap
///
/// @node: name of device/node which the bitmap is tracking
///
/// @name: name of the dirty bitmap
///
/// Since 2.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockDirtyBitmap {
    pub name: String,
    pub node: String,
}
/// #
/// @BlockDirtyBitmapAdd
///
/// @node: name of device/node which the bitmap is tracking
///
/// @name: name of the dirty bitmap
///
/// @granularity: #optional the bitmap granularity, default is 64k for
///               block-dirty-bitmap-add
///
/// Since 2.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockDirtyBitmapAdd {
    pub granularity: u32,
    pub name: String,
    pub node: String,
}
/// #
/// @BlockdevCacheOptions
///
/// Includes cache-related options for block devices
///
/// @writeback:   #optional enables writeback mode for any caches (default: true)
/// @direct:      #optional enables use of O_DIRECT (bypass the host page cache;
///               default: false)
/// @no-flush:    #optional ignore any flush requests for the device (default:
///               false)
///
/// Since: 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevCacheOptions {
    pub direct: bool,
    pub no_flush: bool,
    pub writeback: bool,
}
/// #
/// @BlockdevOptionsBase
///
/// Options that are available for all block devices, independent of the block
/// driver.
///
/// @driver:        block driver name
/// @id:            #optional id by which the new block device can be referred to.
///                 This is a required option on the top level of blockdev-add, and
///                 currently not allowed on any other level.
/// @node-name:     #optional the name of a block driver state node (Since 2.0)
/// @discard:       #optional discard-related options (default: ignore)
/// @cache:         #optional cache-related options
/// @aio:           #optional AIO backend (default: threads)
/// @rerror:        #optional how to handle read errors on the device
///                 (default: report)
/// @werror:        #optional how to handle write errors on the device
///                 (default: enospc)
/// @read-only:     #optional whether the block device should be read-only
///                 (default: false)
/// @detect-zeroes: #optional detect and optimize zero writes (Since 2.1)
///                 (default: off)
///
/// Since: 1.7
/// #
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
/// #
/// @BlockdevOptionsFile
///
/// Driver specific block device options for the file backend and similar
/// protocols.
///
/// @filename:    path to the image file
///
/// Since: 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsFile {
    pub filename: String,
}
/// #
/// @BlockdevOptionsNull
///
/// Driver specific block device options for the null backend.
///
/// @size:    #optional size of the device in bytes.
/// @latency-ns: #optional emulated latency (in nanoseconds) in processing
///              requests. Default to zero which completes requests immediately.
///              (Since 2.4)
///
/// Since: 2.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsNull {
    pub latency_ns: u64,
    pub size: f64,
}
/// #
/// @BlockdevOptionsVVFAT
///
/// Driver specific block device options for the vvfat protocol.
///
/// @dir:         directory to be exported as FAT image
/// @fat-type:    #optional FAT type: 12, 16 or 32
/// @floppy:      #optional whether to export a floppy image (true) or
///               partitioned hard disk (false; default)
/// @rw:          #optional whether to allow write operations (default: false)
///
/// Since: 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsVVFAT {
    pub fat_type: f64,
    pub floppy: bool,
    pub rw: bool,
    pub dir: String,
}
/// #
/// @BlockdevOptionsGenericFormat
///
/// Driver specific block device options for image format that have no option
/// besides their data source.
///
/// @file:        reference to or definition of the data source block device
///
/// Since: 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsGenericFormat {
    pub file: String,
}
/// #
/// @BlockdevOptionsGenericCOWFormat
///
/// Driver specific block device options for image format that have no option
/// besides their data source and an optional backing file.
///
/// @backing:     #optional reference to or definition of the backing file block
///               device (if missing, taken from the image file content). It is
///               allowed to pass an empty string here in order to disable the
///               default backing file.
///
/// Since: 1.7
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsGenericCOWFormat {
    base: BlockdevOptionsGenericFormat,
    pub backing: String,
}
/// #
/// @Qcow2OverlapCheckFlags
///
/// Structure of flags for each metadata structure. Setting a field to "true"
/// makes qemu guard that structure against unintended overwriting. The default
/// value is chosen according to the template given.
///
/// @template: Specifies a template mode which can be adjusted using the other
///            flags, defaults to "cached"
///
/// Since: 2.2
/// #
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
/// #
/// @BlockdevOptionsQcow2
///
/// Driver specific block device options for qcow2.
///
/// @lazy-refcounts:        #optional whether to enable the lazy refcounts
///                         feature (default is taken from the image file)
///
/// @pass-discard-request:  #optional whether discard requests to the qcow2
///                         device should be forwarded to the data source
///
/// @pass-discard-snapshot: #optional whether discard requests for the data source
///                         should be issued when a snapshot operation (e.g.
///                         deleting a snapshot) frees clusters in the qcow2 file
///
/// @pass-discard-other:    #optional whether discard requests for the data source
///                         should be issued on other occasions where a cluster
///                         gets freed
///
/// @overlap-check:         #optional which overlap checks to perform for writes
///                         to the image, defaults to "cached" (since 2.2)
///
/// @cache-size:            #optional the maximum total size of the L2 table and
///                         refcount block caches in bytes (since 2.2)
///
/// @l2-cache-size:         #optional the maximum size of the L2 table cache in
///                         bytes (since 2.2)
///
/// @refcount-cache-size:   #optional the maximum size of the refcount block cache
///                         in bytes (since 2.2)
///
/// Since: 1.7
/// #
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
/// #
/// @BlockdevOptionsArchipelago
///
/// Driver specific block device options for Archipelago.
///
/// @volume:              Name of the Archipelago volume image
///
/// @mport:               #optional The port number on which mapperd is
///                       listening. This is optional
///                       and if not specified, QEMU will make Archipelago
///                       use the default port (1001).
///
/// @vport:               #optional The port number on which vlmcd is
///                       listening. This is optional
///                       and if not specified, QEMU will make Archipelago
///                       use the default port (501).
///
/// @segment:             #optional The name of the shared memory segment
///                       Archipelago stack is using. This is optional
///                       and if not specified, QEMU will make Archipelago
///                       use the default value, "archipelago".
/// Since: 2.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsArchipelago {
    pub mport: f64,
    pub segment: String,
    pub vport: f64,
    pub volume: String,
}
/// #
/// @BlkdebugInjectErrorOptions
///
/// Describes a single error injection for blkdebug.
///
/// @event:       trigger event
///
/// @state:       #optional the state identifier blkdebug needs to be in to
///               actually trigger the event; defaults to "any"
///
/// @errno:       #optional error identifier (errno) to be returned; defaults to
///               EIO
///
/// @sector:      #optional specifies the sector index which has to be affected
///               in order to actually trigger the event; defaults to "any
///               sector"
///
/// @once:        #optional disables further events after this one has been
///               triggered; defaults to false
///
/// @immediately: #optional fail immediately; defaults to false
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlkdebugInjectErrorOptions {
    pub errno: f64,
    pub immediately: bool,
    pub once: bool,
    pub sector: f64,
    pub state: f64,
    pub event: String,
}
/// #
/// @BlkdebugSetStateOptions
///
/// Describes a single state-change event for blkdebug.
///
/// @event:       trigger event
///
/// @state:       #optional the current state identifier blkdebug needs to be in;
///               defaults to "any"
///
/// @new_state:   the state identifier blkdebug is supposed to assume if
///               this event is triggered
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlkdebugSetStateOptions {
    pub state: f64,
    pub event: String,
    pub new_state: f64,
}
/// #
/// @BlockdevOptionsBlkdebug
///
/// Driver specific block device options for blkdebug.
///
/// @image:           underlying raw block device (or image file)
///
/// @config:          #optional filename of the configuration file
///
/// @align:           #optional required alignment for requests in bytes
///
/// @inject-error:    #optional array of error injection descriptions
///
/// @set-state:       #optional array of state-change descriptions
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsBlkdebug {
    pub align: f64,
    pub config: String,
    pub inject_error: Vec<String>,
    pub set_state: Vec<String>,
    pub image: String,
}
/// #
/// @BlockdevOptionsBlkverify
///
/// Driver specific block device options for blkverify.
///
/// @test:    block device to be tested
///
/// @raw:     raw image used for verification
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsBlkverify {
    pub raw: String,
    pub test: String,
}
/// #
/// @BlockdevOptionsQuorum
///
/// Driver specific block device options for Quorum
///
/// @blkverify:      #optional true if the driver must print content mismatch
///                  set to false by default
///
/// @children:       the children block devices to use
///
/// @vote-threshold: the vote limit under which a read will fail
///
/// @rewrite-corrupted: #optional rewrite corrupted data when quorum is reached
///                     (Since 2.1)
///
/// @read-pattern: #optional choose read pattern and set to quorum by default
///                (Since 2.2)
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct BlockdevOptionsQuorum {
    pub blkverify: bool,
    pub read_pattern: String,
    pub rewrite_corrupted: bool,
    pub children: Vec<String>,
    pub vote_threshold: f64,
}
/// #
/// @BlockdevOptions
///
/// Options for creating a block device.
///
/// Since: 1.7
/// #
#[derive(Debug,RustcDecodable)]
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
/// #
/// @VersionTriple
///
/// A three-part version number.
///
/// @qemu.major:  The major version number.
///
/// @qemu.minor:  The minor version number.
///
/// @qemu.micro:  The micro version number.
///
/// Since: 2.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct VersionTriple {
    pub major: f64,
    pub micro: f64,
    pub minor: f64,
}
/// #
/// @VersionInfo:
///
/// A description of QEMU"s version.
///
/// @qemu:        The version of QEMU.  By current convention, a micro
///               version of 50 signifies a development branch.  A micro version
///               greater than or equal to 90 signifies a release candidate for
///               the next minor version.  A micro version of less than 50
///               signifies a stable release.
///
/// @package:     QEMU will always set this field to an empty string.  Downstream
///               versions of QEMU should set this to a non-empty string.  The
///               exact format depends on the downstream however it highly
///               recommended that a unique name is used.
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct VersionInfo {
    pub package: String,
    pub qemu: String,
}
/// #
/// @CommandInfo:
///
/// Information about a QMP command
///
/// @name: The command name
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct CommandInfo {
    pub name: String,
}
/// #
/// @TraceEventInfo:
///
/// Information of a tracing event.
///
/// @name: Event name.
/// @state: Tracing state.
///
/// Since 2.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct TraceEventInfo {
    pub name: String,
    pub state: String,
}
/// #
/// @NameInfo:
///
/// Guest name information.
///
/// @name: #optional The name of the guest
///
/// Since 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct NameInfo {
    pub name: String,
}
/// #
/// @KvmInfo:
///
/// Information about support for KVM acceleration
///
/// @enabled: true if KVM acceleration is active
///
/// @present: true if KVM acceleration is built into this executable
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct KvmInfo {
    pub enabled: bool,
    pub present: bool,
}
/// #
/// @StatusInfo:
///
/// Information about VCPU run state
///
/// @running: true if all VCPUs are runnable, false if not runnable
///
/// @singlestep: true if VCPUs are in single-step mode
///
/// @status: the virtual machine @RunState
///
/// Since:  0.14.0
///
/// Notes: @singlestep is enabled through the GDB stub
/// #
#[derive(Debug, RustcDecodable)]
pub struct StatusInfo {
    pub running: bool,
    pub singlestep: bool,
    pub status: String,
}
/// #
/// @UuidInfo:
///
/// Guest UUID information.
///
/// @UUID: the UUID of the guest
///
/// Since: 0.14.0
///
/// Notes: If no UUID was specified for the guest, a null UUID is returned.
/// #
#[derive(Debug, RustcDecodable)]
pub struct UuidInfo {
    pub UUID: String,
}
/// #
/// @ChardevInfo:
///
/// Information about a character device.
///
/// @label: the label of the character device
///
/// @filename: the filename of the character device
///
/// @frontend-open: shows whether the frontend device attached to this backend
///                 (eg. with the chardev=... option) is in open or closed state
///                 (since 2.1)
///
/// Notes: @filename is encoded using the QEMU command line character device
///        encoding.  See the QEMU man page for details.
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevInfo {
    pub filename: String,
    pub frontend_open: bool,
    pub label: String,
}
/// #
/// @ChardevBackendInfo:
///
/// Information about a character device backend
///
/// @name: The backend name
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevBackendInfo {
    pub name: String,
}
/// #
/// @EventInfo:
///
/// Information about a QMP event
///
/// @name: The event name
///
/// Since: 1.2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct EventInfo {
    pub name: String,
}
/// #
/// @MigrationStats
///
/// Detailed migration status.
///
/// @transferred: amount of bytes already transferred to the target VM
///
/// @remaining: amount of bytes remaining to be transferred to the target VM
///
/// @total: total amount of bytes involved in the migration process
///
/// @duplicate: number of duplicate (zero) pages (since 1.2)
///
/// @skipped: number of skipped zero pages (since 1.5)
///
/// @normal : number of normal pages (since 1.2)
///
/// @normal-bytes: number of normal bytes sent (since 1.2)
///
/// @dirty-pages-rate: number of pages dirtied by second by the
///        guest (since 1.3)
///
/// @mbps: throughput in megabits/sec. (since 1.6)
///
/// @dirty-sync-count: number of times that dirty ram was synchronized (since 2.1)
///
/// Since: 0.14.0
/// #
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
/// #
/// @XBZRLECacheStats
///
/// Detailed XBZRLE migration cache statistics
///
/// @cache-size: XBZRLE cache size
///
/// @bytes: amount of bytes already transferred to the target VM
///
/// @pages: amount of pages transferred to the target VM
///
/// @cache-miss: number of cache miss
///
/// @cache-miss-rate: rate of cache miss (since 2.1)
///
/// @overflow: number of overflows
///
/// Since: 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct XBZRLECacheStats {
    pub bytes: f64,
    pub cache_miss: f64,
    pub cache_miss_rate: String,
    pub cache_size: f64,
    pub overflow: f64,
    pub pages: f64,
}
/// #
/// @MigrationInfo
///
/// Information about current migration process.
///
/// @status: #optional @MigrationStatus describing the current migration status.
///          If this field is not returned, no migration process
///          has been initiated
///
/// @ram: #optional @MigrationStats containing detailed migration
///       status, only returned if status is "active" or
///       "completed"(since 1.2)
///
/// @disk: #optional @MigrationStats containing detailed disk migration
///        status, only returned if status is "active" and it is a block
///        migration
///
/// @xbzrle-cache: #optional @XBZRLECacheStats containing detailed XBZRLE
///                migration statistics, only returned if XBZRLE feature is on and
///                status is "active" or "completed" (since 1.2)
///
/// @total-time: #optional total amount of milliseconds since migration started.
///        If migration has ended, it returns the total migration
///        time. (since 1.2)
///
/// @downtime: #optional only present when migration finishes correctly
///        total downtime in milliseconds for the guest.
///        (since 1.3)
///
/// @expected-downtime: #optional only present while migration is active
///        expected downtime in milliseconds for the guest in last walk
///        of the dirty bitmap. (since 1.3)
///
/// @setup-time: #optional amount of setup time in milliseconds _before_ the
///        iterations begin but _after_ the QMP command is issued. This is designed
///        to provide an accounting of any activities (such as RDMA pinning) which
///        may be expensive, but do not actually occur during the iterative
///        migration rounds themselves. (since 1.6)
///
/// Since: 0.14.0
/// #
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
/// #
/// @MigrationCapabilityStatus
///
/// Migration capability information
///
/// @capability: capability enum
///
/// @state: capability state bool
///
/// Since: 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct MigrationCapabilityStatus {
    pub capability: String,
    pub state: bool,
}
///
/// @MigrationParameters
///
/// @compress-level: compression level
///
/// @compress-threads: compression thread count
///
/// @decompress-threads: decompression thread count
///
/// Since: 2.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct MigrationParameters {
    pub compress_level: f64,
    pub compress_threads: f64,
    pub decompress_threads: f64,
}
/// #
/// @MouseInfo:
///
/// Information about a mouse device.
///
/// @name: the name of the mouse device
///
/// @index: the index of the mouse device
///
/// @current: true if this device is currently receiving mouse events
///
/// @absolute: true if this device supports absolute coordinates as input
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct MouseInfo {
    pub absolute: bool,
    pub current: bool,
    pub index: f64,
    pub name: String,
}
/// #
/// @CpuInfo:
///
/// Information about a virtual CPU
///
/// @CPU: the index of the virtual CPU
///
/// @current: this only exists for backwards compatible and should be ignored
///
/// @halted: true if the virtual CPU is in the halt state.  Halt usually refers
///          to a processor specific low power mode.
///
/// @qom_path: path to the CPU object in the QOM tree (since 2.4)
///
/// @pc: #optional If the target is i386 or x86_64, this is the 64-bit instruction
///                pointer.
///                If the target is Sparc, this is the PC component of the
///                instruction pointer.
///
/// @nip: #optional If the target is PPC, the instruction pointer
///
/// @npc: #optional If the target is Sparc, the NPC component of the instruction
///                 pointer
///
/// @PC: #optional If the target is MIPS, the instruction pointer
///
/// @thread_id: ID of the underlying host thread
///
/// Since: 0.14.0
///
/// Notes: @halted is a transient state that changes frequently.  By the time the
///        data is sent to the client, the guest may no longer be halted.
/// #
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
/// #
/// @IOThreadInfo:
///
/// Information about an iothread
///
/// @id: the identifier of the iothread
///
/// @thread-id: ID of the underlying host thread
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct IOThreadInfo {
    pub id: String,
    pub thread_id: f64,
}
/// #
/// @VncBasicInfo
///
/// The basic information for vnc network connection
///
/// @host: IP address
///
/// @service: The service name of the vnc port. This may depend on the host
///           system"s service database so symbolic names should not be relied
///           on.
///
/// @family: address family
///
/// @websocket: true in case the socket is a websocket (since 2.3).
///
/// Since: 2.1
/// #
#[derive(Debug, RustcDecodable)]
pub struct VncBasicInfo {
    pub family: String,
    pub host: String,
    pub service: String,
    pub websocket: bool,
}
/// #
/// @VncServerInfo
///
/// The network connection information for server
///
/// @auth: #optional, authentication method
///
/// Since: 2.1
/// #
#[derive(Debug, RustcDecodable)]
pub struct VncServerInfo {
    base: VncBasicInfo,
    pub auth: String,
}
/// #
/// @VncClientInfo:
///
/// Information about a connected VNC client.
///
/// @x509_dname: #optional If x509 authentication is in use, the Distinguished
///              Name of the client.
///
/// @sasl_username: #optional If SASL authentication is in use, the SASL username
///                 used for authentication.
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct VncClientInfo {
    base: VncBasicInfo,
    pub sasl_username: String,
    pub x509_dname: String,
}
/// #
/// @VncInfo:
///
/// Information about the VNC session.
///
/// @enabled: true if the VNC server is enabled, false otherwise
///
/// @host: #optional The hostname the VNC server is bound to.  This depends on
///        the name resolution on the host and may be an IP address.
///
/// @family: #optional "ipv6" if the host is listening for IPv6 connections
///                    "ipv4" if the host is listening for IPv4 connections
///                    "unix" if the host is listening on a unix domain socket
///                    "unknown" otherwise
///
/// @service: #optional The service name of the server"s port.  This may depends
///           on the host system"s service database so symbolic names should not
///           be relied on.
///
/// @auth: #optional the current authentication type used by the server
///        "none" if no authentication is being used
///        "vnc" if VNC authentication is being used
///        "vencrypt+plain" if VEncrypt is used with plain text authentication
///        "vencrypt+tls+none" if VEncrypt is used with TLS and no authentication
///        "vencrypt+tls+vnc" if VEncrypt is used with TLS and VNC authentication
///        "vencrypt+tls+plain" if VEncrypt is used with TLS and plain text auth
///        "vencrypt+x509+none" if VEncrypt is used with x509 and no auth
///        "vencrypt+x509+vnc" if VEncrypt is used with x509 and VNC auth
///        "vencrypt+x509+plain" if VEncrypt is used with x509 and plain text auth
///        "vencrypt+tls+sasl" if VEncrypt is used with TLS and SASL auth
///        "vencrypt+x509+sasl" if VEncrypt is used with x509 and SASL auth
///
/// @clients: a list of @VncClientInfo of all currently connected clients
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct VncInfo {
    pub auth: String,
    pub clients: Vec<String>,
    pub family: String,
    pub host: String,
    pub service: String,
    pub enabled: bool,
}
/// #
/// @VncInfo2:
///
/// Information about a vnc server
///
/// @id: vnc server name.
///
/// @server: A list of @VncBasincInfo describing all listening sockets.
///          The list can be empty (in case the vnc server is disabled).
///          It also may have multiple entries: normal + websocket,
///          possibly also ipv4 + ipv6 in the future.
///
/// @clients: A list of @VncClientInfo of all currently connected clients.
///           The list can be empty, for obvious reasons.
///
/// @auth: The current authentication type used by the server
///
/// @vencrypt: #optional The vencrypt sub authentication type used by the server,
///            only specified in case auth == vencrypt.
///
/// @display: #optional The display device the vnc server is linked to.
///
/// Since: 2.3
/// #
#[derive(Debug, RustcDecodable)]
pub struct VncInfo2 {
    pub display: String,
    pub vencrypt: String,
    pub auth: String,
    pub clients: Vec<String>,
    pub id: String,
    pub server: Vec<String>,
}
/// #
/// @SpiceBasicInfo
///
/// The basic information for SPICE network connection
///
/// @host: IP address
///
/// @port: port number
///
/// @family: address family
///
/// Since: 2.1
/// #
#[derive(Debug, RustcDecodable)]
pub struct SpiceBasicInfo {
    pub family: String,
    pub host: String,
    pub port: String,
}
/// #
/// @SpiceServerInfo
///
/// Information about a SPICE server
///
/// @auth: #optional, authentication method
///
/// Since: 2.1
/// #
#[derive(Debug, RustcDecodable)]
pub struct SpiceServerInfo {
    base: SpiceBasicInfo,
    pub auth: String,
}
/// #
/// @SpiceChannel
///
/// Information about a SPICE client channel.
///
/// @connection-id: SPICE connection id number.  All channels with the same id
///                 belong to the same SPICE session.
///
/// @channel-type: SPICE channel type number.  "1" is the main control
///                channel, filter for this one if you want to track spice
///                sessions only
///
/// @channel-id: SPICE channel ID number.  Usually "0", might be different when
///              multiple channels of the same type exist, such as multiple
///              display channels in a multihead setup
///
/// @tls: true if the channel is encrypted, false otherwise.
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct SpiceChannel {
    base: SpiceBasicInfo,
    pub channel_id: f64,
    pub channel_type: f64,
    pub connection_id: f64,
    pub tls: bool,
}
/// #
/// @SpiceInfo
///
/// Information about the SPICE session.
///
/// @enabled: true if the SPICE server is enabled, false otherwise
///
/// @migrated: true if the last guest migration completed and spice
///            migration had completed as well. false otherwise.
///
/// @host: #optional The hostname the SPICE server is bound to.  This depends on
///        the name resolution on the host and may be an IP address.
///
/// @port: #optional The SPICE server"s port number.
///
/// @compiled-version: #optional SPICE server version.
///
/// @tls-port: #optional The SPICE server"s TLS port number.
///
/// @auth: #optional the current authentication type used by the server
///        "none"  if no authentication is being used
///        "spice" uses SASL or direct TLS authentication, depending on command
///                line options
///
/// @mouse-mode: The mode in which the mouse cursor is displayed currently. Can
///              be determined by the client or the server, or unknown if spice
///              server doesn"t provide this information.
///
///              Since: 1.1
///
/// @channels: a list of @SpiceChannel for each active spice channel
///
/// Since: 0.14.0
/// #
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
/// #
/// @BalloonInfo:
///
/// Information about the guest balloon device.
///
/// @actual: the number of bytes the balloon currently contains
///
/// Since: 0.14.0
///
/// #
#[derive(Debug, RustcDecodable)]
pub struct BalloonInfo {
    pub actual: f64,
}
/// #
/// @PciMemoryRange:
///
/// A PCI device memory region
///
/// @base: the starting address (guest physical)
///
/// @limit: the ending address (guest physical)
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct PciMemoryRange {
    pub base: f64,
    pub limit: f64,
}
/// #
/// @PciMemoryRegion
///
/// Information about a PCI device I/O region.
///
/// @bar: the index of the Base Address Register for this region
///
/// @type: "io" if the region is a PIO region
///        "memory" if the region is a MMIO region
///
/// @prefetch: #optional if @type is "memory", true if the memory is prefetchable
///
/// @mem_type_64: #optional if @type is "memory", true if the BAR is 64-bit
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct PciMemoryRegion {
    pub mem_type_64: bool,
    pub prefetch: bool,
    pub address: f64,
    pub bar: f64,
    pub size: f64,
    pub qemu_type: String,
}
/// #
/// @PciBusInfo:
///
/// Information about a bus of a PCI Bridge device
///
/// @number: primary bus interface number.  This should be the number of the
///          bus the device resides on.
///
/// @secondary: secondary bus interface number.  This is the number of the
///             main bus for the bridge
///
/// @subordinate: This is the highest number bus that resides below the
///               bridge.
///
/// @io_range: The PIO range for all devices on this bridge
///
/// @memory_range: The MMIO range for all devices on this bridge
///
/// @prefetchable_range: The range of prefetchable MMIO for all devices on
///                      this bridge
///
/// Since: 2.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct PciBusInfo {
    pub io_range: String,
    pub memory_range: String,
    pub number: f64,
    pub prefetchable_range: String,
    pub secondary: f64,
    pub subordinate: f64,
}
/// #
/// @PciBridgeInfo:
///
/// Information about a PCI Bridge device
///
/// @bus: information about the bus the device resides on
///
/// @devices: a list of @PciDeviceInfo for each device on this bridge
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct PciBridgeInfo {
    pub devices: Vec<String>,
    pub bus: String,
}
/// #
/// @PciDeviceClass:
///
/// Information about the Class of a PCI device
///
/// @desc: #optional a string description of the device"s class
///
/// @class: the class code of the device
///
/// Since: 2.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct PciDeviceClass {
    pub desc: String,
    pub class: f64,
}
/// #
/// @PciDeviceId:
///
/// Information about the Id of a PCI device
///
/// @device: the PCI device id
///
/// @vendor: the PCI vendor id
///
/// Since: 2.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct PciDeviceId {
    pub device: f64,
    pub vendor: f64,
}
/// #
/// @PciDeviceInfo:
///
/// Information about a PCI device
///
/// @bus: the bus number of the device
///
/// @slot: the slot the device is located in
///
/// @function: the function of the slot used by the device
///
/// @class_info: the class of the device
///
/// @id: the PCI device id
///
/// @irq: #optional if an IRQ is assigned to the device, the IRQ number
///
/// @qdev_id: the device name of the PCI device
///
/// @pci_bridge: if the device is a PCI bridge, the bridge information
///
/// @regions: a list of the PCI I/O regions associated with the device
///
/// Notes: the contents of @class_info.desc are not stable and should only be
///        treated as informational.
///
/// Since: 0.14.0
/// #
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
/// #
/// @PciInfo:
///
/// Information about a PCI bus
///
/// @bus: the bus index
///
/// @devices: a list of devices on this bus
///
/// Since: 0.14.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct PciInfo {
    pub bus: f64,
    pub devices: Vec<String>,
}
/// #
/// @Abort
///
/// This action can be used to test transaction failure.
///
/// Since: 1.6
/// ##
#[derive(Debug, RustcDecodable)]
pub struct Abort {

            }
/// #
/// @TransactionAction
///
/// A discriminated record of operations that can be performed with
/// @transaction.
///
/// Since 1.1
///
/// drive-backup since 1.6
/// abort since 1.6
/// blockdev-snapshot-internal-sync since 1.7
/// blockdev-backup since 2.3
/// #
#[derive(Debug,RustcDecodable)]
pub struct TransactionAction {
    pub abort: String,
    pub blockdev_backup: String,
    pub blockdev_snapshot_internal_sync: String,
    pub blockdev_snapshot_sync: String,
    pub drive_backup: String,
}
/// #
/// @ObjectPropertyInfo:
///
/// @name: the name of the property
///
/// @type: the type of the property.  This will typically come in one of four
///        forms:
///
///        1) A primitive type such as "u8", "u16", "bool", "str", or "double".
///           These types are mapped to the appropriate JSON type.
///
///        2) A legacy type in the form "legacy<subtype>" where subtype is the
///           legacy qdev typename.  These types are always treated as strings.
///
///        3) A child type in the form "child<subtype>" where subtype is a qdev
///           device type name.  Child properties create the composition tree.
///
///        4) A link type in the form "link<subtype>" where subtype is a qdev
///           device type name.  Link properties form the device model graph.
///
/// Since: 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct ObjectPropertyInfo {
    pub name: String,
    pub qemu_type: String,
}
/// #
/// @ObjectTypeInfo:
///
/// This structure describes a search result from @qom-list-types
///
/// @name: the type name found in the search
///
/// Since: 1.1
///
/// Notes: This command is experimental and may change syntax in future releases.
/// #
#[derive(Debug, RustcDecodable)]
pub struct ObjectTypeInfo {
    pub name: String,
}
/// #
/// @DevicePropertyInfo:
///
/// Information about device properties.
///
/// @name: the name of the property
/// @type: the typename of the property
/// @description: #optional if specified, the description of the property.
///               (since 2.2)
///
/// Since: 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct DevicePropertyInfo {
    pub description: String,
    pub name: String,
    pub qemu_type: String,
}
/// #
/// @DumpGuestMemoryCapability:
///
/// A list of the available formats for dump-guest-memory
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct DumpGuestMemoryCapability {
    pub formats: Vec<String>,
}
/// #
/// @NetdevNoneOptions
///
/// Use it alone to have zero network devices.
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetdevNoneOptions {

            }
/// #
/// @NetLegacyNicOptions
///
/// Create a new Network Interface Card.
///
/// @netdev: #optional id of -netdev to connect to
///
/// @macaddr: #optional MAC address
///
/// @model: #optional device model (e1000, rtl8139, virtio etc.)
///
/// @addr: #optional PCI device address
///
/// @vectors: #optional number of MSI-x vectors, 0 to disable MSI-X
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetLegacyNicOptions {
    pub addr: String,
    pub macaddr: String,
    pub model: String,
    pub netdev: String,
    pub vectors: u32,
}
/// #
/// @String
///
/// A fat type wrapping "str", to be embedded in lists.
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct qemu_string {
    pub String: String,
}
/// #
/// @NetdevUserOptions
///
/// Use the user mode network stack which requires no administrator privilege to
/// run.
///
/// @hostname: #optional client hostname reported by the builtin DHCP server
///
/// @restrict: #optional isolate the guest from the host
///
/// @ip: #optional legacy parameter, use net= instead
///
/// @net: #optional IP address and optional netmask
///
/// @host: #optional guest-visible address of the host
///
/// @tftp: #optional root directory of the built-in TFTP server
///
/// @bootfile: #optional BOOTP filename, for use with tftp=
///
/// @dhcpstart: #optional the first of the 16 IPs the built-in DHCP server can
///             assign
///
/// @dns: #optional guest-visible address of the virtual nameserver
///
/// @dnssearch: #optional list of DNS suffixes to search, passed as DHCP option
///             to the guest
///
/// @smb: #optional root directory of the built-in SMB server
///
/// @smbserver: #optional IP address of the built-in SMB server
///
/// @hostfwd: #optional redirect incoming TCP or UDP host connections to guest
///           endpoints
///
/// @guestfwd: #optional forward guest TCP connections
///
/// Since 1.2
/// #
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
/// #
/// @NetdevTapOptions
///
/// Connect the host TAP network interface name to the VLAN.
///
/// @ifname: #optional interface name
///
/// @fd: #optional file descriptor of an already opened tap
///
/// @fds: #optional multiple file descriptors of already opened multiqueue capable
/// tap
///
/// @script: #optional script to initialize the interface
///
/// @downscript: #optional script to shut down the interface
///
/// @helper: #optional command to execute to configure bridge
///
/// @sndbuf: #optional send buffer limit. Understands [TGMKkb] suffixes.
///
/// @vnet_hdr: #optional enable the IFF_VNET_HDR flag on the tap interface
///
/// @vhost: #optional enable vhost-net network accelerator
///
/// @vhostfd: #optional file descriptor of an already opened vhost net device
///
/// @vhostfds: #optional file descriptors of multiple already opened vhost net
/// devices
///
/// @vhostforce: #optional vhost on for non-MSIX virtio guests
///
/// @queues: #optional number of queues to be created for multiqueue capable tap
///
/// Since 1.2
/// #
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
/// #
/// @NetdevSocketOptions
///
/// Connect the VLAN to a remote VLAN in another QEMU virtual machine using a TCP
/// socket connection.
///
/// @fd: #optional file descriptor of an already opened socket
///
/// @listen: #optional port number, and optional hostname, to listen on
///
/// @connect: #optional port number, and optional hostname, to connect to
///
/// @mcast: #optional UDP multicast address and port number
///
/// @localaddr: #optional source address and port for multicast and udp packets
///
/// @udp: #optional UDP unicast address and port number
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetdevSocketOptions {
    pub connect: String,
    pub fd: String,
    pub listen: String,
    pub localaddr: String,
    pub mcast: String,
    pub udp: String,
}
/// #
/// @NetdevL2TPv3Options
///
/// Connect the VLAN to Ethernet over L2TPv3 Static tunnel
///
/// @src: source address
///
/// @dst: destination address
///
/// @srcport: #optional source port - mandatory for udp, optional for ip
///
/// @dstport: #optional destination port - mandatory for udp, optional for ip
///
/// @ipv6: #optional - force the use of ipv6
///
/// @udp: #optional - use the udp version of l2tpv3 encapsulation
///
/// @cookie64: #optional - use 64 bit coookies
///
/// @counter: #optional have sequence counter
///
/// @pincounter: #optional pin sequence counter to zero -
///              workaround for buggy implementations or
///              networks with packet reorder
///
/// @txcookie: #optional 32 or 64 bit transmit cookie
///
/// @rxcookie: #optional 32 or 64 bit receive cookie
///
/// @txsession: 32 bit transmit session
///
/// @rxsession: #optional 32 bit receive session - if not specified
///             set to the same value as transmit
///
/// @offset: #optional additional offset - allows the insertion of
///          additional application-specific data before the packet payload
///
/// Since 2.1
/// #
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
/// #
/// @NetdevVdeOptions
///
/// Connect the VLAN to a vde switch running on the host.
///
/// @sock: #optional socket path
///
/// @port: #optional port number
///
/// @group: #optional group owner of socket
///
/// @mode: #optional permissions for socket
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetdevVdeOptions {
    pub group: String,
    pub mode: String,
    pub port: String,
    pub sock: String,
}
/// #
/// @NetdevDumpOptions
///
/// Dump VLAN network traffic to a file.
///
/// @len: #optional per-packet size limit (64k default). Understands [TGMKkb]
/// suffixes.
///
/// @file: #optional dump file path (default is qemu-vlan0.pcap)
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetdevDumpOptions {
    pub file: String,
    pub len: String,
}
/// #
/// @NetdevBridgeOptions
///
/// Connect a host TAP network interface to a host bridge device.
///
/// @br: #optional bridge name
///
/// @helper: #optional command to execute to configure bridge
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetdevBridgeOptions {
    pub br: String,
    pub helper: String,
}
/// #
/// @NetdevHubPortOptions
///
/// Connect two or more net clients through a software hub.
///
/// @hubid: hub identifier number
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetdevHubPortOptions {
    pub hubid: String,
}
/// #
/// @NetdevNetmapOptions
///
/// Connect a client to a netmap-enabled NIC or to a VALE switch port
///
/// @ifname: Either the name of an existing network interface supported by
///          netmap, or the name of a VALE port (created on the fly).
///          A VALE port name is in the form "valeXXX:YYY", where XXX and
///          YYY are non-negative integers. XXX identifies a switch and
///          YYY identifies a port of the switch. VALE ports having the
///          same XXX are therefore connected to the same switch.
///
/// @devname: #optional path of the netmap device (default: "/dev/netmap").
///
/// Since 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetdevNetmapOptions {
    pub devname: String,
    pub ifname: String,
}
/// #
/// @NetdevVhostUserOptions
///
/// Vhost-user network backend
///
/// @chardev: name of a unix socket chardev
///
/// @vhostforce: #optional vhost on for non-MSIX virtio guests (default: false).
///
/// Since 2.1
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetdevVhostUserOptions {
    pub vhostforce: bool,
    pub chardev: String,
}
/// #
/// @NetClientOptions
///
/// A discriminated record of network device traits.
///
/// Since 1.2
///
/// "l2tpv3" - since 2.1
///
/// #
#[derive(Debug,RustcDecodable)]
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
/// #
/// @NetLegacy
///
/// Captures the configuration of a network device; legacy.
///
/// @vlan: #optional vlan number
///
/// @id: #optional identifier for monitor commands
///
/// @name: #optional identifier for monitor commands, ignored if @id is present
///
/// @opts: device type specific properties (legacy)
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct NetLegacy {
    pub id: String,
    pub name: String,
    pub vlan: String,
    pub opts: String,
}
/// #
/// @Netdev
///
/// Captures the configuration of a network device.
///
/// @id: identifier for monitor commands.
///
/// @opts: device type specific properties
///
/// Since 1.2
/// #
#[derive(Debug, RustcDecodable)]
pub struct Netdev {
    pub id: String,
    pub opts: String,
}
/// #
/// @InetSocketAddress
///
/// Captures a socket address or address range in the Internet namespace.
///
/// @host: host part of the address
///
/// @port: port part of the address, or lowest port if @to is present
///
/// @to: highest port to try
///
/// @ipv4: whether to accept IPv4 addresses, default try both IPv4 and IPv6
///        #optional
///
/// @ipv6: whether to accept IPv6 addresses, default try both IPv4 and IPv6
///        #optional
///
/// Since 1.3
/// #
#[derive(Debug, RustcDecodable)]
pub struct InetSocketAddress {
    pub ipv4: bool,
    pub ipv6: bool,
    pub to: String,
    pub host: String,
    pub port: String,
}
/// #
/// @UnixSocketAddress
///
/// Captures a socket address in the local ("Unix socket") namespace.
///
/// @path: filesystem path to use
///
/// Since 1.3
/// #
#[derive(Debug, RustcDecodable)]
pub struct UnixSocketAddress {
    pub path: String,
}
/// #
/// @SocketAddress
///
/// Captures the address of a socket, which could also be a named file descriptor
///
/// Since 1.3
/// #
#[derive(Debug,RustcDecodable)]
pub struct SocketAddress {
    pub fd: String,
    pub inet: String,
    pub unix: String,
}
/// #
/// @MachineInfo:
///
/// Information describing a machine.
///
/// @name: the name of the machine
///
/// @alias: #optional an alias for the machine name
///
/// @default: #optional whether the machine is default
///
/// @cpu-max: maximum number of CPUs supported by the machine type
///           (since 1.5.0)
///
/// Since: 1.2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct MachineInfo {
    pub alias: String,
    pub is_default: bool,
    pub cpu_max: f64,
    pub name: String,
}
/// #
/// @CpuDefinitionInfo:
///
/// Virtual CPU definition.
///
/// @name: the name of the CPU definition
///
/// Since: 1.2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct CpuDefinitionInfo {
    pub name: String,
}
/// @AddfdInfo:
///
/// Information about a file descriptor that was added to an fd set.
///
/// @fdset-id: The ID of the fd set that @fd was added to.
///
/// @fd: The file descriptor that was received via SCM rights and
///      added to the fd set.
///
/// Since: 1.2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct AddfdInfo {
    pub fd: f64,
    pub fdset_id: f64,
}
/// #
/// @FdsetFdInfo:
///
/// Information about a file descriptor that belongs to an fd set.
///
/// @fd: The file descriptor value.
///
/// @opaque: #optional A free-form string that can be used to describe the fd.
///
/// Since: 1.2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct FdsetFdInfo {
    pub opaque: String,
    pub fd: f64,
}
/// #
/// @FdsetInfo:
///
/// Information about an fd set.
///
/// @fdset-id: The ID of the fd set.
///
/// @fds: A list of file descriptors that belong to this fd set.
///
/// Since: 1.2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct FdsetInfo {
    pub fds: Vec<String>,
    pub fdset_id: f64,
}
/// #
/// @TargetInfo:
///
/// Information describing the QEMU target.
///
/// @arch: the target architecture (eg "x86_64", "i386", etc)
///
/// Since: 1.2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct TargetInfo {
    pub arch: String,
}
/// #
/// @KeyValue
///
/// Represents a keyboard key.
///
/// Since: 1.3.0
/// #
#[derive(Debug,RustcDecodable)]
pub struct KeyValue {
    pub number: f64,
    pub qcode: String,
}
/// #
/// @ChardevFile:
///
/// Configuration info for file chardevs.
///
/// @in:  #optional The name of the input file
/// @out: The name of the output file
///
/// Since: 1.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevFile {
    pub qemu_in: String,
    pub out: String,
}
/// #
/// @ChardevHostdev:
///
/// Configuration info for device and pipe chardevs.
///
/// @device: The name of the special file for the device,
///          i.e. /dev/ttyS0 on Unix or COM1: on Windows
/// @type: What kind of device this is.
///
/// Since: 1.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevHostdev {
    pub device: String,
}
/// #
/// @ChardevSocket:
///
/// Configuration info for (stream) socket chardevs.
///
/// @addr: socket address to listen on (server=true)
///        or connect to (server=false)
/// @server: #optional create server socket (default: true)
/// @wait: #optional wait for incoming connection on server
///        sockets (default: false).
/// @nodelay: #optional set TCP_NODELAY socket option (default: false)
/// @telnet: #optional enable telnet protocol on server
///          sockets (default: false)
/// @reconnect: #optional For a client socket, if a socket is disconnected,
///          then attempt a reconnect after the given number of seconds.
///          Setting this to zero disables this function. (default: 0)
///          (Since: 2.2)
///
/// Since: 1.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevSocket {
    pub nodelay: bool,
    pub reconnect: f64,
    pub server: bool,
    pub telnet: bool,
    pub wait: bool,
    pub addr: String,
}
/// #
/// @ChardevUdp:
///
/// Configuration info for datagram socket chardevs.
///
/// @remote: remote address
/// @local: #optional local address
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevUdp {
    pub local: String,
    pub remote: String,
}
/// #
/// @ChardevMux:
///
/// Configuration info for mux chardevs.
///
/// @chardev: name of the base chardev.
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevMux {
    pub chardev: String,
}
/// #
/// @ChardevStdio:
///
/// Configuration info for stdio chardevs.
///
/// @signal: #optional Allow signals (such as SIGINT triggered by ^C)
///          be delivered to qemu.  Default: true in -nographic mode,
///          false otherwise.
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevStdio {
    pub signal: bool,
}
/// #
/// @ChardevSpiceChannel:
///
/// Configuration info for spice vm channel chardevs.
///
/// @type: kind of channel (for example vdagent).
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevSpiceChannel {
    pub qemu_type: String,
}
/// #
/// @ChardevSpicePort:
///
/// Configuration info for spice port chardevs.
///
/// @fqdn: name of the channel (see docs/spice-port-fqdn.txt)
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevSpicePort {
    pub fqdn: String,
}
/// #
/// @ChardevVC:
///
/// Configuration info for virtual console chardevs.
///
/// @width:  console width,  in pixels
/// @height: console height, in pixels
/// @cols:   console width,  in chars
/// @rows:   console height, in chars
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevVC {
    pub cols: f64,
    pub height: f64,
    pub rows: f64,
    pub width: f64,
}
/// #
/// @ChardevRingbuf:
///
/// Configuration info for ring buffer chardevs.
///
/// @size: #optional ring buffer size, must be power of two, default is 65536
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevRingbuf {
    pub size: f64,
}
/// #
/// @ChardevBackend:
///
/// Configuration info for the new chardev backend.
///
/// Since: 1.4 (testdev since 2.2)
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevDummy {

            }

#[derive(Debug,RustcDecodable)]
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
/// #
/// @ChardevReturn:
///
/// Return info about the chardev backend just created.
///
/// @pty: #optional name of the slave pseudoterminal device, present if
///       and only if a chardev of type "pty" was created
///
/// Since: 1.4
/// #
#[derive(Debug, RustcDecodable)]
pub struct ChardevReturn {
    pub pty: String,
}
/// #
/// @TPMPassthroughOptions:
///
/// Information about the TPM passthrough type
///
/// @path: #optional string describing the path used for accessing the TPM device
///
/// @cancel-path: #optional string showing the TPM"s sysfs cancel file
///               for cancellation of TPM commands while they are executing
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct TPMPassthroughOptions {
    pub cancel_path: String,
    pub path: String,
}
/// #
/// @TpmTypeOptions:
///
/// A union referencing different TPM backend types" configuration options
///
/// @passthrough: The configuration options for the TPM passthrough type
///
/// Since: 1.5
/// #
#[derive(Debug,RustcDecodable)]
pub struct TpmTypeOptions {
    pub passthrough: String,
}
/// #
/// @TpmInfo:
///
/// Information about the TPM
///
/// @id: The Id of the TPM
///
/// @model: The TPM frontend model
///
/// @options: The TPM (backend) type configuration options
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct TPMInfo {
    pub id: String,
    pub model: String,
    pub options: String,
}
/// #
/// @AcpiTableOptions
///
/// Specify an ACPI table on the command line to load.
///
/// At most one of @file and @data can be specified. The list of files specified
/// by any one of them is loaded and concatenated in order. If both are omitted,
/// @data is implied.
///
/// Other fields / optargs can be used to override fields of the generic ACPI
/// table header; refer to the ACPI specification 5.0, section 5.2.6 System
/// Description Table Header. If a header field is not overridden, then the
/// corresponding value from the concatenated blob is used (in case of @file), or
/// it is filled in with a hard-coded value (in case of @data).
///
/// String fields are copied into the matching ACPI member from lowest address
/// upwards, and silently truncated / NUL-padded to length.
///
/// @sig: #optional table signature / identifier (4 bytes)
///
/// @rev: #optional table revision number (dependent on signature, 1 byte)
///
/// @oem_id: #optional OEM identifier (6 bytes)
///
/// @oem_table_id: #optional OEM table identifier (8 bytes)
///
/// @oem_rev: #optional OEM-supplied revision number (4 bytes)
///
/// @asl_compiler_id: #optional identifier of the utility that created the table
///                   (4 bytes)
///
/// @asl_compiler_rev: #optional revision number of the utility that created the
///                    table (4 bytes)
///
/// @file: #optional colon (:) separated list of pathnames to load and
///        concatenate as table data. The resultant binary blob is expected to
///        have an ACPI table header. At least one file is required. This field
///        excludes @data.
///
/// @data: #optional colon (:) separated list of pathnames to load and
///        concatenate as table data. The resultant binary blob must not have an
///        ACPI table header. At least one file is required. This field excludes
///        @file.
///
/// Since 1.5
/// #
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
/// #
/// @CommandLineParameterInfo:
///
/// Details about a single parameter of a command line option.
///
/// @name: parameter name
///
/// @type: parameter @CommandLineParameterType
///
/// @help: #optional human readable text string, not suitable for parsing.
///
/// @default: #optional default value string (since 2.1)
///
/// Since 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct CommandLineParameterInfo {
    pub default: String,
    pub help: String,
    pub name: String,
    pub qemu_type: String,
}
/// #
/// @CommandLineOptionInfo:
///
/// Details about a command line option, including its list of parameter details
///
/// @option: option name
///
/// @parameters: an array of @CommandLineParameterInfo
///
/// Since 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct CommandLineOptionInfo {
    pub option: String,
    pub parameters: Vec<String>,
}
/// #
/// @X86CPUFeatureWordInfo
///
/// Information about a X86 CPU feature word
///
/// @cpuid-input-eax: Input EAX value for CPUID instruction for that feature word
///
/// @cpuid-input-ecx: #optional Input ECX value for CPUID instruction for that
///                   feature word
///
/// @cpuid-register: Output register containing the feature bits
///
/// @features: value of output register, containing the feature bits
///
/// Since: 1.5
/// #
#[derive(Debug, RustcDecodable)]
pub struct X86CPUFeatureWordInfo {
    pub cpuid_input_ecx: f64,
    pub cpuid_input_eax: f64,
    pub cpuid_register: String,
    pub features: f64,
}
/// #
/// @RxFilterInfo:
///
/// Rx-filter information for a NIC.
///
/// @name: net client name
///
/// @promiscuous: whether promiscuous mode is enabled
///
/// @multicast: multicast receive state
///
/// @unicast: unicast receive state
///
/// @vlan: vlan receive state (Since 2.0)
///
/// @broadcast-allowed: whether to receive broadcast
///
/// @multicast-overflow: multicast table is overflowed or not
///
/// @unicast-overflow: unicast table is overflowed or not
///
/// @main-mac: the main macaddr string
///
/// @vlan-table: a list of active vlan id
///
/// @unicast-table: a list of unicast macaddr string
///
/// @multicast-table: a list of multicast macaddr string
///
/// Since 1.6
/// #
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
/// #
/// @InputKeyEvent
///
/// Keyboard input event.
///
/// @key:    Which key this event is for.
/// @down:   True for key-down and false for key-up events.
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct InputKeyEvent {
    pub down: bool,
    pub key: String,
}
/// #
/// @InputBtnEvent
///
/// Pointer button input event.
///
/// @button: Which button this event is for.
/// @down:   True for key-down and false for key-up events.
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct InputBtnEvent {
    pub button: String,
    pub down: bool,
}
/// #
/// @InputMoveEvent
///
/// Pointer motion input event.
///
/// @axis:   Which axis is referenced by @value.
/// @value:  Pointer position.  For absolute coordinates the
///          valid range is 0 -> 0x7ffff
///
/// Since: 2.0
/// #
#[derive(Debug, RustcDecodable)]
pub struct InputMoveEvent {
    pub axis: String,
    pub value: f64,
}
/// #
/// @InputEvent
///
/// Input event union.
///
/// @key: Input event of Keyboard
/// @btn: Input event of pointer buttons
/// @rel: Input event of relative pointer motion
/// @abs: Input event of absolute pointer motion
///
/// Since: 2.0
/// #
#[derive(Debug,RustcDecodable)]
pub struct InputEvent {
    pub abs: String,
    pub btn: String,
    pub key: String,
    pub rel: String,
}
/// #
/// @NumaOptions
///
/// A discriminated record of NUMA options. (for OptsVisitor)
///
/// Since 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub struct NumaOptions {
    pub node: String,
}
/// #
/// @NumaNodeOptions
///
/// Create a guest NUMA node. (for OptsVisitor)
///
/// @nodeid: #optional NUMA node ID (increase by 1 from 0 if omitted)
///
/// @cpus: #optional VCPUs belonging to this node (assign VCPUS round-robin
///         if omitted)
///
/// @mem: #optional memory size of this node; mutually exclusive with @memdev.
///       Equally divide total memory among nodes if both @mem and @memdev are
///       omitted.
///
/// @memdev: #optional memory backend object.  If specified for one node,
///          it must be specified for all nodes.
///
/// Since: 2.1
/// #
#[derive(Debug, RustcDecodable)]
pub struct NumaNodeOptions {
    pub cpus: Vec<String>,
    pub mem: String,
    pub memdev: String,
    pub nodeid: String,
}
/// #
/// @Memdev:
///
/// Information about memory backend
///
/// @size: memory backend size
///
/// @merge: enables or disables memory merge support
///
/// @dump: includes memory backend"s memory in a core dump or not
///
/// @prealloc: enables or disables memory preallocation
///
/// @host-nodes: host nodes for its memory policy
///
/// @policy: memory policy of memory backend
///
/// Since: 2.1
/// #
#[derive(Debug, RustcDecodable)]
pub struct Memdev {
    pub dump: bool,
    pub host_nodes: Vec<String>,
    pub merge: bool,
    pub policy: String,
    pub prealloc: bool,
    pub size: String,
}
/// #
/// @PCDIMMDeviceInfo:
///
/// PCDIMMDevice state information
///
/// @id: #optional device"s ID
///
/// @addr: physical address, where device is mapped
///
/// @size: size of memory that the device provides
///
/// @slot: slot number at which device is plugged in
///
/// @node: NUMA node number where device is plugged in
///
/// @memdev: memory backend linked with device
///
/// @hotplugged: true if device was hotplugged
///
/// @hotpluggable: true if device if could be added/removed while machine is running
///
/// Since: 2.1
/// #
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
/// #
/// @MemoryDeviceInfo:
///
/// Union containing information about a memory device
///
/// Since: 2.1
/// #
#[derive(Debug,RustcDecodable)]
pub struct MemoryDeviceInfo {
    pub dimm: String,
}
/// # @ACPIOSTInfo
///
/// OSPM Status Indication for a device
/// For description of possible values of @source and @status fields
/// see "_OST (OSPM Status Indication)" chapter of ACPI5.0 spec.
///
/// @device: #optional device ID associated with slot
///
/// @slot: slot ID, unique per slot of a given @slot-type
///
/// @slot-type: type of the slot
///
/// @source: an integer containing the source event
///
/// @status: an integer containing the status code
///
/// Since: 2.1
/// #
#[derive(Debug, RustcDecodable)]
pub struct ACPIOSTInfo {
    pub device: String,
    pub slot: String,
    pub slot_type: String,
    pub source: f64,
    pub status: f64,
}
