use QemuCmd;
use rustc_serialize::json as rustc_json;
use rustc_serialize::Decodable as rustc_decodable;
use json;
use events::*;
use enums::*;
use structs::*;

#[derive(Debug)]
pub struct blockdev_snapshot_internal_sync {

        }
impl blockdev_snapshot_internal_sync {
    pub fn new() -> blockdev_snapshot_internal_sync {
        blockdev_snapshot_internal_sync {}
    }
}
impl<T> QemuCmd<T> for blockdev_snapshot_internal_sync {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-snapshot-internal-sync".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for blockdev_snapshot_delete_internal_sync {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-snapshot-delete-internal-sync".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["blockdev-snapshot-delete-internal-sync"]["arguments"]["id"] =
            self.id.clone().into();
        to_json["blockdev-snapshot-delete-internal-sync"]["arguments"]["name"] =
            self.name.clone().into();
        to_json["blockdev-snapshot-delete-internal-sync"]["arguments"]["device"] =
            self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for eject {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "eject".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["eject"]["arguments"]["force"] = self.force.clone().into();
        to_json["eject"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct nbd_server_start {
    pub addr: String,
}
impl nbd_server_start {
    pub fn new(addr: String) -> nbd_server_start {
        nbd_server_start { addr: addr }
    }
}
impl<T> QemuCmd<T> for nbd_server_start {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "nbd-server-start".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["nbd-server-start"]["arguments"]["addr"] = self.addr.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for nbd_server_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "nbd-server-add".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["nbd-server-add"]["arguments"]["writable"] = self.writable.clone().into();
        to_json["nbd-server-add"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct nbd_server_stop {

        }
impl nbd_server_stop {
    pub fn new() -> nbd_server_stop {
        nbd_server_stop {}
    }
}
impl<T> QemuCmd<T> for nbd_server_stop {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "nbd-server-stop".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_block {

        }
impl query_block {
    pub fn new() -> query_block {
        query_block {}
    }
}
impl<T> QemuCmd<T> for query_block {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-block".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_blockstats {
    pub query_nodes: bool,
}
impl query_blockstats {
    pub fn new(query_nodes: bool) -> query_blockstats {
        query_blockstats { query_nodes: query_nodes }
    }
}
impl<T> QemuCmd<T> for query_blockstats {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-blockstats".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["query-blockstats"]["arguments"]["query_nodes"] = self.query_nodes.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_block_jobs {

        }
impl query_block_jobs {
    pub fn new() -> query_block_jobs {
        query_block_jobs {}
    }
}
impl<T> QemuCmd<T> for query_block_jobs {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-block-jobs".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for block_passwd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block_passwd".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block_passwd"]["arguments"]["device"] = self.device.clone().into();
        to_json["block_passwd"]["arguments"]["node_name"] = self.node_name.clone().into();
        to_json["block_passwd"]["arguments"]["password"] = self.password.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for block_resize {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block_resize".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block_resize"]["arguments"]["device"] = self.device.clone().into();
        to_json["block_resize"]["arguments"]["node_name"] = self.node_name.clone().into();
        to_json["block_resize"]["arguments"]["size"] = self.size.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct blockdev_snapshot_sync {

        }
impl blockdev_snapshot_sync {
    pub fn new() -> blockdev_snapshot_sync {
        blockdev_snapshot_sync {}
    }
}
impl<T> QemuCmd<T> for blockdev_snapshot_sync {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-snapshot-sync".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for change_backing_file {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "change-backing-file".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["change-backing-file"]["arguments"]["backing_file"] =
            self.backing_file.clone().into();
        to_json["change-backing-file"]["arguments"]["device"] = self.device.clone().into();
        to_json["change-backing-file"]["arguments"]["image_node_name"] =
            self.image_node_name.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for block_commit {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-commit".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block-commit"]["arguments"]["backing_file"] = self.backing_file.clone().into();
        to_json["block-commit"]["arguments"]["base"] = self.base.clone().into();
        to_json["block-commit"]["arguments"]["speed"] = self.speed.clone().into();
        to_json["block-commit"]["arguments"]["top"] = self.top.clone().into();
        to_json["block-commit"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct drive_backup {

        }
impl drive_backup {
    pub fn new() -> drive_backup {
        drive_backup {}
    }
}
impl<T> QemuCmd<T> for drive_backup {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "drive-backup".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct blockdev_backup {

        }
impl blockdev_backup {
    pub fn new() -> blockdev_backup {
        blockdev_backup {}
    }
}
impl<T> QemuCmd<T> for blockdev_backup {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-backup".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_named_block_nodes {

        }
impl query_named_block_nodes {
    pub fn new() -> query_named_block_nodes {
        query_named_block_nodes {}
    }
}
impl<T> QemuCmd<T> for query_named_block_nodes {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-named-block-nodes".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for drive_mirror {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "drive-mirror".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["drive-mirror"]["arguments"]["buf_size"] = self.buf_size.clone().into();
        to_json["drive-mirror"]["arguments"]["format"] = self.format.clone().into();
        to_json["drive-mirror"]["arguments"]["granularity"] = self.granularity.clone().into();
        to_json["drive-mirror"]["arguments"]["mode"] = self.mode.clone().into();
        to_json["drive-mirror"]["arguments"]["node_name"] = self.node_name.clone().into();
        to_json["drive-mirror"]["arguments"]["on_source_error"] =
            self.on_source_error.clone().into();
        to_json["drive-mirror"]["arguments"]["on_target_error"] =
            self.on_target_error.clone().into();
        to_json["drive-mirror"]["arguments"]["replaces"] = self.replaces.clone().into();
        to_json["drive-mirror"]["arguments"]["speed"] = self.speed.clone().into();
        to_json["drive-mirror"]["arguments"]["device"] = self.device.clone().into();
        to_json["drive-mirror"]["arguments"]["sync"] = self.sync.clone().into();
        to_json["drive-mirror"]["arguments"]["target"] = self.target.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct block_dirty_bitmap_add {

        }
impl block_dirty_bitmap_add {
    pub fn new() -> block_dirty_bitmap_add {
        block_dirty_bitmap_add {}
    }
}
impl<T> QemuCmd<T> for block_dirty_bitmap_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-dirty-bitmap-add".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct block_dirty_bitmap_remove {

        }
impl block_dirty_bitmap_remove {
    pub fn new() -> block_dirty_bitmap_remove {
        block_dirty_bitmap_remove {}
    }
}
impl<T> QemuCmd<T> for block_dirty_bitmap_remove {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-dirty-bitmap-remove".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct block_dirty_bitmap_clear {

        }
impl block_dirty_bitmap_clear {
    pub fn new() -> block_dirty_bitmap_clear {
        block_dirty_bitmap_clear {}
    }
}
impl<T> QemuCmd<T> for block_dirty_bitmap_clear {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-dirty-bitmap-clear".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for block_set_io_throttle {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block_set_io_throttle".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block_set_io_throttle"]["arguments"]["bps_max"] = self.bps_max.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["bps_rd_max"] =
            self.bps_rd_max.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["bps_wr_max"] =
            self.bps_wr_max.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["iops_max"] = self.iops_max.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["iops_rd_max"] =
            self.iops_rd_max.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["iops_size"] = self.iops_size.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["iops_wr_max"] =
            self.iops_wr_max.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["bps"] = self.bps.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["bps_rd"] = self.bps_rd.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["bps_wr"] = self.bps_wr.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["device"] = self.device.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["iops"] = self.iops.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["iops_rd"] = self.iops_rd.clone().into();
        to_json["block_set_io_throttle"]["arguments"]["iops_wr"] = self.iops_wr.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for block_stream {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-stream".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block-stream"]["arguments"]["backing_file"] = self.backing_file.clone().into();
        to_json["block-stream"]["arguments"]["base"] = self.base.clone().into();
        to_json["block-stream"]["arguments"]["on_error"] = self.on_error.clone().into();
        to_json["block-stream"]["arguments"]["speed"] = self.speed.clone().into();
        to_json["block-stream"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for block_job_set_speed {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-set-speed".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block-job-set-speed"]["arguments"]["device"] = self.device.clone().into();
        to_json["block-job-set-speed"]["arguments"]["speed"] = self.speed.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for block_job_cancel {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-cancel".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block-job-cancel"]["arguments"]["force"] = self.force.clone().into();
        to_json["block-job-cancel"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct block_job_pause {
    pub device: String,
}
impl block_job_pause {
    pub fn new(device: String) -> block_job_pause {
        block_job_pause { device: device }
    }
}
impl<T> QemuCmd<T> for block_job_pause {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-pause".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block-job-pause"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct block_job_resume {
    pub device: String,
}
impl block_job_resume {
    pub fn new(device: String) -> block_job_resume {
        block_job_resume { device: device }
    }
}
impl<T> QemuCmd<T> for block_job_resume {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-resume".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block-job-resume"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct block_job_complete {
    pub device: String,
}
impl block_job_complete {
    pub fn new(device: String) -> block_job_complete {
        block_job_complete { device: device }
    }
}
impl<T> QemuCmd<T> for block_job_complete {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-job-complete".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block-job-complete"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct blockdev_add {
    pub options: String,
}
impl blockdev_add {
    pub fn new(options: String) -> blockdev_add {
        blockdev_add { options: options }
    }
}
impl<T> QemuCmd<T> for blockdev_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "blockdev-add".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["blockdev-add"]["arguments"]["options"] = self.options.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for block_set_write_threshold {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "block-set-write-threshold".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["block-set-write-threshold"]["arguments"]["node_name"] =
            self.node_name.clone().into();
        to_json["block-set-write-threshold"]["arguments"]["write_threshold"] =
            self.write_threshold.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_version {

        }
impl query_version {
    pub fn new() -> query_version {
        query_version {}
    }
}
impl<T> QemuCmd<T> for query_version {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-version".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_commands {

        }
impl query_commands {
    pub fn new() -> query_commands {
        query_commands {}
    }
}
impl<T> QemuCmd<T> for query_commands {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-commands".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct trace_event_get_state {
    pub name: String,
}
impl trace_event_get_state {
    pub fn new(name: String) -> trace_event_get_state {
        trace_event_get_state { name: name }
    }
}
impl<T> QemuCmd<T> for trace_event_get_state {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "trace-event-get-state".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["trace-event-get-state"]["arguments"]["name"] = self.name.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for trace_event_set_state {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "trace-event-set-state".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["trace-event-set-state"]["arguments"]["ignore_unavailable"] =
            self.ignore_unavailable.clone().into();
        to_json["trace-event-set-state"]["arguments"]["enable"] = self.enable.clone().into();
        to_json["trace-event-set-state"]["arguments"]["name"] = self.name.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for add_client {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "add_client".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["add_client"]["arguments"]["skipauth"] = self.skipauth.clone().into();
        to_json["add_client"]["arguments"]["tls"] = self.tls.clone().into();
        to_json["add_client"]["arguments"]["fdname"] = self.fdname.clone().into();
        to_json["add_client"]["arguments"]["protocol"] = self.protocol.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_name {

        }
impl query_name {
    pub fn new() -> query_name {
        query_name {}
    }
}
impl<T> QemuCmd<T> for query_name {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-name".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_kvm {

        }
impl query_kvm {
    pub fn new() -> query_kvm {
        query_kvm {}
    }
}
impl<T> QemuCmd<T> for query_kvm {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-kvm".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_status {

        }
impl query_status {
    pub fn new() -> query_status {
        query_status {}
    }
}
impl<T> QemuCmd<T> for query_status {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-status".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_uuid {

        }
impl query_uuid {
    pub fn new() -> query_uuid {
        query_uuid {}
    }
}
impl<T> QemuCmd<T> for query_uuid {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-uuid".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_chardev {

        }
impl query_chardev {
    pub fn new() -> query_chardev {
        query_chardev {}
    }
}
impl<T> QemuCmd<T> for query_chardev {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-chardev".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_chardev_backends {

        }
impl query_chardev_backends {
    pub fn new() -> query_chardev_backends {
        query_chardev_backends {}
    }
}
impl<T> QemuCmd<T> for query_chardev_backends {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-chardev-backends".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for ringbuf_write {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "ringbuf-write".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["ringbuf-write"]["arguments"]["format"] = self.format.clone().into();
        to_json["ringbuf-write"]["arguments"]["data"] = self.data.clone().into();
        to_json["ringbuf-write"]["arguments"]["device"] = self.device.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for ringbuf_read {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "ringbuf-read".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["ringbuf-read"]["arguments"]["format"] = self.format.clone().into();
        to_json["ringbuf-read"]["arguments"]["device"] = self.device.clone().into();
        to_json["ringbuf-read"]["arguments"]["size"] = self.size.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_events {

        }
impl query_events {
    pub fn new() -> query_events {
        query_events {}
    }
}
impl<T> QemuCmd<T> for query_events {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-events".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_migrate {

        }
impl query_migrate {
    pub fn new() -> query_migrate {
        query_migrate {}
    }
}
impl<T> QemuCmd<T> for query_migrate {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-migrate".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct migrate_set_capabilities {
    pub capabilities: Vec<String>,
}
impl migrate_set_capabilities {
    pub fn new(capabilities: Vec<String>) -> migrate_set_capabilities {
        migrate_set_capabilities { capabilities: capabilities }
    }
}
impl<T> QemuCmd<T> for migrate_set_capabilities {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate-set-capabilities".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["migrate-set-capabilities"]["arguments"]["capabilities"] =
            self.capabilities.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_migrate_capabilities {

        }
impl query_migrate_capabilities {
    pub fn new() -> query_migrate_capabilities {
        query_migrate_capabilities {}
    }
}
impl<T> QemuCmd<T> for query_migrate_capabilities {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-migrate-capabilities".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for migrate_set_parameters {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate-set-parameters".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["migrate-set-parameters"]["arguments"]["compress_level"] =
            self.compress_level.clone().into();
        to_json["migrate-set-parameters"]["arguments"]["compress_threads"] =
            self.compress_threads.clone().into();
        to_json["migrate-set-parameters"]["arguments"]["decompress_threads"] =
            self.decompress_threads.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_migrate_parameters {

        }
impl query_migrate_parameters {
    pub fn new() -> query_migrate_parameters {
        query_migrate_parameters {}
    }
}
impl<T> QemuCmd<T> for query_migrate_parameters {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-migrate-parameters".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_mice {

        }
impl query_mice {
    pub fn new() -> query_mice {
        query_mice {}
    }
}
impl<T> QemuCmd<T> for query_mice {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-mice".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_cpus {

        }
impl query_cpus {
    pub fn new() -> query_cpus {
        query_cpus {}
    }
}
impl<T> QemuCmd<T> for query_cpus {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-cpus".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_iothreads {

        }
impl query_iothreads {
    pub fn new() -> query_iothreads {
        query_iothreads {}
    }
}
impl<T> QemuCmd<T> for query_iothreads {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-iothreads".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_vnc {

        }
impl query_vnc {
    pub fn new() -> query_vnc {
        query_vnc {}
    }
}
impl<T> QemuCmd<T> for query_vnc {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-vnc".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_vnc_servers {

        }
impl query_vnc_servers {
    pub fn new() -> query_vnc_servers {
        query_vnc_servers {}
    }
}
impl<T> QemuCmd<T> for query_vnc_servers {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-vnc-servers".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_spice {

        }
impl query_spice {
    pub fn new() -> query_spice {
        query_spice {}
    }
}
impl<T> QemuCmd<T> for query_spice {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-spice".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_balloon {

        }
impl query_balloon {
    pub fn new() -> query_balloon {
        query_balloon {}
    }
}
impl<T> QemuCmd<T> for query_balloon {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-balloon".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_pci {

        }
impl query_pci {
    pub fn new() -> query_pci {
        query_pci {}
    }
}
impl<T> QemuCmd<T> for query_pci {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-pci".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct quit {

        }
impl quit {
    pub fn new() -> quit {
        quit {}
    }
}
impl<T> QemuCmd<T> for quit {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "quit".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct stop {

        }
impl stop {
    pub fn new() -> stop {
        stop {}
    }
}
impl<T> QemuCmd<T> for stop {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "stop".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct system_reset {

        }
impl system_reset {
    pub fn new() -> system_reset {
        system_reset {}
    }
}
impl<T> QemuCmd<T> for system_reset {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "system_reset".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct system_powerdown {

        }
impl system_powerdown {
    pub fn new() -> system_powerdown {
        system_powerdown {}
    }
}
impl<T> QemuCmd<T> for system_powerdown {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "system_powerdown".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct cpu {
    pub index: f64,
}
impl cpu {
    pub fn new(index: f64) -> cpu {
        cpu { index: index }
    }
}
impl<T> QemuCmd<T> for cpu {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "cpu".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["cpu"]["arguments"]["index"] = self.index.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct cpu_add {
    pub id: f64,
}
impl cpu_add {
    pub fn new(id: f64) -> cpu_add {
        cpu_add { id: id }
    }
}
impl<T> QemuCmd<T> for cpu_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "cpu-add".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["cpu-add"]["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for memsave {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "memsave".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["memsave"]["arguments"]["cpu_index"] = self.cpu_index.clone().into();
        to_json["memsave"]["arguments"]["filename"] = self.filename.clone().into();
        to_json["memsave"]["arguments"]["size"] = self.size.clone().into();
        to_json["memsave"]["arguments"]["val"] = self.val.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for pmemsave {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "pmemsave".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["pmemsave"]["arguments"]["filename"] = self.filename.clone().into();
        to_json["pmemsave"]["arguments"]["size"] = self.size.clone().into();
        to_json["pmemsave"]["arguments"]["val"] = self.val.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct cont {

        }
impl cont {
    pub fn new() -> cont {
        cont {}
    }
}
impl<T> QemuCmd<T> for cont {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "cont".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct system_wakeup {

        }
impl system_wakeup {
    pub fn new() -> system_wakeup {
        system_wakeup {}
    }
}
impl<T> QemuCmd<T> for system_wakeup {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "system_wakeup".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct inject_nmi {

        }
impl inject_nmi {
    pub fn new() -> inject_nmi {
        inject_nmi {}
    }
}
impl<T> QemuCmd<T> for inject_nmi {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "inject-nmi".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for set_link {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "set_link".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["set_link"]["arguments"]["name"] = self.name.clone().into();
        to_json["set_link"]["arguments"]["up"] = self.up.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct balloon {
    pub value: f64,
}
impl balloon {
    pub fn new(value: f64) -> balloon {
        balloon { value: value }
    }
}
impl<T> QemuCmd<T> for balloon {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "balloon".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["balloon"]["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct transaction {
    pub actions: Vec<String>,
}
impl transaction {
    pub fn new(actions: Vec<String>) -> transaction {
        transaction { actions: actions }
    }
}
impl<T> QemuCmd<T> for transaction {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "transaction".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["transaction"]["arguments"]["actions"] = self.actions.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for human_monitor_command {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "human-monitor-command".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["human-monitor-command"]["arguments"]["cpu_index"] = self.cpu_index.clone().into();
        to_json["human-monitor-command"]["arguments"]["command_line"] =
            self.command_line.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct migrate_cancel {

        }
impl migrate_cancel {
    pub fn new() -> migrate_cancel {
        migrate_cancel {}
    }
}
impl<T> QemuCmd<T> for migrate_cancel {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate_cancel".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct migrate_set_downtime {
    pub value: String,
}
impl migrate_set_downtime {
    pub fn new(value: String) -> migrate_set_downtime {
        migrate_set_downtime { value: value }
    }
}
impl<T> QemuCmd<T> for migrate_set_downtime {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate_set_downtime".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["migrate_set_downtime"]["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct migrate_set_speed {
    pub value: f64,
}
impl migrate_set_speed {
    pub fn new(value: f64) -> migrate_set_speed {
        migrate_set_speed { value: value }
    }
}
impl<T> QemuCmd<T> for migrate_set_speed {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate_set_speed".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["migrate_set_speed"]["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct migrate_set_cache_size {
    pub value: f64,
}
impl migrate_set_cache_size {
    pub fn new(value: f64) -> migrate_set_cache_size {
        migrate_set_cache_size { value: value }
    }
}
impl<T> QemuCmd<T> for migrate_set_cache_size {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate-set-cache-size".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["migrate-set-cache-size"]["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_migrate_cache_size {

        }
impl query_migrate_cache_size {
    pub fn new() -> query_migrate_cache_size {
        query_migrate_cache_size {}
    }
}
impl<T> QemuCmd<T> for query_migrate_cache_size {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-migrate-cache-size".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct qom_list {
    pub path: String,
}
impl qom_list {
    pub fn new(path: String) -> qom_list {
        qom_list { path: path }
    }
}
impl<T> QemuCmd<T> for qom_list {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "qom-list".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["qom-list"]["arguments"]["path"] = self.path.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for qom_set {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "qom-set".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["qom-set"]["arguments"]["path"] = self.path.clone().into();
        to_json["qom-set"]["arguments"]["property"] = self.property.clone().into();
        to_json["qom-set"]["arguments"]["value"] = self.value.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for set_password {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "set_password".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["set_password"]["arguments"]["connected"] = self.connected.clone().into();
        to_json["set_password"]["arguments"]["password"] = self.password.clone().into();
        to_json["set_password"]["arguments"]["protocol"] = self.protocol.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for expire_password {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "expire_password".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["expire_password"]["arguments"]["protocol"] = self.protocol.clone().into();
        to_json["expire_password"]["arguments"]["time"] = self.time.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct change_vnc_password {
    pub password: String,
}
impl change_vnc_password {
    pub fn new(password: String) -> change_vnc_password {
        change_vnc_password { password: password }
    }
}
impl<T> QemuCmd<T> for change_vnc_password {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "change-vnc-password".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["change-vnc-password"]["arguments"]["password"] = self.password.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for change {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "change".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["change"]["arguments"]["arg"] = self.arg.clone().into();
        to_json["change"]["arguments"]["device"] = self.device.clone().into();
        to_json["change"]["arguments"]["target"] = self.target.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for qom_list_types {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "qom-list-types".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["qom-list-types"]["arguments"]["qemu_abstract"] = self.qemu_abstract.clone().into();
        to_json["qom-list-types"]["arguments"]["implements"] = self.implements.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct device_list_properties {
    pub typename: String,
}
impl device_list_properties {
    pub fn new(typename: String) -> device_list_properties {
        device_list_properties { typename: typename }
    }
}
impl<T> QemuCmd<T> for device_list_properties {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "device-list-properties".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["device-list-properties"]["arguments"]["typename"] = self.typename.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for migrate {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["migrate"]["arguments"]["blk"] = self.blk.clone().into();
        to_json["migrate"]["arguments"]["detach"] = self.detach.clone().into();
        to_json["migrate"]["arguments"]["inc"] = self.inc.clone().into();
        to_json["migrate"]["arguments"]["uri"] = self.uri.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct migrate_incoming {
    pub uri: String,
}
impl migrate_incoming {
    pub fn new(uri: String) -> migrate_incoming {
        migrate_incoming { uri: uri }
    }
}
impl<T> QemuCmd<T> for migrate_incoming {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "migrate-incoming".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["migrate-incoming"]["arguments"]["uri"] = self.uri.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct xen_save_devices_state {
    pub filename: String,
}
impl xen_save_devices_state {
    pub fn new(filename: String) -> xen_save_devices_state {
        xen_save_devices_state { filename: filename }
    }
}
impl<T> QemuCmd<T> for xen_save_devices_state {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "xen-save-devices-state".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["xen-save-devices-state"]["arguments"]["filename"] = self.filename.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct xen_set_global_dirty_log {
    pub enable: bool,
}
impl xen_set_global_dirty_log {
    pub fn new(enable: bool) -> xen_set_global_dirty_log {
        xen_set_global_dirty_log { enable: enable }
    }
}
impl<T> QemuCmd<T> for xen_set_global_dirty_log {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "xen-set-global-dirty-log".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["xen-set-global-dirty-log"]["arguments"]["enable"] = self.enable.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct device_del {
    pub id: String,
}
impl device_del {
    pub fn new(id: String) -> device_del {
        device_del { id: id }
    }
}
impl<T> QemuCmd<T> for device_del {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "device_del".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["device_del"]["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for dump_guest_memory {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "dump-guest-memory".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["dump-guest-memory"]["arguments"]["begin"] = self.begin.clone().into();
        to_json["dump-guest-memory"]["arguments"]["format"] = self.format.clone().into();
        to_json["dump-guest-memory"]["arguments"]["length"] = self.length.clone().into();
        to_json["dump-guest-memory"]["arguments"]["paging"] = self.paging.clone().into();
        to_json["dump-guest-memory"]["arguments"]["protocol"] = self.protocol.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_dump_guest_memory_capability {

        }
impl query_dump_guest_memory_capability {
    pub fn new() -> query_dump_guest_memory_capability {
        query_dump_guest_memory_capability {}
    }
}
impl<T> QemuCmd<T> for query_dump_guest_memory_capability {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-dump-guest-memory-capability".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for netdev_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "netdev_add".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["netdev_add"]["arguments"]["props"] = self.props.clone().into();
        to_json["netdev_add"]["arguments"]["id"] = self.id.clone().into();
        to_json["netdev_add"]["arguments"]["qemu_type"] = self.qemu_type.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct netdev_del {
    pub id: String,
}
impl netdev_del {
    pub fn new(id: String) -> netdev_del {
        netdev_del { id: id }
    }
}
impl<T> QemuCmd<T> for netdev_del {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "netdev_del".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["netdev_del"]["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for object_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "object-add".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["object-add"]["arguments"]["props"] = self.props.clone().into();
        to_json["object-add"]["arguments"]["id"] = self.id.clone().into();
        to_json["object-add"]["arguments"]["qom_type"] = self.qom_type.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct object_del {
    pub id: String,
}
impl object_del {
    pub fn new(id: String) -> object_del {
        object_del { id: id }
    }
}
impl<T> QemuCmd<T> for object_del {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "object-del".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["object-del"]["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct getfd {
    pub fdname: String,
}
impl getfd {
    pub fn new(fdname: String) -> getfd {
        getfd { fdname: fdname }
    }
}
impl<T> QemuCmd<T> for getfd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "getfd".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["getfd"]["arguments"]["fdname"] = self.fdname.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct closefd {
    pub fdname: String,
}
impl closefd {
    pub fn new(fdname: String) -> closefd {
        closefd { fdname: fdname }
    }
}
impl<T> QemuCmd<T> for closefd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "closefd".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["closefd"]["arguments"]["fdname"] = self.fdname.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_machines {

        }
impl query_machines {
    pub fn new() -> query_machines {
        query_machines {}
    }
}
impl<T> QemuCmd<T> for query_machines {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-machines".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_cpu_definitions {

        }
impl query_cpu_definitions {
    pub fn new() -> query_cpu_definitions {
        query_cpu_definitions {}
    }
}
impl<T> QemuCmd<T> for query_cpu_definitions {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-cpu-definitions".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for add_fd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "add-fd".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["add-fd"]["arguments"]["fdset_id"] = self.fdset_id.clone().into();
        to_json["add-fd"]["arguments"]["opaque"] = self.opaque.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for remove_fd {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "remove-fd".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["remove-fd"]["arguments"]["fd"] = self.fd.clone().into();
        to_json["remove-fd"]["arguments"]["fdset_id"] = self.fdset_id.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_fdsets {

        }
impl query_fdsets {
    pub fn new() -> query_fdsets {
        query_fdsets {}
    }
}
impl<T> QemuCmd<T> for query_fdsets {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-fdsets".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_target {

        }
impl query_target {
    pub fn new() -> query_target {
        query_target {}
    }
}
impl<T> QemuCmd<T> for query_target {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-target".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for send_key {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "send-key".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["send-key"]["arguments"]["hold_time"] = self.hold_time.clone().into();
        to_json["send-key"]["arguments"]["keys"] = self.keys.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct screendump {
    pub filename: String,
}
impl screendump {
    pub fn new(filename: String) -> screendump {
        screendump { filename: filename }
    }
}
impl<T> QemuCmd<T> for screendump {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "screendump".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["screendump"]["arguments"]["filename"] = self.filename.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for chardev_add {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "chardev-add".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["chardev-add"]["arguments"]["backend"] = self.backend.clone().into();
        to_json["chardev-add"]["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct chardev_remove {
    pub id: String,
}
impl chardev_remove {
    pub fn new(id: String) -> chardev_remove {
        chardev_remove { id: id }
    }
}
impl<T> QemuCmd<T> for chardev_remove {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "chardev-remove".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["chardev-remove"]["arguments"]["id"] = self.id.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_tpm_models {

        }
impl query_tpm_models {
    pub fn new() -> query_tpm_models {
        query_tpm_models {}
    }
}
impl<T> QemuCmd<T> for query_tpm_models {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-tpm-models".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_tpm_types {

        }
impl query_tpm_types {
    pub fn new() -> query_tpm_types {
        query_tpm_types {}
    }
}
impl<T> QemuCmd<T> for query_tpm_types {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-tpm-types".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_tpm {

        }
impl query_tpm {
    pub fn new() -> query_tpm {
        query_tpm {}
    }
}
impl<T> QemuCmd<T> for query_tpm {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-tpm".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_command_line_options {
    pub option: String,
}
impl query_command_line_options {
    pub fn new(option: String) -> query_command_line_options {
        query_command_line_options { option: option }
    }
}
impl<T> QemuCmd<T> for query_command_line_options {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-command-line-options".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["query-command-line-options"]["arguments"]["option"] = self.option.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_rx_filter {
    pub name: String,
}
impl query_rx_filter {
    pub fn new(name: String) -> query_rx_filter {
        query_rx_filter { name: name }
    }
}
impl<T> QemuCmd<T> for query_rx_filter {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-rx-filter".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["query-rx-filter"]["arguments"]["name"] = self.name.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

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
impl<T> QemuCmd<T> for x_input_send_event {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "x-input-send-event".into();
        to_json["arguments"] = json::JsonValue::new_object();
        to_json["x-input-send-event"]["arguments"]["console"] = self.console.clone().into();
        to_json["x-input-send-event"]["arguments"]["events"] = self.events.clone().into();
        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_memdev {

        }
impl query_memdev {
    pub fn new() -> query_memdev {
        query_memdev {}
    }
}
impl<T> QemuCmd<T> for query_memdev {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-memdev".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_memory_devices {

        }
impl query_memory_devices {
    pub fn new() -> query_memory_devices {
        query_memory_devices {}
    }
}
impl<T> QemuCmd<T> for query_memory_devices {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-memory-devices".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct query_acpi_ospm_status {

        }
impl query_acpi_ospm_status {
    pub fn new() -> query_acpi_ospm_status {
        query_acpi_ospm_status {}
    }
}
impl<T> QemuCmd<T> for query_acpi_ospm_status {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "query-acpi-ospm-status".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}

#[derive(Debug)]
pub struct rtc_reset_reinjection {

        }
impl rtc_reset_reinjection {
    pub fn new() -> rtc_reset_reinjection {
        rtc_reset_reinjection {}
    }
}
impl<T> QemuCmd<T> for rtc_reset_reinjection {
    fn to_json(&self) -> String {
        let mut to_json = json::JsonValue::new_object();
        to_json["execute"] = "rtc-reset-reinjection".into();
        to_json["arguments"] = json::JsonValue::new_object();

        to_json.dump()
    }

    fn parse_qemu_response(&self, response: &String) -> rustc_json::DecodeResult<T>
        where T: rustc_decodable
    {
        rustc_json::decode(&response)
    }
}
