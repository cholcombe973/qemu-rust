use QemuCmd;
use rustc_serialize::json as rustc_json;
use rustc_serialize::Decodable as rustc_decodable;
use json;
use events::*;
use enums::*;
use structs::*;
/// #
/// @blockdev-snapshot-internal-sync
///
/// Synchronously take an internal snapshot of a block device, when the format
/// of the image used supports it.
///
/// For the arguments, see the documentation of BlockdevSnapshotInternal.
///
/// Returns: nothing on success
///          If @device is not a valid block device, DeviceNotFound
///          If any snapshot matching @name exists, or @name is empty,
///          GenericError
///          If the format of the image used does not support it,
///          BlockFormatFeatureNotSupported
///
/// Since 1.7
/// #
#[derive(Debug)]
pub struct blockdev_snapshot_internal_sync {

        }
impl blockdev_snapshot_internal_sync {
    pub fn new() -> blockdev_snapshot_internal_sync {
        blockdev_snapshot_internal_sync {}
    }
}
impl QemuCmd for blockdev_snapshot_internal_sync {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-snapshot-internal-sync".into();

        to_json.dump()
    }
}
/// #
/// @blockdev-snapshot-delete-internal-sync
///
/// Synchronously delete an internal snapshot of a block device, when the format
/// of the image used support it. The snapshot is identified by name or id or
/// both. One of the name or id is required. Return SnapshotInfo for the
/// successfully deleted snapshot.
///
/// @device: the name of the device to delete the snapshot from
///
/// @id: optional the snapshot"s ID to be deleted
///
/// @name: optional the snapshot"s name to be deleted
///
/// Returns: SnapshotInfo on success
///          If @device is not a valid block device, DeviceNotFound
///          If snapshot not found, GenericError
///          If the format of the image used does not support it,
///          BlockFormatFeatureNotSupported
///          If @id and @name are both not specified, GenericError
///
/// Since 1.7
/// #
#[derive(Debug)]
pub struct blockdev_snapshot_delete_internal_sync {
    pub id: String,
    pub name: String,
    pub device: String,
}
impl blockdev_snapshot_delete_internal_sync {
    pub fn new(id: String, name: String, device: String) -> blockdev_snapshot_delete_internal_sync {
        blockdev_snapshot_delete_internal_sync {
            id: id,
            name: name,
            device: device,
        }
    }
}
impl QemuCmd for blockdev_snapshot_delete_internal_sync {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-snapshot-delete-internal-sync".into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json["arguments"]["name"] = self.name.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @eject:
///
/// Ejects a device from a removable drive.
///
/// @device:  The name of the device
///
/// @force:   @optional If true, eject regardless of whether the drive is locked.
///           If not specified, the default value is false.
///
/// Returns:  Nothing on success
///           If @device is not a valid block device, DeviceNotFound
///
/// Notes:    Ejecting a device will no media results in success
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct eject {
    pub force: bool,
    pub device: String,
}
impl eject {
    pub fn new(force: bool, device: String) -> eject {
        eject {
            force: force,
            device: device,
        }
    }
}
impl QemuCmd for eject {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "eject".into();
        to_json["arguments"]["force"] = self.force.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @nbd-server-start:
///
/// Start an NBD server listening on the given host and port.  Block
/// devices can then be exported using @nbd-server-add.  The NBD
/// server will present them as named exports; for example, another
/// QEMU instance could refer to them as "nbd:HOST:PORT:exportname=NAME".
///
/// @addr: Address on which to listen.
///
/// Returns: error if the server is already running.
///
/// Since: 1.3.0
/// #
#[derive(Debug)]
pub struct nbd_server_start {
    pub addr: String,
}
impl nbd_server_start {
    pub fn new(addr: String) -> nbd_server_start {
        nbd_server_start { addr: addr }
    }
}
impl QemuCmd for nbd_server_start {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "nbd-server-start".into();
        to_json["arguments"]["addr"] = self.addr.clone().into();
        to_json.dump()
    }
}
/// #
/// @nbd-server-add:
///
/// Export a device to QEMU"s embedded NBD server.
///
/// @device: Block device to be exported
///
/// @writable: Whether clients should be able to write to the device via the
///     NBD connection (default false). #optional
///
/// Returns: error if the device is already marked for export.
///
/// Since: 1.3.0
/// #
#[derive(Debug)]
pub struct nbd_server_add {
    pub writable: bool,
    pub device: String,
}
impl nbd_server_add {
    pub fn new(writable: bool, device: String) -> nbd_server_add {
        nbd_server_add {
            writable: writable,
            device: device,
        }
    }
}
impl QemuCmd for nbd_server_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "nbd-server-add".into();
        to_json["arguments"]["writable"] = self.writable.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @nbd-server-stop:
///
/// Stop QEMU"s embedded NBD server, and unregister all devices previously
/// added via @nbd-server-add.
///
/// Since: 1.3.0
/// #
#[derive(Debug)]
pub struct nbd_server_stop {

        }
impl nbd_server_stop {
    pub fn new() -> nbd_server_stop {
        nbd_server_stop {}
    }
}
impl QemuCmd for nbd_server_stop {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "nbd-server-stop".into();

        to_json.dump()
    }
}
/// -*- Mode: Python -*-
///
/// QAPI block core definitions (vm unrelated)
/// #
/// @query-block:
///
/// Get a list of BlockInfo for all virtual block devices.
///
/// Returns: a list of @BlockInfo describing each virtual block device
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_block {

        }
impl query_block {
    pub fn new() -> query_block {
        query_block {}
    }
}
impl QemuCmd for query_block {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-block".into();

        to_json.dump()
    }
}
/// #
/// @query-blockstats:
///
/// Query the @BlockStats for all virtual block devices.
///
/// @query-nodes: #optional If true, the command will query all the block nodes
///               that have a node name, in a list which will include "parent"
///               information, but not "backing".
///               If false or omitted, the behavior is as before - query all the
///               device backends, recursively including their "parent" and
///               "backing". (Since 2.3)
///
/// Returns: A list of @BlockStats for each virtual block devices.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_blockstats {
    pub query_nodes: bool,
}
impl query_blockstats {
    pub fn new(query_nodes: bool) -> query_blockstats {
        query_blockstats { query_nodes: query_nodes }
    }
}
impl QemuCmd for query_blockstats {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-blockstats".into();
        to_json["arguments"]["query-nodes"] = self.query_nodes.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-block-jobs:
///
/// Return information about long-running block device operations.
///
/// Returns: a list of @BlockJobInfo for each active block job
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct query_block_jobs {

        }
impl query_block_jobs {
    pub fn new() -> query_block_jobs {
        query_block_jobs {}
    }
}
impl QemuCmd for query_block_jobs {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-block-jobs".into();

        to_json.dump()
    }
}
/// #
/// @block_passwd:
///
/// This command sets the password of a block device that has not been open
/// with a password and requires one.
///
/// The two cases where this can happen are a block device is created through
/// QEMU"s initial command line or a block device is changed through the legacy
/// @change interface.
///
/// In the event that the block device is created through the initial command
/// line, the VM will start in the stopped state regardless of whether "-S" is
/// used.  The intention is for a management tool to query the block devices to
/// determine which ones are encrypted, set the passwords with this command, and
/// then start the guest with the @cont command.
///
/// Either @device or @node-name must be set but not both.
///
/// @device: #optional the name of the block backend device to set the password on
///
/// @node-name: #optional graph node name to set the password on (Since 2.0)
///
/// @password: the password to use for the device
///
/// Returns: nothing on success
///          If @device is not a valid block device, DeviceNotFound
///          If @device is not encrypted, DeviceNotEncrypted
///
/// Notes:  Not all block formats support encryption and some that do are not
///         able to validate that a password is correct.  Disk corruption may
///         occur if an invalid password is specified.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct block_passwd {
    pub device: String,
    pub node_name: String,
    pub password: String,
}
impl block_passwd {
    pub fn new(device: String, node_name: String, password: String) -> block_passwd {
        block_passwd {
            device: device,
            node_name: node_name,
            password: password,
        }
    }
}
impl QemuCmd for block_passwd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block_passwd".into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json["arguments"]["node-name"] = self.node_name.clone().into();
        to_json["arguments"]["password"] = self.password.clone().into();
        to_json.dump()
    }
}
/// #
/// @block_resize
///
/// Resize a block image while a guest is running.
///
/// Either @device or @node-name must be set but not both.
///
/// @device: #optional the name of the device to get the image resized
///
/// @node-name: #optional graph node name to get the image resized (Since 2.0)
///
/// @size:  new image size in bytes
///
/// Returns: nothing on success
///          If @device is not a valid block device, DeviceNotFound
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct block_resize {
    pub device: String,
    pub node_name: String,
    pub size: f64,
}
impl block_resize {
    pub fn new(device: String, node_name: String, size: f64) -> block_resize {
        block_resize {
            device: device,
            node_name: node_name,
            size: size,
        }
    }
}
impl QemuCmd for block_resize {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block_resize".into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json["arguments"]["node-name"] = self.node_name.clone().into();
        to_json["arguments"]["size"] = self.size.clone().into();
        to_json.dump()
    }
}
/// #
/// @blockdev-snapshot-sync
///
/// Generates a synchronous snapshot of a block device.
///
/// For the arguments, see the documentation of BlockdevSnapshot.
///
/// Returns: nothing on success
///          If @device is not a valid block device, DeviceNotFound
///
/// Since 0.14.0
/// #
#[derive(Debug)]
pub struct blockdev_snapshot_sync {

        }
impl blockdev_snapshot_sync {
    pub fn new() -> blockdev_snapshot_sync {
        blockdev_snapshot_sync {}
    }
}
impl QemuCmd for blockdev_snapshot_sync {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-snapshot-sync".into();

        to_json.dump()
    }
}
/// #
/// @change-backing-file
///
/// Change the backing file in the image file metadata.  This does not
/// cause QEMU to reopen the image file to reparse the backing filename
/// (it may, however, perform a reopen to change permissions from
/// r/o -> r/w -> r/o, if needed). The new backing file string is written
/// into the image file metadata, and the QEMU internal strings are
/// updated.
///
/// @image-node-name: The name of the block driver state node of the
///                   image to modify.
///
/// @device:          The name of the device that owns image-node-name.
///
/// @backing-file:    The string to write as the backing file.  This
///                   string is not validated, so care should be taken
///                   when specifying the string or the image chain may
///                   not be able to be reopened again.
///
/// Since: 2.1
/// #
#[derive(Debug)]
pub struct change_backing_file {
    pub backing_file: String,
    pub device: String,
    pub image_node_name: String,
}
impl change_backing_file {
    pub fn new(backing_file: String,
               device: String,
               image_node_name: String)
               -> change_backing_file {
        change_backing_file {
            backing_file: backing_file,
            device: device,
            image_node_name: image_node_name,
        }
    }
}
impl QemuCmd for change_backing_file {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "change-backing-file".into();
        to_json["arguments"]["backing-file"] = self.backing_file.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json["arguments"]["image-node-name"] = self.image_node_name.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-commit
///
/// Live commit of data from overlay image nodes into backing nodes - i.e.,
/// writes data between "top" and "base" into "base".
///
/// @device:  the name of the device
///
/// @base:   #optional The file name of the backing image to write data into.
///                    If not specified, this is the deepest backing image
///
/// @top:    #optional The file name of the backing image within the image chain,
///                    which contains the topmost data to be committed down. If
///                    not specified, this is the active layer.
///
/// @backing-file:  #optional The backing file string to write into the overlay
///                           image of "top".  If "top" is the active layer,
///                           specifying a backing file string is an error. This
///                           filename is not validated.
///
///                           If a pathname string is such that it cannot be
///                           resolved by QEMU, that means that subsequent QMP or
///                           HMP commands must use node-names for the image in
///                           question, as filename lookup methods will fail.
///
///                           If not specified, QEMU will automatically determine
///                           the backing file string to use, or error out if
///                           there is no obvious choice. Care should be taken
///                           when specifying the string, to specify a valid
///                           filename or protocol.
///                           (Since 2.1)
///
///                    If top == base, that is an error.
///                    If top == active, the job will not be completed by itself,
///                    user needs to complete the job with the block-job-complete
///                    command after getting the ready event. (Since 2.0)
///
///                    If the base image is smaller than top, then the base image
///                    will be resized to be the same size as top.  If top is
///                    smaller than the base image, the base will not be
///                    truncated.  If you want the base image size to match the
///                    size of the smaller top, you can safely truncate it
///                    yourself once the commit operation successfully completes.
///
/// @speed:  #optional the maximum speed, in bytes per second
///
/// Returns: Nothing on success
///          If commit or stream is already active on this device, DeviceInUse
///          If @device does not exist, DeviceNotFound
///          If image commit is not supported by this device, NotSupported
///          If @base or @top is invalid, a generic error is returned
///          If @speed is invalid, InvalidParameter
///
/// Since: 1.3
///
/// #
#[derive(Debug)]
pub struct block_commit {
    pub backing_file: String,
    pub base: String,
    pub speed: f64,
    pub top: String,
    pub device: String,
}
impl block_commit {
    pub fn new(backing_file: String,
               base: String,
               speed: f64,
               top: String,
               device: String)
               -> block_commit {
        block_commit {
            backing_file: backing_file,
            base: base,
            speed: speed,
            top: top,
            device: device,
        }
    }
}
impl QemuCmd for block_commit {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-commit".into();
        to_json["arguments"]["backing-file"] = self.backing_file.clone().into();
        to_json["arguments"]["base"] = self.base.clone().into();
        to_json["arguments"]["speed"] = self.speed.clone().into();
        to_json["arguments"]["top"] = self.top.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @drive-backup
///
/// Start a point-in-time copy of a block device to a new destination.  The
/// status of ongoing drive-backup operations can be checked with
/// query-block-jobs where the BlockJobInfo.type field has the value "backup".
/// The operation can be stopped before it has completed using the
/// block-job-cancel command.
///
/// For the arguments, see the documentation of DriveBackup.
///
/// Returns: nothing on success
///          If @device is not a valid block device, DeviceNotFound
///
/// Since 1.6
/// #
#[derive(Debug)]
pub struct drive_backup {

        }
impl drive_backup {
    pub fn new() -> drive_backup {
        drive_backup {}
    }
}
impl QemuCmd for drive_backup {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "drive-backup".into();

        to_json.dump()
    }
}
/// #
/// @blockdev-backup
///
/// Start a point-in-time copy of a block device to a new destination.  The
/// status of ongoing blockdev-backup operations can be checked with
/// query-block-jobs where the BlockJobInfo.type field has the value "backup".
/// The operation can be stopped before it has completed using the
/// block-job-cancel command.
///
/// For the arguments, see the documentation of BlockdevBackup.
///
/// Since 2.3
/// #
#[derive(Debug)]
pub struct blockdev_backup {

        }
impl blockdev_backup {
    pub fn new() -> blockdev_backup {
        blockdev_backup {}
    }
}
impl QemuCmd for blockdev_backup {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-backup".into();

        to_json.dump()
    }
}
/// #
/// @query-named-block-nodes
///
/// Get the named block driver list
///
/// Returns: the list of BlockDeviceInfo
///
/// Since 2.0
/// #
#[derive(Debug)]
pub struct query_named_block_nodes {

        }
impl query_named_block_nodes {
    pub fn new() -> query_named_block_nodes {
        query_named_block_nodes {}
    }
}
impl QemuCmd for query_named_block_nodes {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-named-block-nodes".into();

        to_json.dump()
    }
}
/// #
/// @drive-mirror
///
/// Start mirroring a block device"s writes to a new destination.
///
/// @device:  the name of the device whose writes should be mirrored.
///
/// @target: the target of the new image. If the file exists, or if it
///          is a device, the existing file/device will be used as the new
///          destination.  If it does not exist, a new file will be created.
///
/// @format: #optional the format of the new destination, default is to
///          probe if @mode is "existing", else the format of the source
///
/// @node-name: #optional the new block driver state node name in the graph
///             (Since 2.1)
///
/// @replaces: #optional with sync=full graph node name to be replaced by the new
///            image when a whole image copy is done. This can be used to repair
///            broken Quorum files. (Since 2.1)
///
/// @mode: #optional whether and how QEMU should create a new image, default is
///        "absolute-paths".
///
/// @speed:  #optional the maximum speed, in bytes per second
///
/// @sync: what parts of the disk image should be copied to the destination
///        (all the disk, only the sectors allocated in the topmost image, or
///        only new I/O).
///
/// @granularity: #optional granularity of the dirty bitmap, default is 64K
///               if the image format doesn"t have clusters, 4K if the clusters
///               are smaller than that, else the cluster size.  Must be a
///               power of 2 between 512 and 64M (since 1.4).
///
/// @buf-size: #optional maximum amount of data in flight from source to
///            target (since 1.4).
///
/// @on-source-error: #optional the action to take on an error on the source,
///                   default "report".  "stop" and "enospc" can only be used
///                   if the block device supports io-status (see BlockInfo).
///
/// @on-target-error: #optional the action to take on an error on the target,
///                   default "report" (no limitations, since this applies to
///                   a different block device than @device).
///
/// Returns: nothing on success
///          If @device is not a valid block device, DeviceNotFound
///
/// Since 1.3
/// #
#[derive(Debug)]
pub struct drive_mirror {
    pub buf_size: f64,
    pub format: String,
    pub granularity: u32,
    pub mode: String,
    pub node_name: String,
    pub on_source_error: String,
    pub on_target_error: String,
    pub replaces: String,
    pub speed: f64,
    pub device: String,
    pub sync: String,
    pub target: String,
}
impl drive_mirror {
    pub fn new(buf_size: f64,
               format: String,
               granularity: u32,
               mode: String,
               node_name: String,
               on_source_error: String,
               on_target_error: String,
               replaces: String,
               speed: f64,
               device: String,
               sync: String,
               target: String)
               -> drive_mirror {
        drive_mirror {
            buf_size: buf_size,
            format: format,
            granularity: granularity,
            mode: mode,
            node_name: node_name,
            on_source_error: on_source_error,
            on_target_error: on_target_error,
            replaces: replaces,
            speed: speed,
            device: device,
            sync: sync,
            target: target,
        }
    }
}
impl QemuCmd for drive_mirror {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "drive-mirror".into();
        to_json["arguments"]["buf-size"] = self.buf_size.clone().into();
        to_json["arguments"]["format"] = self.format.clone().into();
        to_json["arguments"]["granularity"] = self.granularity.clone().into();
        to_json["arguments"]["mode"] = self.mode.clone().into();
        to_json["arguments"]["node-name"] = self.node_name.clone().into();
        to_json["arguments"]["on-source-error"] = self.on_source_error.clone().into();
        to_json["arguments"]["on-target-error"] = self.on_target_error.clone().into();
        to_json["arguments"]["replaces"] = self.replaces.clone().into();
        to_json["arguments"]["speed"] = self.speed.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json["arguments"]["sync"] = self.sync.clone().into();
        to_json["arguments"]["target"] = self.target.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-dirty-bitmap-add
///
/// Create a dirty bitmap with a name on the node
///
/// Returns: nothing on success
///          If @node is not a valid block device or node, DeviceNotFound
///          If @name is already taken, GenericError with an explanation
///
/// Since 2.4
/// #
#[derive(Debug)]
pub struct block_dirty_bitmap_add {

        }
impl block_dirty_bitmap_add {
    pub fn new() -> block_dirty_bitmap_add {
        block_dirty_bitmap_add {}
    }
}
impl QemuCmd for block_dirty_bitmap_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-dirty-bitmap-add".into();

        to_json.dump()
    }
}
/// #
/// @block-dirty-bitmap-remove
///
/// Remove a dirty bitmap on the node
///
/// Returns: nothing on success
///          If @node is not a valid block device or node, DeviceNotFound
///          If @name is not found, GenericError with an explanation
///          if @name is frozen by an operation, GenericError
///
/// Since 2.4
/// #
#[derive(Debug)]
pub struct block_dirty_bitmap_remove {

        }
impl block_dirty_bitmap_remove {
    pub fn new() -> block_dirty_bitmap_remove {
        block_dirty_bitmap_remove {}
    }
}
impl QemuCmd for block_dirty_bitmap_remove {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-dirty-bitmap-remove".into();

        to_json.dump()
    }
}
/// #
/// @block-dirty-bitmap-clear
///
/// Clear (reset) a dirty bitmap on the device
///
/// Returns: nothing on success
///          If @node is not a valid block device, DeviceNotFound
///          If @name is not found, GenericError with an explanation
///
/// Since 2.4
/// #
#[derive(Debug)]
pub struct block_dirty_bitmap_clear {

        }
impl block_dirty_bitmap_clear {
    pub fn new() -> block_dirty_bitmap_clear {
        block_dirty_bitmap_clear {}
    }
}
impl QemuCmd for block_dirty_bitmap_clear {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-dirty-bitmap-clear".into();

        to_json.dump()
    }
}
/// #
/// @block_set_io_throttle:
///
/// Change I/O throttle limits for a block drive.
///
/// @device: The name of the device
///
/// @bps: total throughput limit in bytes per second
///
/// @bps_rd: read throughput limit in bytes per second
///
/// @bps_wr: write throughput limit in bytes per second
///
/// @iops: total I/O operations per second
///
/// @ops_rd: read I/O operations per second
///
/// @iops_wr: write I/O operations per second
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
/// Returns: Nothing on success
///          If @device is not a valid block device, DeviceNotFound
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct block_set_io_throttle {
    pub bps_max: f64,
    pub bps_rd_max: f64,
    pub bps_wr_max: f64,
    pub iops_max: f64,
    pub iops_rd_max: f64,
    pub iops_size: f64,
    pub iops_wr_max: f64,
    pub bps: f64,
    pub bps_rd: f64,
    pub bps_wr: f64,
    pub device: String,
    pub iops: f64,
    pub iops_rd: f64,
    pub iops_wr: f64,
}
impl block_set_io_throttle {
    pub fn new(bps_max: f64,
               bps_rd_max: f64,
               bps_wr_max: f64,
               iops_max: f64,
               iops_rd_max: f64,
               iops_size: f64,
               iops_wr_max: f64,
               bps: f64,
               bps_rd: f64,
               bps_wr: f64,
               device: String,
               iops: f64,
               iops_rd: f64,
               iops_wr: f64)
               -> block_set_io_throttle {
        block_set_io_throttle {
            bps_max: bps_max,
            bps_rd_max: bps_rd_max,
            bps_wr_max: bps_wr_max,
            iops_max: iops_max,
            iops_rd_max: iops_rd_max,
            iops_size: iops_size,
            iops_wr_max: iops_wr_max,
            bps: bps,
            bps_rd: bps_rd,
            bps_wr: bps_wr,
            device: device,
            iops: iops,
            iops_rd: iops_rd,
            iops_wr: iops_wr,
        }
    }
}
impl QemuCmd for block_set_io_throttle {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block_set_io_throttle".into();
        to_json["arguments"]["bps_max"] = self.bps_max.clone().into();
        to_json["arguments"]["bps_rd_max"] = self.bps_rd_max.clone().into();
        to_json["arguments"]["bps_wr_max"] = self.bps_wr_max.clone().into();
        to_json["arguments"]["iops_max"] = self.iops_max.clone().into();
        to_json["arguments"]["iops_rd_max"] = self.iops_rd_max.clone().into();
        to_json["arguments"]["iops_size"] = self.iops_size.clone().into();
        to_json["arguments"]["iops_wr_max"] = self.iops_wr_max.clone().into();
        to_json["arguments"]["bps"] = self.bps.clone().into();
        to_json["arguments"]["bps_rd"] = self.bps_rd.clone().into();
        to_json["arguments"]["bps_wr"] = self.bps_wr.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json["arguments"]["iops"] = self.iops.clone().into();
        to_json["arguments"]["iops_rd"] = self.iops_rd.clone().into();
        to_json["arguments"]["iops_wr"] = self.iops_wr.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-stream:
///
/// Copy data from a backing file into a block device.
///
/// The block streaming operation is performed in the background until the entire
/// backing file has been copied.  This command returns immediately once streaming
/// has started.  The status of ongoing block streaming operations can be checked
/// with query-block-jobs.  The operation can be stopped before it has completed
/// using the block-job-cancel command.
///
/// If a base file is specified then sectors are not copied from that base file and
/// its backing chain.  When streaming completes the image file will have the base
/// file as its backing file.  This can be used to stream a subset of the backing
/// file chain instead of flattening the entire image.
///
/// On successful completion the image file is updated to drop the backing file
/// and the BLOCK_JOB_COMPLETED event is emitted.
///
/// @device: the device name
///
/// @base:   #optional the common backing file name
///
/// @backing-file: #optional The backing file string to write into the active
///                          layer. This filename is not validated.
///
///                          If a pathname string is such that it cannot be
///                          resolved by QEMU, that means that subsequent QMP or
///                          HMP commands must use node-names for the image in
///                          question, as filename lookup methods will fail.
///
///                          If not specified, QEMU will automatically determine
///                          the backing file string to use, or error out if there
///                          is no obvious choice.  Care should be taken when
///                          specifying the string, to specify a valid filename or
///                          protocol.
///                          (Since 2.1)
///
/// @speed:  #optional the maximum speed, in bytes per second
///
/// @on-error: #optional the action to take on an error (default report).
///            "stop" and "enospc" can only be used if the block device
///            supports io-status (see BlockInfo).  Since 1.3.
///
/// Returns: Nothing on success
///          If @device does not exist, DeviceNotFound
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct block_stream {
    pub backing_file: String,
    pub base: String,
    pub on_error: String,
    pub speed: f64,
    pub device: String,
}
impl block_stream {
    pub fn new(backing_file: String,
               base: String,
               on_error: String,
               speed: f64,
               device: String)
               -> block_stream {
        block_stream {
            backing_file: backing_file,
            base: base,
            on_error: on_error,
            speed: speed,
            device: device,
        }
    }
}
impl QemuCmd for block_stream {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-stream".into();
        to_json["arguments"]["backing-file"] = self.backing_file.clone().into();
        to_json["arguments"]["base"] = self.base.clone().into();
        to_json["arguments"]["on-error"] = self.on_error.clone().into();
        to_json["arguments"]["speed"] = self.speed.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-job-set-speed:
///
/// Set maximum speed for a background block operation.
///
/// This command can only be issued when there is an active block job.
///
/// Throttling can be disabled by setting the speed to 0.
///
/// @device: the device name
///
/// @speed:  the maximum speed, in bytes per second, or 0 for unlimited.
///          Defaults to 0.
///
/// Returns: Nothing on success
///          If no background operation is active on this device, DeviceNotActive
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct block_job_set_speed {
    pub device: String,
    pub speed: f64,
}
impl block_job_set_speed {
    pub fn new(device: String, speed: f64) -> block_job_set_speed {
        block_job_set_speed {
            device: device,
            speed: speed,
        }
    }
}
impl QemuCmd for block_job_set_speed {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-set-speed".into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json["arguments"]["speed"] = self.speed.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-job-cancel:
///
/// Stop an active background block operation.
///
/// This command returns immediately after marking the active background block
/// operation for cancellation.  It is an error to call this command if no
/// operation is in progress.
///
/// The operation will cancel as soon as possible and then emit the
/// BLOCK_JOB_CANCELLED event.  Before that happens the job is still visible when
/// enumerated using query-block-jobs.
///
/// For streaming, the image file retains its backing file unless the streaming
/// operation happens to complete just as it is being cancelled.  A new streaming
/// operation can be started at a later time to finish copying all data from the
/// backing file.
///
/// @device: the device name
///
/// @force: #optional whether to allow cancellation of a paused job (default
///         false).  Since 1.3.
///
/// Returns: Nothing on success
///          If no background operation is active on this device, DeviceNotActive
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct block_job_cancel {
    pub force: bool,
    pub device: String,
}
impl block_job_cancel {
    pub fn new(force: bool, device: String) -> block_job_cancel {
        block_job_cancel {
            force: force,
            device: device,
        }
    }
}
impl QemuCmd for block_job_cancel {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-cancel".into();
        to_json["arguments"]["force"] = self.force.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-job-pause:
///
/// Pause an active background block operation.
///
/// This command returns immediately after marking the active background block
/// operation for pausing.  It is an error to call this command if no
/// operation is in progress.  Pausing an already paused job has no cumulative
/// effect; a single block-job-resume command will resume the job.
///
/// The operation will pause as soon as possible.  No event is emitted when
/// the operation is actually paused.  Cancelling a paused job automatically
/// resumes it.
///
/// @device: the device name
///
/// Returns: Nothing on success
///          If no background operation is active on this device, DeviceNotActive
///
/// Since: 1.3
/// #
#[derive(Debug)]
pub struct block_job_pause {
    pub device: String,
}
impl block_job_pause {
    pub fn new(device: String) -> block_job_pause {
        block_job_pause { device: device }
    }
}
impl QemuCmd for block_job_pause {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-pause".into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-job-resume:
///
/// Resume an active background block operation.
///
/// This command returns immediately after resuming a paused background block
/// operation.  It is an error to call this command if no operation is in
/// progress.  Resuming an already running job is not an error.
///
/// This command also clears the error status of the job.
///
/// @device: the device name
///
/// Returns: Nothing on success
///          If no background operation is active on this device, DeviceNotActive
///
/// Since: 1.3
/// #
#[derive(Debug)]
pub struct block_job_resume {
    pub device: String,
}
impl block_job_resume {
    pub fn new(device: String) -> block_job_resume {
        block_job_resume { device: device }
    }
}
impl QemuCmd for block_job_resume {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-resume".into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-job-complete:
///
/// Manually trigger completion of an active background block operation.  This
/// is supported for drive mirroring, where it also switches the device to
/// write to the target path only.  The ability to complete is signaled with
/// a BLOCK_JOB_READY event.
///
/// This command completes an active background block operation synchronously.
/// The ordering of this command"s return with the BLOCK_JOB_COMPLETED event
/// is not defined.  Note that if an I/O error occurs during the processing of
/// this command: 1) the command itself will fail; 2) the error will be processed
/// according to the rerror/werror arguments that were specified when starting
/// the operation.
///
/// A cancelled or paused job cannot be completed.
///
/// @device: the device name
///
/// Returns: Nothing on success
///          If no background operation is active on this device, DeviceNotActive
///
/// Since: 1.3
/// #
#[derive(Debug)]
pub struct block_job_complete {
    pub device: String,
}
impl block_job_complete {
    pub fn new(device: String) -> block_job_complete {
        block_job_complete { device: device }
    }
}
impl QemuCmd for block_job_complete {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-complete".into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @blockdev-add:
///
/// Creates a new block device.
///
/// This command is still a work in progress.  It doesn"t support all
/// block drivers, it lacks a matching blockdev-del, and more.  Stay
/// away from it unless you want to help with its development.
///
/// @options: block device options for the new device
///
/// Since: 1.7
/// #
#[derive(Debug)]
pub struct blockdev_add {
    pub options: String,
}
impl blockdev_add {
    pub fn new(options: String) -> blockdev_add {
        blockdev_add { options: options }
    }
}
impl QemuCmd for blockdev_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-add".into();
        to_json["arguments"]["options"] = self.options.clone().into();
        to_json.dump()
    }
}
/// #
/// @block-set-write-threshold
///
/// Change the write threshold for a block drive. An event will be delivered
/// if a write to this block drive crosses the configured threshold.
/// This is useful to transparently resize thin-provisioned drives without
/// the guest OS noticing.
///
/// @node-name: graph node name on which the threshold must be set.
///
/// @write-threshold: configured threshold for the block device, bytes.
///                   Use 0 to disable the threshold.
///
/// Since: 2.3
/// #
#[derive(Debug)]
pub struct block_set_write_threshold {
    pub node_name: String,
    pub write_threshold: u64,
}
impl block_set_write_threshold {
    pub fn new(node_name: String, write_threshold: u64) -> block_set_write_threshold {
        block_set_write_threshold {
            node_name: node_name,
            write_threshold: write_threshold,
        }
    }
}
impl QemuCmd for block_set_write_threshold {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-set-write-threshold".into();
        to_json["arguments"]["node-name"] = self.node_name.clone().into();
        to_json["arguments"]["write-threshold"] = self.write_threshold.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-version:
///
/// Returns the current version of QEMU.
///
/// Returns:  A @VersionInfo object describing the current version of QEMU.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_version {

        }
impl query_version {
    pub fn new() -> query_version {
        query_version {}
    }
}
impl QemuCmd for query_version {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-version".into();

        to_json.dump()
    }
}
/// #
/// @query-commands:
///
/// Return a list of supported QMP commands by this server
///
/// Returns: A list of @CommandInfo for all supported commands
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_commands {

        }
impl query_commands {
    pub fn new() -> query_commands {
        query_commands {}
    }
}
impl QemuCmd for query_commands {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-commands".into();

        to_json.dump()
    }
}
/// #
/// @trace-event-get-state:
///
/// Query the state of events.
///
/// @name: Event name pattern (case-sensitive glob).
///
/// Returns: a list of @TraceEventInfo for the matching events
///
/// Since 2.2
/// #
#[derive(Debug)]
pub struct trace_event_get_state {
    pub name: String,
}
impl trace_event_get_state {
    pub fn new(name: String) -> trace_event_get_state {
        trace_event_get_state { name: name }
    }
}
impl QemuCmd for trace_event_get_state {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "trace-event-get-state".into();
        to_json["arguments"]["name"] = self.name.clone().into();
        to_json.dump()
    }
}
/// #
/// @trace-event-set-state:
///
/// Set the dynamic tracing state of events.
///
/// @name: Event name pattern (case-sensitive glob).
/// @enable: Whether to enable tracing.
/// @ignore-unavailable: #optional Do not match unavailable events with @name.
///
/// Since 2.2
/// #
#[derive(Debug)]
pub struct trace_event_set_state {
    pub ignore_unavailable: bool,
    pub enable: bool,
    pub name: String,
}
impl trace_event_set_state {
    pub fn new(ignore_unavailable: bool, enable: bool, name: String) -> trace_event_set_state {
        trace_event_set_state {
            ignore_unavailable: ignore_unavailable,
            enable: enable,
            name: name,
        }
    }
}
impl QemuCmd for trace_event_set_state {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "trace-event-set-state".into();
        to_json["arguments"]["ignore-unavailable"] = self.ignore_unavailable.clone().into();
        to_json["arguments"]["enable"] = self.enable.clone().into();
        to_json["arguments"]["name"] = self.name.clone().into();
        to_json.dump()
    }
}
/// -*- Mode: Python -*-
///
/// QAPI Schema
/// QAPI block definitions
/// QAPI event definitions
/// Tracing commands
/// @add_client
///
/// Allow client connections for VNC, Spice and socket based
/// character devices to be passed in to QEMU via SCM_RIGHTS.
///
/// @protocol: protocol name. Valid names are "vnc", "spice" or the
///            name of a character device (eg. from -chardev id=XXXX)
///
/// @fdname: file descriptor name previously passed via "getfd" command
///
/// @skipauth: #optional whether to skip authentication. Only applies
///            to "vnc" and "spice" protocols
///
/// @tls: #optional whether to perform TLS. Only applies to the "spice"
///       protocol
///
/// Returns: nothing on success.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct add_client {
    pub skipauth: bool,
    pub tls: bool,
    pub fdname: String,
    pub protocol: String,
}
impl add_client {
    pub fn new(skipauth: bool, tls: bool, fdname: String, protocol: String) -> add_client {
        add_client {
            skipauth: skipauth,
            tls: tls,
            fdname: fdname,
            protocol: protocol,
        }
    }
}
impl QemuCmd for add_client {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "add_client".into();
        to_json["arguments"]["skipauth"] = self.skipauth.clone().into();
        to_json["arguments"]["tls"] = self.tls.clone().into();
        to_json["arguments"]["fdname"] = self.fdname.clone().into();
        to_json["arguments"]["protocol"] = self.protocol.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-name:
///
/// Return the name information of a guest.
///
/// Returns: @NameInfo of the guest
///
/// Since 0.14.0
/// #
#[derive(Debug)]
pub struct query_name {

        }
impl query_name {
    pub fn new() -> query_name {
        query_name {}
    }
}
impl QemuCmd for query_name {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-name".into();

        to_json.dump()
    }
}
/// #
/// @query-kvm:
///
/// Returns information about KVM acceleration
///
/// Returns: @KvmInfo
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_kvm {

        }
impl query_kvm {
    pub fn new() -> query_kvm {
        query_kvm {}
    }
}
impl QemuCmd for query_kvm {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-kvm".into();

        to_json.dump()
    }
}
/// #
/// @query-status:
///
/// Query the run status of all VCPUs
///
/// Returns: @StatusInfo reflecting all VCPUs
///
/// Since:  0.14.0
/// #
#[derive(Debug)]
pub struct query_status {

        }
impl query_status {
    pub fn new() -> query_status {
        query_status {}
    }
}
impl QemuCmd for query_status {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-status".into();

        to_json.dump()
    }
}
/// #
/// @query-uuid:
///
/// Query the guest UUID information.
///
/// Returns: The @UuidInfo for the guest
///
/// Since 0.14.0
/// #
#[derive(Debug)]
pub struct query_uuid {

        }
impl query_uuid {
    pub fn new() -> query_uuid {
        query_uuid {}
    }
}
impl QemuCmd for query_uuid {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-uuid".into();

        to_json.dump()
    }
}
/// #
/// @query-chardev:
///
/// Returns information about current character devices.
///
/// Returns: a list of @ChardevInfo
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_chardev {

        }
impl query_chardev {
    pub fn new() -> query_chardev {
        query_chardev {}
    }
}
impl QemuCmd for query_chardev {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-chardev".into();

        to_json.dump()
    }
}
/// #
/// @query-chardev-backends:
///
/// Returns information about character device backends.
///
/// Returns: a list of @ChardevBackendInfo
///
/// Since: 2.0
/// #
#[derive(Debug)]
pub struct query_chardev_backends {

        }
impl query_chardev_backends {
    pub fn new() -> query_chardev_backends {
        query_chardev_backends {}
    }
}
impl QemuCmd for query_chardev_backends {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-chardev-backends".into();

        to_json.dump()
    }
}
/// #
/// @ringbuf-write:
///
/// Write to a ring buffer character device.
///
/// @device: the ring buffer character device name
///
/// @data: data to write
///
/// @format: #optional data encoding (default "utf8").
///          - base64: data must be base64 encoded text.  Its binary
///            decoding gets written.
///            Bug: invalid base64 is currently not rejected.
///            Whitespace *is* invalid.
///          - utf8: data"s UTF-8 encoding is written
///          - data itself is always Unicode regardless of format, like
///            any other string.
///
/// Returns: Nothing on success
///
/// Since: 1.4
/// #
#[derive(Debug)]
pub struct ringbuf_write {
    pub format: String,
    pub data: String,
    pub device: String,
}
impl ringbuf_write {
    pub fn new(format: String, data: String, device: String) -> ringbuf_write {
        ringbuf_write {
            format: format,
            data: data,
            device: device,
        }
    }
}
impl QemuCmd for ringbuf_write {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "ringbuf-write".into();
        to_json["arguments"]["format"] = self.format.clone().into();
        to_json["arguments"]["data"] = self.data.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }
}
/// #
/// @ringbuf-read:
///
/// Read from a ring buffer character device.
///
/// @device: the ring buffer character device name
///
/// @size: how many bytes to read at most
///
/// @format: #optional data encoding (default "utf8").
///          - base64: the data read is returned in base64 encoding.
///          - utf8: the data read is interpreted as UTF-8.
///            Bug: can screw up when the buffer contains invalid UTF-8
///            sequences, NUL characters, after the ring buffer lost
///            data, and when reading stops because the size limit is
///            reached.
///          - The return value is always Unicode regardless of format,
///            like any other string.
///
/// Returns: data read from the device
///
/// Since: 1.4
/// #
#[derive(Debug)]
pub struct ringbuf_read {
    pub format: String,
    pub device: String,
    pub size: f64,
}
impl ringbuf_read {
    pub fn new(format: String, device: String, size: f64) -> ringbuf_read {
        ringbuf_read {
            format: format,
            device: device,
            size: size,
        }
    }
}
impl QemuCmd for ringbuf_read {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "ringbuf-read".into();
        to_json["arguments"]["format"] = self.format.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json["arguments"]["size"] = self.size.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-events:
///
/// Return a list of supported QMP events by this server
///
/// Returns: A list of @EventInfo for all supported events
///
/// Since: 1.2.0
/// #
#[derive(Debug)]
pub struct query_events {

        }
impl query_events {
    pub fn new() -> query_events {
        query_events {}
    }
}
impl QemuCmd for query_events {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-events".into();

        to_json.dump()
    }
}
/// #
/// @query-migrate
///
/// Returns information about current migration process.
///
/// Returns: @MigrationInfo
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_migrate {

        }
impl query_migrate {
    pub fn new() -> query_migrate {
        query_migrate {}
    }
}
impl QemuCmd for query_migrate {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-migrate".into();

        to_json.dump()
    }
}
/// #
/// @migrate-set-capabilities
///
/// Enable/Disable the following migration capabilities (like xbzrle)
///
/// @capabilities: json array of capability modifications to make
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct migrate_set_capabilities {
    pub capabilities: Vec<String>,
}
impl migrate_set_capabilities {
    pub fn new(capabilities: Vec<String>) -> migrate_set_capabilities {
        migrate_set_capabilities { capabilities: capabilities }
    }
}
impl QemuCmd for migrate_set_capabilities {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate-set-capabilities".into();
        to_json["arguments"]["capabilities"] = self.capabilities.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-migrate-capabilities
///
/// Returns information about the current migration capabilities status
///
/// Returns: @MigrationCapabilitiesStatus
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct query_migrate_capabilities {

        }
impl query_migrate_capabilities {
    pub fn new() -> query_migrate_capabilities {
        query_migrate_capabilities {}
    }
}
impl QemuCmd for query_migrate_capabilities {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-migrate-capabilities".into();

        to_json.dump()
    }
}
///
/// @migrate-set-parameters
///
/// Set the following migration parameters
///
/// @compress-level: compression level
///
/// @compress-threads: compression thread count
///
/// @decompress-threads: decompression thread count
///
/// Since: 2.4
/// #
#[derive(Debug)]
pub struct migrate_set_parameters {
    pub compress_level: f64,
    pub compress_threads: f64,
    pub decompress_threads: f64,
}
impl migrate_set_parameters {
    pub fn new(compress_level: f64,
               compress_threads: f64,
               decompress_threads: f64)
               -> migrate_set_parameters {
        migrate_set_parameters {
            compress_level: compress_level,
            compress_threads: compress_threads,
            decompress_threads: decompress_threads,
        }
    }
}
impl QemuCmd for migrate_set_parameters {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate-set-parameters".into();
        to_json["arguments"]["compress-level"] = self.compress_level.clone().into();
        to_json["arguments"]["compress-threads"] = self.compress_threads.clone().into();
        to_json["arguments"]["decompress-threads"] = self.decompress_threads.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-migrate-parameters
///
/// Returns information about the current migration parameters
///
/// Returns: @MigrationParameters
///
/// Since: 2.4
/// #
#[derive(Debug)]
pub struct query_migrate_parameters {

        }
impl query_migrate_parameters {
    pub fn new() -> query_migrate_parameters {
        query_migrate_parameters {}
    }
}
impl QemuCmd for query_migrate_parameters {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-migrate-parameters".into();

        to_json.dump()
    }
}
/// #
/// @query-mice:
///
/// Returns information about each active mouse device
///
/// Returns: a list of @MouseInfo for each device
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_mice {

        }
impl query_mice {
    pub fn new() -> query_mice {
        query_mice {}
    }
}
impl QemuCmd for query_mice {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-mice".into();

        to_json.dump()
    }
}
/// #
/// @query-cpus:
///
/// Returns a list of information about each virtual CPU.
///
/// Returns: a list of @CpuInfo for each virtual CPU
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_cpus {

        }
impl query_cpus {
    pub fn new() -> query_cpus {
        query_cpus {}
    }
}
impl QemuCmd for query_cpus {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-cpus".into();

        to_json.dump()
    }
}
/// #
/// @query-iothreads:
///
/// Returns a list of information about each iothread.
///
/// Note this list excludes the QEMU main loop thread, which is not declared
/// using the -object iothread command-line option.  It is always the main thread
/// of the process.
///
/// Returns: a list of @IOThreadInfo for each iothread
///
/// Since: 2.0
/// #
#[derive(Debug)]
pub struct query_iothreads {

        }
impl query_iothreads {
    pub fn new() -> query_iothreads {
        query_iothreads {}
    }
}
impl QemuCmd for query_iothreads {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-iothreads".into();

        to_json.dump()
    }
}
/// #
/// @query-vnc:
///
/// Returns information about the current VNC server
///
/// Returns: @VncInfo
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_vnc {

        }
impl query_vnc {
    pub fn new() -> query_vnc {
        query_vnc {}
    }
}
impl QemuCmd for query_vnc {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-vnc".into();

        to_json.dump()
    }
}
/// #
/// @query-vnc-servers:
///
/// Returns a list of vnc servers.  The list can be empty.
///
/// Returns: a list of @VncInfo2
///
/// Since: 2.3
/// #
#[derive(Debug)]
pub struct query_vnc_servers {

        }
impl query_vnc_servers {
    pub fn new() -> query_vnc_servers {
        query_vnc_servers {}
    }
}
impl QemuCmd for query_vnc_servers {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-vnc-servers".into();

        to_json.dump()
    }
}
/// #
/// @query-spice
///
/// Returns information about the current SPICE server
///
/// Returns: @SpiceInfo
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_spice {

        }
impl query_spice {
    pub fn new() -> query_spice {
        query_spice {}
    }
}
impl QemuCmd for query_spice {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-spice".into();

        to_json.dump()
    }
}
/// #
/// @query-balloon:
///
/// Return information about the balloon device.
///
/// Returns: @BalloonInfo on success
///          If the balloon driver is enabled but not functional because the KVM
///          kernel module cannot support it, KvmMissingCap
///          If no balloon device is present, DeviceNotActive
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_balloon {

        }
impl query_balloon {
    pub fn new() -> query_balloon {
        query_balloon {}
    }
}
impl QemuCmd for query_balloon {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-balloon".into();

        to_json.dump()
    }
}
/// #
/// @query-pci:
///
/// Return information about the PCI bus topology of the guest.
///
/// Returns: a list of @PciInfo for each PCI bus
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct query_pci {

        }
impl query_pci {
    pub fn new() -> query_pci {
        query_pci {}
    }
}
impl QemuCmd for query_pci {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-pci".into();

        to_json.dump()
    }
}
/// #
/// @quit:
///
/// This command will cause the QEMU process to exit gracefully.  While every
/// attempt is made to send the QMP response before terminating, this is not
/// guaranteed.  When using this interface, a premature EOF would not be
/// unexpected.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct quit {

        }
impl quit {
    pub fn new() -> quit {
        quit {}
    }
}
impl QemuCmd for quit {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "quit".into();

        to_json.dump()
    }
}
/// #
/// @stop:
///
/// Stop all guest VCPU execution.
///
/// Since:  0.14.0
///
/// Notes:  This function will succeed even if the guest is already in the stopped
///         state.  In "inmigrate" state, it will ensure that the guest
///         remains paused once migration finishes, as if the -S option was
///         passed on the command line.
/// #
#[derive(Debug)]
pub struct stop {

        }
impl stop {
    pub fn new() -> stop {
        stop {}
    }
}
impl QemuCmd for stop {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "stop".into();

        to_json.dump()
    }
}
/// #
/// @system_reset:
///
/// Performs a hard reset of a guest.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct system_reset {

        }
impl system_reset {
    pub fn new() -> system_reset {
        system_reset {}
    }
}
impl QemuCmd for system_reset {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "system_reset".into();

        to_json.dump()
    }
}
/// #
/// @system_powerdown:
///
/// Requests that a guest perform a powerdown operation.
///
/// Since: 0.14.0
///
/// Notes: A guest may or may not respond to this command.  This command
///        returning does not indicate that a guest has accepted the request or
///        that it has shut down.  Many guests will respond to this command by
///        prompting the user in some way.
/// #
#[derive(Debug)]
pub struct system_powerdown {

        }
impl system_powerdown {
    pub fn new() -> system_powerdown {
        system_powerdown {}
    }
}
impl QemuCmd for system_powerdown {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "system_powerdown".into();

        to_json.dump()
    }
}
/// #
/// @cpu:
///
/// This command is a nop that is only provided for the purposes of compatibility.
///
/// Since: 0.14.0
///
/// Notes: Do not use this command.
/// #
#[derive(Debug)]
pub struct cpu {
    pub index: f64,
}
impl cpu {
    pub fn new(index: f64) -> cpu {
        cpu { index: index }
    }
}
impl QemuCmd for cpu {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "cpu".into();
        to_json["arguments"]["index"] = self.index.clone().into();
        to_json.dump()
    }
}
/// #
/// @cpu-add
///
/// Adds CPU with specified ID
///
/// @id: ID of CPU to be created, valid values [0..max_cpus)
///
/// Returns: Nothing on success
///
/// Since 1.5
/// #
#[derive(Debug)]
pub struct cpu_add {
    pub id: f64,
}
impl cpu_add {
    pub fn new(id: f64) -> cpu_add {
        cpu_add { id: id }
    }
}
impl QemuCmd for cpu_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "cpu-add".into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }
}
/// #
/// @memsave:
///
/// Save a portion of guest memory to a file.
///
/// @val: the virtual address of the guest to start from
///
/// @size: the size of memory region to save
///
/// @filename: the file to save the memory to as binary data
///
/// @cpu-index: #optional the index of the virtual CPU to use for translating the
///                       virtual address (defaults to CPU 0)
///
/// Returns: Nothing on success
///
/// Since: 0.14.0
///
/// Notes: Errors were not reliably returned until 1.1
/// #
#[derive(Debug)]
pub struct memsave {
    pub cpu_index: f64,
    pub filename: String,
    pub size: f64,
    pub val: f64,
}
impl memsave {
    pub fn new(cpu_index: f64, filename: String, size: f64, val: f64) -> memsave {
        memsave {
            cpu_index: cpu_index,
            filename: filename,
            size: size,
            val: val,
        }
    }
}
impl QemuCmd for memsave {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "memsave".into();
        to_json["arguments"]["cpu-index"] = self.cpu_index.clone().into();
        to_json["arguments"]["filename"] = self.filename.clone().into();
        to_json["arguments"]["size"] = self.size.clone().into();
        to_json["arguments"]["val"] = self.val.clone().into();
        to_json.dump()
    }
}
/// #
/// @pmemsave:
///
/// Save a portion of guest physical memory to a file.
///
/// @val: the physical address of the guest to start from
///
/// @size: the size of memory region to save
///
/// @filename: the file to save the memory to as binary data
///
/// Returns: Nothing on success
///
/// Since: 0.14.0
///
/// Notes: Errors were not reliably returned until 1.1
/// #
#[derive(Debug)]
pub struct pmemsave {
    pub filename: String,
    pub size: f64,
    pub val: f64,
}
impl pmemsave {
    pub fn new(filename: String, size: f64, val: f64) -> pmemsave {
        pmemsave {
            filename: filename,
            size: size,
            val: val,
        }
    }
}
impl QemuCmd for pmemsave {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "pmemsave".into();
        to_json["arguments"]["filename"] = self.filename.clone().into();
        to_json["arguments"]["size"] = self.size.clone().into();
        to_json["arguments"]["val"] = self.val.clone().into();
        to_json.dump()
    }
}
/// #
/// @cont:
///
/// Resume guest VCPU execution.
///
/// Since:  0.14.0
///
/// Returns:  If successful, nothing
///           If QEMU was started with an encrypted block device and a key has
///              not yet been set, DeviceEncrypted.
///
/// Notes:  This command will succeed if the guest is currently running.  It
///         will also succeed if the guest is in the "inmigrate" state; in
///         this case, the effect of the command is to make sure the guest
///         starts once migration finishes, removing the effect of the -S
///         command line option if it was passed.
/// #
#[derive(Debug)]
pub struct cont {

        }
impl cont {
    pub fn new() -> cont {
        cont {}
    }
}
impl QemuCmd for cont {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "cont".into();

        to_json.dump()
    }
}
/// #
/// @system_wakeup:
///
/// Wakeup guest from suspend.  Does nothing in case the guest isn"t suspended.
///
/// Since:  1.1
///
/// Returns:  nothing.
/// #
#[derive(Debug)]
pub struct system_wakeup {

        }
impl system_wakeup {
    pub fn new() -> system_wakeup {
        system_wakeup {}
    }
}
impl QemuCmd for system_wakeup {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "system_wakeup".into();

        to_json.dump()
    }
}
/// #
/// @inject-nmi:
///
/// Injects a Non-Maskable Interrupt into the default CPU (x86/s390) or all CPUs (ppc64).
///
/// Returns:  If successful, nothing
///
/// Since:  0.14.0
///
/// Note: prior to 2.1, this command was only supported for x86 and s390 VMs
/// #
#[derive(Debug)]
pub struct inject_nmi {

        }
impl inject_nmi {
    pub fn new() -> inject_nmi {
        inject_nmi {}
    }
}
impl QemuCmd for inject_nmi {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "inject-nmi".into();

        to_json.dump()
    }
}
/// #
/// @set_link:
///
/// Sets the link status of a virtual network adapter.
///
/// @name: the device name of the virtual network adapter
///
/// @up: true to set the link status to be up
///
/// Returns: Nothing on success
///          If @name is not a valid network device, DeviceNotFound
///
/// Since: 0.14.0
///
/// Notes: Not all network adapters support setting link status.  This command
///        will succeed even if the network adapter does not support link status
///        notification.
/// #
#[derive(Debug)]
pub struct set_link {
    pub name: String,
    pub up: bool,
}
impl set_link {
    pub fn new(name: String, up: bool) -> set_link {
        set_link {
            name: name,
            up: up,
        }
    }
}
impl QemuCmd for set_link {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "set_link".into();
        to_json["arguments"]["name"] = self.name.clone().into();
        to_json["arguments"]["up"] = self.up.clone().into();
        to_json.dump()
    }
}
/// #
/// @balloon:
///
/// Request the balloon driver to change its balloon size.
///
/// @value: the target size of the balloon in bytes
///
/// Returns: Nothing on success
///          If the balloon driver is enabled but not functional because the KVM
///            kernel module cannot support it, KvmMissingCap
///          If no balloon device is present, DeviceNotActive
///
/// Notes: This command just issues a request to the guest.  When it returns,
///        the balloon size may not have changed.  A guest can change the balloon
///        size independent of this command.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct balloon {
    pub value: f64,
}
impl balloon {
    pub fn new(value: f64) -> balloon {
        balloon { value: value }
    }
}
impl QemuCmd for balloon {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "balloon".into();
        to_json["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }
}
/// #
/// @transaction
///
/// Executes a number of transactionable QMP commands atomically. If any
/// operation fails, then the entire set of actions will be abandoned and the
/// appropriate error returned.
///
///  List of:
///  @TransactionAction: information needed for the respective operation
///
/// Returns: nothing on success
///          Errors depend on the operations of the transaction
///
/// Note: The transaction aborts on the first failure.  Therefore, there will be
/// information on only one failed operation returned in an error condition, and
/// subsequent actions will not have been attempted.
///
/// Since 1.1
/// #
#[derive(Debug)]
pub struct transaction {
    pub actions: Vec<String>,
}
impl transaction {
    pub fn new(actions: Vec<String>) -> transaction {
        transaction { actions: actions }
    }
}
impl QemuCmd for transaction {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "transaction".into();
        to_json["arguments"]["actions"] = self.actions.clone().into();
        to_json.dump()
    }
}
/// #
/// @human-monitor-command:
///
/// Execute a command on the human monitor and return the output.
///
/// @command-line: the command to execute in the human monitor
///
/// @cpu-index: #optional The CPU to use for commands that require an implicit CPU
///
/// Returns: the output of the command as a string
///
/// Since: 0.14.0
///
/// Notes: This command only exists as a stop-gap.  Its use is highly
///        discouraged.  The semantics of this command are not guaranteed.
///
///        Known limitations:
///
///        o This command is stateless, this means that commands that depend
///          on state information (such as getfd) might not work
///
///       o Commands that prompt the user for data (eg. "cont" when the block
///         device is encrypted) don"t currently work
/// #
#[derive(Debug)]
pub struct human_monitor_command {
    pub cpu_index: f64,
    pub command_line: String,
}
impl human_monitor_command {
    pub fn new(cpu_index: f64, command_line: String) -> human_monitor_command {
        human_monitor_command {
            cpu_index: cpu_index,
            command_line: command_line,
        }
    }
}
impl QemuCmd for human_monitor_command {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "human-monitor-command".into();
        to_json["arguments"]["cpu-index"] = self.cpu_index.clone().into();
        to_json["arguments"]["command-line"] = self.command_line.clone().into();
        to_json.dump()
    }
}
/// #
/// @migrate_cancel
///
/// Cancel the current executing migration process.
///
/// Returns: nothing on success
///
/// Notes: This command succeeds even if there is no migration process running.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct migrate_cancel {

        }
impl migrate_cancel {
    pub fn new() -> migrate_cancel {
        migrate_cancel {}
    }
}
impl QemuCmd for migrate_cancel {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate_cancel".into();

        to_json.dump()
    }
}
/// #
/// @migrate_set_downtime
///
/// Set maximum tolerated downtime for migration.
///
/// @value: maximum downtime in seconds
///
/// Returns: nothing on success
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct migrate_set_downtime {
    pub value: String,
}
impl migrate_set_downtime {
    pub fn new(value: String) -> migrate_set_downtime {
        migrate_set_downtime { value: value }
    }
}
impl QemuCmd for migrate_set_downtime {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate_set_downtime".into();
        to_json["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }
}
/// #
/// @migrate_set_speed
///
/// Set maximum speed for migration.
///
/// @value: maximum speed in bytes.
///
/// Returns: nothing on success
///
/// Notes: A value lesser than zero will be automatically round up to zero.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct migrate_set_speed {
    pub value: f64,
}
impl migrate_set_speed {
    pub fn new(value: f64) -> migrate_set_speed {
        migrate_set_speed { value: value }
    }
}
impl QemuCmd for migrate_set_speed {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate_set_speed".into();
        to_json["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }
}
/// #
/// @migrate-set-cache-size
///
/// Set XBZRLE cache size
///
/// @value: cache size in bytes
///
/// The size will be rounded down to the nearest power of 2.
/// The cache size can be modified before and during ongoing migration
///
/// Returns: nothing on success
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct migrate_set_cache_size {
    pub value: f64,
}
impl migrate_set_cache_size {
    pub fn new(value: f64) -> migrate_set_cache_size {
        migrate_set_cache_size { value: value }
    }
}
impl QemuCmd for migrate_set_cache_size {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate-set-cache-size".into();
        to_json["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-migrate-cache-size
///
/// query XBZRLE cache size
///
/// Returns: XBZRLE cache size in bytes
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct query_migrate_cache_size {

        }
impl query_migrate_cache_size {
    pub fn new() -> query_migrate_cache_size {
        query_migrate_cache_size {}
    }
}
impl QemuCmd for query_migrate_cache_size {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-migrate-cache-size".into();

        to_json.dump()
    }
}
/// #
/// @qom-list:
///
/// This command will list any properties of a object given a path in the object
/// model.
///
/// @path: the path within the object model.  See @qom-get for a description of
///        this parameter.
///
/// Returns: a list of @ObjectPropertyInfo that describe the properties of the
///          object.
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct qom_list {
    pub path: String,
}
impl qom_list {
    pub fn new(path: String) -> qom_list {
        qom_list { path: path }
    }
}
impl QemuCmd for qom_list {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "qom-list".into();
        to_json["arguments"]["path"] = self.path.clone().into();
        to_json.dump()
    }
}
/// #
/// @qom-get:
///
/// This command will get a property from a object model path and return the
/// value.
///
/// @path: The path within the object model.  There are two forms of supported
///        paths--absolute and partial paths.
///
///        Absolute paths are derived from the root object and can follow child<>
///        or link<> properties.  Since they can follow link<> properties, they
///        can be arbitrarily long.  Absolute paths look like absolute filenames
///        and are prefixed  with a leading slash.
///
///        Partial paths look like relative filenames.  They do not begin
///        with a prefix.  The matching rules for partial paths are subtle but
///        designed to make specifying objects easy.  At each level of the
///        composition tree, the partial path is matched as an absolute path.
///        The first match is not returned.  At least two matches are searched
///        for.  A successful result is only returned if only one match is
///        found.  If more than one match is found, a flag is return to
///        indicate that the match was ambiguous.
///
/// @property: The property name to read
///
/// Returns: The property value.  The type depends on the property type.  legacy<>
///          properties are returned as #str.  child<> and link<> properties are
///          returns as #str pathnames.  All integer property types (u8, u16, etc)
///          are returned as #int.
///
/// Since: 1.2
/// #
/// #
/// @qom-set:
///
/// This command will set a property from a object model path.
///
/// @path: see @qom-get for a description of this parameter
///
/// @property: the property name to set
///
/// @value: a value who"s type is appropriate for the property type.  See @qom-get
///         for a description of type mapping.
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct qom_set {
    pub path: String,
    pub property: String,
    pub value: String,
    gen: bool,
}
impl qom_set {
    pub fn new(path: String, property: String, value: String, gen: bool) -> qom_set {
        qom_set {
            path: path,
            property: property,
            value: value,
            gen: gen,
        }
    }
}
impl QemuCmd for qom_set {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "qom-set".into();
        to_json["arguments"]["path"] = self.path.clone().into();
        to_json["arguments"]["property"] = self.property.clone().into();
        to_json["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }
}
/// #
/// @set_password:
///
/// Sets the password of a remote display session.
///
/// @protocol: `vnc" to modify the VNC server password
///            `spice" to modify the Spice server password
///
/// @password: the new password
///
/// @connected: #optional how to handle existing clients when changing the
///                       password.  If nothing is specified, defaults to `keep"
///                       `fail" to fail the command if clients are connected
///                       `disconnect" to disconnect existing clients
///                       `keep" to maintain existing clients
///
/// Returns: Nothing on success
///          If Spice is not enabled, DeviceNotFound
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct set_password {
    pub connected: String,
    pub password: String,
    pub protocol: String,
}
impl set_password {
    pub fn new(connected: String, password: String, protocol: String) -> set_password {
        set_password {
            connected: connected,
            password: password,
            protocol: protocol,
        }
    }
}
impl QemuCmd for set_password {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "set_password".into();
        to_json["arguments"]["connected"] = self.connected.clone().into();
        to_json["arguments"]["password"] = self.password.clone().into();
        to_json["arguments"]["protocol"] = self.protocol.clone().into();
        to_json.dump()
    }
}
/// #
/// @expire_password:
///
/// Expire the password of a remote display server.
///
/// @protocol: the name of the remote display protocol `vnc" or `spice"
///
/// @time: when to expire the password.
///        `now" to expire the password immediately
///        `never" to cancel password expiration
///        `+INT" where INT is the number of seconds from now (integer)
///        `INT" where INT is the absolute time in seconds
///
/// Returns: Nothing on success
///          If @protocol is `spice" and Spice is not active, DeviceNotFound
///
/// Since: 0.14.0
///
/// Notes: Time is relative to the server and currently there is no way to
///        coordinate server time with client time.  It is not recommended to
///        use the absolute time version of the @time parameter unless you"re
///        sure you are on the same machine as the QEMU instance.
/// #
#[derive(Debug)]
pub struct expire_password {
    pub protocol: String,
    pub time: String,
}
impl expire_password {
    pub fn new(protocol: String, time: String) -> expire_password {
        expire_password {
            protocol: protocol,
            time: time,
        }
    }
}
impl QemuCmd for expire_password {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "expire_password".into();
        to_json["arguments"]["protocol"] = self.protocol.clone().into();
        to_json["arguments"]["time"] = self.time.clone().into();
        to_json.dump()
    }
}
/// #
/// @change-vnc-password:
///
/// Change the VNC server password.
///
/// @password:  the new password to use with VNC authentication
///
/// Since: 1.1
///
/// Notes:  An empty password in this command will set the password to the empty
///         string.  Existing clients are unaffected by executing this command.
/// #
#[derive(Debug)]
pub struct change_vnc_password {
    pub password: String,
}
impl change_vnc_password {
    pub fn new(password: String) -> change_vnc_password {
        change_vnc_password { password: password }
    }
}
impl QemuCmd for change_vnc_password {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "change-vnc-password".into();
        to_json["arguments"]["password"] = self.password.clone().into();
        to_json.dump()
    }
}
/// #
/// @change:
///
/// This command is multiple commands multiplexed together.
///
/// @device: This is normally the name of a block device but it may also be "vnc".
///          when it"s "vnc", then sub command depends on @target
///
/// @target: If @device is a block device, then this is the new filename.
///          If @device is "vnc", then if the value "password" selects the vnc
///          change password command.   Otherwise, this specifies a new server URI
///          address to listen to for VNC connections.
///
/// @arg:    If @device is a block device, then this is an optional format to open
///          the device with.
///          If @device is "vnc" and @target is "password", this is the new VNC
///          password to set.  If this argument is an empty string, then no future
///          logins will be allowed.
///
/// Returns: Nothing on success.
///          If @device is not a valid block device, DeviceNotFound
///          If the new block device is encrypted, DeviceEncrypted.  Note that
///          if this error is returned, the device has been opened successfully
///          and an additional call to @block_passwd is required to set the
///          device"s password.  The behavior of reads and writes to the block
///          device between when these calls are executed is undefined.
///
/// Notes:  It is strongly recommended that this interface is not used especially
///         for changing block devices.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct change {
    pub arg: String,
    pub device: String,
    pub target: String,
}
impl change {
    pub fn new(arg: String, device: String, target: String) -> change {
        change {
            arg: arg,
            device: device,
            target: target,
        }
    }
}
impl QemuCmd for change {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "change".into();
        to_json["arguments"]["arg"] = self.arg.clone().into();
        to_json["arguments"]["device"] = self.device.clone().into();
        to_json["arguments"]["target"] = self.target.clone().into();
        to_json.dump()
    }
}
/// #
/// @qom-list-types:
///
/// This command will return a list of types given search parameters
///
/// @implements: if specified, only return types that implement this type name
///
/// @abstract: if true, include abstract types in the results
///
/// Returns: a list of @ObjectTypeInfo or an empty list if no results are found
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct qom_list_types {
    pub qemu_abstract: bool,
    pub implements: String,
}
impl qom_list_types {
    pub fn new(qemu_abstract: bool, implements: String) -> qom_list_types {
        qom_list_types {
            qemu_abstract: qemu_abstract,
            implements: implements,
        }
    }
}
impl QemuCmd for qom_list_types {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "qom-list-types".into();
        to_json["arguments"]["abstract"] = self.qemu_abstract.clone().into();
        to_json["arguments"]["implements"] = self.implements.clone().into();
        to_json.dump()
    }
}
/// #
/// @device-list-properties:
///
/// List properties associated with a device.
///
/// @typename: the type name of a device
///
/// Returns: a list of DevicePropertyInfo describing a devices properties
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct device_list_properties {
    pub typename: String,
}
impl device_list_properties {
    pub fn new(typename: String) -> device_list_properties {
        device_list_properties { typename: typename }
    }
}
impl QemuCmd for device_list_properties {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "device-list-properties".into();
        to_json["arguments"]["typename"] = self.typename.clone().into();
        to_json.dump()
    }
}
/// #
/// @migrate
///
/// Migrates the current running guest to another Virtual Machine.
///
/// @uri: the Uniform Resource Identifier of the destination VM
///
/// @blk: #optional do block migration (full disk copy)
///
/// @inc: #optional incremental disk copy migration
///
/// @detach: this argument exists only for compatibility reasons and
///          is ignored by QEMU
///
/// Returns: nothing on success
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct migrate {
    pub blk: bool,
    pub detach: bool,
    pub inc: bool,
    pub uri: String,
}
impl migrate {
    pub fn new(blk: bool, detach: bool, inc: bool, uri: String) -> migrate {
        migrate {
            blk: blk,
            detach: detach,
            inc: inc,
            uri: uri,
        }
    }
}
impl QemuCmd for migrate {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate".into();
        to_json["arguments"]["blk"] = self.blk.clone().into();
        to_json["arguments"]["detach"] = self.detach.clone().into();
        to_json["arguments"]["inc"] = self.inc.clone().into();
        to_json["arguments"]["uri"] = self.uri.clone().into();
        to_json.dump()
    }
}
/// #
/// @migrate-incoming
///
/// Start an incoming migration, the qemu must have been started
/// with -incoming defer
///
/// @uri: The Uniform Resource Identifier identifying the source or
///       address to listen on
///
/// Returns: nothing on success
///
/// Since: 2.3
/// Note: It"s a bad idea to use a string for the uri, but it needs to stay
/// compatible with -incoming and the format of the uri is already exposed
/// above libvirt
/// #
#[derive(Debug)]
pub struct migrate_incoming {
    pub uri: String,
}
impl migrate_incoming {
    pub fn new(uri: String) -> migrate_incoming {
        migrate_incoming { uri: uri }
    }
}
impl QemuCmd for migrate_incoming {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate-incoming".into();
        to_json["arguments"]["uri"] = self.uri.clone().into();
        to_json.dump()
    }
}
/// @xen-save-devices-state:
///
/// Save the state of all devices to file. The RAM and the block devices
/// of the VM are not saved by this command.
///
/// @filename: the file to save the state of the devices to as binary
/// data. See xen-save-devices-state.txt for a description of the binary
/// format.
///
/// Returns: Nothing on success
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct xen_save_devices_state {
    pub filename: String,
}
impl xen_save_devices_state {
    pub fn new(filename: String) -> xen_save_devices_state {
        xen_save_devices_state { filename: filename }
    }
}
impl QemuCmd for xen_save_devices_state {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "xen-save-devices-state".into();
        to_json["arguments"]["filename"] = self.filename.clone().into();
        to_json.dump()
    }
}
/// #
/// @xen-set-global-dirty-log
///
/// Enable or disable the global dirty log mode.
///
/// @enable: true to enable, false to disable.
///
/// Returns: nothing
///
/// Since: 1.3
/// #
#[derive(Debug)]
pub struct xen_set_global_dirty_log {
    pub enable: bool,
}
impl xen_set_global_dirty_log {
    pub fn new(enable: bool) -> xen_set_global_dirty_log {
        xen_set_global_dirty_log { enable: enable }
    }
}
impl QemuCmd for xen_set_global_dirty_log {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "xen-set-global-dirty-log".into();
        to_json["arguments"]["enable"] = self.enable.clone().into();
        to_json.dump()
    }
}
/// #
/// @device_del:
///
/// Remove a device from a guest
///
/// @id: the name of the device
///
/// Returns: Nothing on success
///          If @id is not a valid device, DeviceNotFound
///
/// Notes: When this command completes, the device may not be removed from the
///        guest.  Hot removal is an operation that requires guest cooperation.
///        This command merely requests that the guest begin the hot removal
///        process.  Completion of the device removal process is signaled with a
///        DEVICE_DELETED event. Guest reset will automatically complete removal
///        for all devices.
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct device_del {
    pub id: String,
}
impl device_del {
    pub fn new(id: String) -> device_del {
        device_del { id: id }
    }
}
impl QemuCmd for device_del {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "device_del".into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }
}
/// #
/// @dump-guest-memory
///
/// Dump guest"s memory to vmcore. It is a synchronous operation that can take
/// very long depending on the amount of guest memory. This command is only
/// supported on i386 and x86_64.
///
/// @paging: if true, do paging to get guest"s memory mapping. This allows
///          using gdb to process the core file.
///
///          IMPORTANT: this option can make QEMU allocate several gigabytes
///                     of RAM. This can happen for a large guest, or a
///                     malicious guest pretending to be large.
///
///          Also, paging=true has the following limitations:
///
///             1. The guest may be in a catastrophic state or can have corrupted
///                memory, which cannot be trusted
///             2. The guest can be in real-mode even if paging is enabled. For
///                example, the guest uses ACPI to sleep, and ACPI sleep state
///                goes in real-mode
///
/// @protocol: the filename or file descriptor of the vmcore. The supported
///            protocols are:
///
///            1. file: the protocol starts with "file:", and the following
///               string is the file"s path.
///            2. fd: the protocol starts with "fd:", and the following string
///               is the fd"s name.
///
/// @begin: #optional if specified, the starting physical address.
///
/// @length: #optional if specified, the memory size, in bytes. If you don"t
///          want to dump all guest"s memory, please specify the start @begin
///          and @length
///
/// @format: #optional if specified, the format of guest memory dump. But non-elf
///          format is conflict with paging and filter, ie. @paging, @begin and
///          @length is not allowed to be specified with non-elf @format at the
///          same time (since 2.0)
///
/// Returns: nothing on success
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct dump_guest_memory {
    pub begin: f64,
    pub format: String,
    pub length: f64,
    pub paging: bool,
    pub protocol: String,
}
impl dump_guest_memory {
    pub fn new(begin: f64,
               format: String,
               length: f64,
               paging: bool,
               protocol: String)
               -> dump_guest_memory {
        dump_guest_memory {
            begin: begin,
            format: format,
            length: length,
            paging: paging,
            protocol: protocol,
        }
    }
}
impl QemuCmd for dump_guest_memory {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "dump-guest-memory".into();
        to_json["arguments"]["begin"] = self.begin.clone().into();
        to_json["arguments"]["format"] = self.format.clone().into();
        to_json["arguments"]["length"] = self.length.clone().into();
        to_json["arguments"]["paging"] = self.paging.clone().into();
        to_json["arguments"]["protocol"] = self.protocol.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-dump-guest-memory-capability:
///
/// Returns the available formats for dump-guest-memory
///
/// Returns:  A @DumpGuestMemoryCapability object listing available formats for
///           dump-guest-memory
///
/// Since: 2.0
/// #
#[derive(Debug)]
pub struct query_dump_guest_memory_capability {

        }
impl query_dump_guest_memory_capability {
    pub fn new() -> query_dump_guest_memory_capability {
        query_dump_guest_memory_capability {}
    }
}
impl QemuCmd for query_dump_guest_memory_capability {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-dump-guest-memory-capability".into();

        to_json.dump()
    }
}
/// #
/// @netdev_add:
///
/// Add a network backend.
///
/// @type: the type of network backend.  Current valid values are "user", "tap",
///        "vde", "socket", "dump" and "bridge"
///
/// @id: the name of the new network backend
///
/// @props: #optional a list of properties to be passed to the backend in
///         the format "name=value", like "ifname=tap0,script=no"
///
/// Notes: The semantics of @props is not well defined.  Future commands will be
///        introduced that provide stronger typing for backend creation.
///
/// Since: 0.14.0
///
/// Returns: Nothing on success
///          If @type is not a valid network backend, DeviceNotFound
/// #
#[derive(Debug)]
pub struct netdev_add {
    pub props: String,
    pub id: String,
    pub qemu_type: String,
    gen: bool,
}
impl netdev_add {
    pub fn new(props: String, id: String, qemu_type: String, gen: bool) -> netdev_add {
        netdev_add {
            props: props,
            id: id,
            qemu_type: qemu_type,
            gen: gen,
        }
    }
}
impl QemuCmd for netdev_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "netdev_add".into();
        to_json["arguments"]["props"] = self.props.clone().into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json["arguments"]["type"] = self.qemu_type.clone().into();
        to_json.dump()
    }
}
/// #
/// @netdev_del:
///
/// Remove a network backend.
///
/// @id: the name of the network backend to remove
///
/// Returns: Nothing on success
///          If @id is not a valid network backend, DeviceNotFound
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct netdev_del {
    pub id: String,
}
impl netdev_del {
    pub fn new(id: String) -> netdev_del {
        netdev_del { id: id }
    }
}
impl QemuCmd for netdev_del {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "netdev_del".into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }
}
/// #
/// @object-add:
///
/// Create a QOM object.
///
/// @qom-type: the class name for the object to be created
///
/// @id: the name of the new object
///
/// @props: #optional a dictionary of properties to be passed to the backend
///
/// Returns: Nothing on success
///          Error if @qom-type is not a valid class name
///
/// Since: 2.0
/// #
#[derive(Debug)]
pub struct object_add {
    pub props: String,
    pub id: String,
    pub qom_type: String,
    gen: bool,
}
impl object_add {
    pub fn new(props: String, id: String, qom_type: String, gen: bool) -> object_add {
        object_add {
            props: props,
            id: id,
            qom_type: qom_type,
            gen: gen,
        }
    }
}
impl QemuCmd for object_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "object-add".into();
        to_json["arguments"]["props"] = self.props.clone().into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json["arguments"]["qom-type"] = self.qom_type.clone().into();
        to_json.dump()
    }
}
/// #
/// @object-del:
///
/// Remove a QOM object.
///
/// @id: the name of the QOM object to remove
///
/// Returns: Nothing on success
///          Error if @id is not a valid id for a QOM object
///
/// Since: 2.0
/// #
#[derive(Debug)]
pub struct object_del {
    pub id: String,
}
impl object_del {
    pub fn new(id: String) -> object_del {
        object_del { id: id }
    }
}
impl QemuCmd for object_del {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "object-del".into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }
}
/// #
/// @getfd:
///
/// Receive a file descriptor via SCM rights and assign it a name
///
/// @fdname: file descriptor name
///
/// Returns: Nothing on success
///
/// Since: 0.14.0
///
/// Notes: If @fdname already exists, the file descriptor assigned to
///        it will be closed and replaced by the received file
///        descriptor.
///        The "closefd" command can be used to explicitly close the
///        file descriptor when it is no longer needed.
/// #
#[derive(Debug)]
pub struct getfd {
    pub fdname: String,
}
impl getfd {
    pub fn new(fdname: String) -> getfd {
        getfd { fdname: fdname }
    }
}
impl QemuCmd for getfd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "getfd".into();
        to_json["arguments"]["fdname"] = self.fdname.clone().into();
        to_json.dump()
    }
}
/// #
/// @closefd:
///
/// Close a file descriptor previously passed via SCM rights
///
/// @fdname: file descriptor name
///
/// Returns: Nothing on success
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct closefd {
    pub fdname: String,
}
impl closefd {
    pub fn new(fdname: String) -> closefd {
        closefd { fdname: fdname }
    }
}
impl QemuCmd for closefd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "closefd".into();
        to_json["arguments"]["fdname"] = self.fdname.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-machines:
///
/// Return a list of supported machines
///
/// Returns: a list of MachineInfo
///
/// Since: 1.2.0
/// #
#[derive(Debug)]
pub struct query_machines {

        }
impl query_machines {
    pub fn new() -> query_machines {
        query_machines {}
    }
}
impl QemuCmd for query_machines {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-machines".into();

        to_json.dump()
    }
}
/// #
/// @query-cpu-definitions:
///
/// Return a list of supported virtual CPU definitions
///
/// Returns: a list of CpuDefInfo
///
/// Since: 1.2.0
/// #
#[derive(Debug)]
pub struct query_cpu_definitions {

        }
impl query_cpu_definitions {
    pub fn new() -> query_cpu_definitions {
        query_cpu_definitions {}
    }
}
impl QemuCmd for query_cpu_definitions {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-cpu-definitions".into();

        to_json.dump()
    }
}
/// #
/// @add-fd:
///
/// Add a file descriptor, that was passed via SCM rights, to an fd set.
///
/// @fdset-id: #optional The ID of the fd set to add the file descriptor to.
///
/// @opaque: #optional A free-form string that can be used to describe the fd.
///
/// Returns: @AddfdInfo on success
///          If file descriptor was not received, FdNotSupplied
///          If @fdset-id is a negative value, InvalidParameterValue
///
/// Notes: The list of fd sets is shared by all monitor connections.
///
///        If @fdset-id is not specified, a new fd set will be created.
///
/// Since: 1.2.0
/// #
#[derive(Debug)]
pub struct add_fd {
    pub fdset_id: f64,
    pub opaque: String,
}
impl add_fd {
    pub fn new(fdset_id: f64, opaque: String) -> add_fd {
        add_fd {
            fdset_id: fdset_id,
            opaque: opaque,
        }
    }
}
impl QemuCmd for add_fd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "add-fd".into();
        to_json["arguments"]["fdset-id"] = self.fdset_id.clone().into();
        to_json["arguments"]["opaque"] = self.opaque.clone().into();
        to_json.dump()
    }
}
/// #
/// @remove-fd:
///
/// Remove a file descriptor from an fd set.
///
/// @fdset-id: The ID of the fd set that the file descriptor belongs to.
///
/// @fd: #optional The file descriptor that is to be removed.
///
/// Returns: Nothing on success
///          If @fdset-id or @fd is not found, FdNotFound
///
/// Since: 1.2.0
///
/// Notes: The list of fd sets is shared by all monitor connections.
///
///        If @fd is not specified, all file descriptors in @fdset-id
///        will be removed.
/// #
#[derive(Debug)]
pub struct remove_fd {
    pub fd: f64,
    pub fdset_id: f64,
}
impl remove_fd {
    pub fn new(fd: f64, fdset_id: f64) -> remove_fd {
        remove_fd {
            fd: fd,
            fdset_id: fdset_id,
        }
    }
}
impl QemuCmd for remove_fd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "remove-fd".into();
        to_json["arguments"]["fd"] = self.fd.clone().into();
        to_json["arguments"]["fdset-id"] = self.fdset_id.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-fdsets:
///
/// Return information describing all fd sets.
///
/// Returns: A list of @FdsetInfo
///
/// Since: 1.2.0
///
/// Note: The list of fd sets is shared by all monitor connections.
///
/// #
#[derive(Debug)]
pub struct query_fdsets {

        }
impl query_fdsets {
    pub fn new() -> query_fdsets {
        query_fdsets {}
    }
}
impl QemuCmd for query_fdsets {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-fdsets".into();

        to_json.dump()
    }
}
/// #
/// @query-target:
///
/// Return information about the target for this QEMU
///
/// Returns: TargetInfo
///
/// Since: 1.2.0
/// #
#[derive(Debug)]
pub struct query_target {

        }
impl query_target {
    pub fn new() -> query_target {
        query_target {}
    }
}
impl QemuCmd for query_target {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-target".into();

        to_json.dump()
    }
}
/// #
/// @send-key:
///
/// Send keys to guest.
///
/// @keys: An array of @KeyValue elements. All @KeyValues in this array are
///        simultaneously sent to the guest. A @KeyValue.number value is sent
///        directly to the guest, while @KeyValue.qcode must be a valid
///        @QKeyCode value
///
/// @hold-time: #optional time to delay key up events, milliseconds. Defaults
///             to 100
///
/// Returns: Nothing on success
///          If key is unknown or redundant, InvalidParameter
///
/// Since: 1.3.0
///
/// #
#[derive(Debug)]
pub struct send_key {
    pub hold_time: f64,
    pub keys: Vec<String>,
}
impl send_key {
    pub fn new(hold_time: f64, keys: Vec<String>) -> send_key {
        send_key {
            hold_time: hold_time,
            keys: keys,
        }
    }
}
impl QemuCmd for send_key {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "send-key".into();
        to_json["arguments"]["hold-time"] = self.hold_time.clone().into();
        to_json["arguments"]["keys"] = self.keys.clone().into();
        to_json.dump()
    }
}
/// #
/// @screendump:
///
/// Write a PPM of the VGA screen to a file.
///
/// @filename: the path of a new PPM file to store the image
///
/// Returns: Nothing on success
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct screendump {
    pub filename: String,
}
impl screendump {
    pub fn new(filename: String) -> screendump {
        screendump { filename: filename }
    }
}
impl QemuCmd for screendump {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "screendump".into();
        to_json["arguments"]["filename"] = self.filename.clone().into();
        to_json.dump()
    }
}
/// #
/// @chardev-add:
///
/// Add a character device backend
///
/// @id: the chardev"s ID, must be unique
/// @backend: backend type and parameters
///
/// Returns: ChardevReturn.
///
/// Since: 1.4
/// #
#[derive(Debug)]
pub struct chardev_add {
    pub backend: String,
    pub id: String,
}
impl chardev_add {
    pub fn new(backend: String, id: String) -> chardev_add {
        chardev_add {
            backend: backend,
            id: id,
        }
    }
}
impl QemuCmd for chardev_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "chardev-add".into();
        to_json["arguments"]["backend"] = self.backend.clone().into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }
}
/// #
/// @chardev-remove:
///
/// Remove a character device backend
///
/// @id: the chardev"s ID, must exist and not be in use
///
/// Returns: Nothing on success
///
/// Since: 1.4
/// #
#[derive(Debug)]
pub struct chardev_remove {
    pub id: String,
}
impl chardev_remove {
    pub fn new(id: String) -> chardev_remove {
        chardev_remove { id: id }
    }
}
impl QemuCmd for chardev_remove {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "chardev-remove".into();
        to_json["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-tpm-models:
///
/// Return a list of supported TPM models
///
/// Returns: a list of TpmModel
///
/// Since: 1.5
/// #
#[derive(Debug)]
pub struct query_tpm_models {

        }
impl query_tpm_models {
    pub fn new() -> query_tpm_models {
        query_tpm_models {}
    }
}
impl QemuCmd for query_tpm_models {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-tpm-models".into();

        to_json.dump()
    }
}
/// #
/// @query-tpm-types:
///
/// Return a list of supported TPM types
///
/// Returns: a list of TpmType
///
/// Since: 1.5
/// #
#[derive(Debug)]
pub struct query_tpm_types {

        }
impl query_tpm_types {
    pub fn new() -> query_tpm_types {
        query_tpm_types {}
    }
}
impl QemuCmd for query_tpm_types {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-tpm-types".into();

        to_json.dump()
    }
}
/// #
/// @query-tpm:
///
/// Return information about the TPM device
///
/// Returns: @TPMInfo on success
///
/// Since: 1.5
/// #
#[derive(Debug)]
pub struct query_tpm {

        }
impl query_tpm {
    pub fn new() -> query_tpm {
        query_tpm {}
    }
}
impl QemuCmd for query_tpm {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-tpm".into();

        to_json.dump()
    }
}
/// #
/// @query-command-line-options:
///
/// Query command line option schema.
///
/// @option: #optional option name
///
/// Returns: list of @CommandLineOptionInfo for all options (or for the given
///          @option).  Returns an error if the given @option doesn"t exist.
///
/// Since 1.5
/// #
#[derive(Debug)]
pub struct query_command_line_options {
    pub option: String,
}
impl query_command_line_options {
    pub fn new(option: String) -> query_command_line_options {
        query_command_line_options { option: option }
    }
}
impl QemuCmd for query_command_line_options {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-command-line-options".into();
        to_json["arguments"]["option"] = self.option.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-rx-filter:
///
/// Return rx-filter information for all NICs (or for the given NIC).
///
/// @name: #optional net client name
///
/// Returns: list of @RxFilterInfo for all NICs (or for the given NIC).
///          Returns an error if the given @name doesn"t exist, or given
///          NIC doesn"t support rx-filter querying, or given net client
///          isn"t a NIC.
///
/// Since: 1.6
/// #
#[derive(Debug)]
pub struct query_rx_filter {
    pub name: String,
}
impl query_rx_filter {
    pub fn new(name: String) -> query_rx_filter {
        query_rx_filter { name: name }
    }
}
impl QemuCmd for query_rx_filter {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-rx-filter".into();
        to_json["arguments"]["name"] = self.name.clone().into();
        to_json.dump()
    }
}
/// #
/// @x-input-send-event
///
/// Send input event(s) to guest.
///
/// @console: #optional console to send event(s) to.
///           This parameter can be used to send the input event to
///           specific input devices in case (a) multiple input devices
///           of the same kind are added to the virtual machine and (b)
///           you have configured input routing (see docs/multiseat.txt)
///           for those input devices.  If input routing is not
///           configured this parameter has no effect.
///           If @console is missing, only devices that aren"t associated
///           with a console are admissible.
///           If @console is specified, it must exist, and both devices
///           associated with that console and devices not associated with a
///           console are admissible, but the former take precedence.
#[derive(Debug)]
pub struct x_input_send_event {
    pub console: f64,
    pub events: Vec<String>,
}
impl x_input_send_event {
    pub fn new(console: f64, events: Vec<String>) -> x_input_send_event {
        x_input_send_event {
            console: console,
            events: events,
        }
    }
}
impl QemuCmd for x_input_send_event {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "x-input-send-event".into();
        to_json["arguments"]["console"] = self.console.clone().into();
        to_json["arguments"]["events"] = self.events.clone().into();
        to_json.dump()
    }
}
/// #
/// @query-memdev:
///
/// Returns information for all memory backends.
///
/// Returns: a list of @Memdev.
///
/// Since: 2.1
/// #
#[derive(Debug)]
pub struct query_memdev {

        }
impl query_memdev {
    pub fn new() -> query_memdev {
        query_memdev {}
    }
}
impl QemuCmd for query_memdev {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-memdev".into();

        to_json.dump()
    }
}
/// #
/// @query-memory-devices
///
/// Lists available memory devices and their state
///
/// Since: 2.1
/// #
#[derive(Debug)]
pub struct query_memory_devices {

        }
impl query_memory_devices {
    pub fn new() -> query_memory_devices {
        query_memory_devices {}
    }
}
impl QemuCmd for query_memory_devices {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-memory-devices".into();

        to_json.dump()
    }
}
/// #
/// @query-acpi-ospm-status
///
/// Lists ACPI OSPM status of ACPI device objects,
/// which might be reported via _OST method
///
/// Since: 2.1
/// #
#[derive(Debug)]
pub struct query_acpi_ospm_status {

        }
impl query_acpi_ospm_status {
    pub fn new() -> query_acpi_ospm_status {
        query_acpi_ospm_status {}
    }
}
impl QemuCmd for query_acpi_ospm_status {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-acpi-ospm-status".into();

        to_json.dump()
    }
}
/// #
/// @rtc-reset-reinjection
///
/// This command will reset the RTC interrupt reinjection backlog.
/// Can be used if another mechanism to synchronize guest time
/// is in effect, for example QEMU guest agent"s guest-set-time
/// command.
///
/// Since: 2.1
/// #
#[derive(Debug)]
pub struct rtc_reset_reinjection {

        }
impl rtc_reset_reinjection {
    pub fn new() -> rtc_reset_reinjection {
        rtc_reset_reinjection {}
    }
}
impl QemuCmd for rtc_reset_reinjection {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "rtc-reset-reinjection".into();

        to_json.dump()
    }
}
