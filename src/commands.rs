use rustc_serialize::json;
use events::*;
use enums::*;
use structs::*;

#[derive(Debug)]
pub struct blockdev_snapshot_internal_sync {
    execute: String,
}
impl blockdev_snapshot_internal_sync {
    pub fn new() -> blockdev_snapshot_internal_sync {
        blockdev_snapshot_internal_sync { execute: "blockdev_snapshot_internal_sync".to_string() }
    }
}

#[derive(Debug)]
pub struct blockdev_snapshot_delete_internal_sync {
    execute: String,
    pub id: String,
    pub name: String,
    pub device: String,
}
impl blockdev_snapshot_delete_internal_sync {
    pub fn new(id: String, name: String, device: String) -> blockdev_snapshot_delete_internal_sync {
        blockdev_snapshot_delete_internal_sync {
            execute: "blockdev_snapshot_delete_internal_sync".to_string(),
            id: id,
            name: name,
            device: device,
        }
    }

    pub fn parse_qemu_response(response: &String) -> Result<SnapshotInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct eject {
    execute: String,
    pub force: String,
    pub device: String,
}
impl eject {
    pub fn new(force: String, device: String) -> eject {
        eject {
            execute: "eject".to_string(),
            force: force,
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct nbd_server_start {
    execute: String,
    pub addr: String,
}
impl nbd_server_start {
    pub fn new(addr: String) -> nbd_server_start {
        nbd_server_start {
            execute: "nbd_server_start".to_string(),
            addr: addr,
        }
    }
}

#[derive(Debug)]
pub struct nbd_server_add {
    execute: String,
    pub writable: String,
    pub device: String,
}
impl nbd_server_add {
    pub fn new(writable: String, device: String) -> nbd_server_add {
        nbd_server_add {
            execute: "nbd_server_add".to_string(),
            writable: writable,
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct nbd_server_stop {
    execute: String,
}
impl nbd_server_stop {
    pub fn new() -> nbd_server_stop {
        nbd_server_stop { execute: "nbd_server_stop".to_string() }
    }
}

#[derive(Debug)]
pub struct query_block {
    execute: String,
}
impl query_block {
    pub fn new() -> query_block {
        query_block { execute: "query_block".to_string() }
    }
}

#[derive(Debug)]
pub struct query_blockstats {
    execute: String,
    pub query_nodes: String,
}
impl query_blockstats {
    pub fn new(query_nodes: String) -> query_blockstats {
        query_blockstats {
            execute: "query_blockstats".to_string(),
            query_nodes: query_nodes,
        }
    }
}

#[derive(Debug)]
pub struct query_block_jobs {
    execute: String,
}
impl query_block_jobs {
    pub fn new() -> query_block_jobs {
        query_block_jobs { execute: "query_block_jobs".to_string() }
    }
}

#[derive(Debug)]
pub struct block_passwd {
    execute: String,
    pub device: String,
    pub node_name: String,
    pub password: String,
}
impl block_passwd {
    pub fn new(device: String, node_name: String, password: String) -> block_passwd {
        block_passwd {
            execute: "block_passwd".to_string(),
            device: device,
            node_name: node_name,
            password: password,
        }
    }
}

#[derive(Debug)]
pub struct block_resize {
    execute: String,
    pub device: String,
    pub node_name: String,
    pub size: String,
}
impl block_resize {
    pub fn new(device: String, node_name: String, size: String) -> block_resize {
        block_resize {
            execute: "block_resize".to_string(),
            device: device,
            node_name: node_name,
            size: size,
        }
    }
}

#[derive(Debug)]
pub struct blockdev_snapshot_sync {
    execute: String,
}
impl blockdev_snapshot_sync {
    pub fn new() -> blockdev_snapshot_sync {
        blockdev_snapshot_sync { execute: "blockdev_snapshot_sync".to_string() }
    }
}

#[derive(Debug)]
pub struct change_backing_file {
    execute: String,
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
            execute: "change_backing_file".to_string(),
            backing_file: backing_file,
            device: device,
            image_node_name: image_node_name,
        }
    }
}

#[derive(Debug)]
pub struct block_commit {
    execute: String,
    pub backing_file: String,
    pub base: String,
    pub speed: String,
    pub top: String,
    pub device: String,
}
impl block_commit {
    pub fn new(backing_file: String,
               base: String,
               speed: String,
               top: String,
               device: String)
               -> block_commit {
        block_commit {
            execute: "block_commit".to_string(),
            backing_file: backing_file,
            base: base,
            speed: speed,
            top: top,
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct drive_backup {
    execute: String,
}
impl drive_backup {
    pub fn new() -> drive_backup {
        drive_backup { execute: "drive_backup".to_string() }
    }
}

#[derive(Debug)]
pub struct blockdev_backup {
    execute: String,
}
impl blockdev_backup {
    pub fn new() -> blockdev_backup {
        blockdev_backup { execute: "blockdev_backup".to_string() }
    }
}

#[derive(Debug)]
pub struct query_named_block_nodes {
    execute: String,
}
impl query_named_block_nodes {
    pub fn new() -> query_named_block_nodes {
        query_named_block_nodes { execute: "query_named_block_nodes".to_string() }
    }
}

#[derive(Debug)]
pub struct drive_mirror {
    execute: String,
    pub buf_size: String,
    pub format: String,
    pub granularity: String,
    pub mode: String,
    pub node_name: String,
    pub on_source_error: String,
    pub on_target_error: String,
    pub replaces: String,
    pub speed: String,
    pub device: String,
    pub sync: String,
    pub target: String,
}
impl drive_mirror {
    pub fn new(buf_size: String,
               format: String,
               granularity: String,
               mode: String,
               node_name: String,
               on_source_error: String,
               on_target_error: String,
               replaces: String,
               speed: String,
               device: String,
               sync: String,
               target: String)
               -> drive_mirror {
        drive_mirror {
            execute: "drive_mirror".to_string(),
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

#[derive(Debug)]
pub struct block_dirty_bitmap_add {
    execute: String,
}
impl block_dirty_bitmap_add {
    pub fn new() -> block_dirty_bitmap_add {
        block_dirty_bitmap_add { execute: "block_dirty_bitmap_add".to_string() }
    }
}

#[derive(Debug)]
pub struct block_dirty_bitmap_remove {
    execute: String,
}
impl block_dirty_bitmap_remove {
    pub fn new() -> block_dirty_bitmap_remove {
        block_dirty_bitmap_remove { execute: "block_dirty_bitmap_remove".to_string() }
    }
}

#[derive(Debug)]
pub struct block_dirty_bitmap_clear {
    execute: String,
}
impl block_dirty_bitmap_clear {
    pub fn new() -> block_dirty_bitmap_clear {
        block_dirty_bitmap_clear { execute: "block_dirty_bitmap_clear".to_string() }
    }
}

#[derive(Debug)]
pub struct block_set_io_throttle {
    execute: String,
    pub bps_max: String,
    pub bps_rd_max: String,
    pub bps_wr_max: String,
    pub iops_max: String,
    pub iops_rd_max: String,
    pub iops_size: String,
    pub iops_wr_max: String,
    pub bps: String,
    pub bps_rd: String,
    pub bps_wr: String,
    pub device: String,
    pub iops: String,
    pub iops_rd: String,
    pub iops_wr: String,
}
impl block_set_io_throttle {
    pub fn new(bps_max: String,
               bps_rd_max: String,
               bps_wr_max: String,
               iops_max: String,
               iops_rd_max: String,
               iops_size: String,
               iops_wr_max: String,
               bps: String,
               bps_rd: String,
               bps_wr: String,
               device: String,
               iops: String,
               iops_rd: String,
               iops_wr: String)
               -> block_set_io_throttle {
        block_set_io_throttle {
            execute: "block_set_io_throttle".to_string(),
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

#[derive(Debug)]
pub struct block_stream {
    execute: String,
    pub backing_file: String,
    pub base: String,
    pub on_error: String,
    pub speed: String,
    pub device: String,
}
impl block_stream {
    pub fn new(backing_file: String,
               base: String,
               on_error: String,
               speed: String,
               device: String)
               -> block_stream {
        block_stream {
            execute: "block_stream".to_string(),
            backing_file: backing_file,
            base: base,
            on_error: on_error,
            speed: speed,
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct block_job_set_speed {
    execute: String,
    pub device: String,
    pub speed: String,
}
impl block_job_set_speed {
    pub fn new(device: String, speed: String) -> block_job_set_speed {
        block_job_set_speed {
            execute: "block_job_set_speed".to_string(),
            device: device,
            speed: speed,
        }
    }
}

#[derive(Debug)]
pub struct block_job_cancel {
    execute: String,
    pub force: String,
    pub device: String,
}
impl block_job_cancel {
    pub fn new(force: String, device: String) -> block_job_cancel {
        block_job_cancel {
            execute: "block_job_cancel".to_string(),
            force: force,
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct block_job_pause {
    execute: String,
    pub device: String,
}
impl block_job_pause {
    pub fn new(device: String) -> block_job_pause {
        block_job_pause {
            execute: "block_job_pause".to_string(),
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct block_job_resume {
    execute: String,
    pub device: String,
}
impl block_job_resume {
    pub fn new(device: String) -> block_job_resume {
        block_job_resume {
            execute: "block_job_resume".to_string(),
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct block_job_complete {
    execute: String,
    pub device: String,
}
impl block_job_complete {
    pub fn new(device: String) -> block_job_complete {
        block_job_complete {
            execute: "block_job_complete".to_string(),
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct blockdev_add {
    execute: String,
    pub options: String,
}
impl blockdev_add {
    pub fn new(options: String) -> blockdev_add {
        blockdev_add {
            execute: "blockdev_add".to_string(),
            options: options,
        }
    }
}

#[derive(Debug)]
pub struct block_set_write_threshold {
    execute: String,
    pub node_name: String,
    pub write_threshold: String,
}
impl block_set_write_threshold {
    pub fn new(node_name: String, write_threshold: String) -> block_set_write_threshold {
        block_set_write_threshold {
            execute: "block_set_write_threshold".to_string(),
            node_name: node_name,
            write_threshold: write_threshold,
        }
    }
}

#[derive(Debug)]
pub struct query_version {
    execute: String,
}
impl query_version {
    pub fn new() -> query_version {
        query_version { execute: "query_version".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<VersionInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_commands {
    execute: String,
}
impl query_commands {
    pub fn new() -> query_commands {
        query_commands { execute: "query_commands".to_string() }
    }
}

#[derive(Debug)]
pub struct trace_event_get_state {
    execute: String,
    pub name: String,
}
impl trace_event_get_state {
    pub fn new(name: String) -> trace_event_get_state {
        trace_event_get_state {
            execute: "trace_event_get_state".to_string(),
            name: name,
        }
    }
}

#[derive(Debug)]
pub struct trace_event_set_state {
    execute: String,
    pub ignore_unavailable: String,
    pub enable: String,
    pub name: String,
}
impl trace_event_set_state {
    pub fn new(ignore_unavailable: String, enable: String, name: String) -> trace_event_set_state {
        trace_event_set_state {
            execute: "trace_event_set_state".to_string(),
            ignore_unavailable: ignore_unavailable,
            enable: enable,
            name: name,
        }
    }
}

#[derive(Debug)]
pub struct add_client {
    execute: String,
    pub skipauth: String,
    pub tls: String,
    pub fdname: String,
    pub protocol: String,
}
impl add_client {
    pub fn new(skipauth: String, tls: String, fdname: String, protocol: String) -> add_client {
        add_client {
            execute: "add_client".to_string(),
            skipauth: skipauth,
            tls: tls,
            fdname: fdname,
            protocol: protocol,
        }
    }
}

#[derive(Debug)]
pub struct query_name {
    execute: String,
}
impl query_name {
    pub fn new() -> query_name {
        query_name { execute: "query_name".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<NameInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_kvm {
    execute: String,
}
impl query_kvm {
    pub fn new() -> query_kvm {
        query_kvm { execute: "query_kvm".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<KvmInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_status {
    execute: String,
}
impl query_status {
    pub fn new() -> query_status {
        query_status { execute: "query_status".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<StatusInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_uuid {
    execute: String,
}
impl query_uuid {
    pub fn new() -> query_uuid {
        query_uuid { execute: "query_uuid".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<UuidInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_chardev {
    execute: String,
}
impl query_chardev {
    pub fn new() -> query_chardev {
        query_chardev { execute: "query_chardev".to_string() }
    }
}

#[derive(Debug)]
pub struct query_chardev_backends {
    execute: String,
}
impl query_chardev_backends {
    pub fn new() -> query_chardev_backends {
        query_chardev_backends { execute: "query_chardev_backends".to_string() }
    }
}

#[derive(Debug)]
pub struct ringbuf_write {
    execute: String,
    pub format: String,
    pub data: String,
    pub device: String,
}
impl ringbuf_write {
    pub fn new(format: String, data: String, device: String) -> ringbuf_write {
        ringbuf_write {
            execute: "ringbuf_write".to_string(),
            format: format,
            data: data,
            device: device,
        }
    }
}

#[derive(Debug)]
pub struct ringbuf_read {
    execute: String,
    pub format: String,
    pub device: String,
    pub size: String,
}
impl ringbuf_read {
    pub fn new(format: String, device: String, size: String) -> ringbuf_read {
        ringbuf_read {
            execute: "ringbuf_read".to_string(),
            format: format,
            device: device,
            size: size,
        }
    }

    pub fn parse_qemu_response(response: &String) -> Result<String, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_events {
    execute: String,
}
impl query_events {
    pub fn new() -> query_events {
        query_events { execute: "query_events".to_string() }
    }
}

#[derive(Debug)]
pub struct query_migrate {
    execute: String,
}
impl query_migrate {
    pub fn new() -> query_migrate {
        query_migrate { execute: "query_migrate".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<MigrationInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct migrate_set_capabilities {
    execute: String,
    pub capabilities: Vec<String>,
}
impl migrate_set_capabilities {
    pub fn new(capabilities: Vec<String>) -> migrate_set_capabilities {
        migrate_set_capabilities {
            execute: "migrate_set_capabilities".to_string(),
            capabilities: capabilities,
        }
    }
}

#[derive(Debug)]
pub struct query_migrate_capabilities {
    execute: String,
}
impl query_migrate_capabilities {
    pub fn new() -> query_migrate_capabilities {
        query_migrate_capabilities { execute: "query_migrate_capabilities".to_string() }
    }
}

#[derive(Debug)]
pub struct migrate_set_parameters {
    execute: String,
    pub compress_level: String,
    pub compress_threads: String,
    pub decompress_threads: String,
}
impl migrate_set_parameters {
    pub fn new(compress_level: String,
               compress_threads: String,
               decompress_threads: String)
               -> migrate_set_parameters {
        migrate_set_parameters {
            execute: "migrate_set_parameters".to_string(),
            compress_level: compress_level,
            compress_threads: compress_threads,
            decompress_threads: decompress_threads,
        }
    }
}

#[derive(Debug)]
pub struct query_migrate_parameters {
    execute: String,
}
impl query_migrate_parameters {
    pub fn new() -> query_migrate_parameters {
        query_migrate_parameters { execute: "query_migrate_parameters".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<MigrationParameters, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_mice {
    execute: String,
}
impl query_mice {
    pub fn new() -> query_mice {
        query_mice { execute: "query_mice".to_string() }
    }
}

#[derive(Debug)]
pub struct query_cpus {
    execute: String,
}
impl query_cpus {
    pub fn new() -> query_cpus {
        query_cpus { execute: "query_cpus".to_string() }
    }
}

#[derive(Debug)]
pub struct query_iothreads {
    execute: String,
}
impl query_iothreads {
    pub fn new() -> query_iothreads {
        query_iothreads { execute: "query_iothreads".to_string() }
    }
}

#[derive(Debug)]
pub struct query_vnc {
    execute: String,
}
impl query_vnc {
    pub fn new() -> query_vnc {
        query_vnc { execute: "query_vnc".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<VncInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_vnc_servers {
    execute: String,
}
impl query_vnc_servers {
    pub fn new() -> query_vnc_servers {
        query_vnc_servers { execute: "query_vnc_servers".to_string() }
    }
}

#[derive(Debug)]
pub struct query_spice {
    execute: String,
}
impl query_spice {
    pub fn new() -> query_spice {
        query_spice { execute: "query_spice".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<SpiceInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_balloon {
    execute: String,
}
impl query_balloon {
    pub fn new() -> query_balloon {
        query_balloon { execute: "query_balloon".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<BalloonInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct query_pci {
    execute: String,
}
impl query_pci {
    pub fn new() -> query_pci {
        query_pci { execute: "query_pci".to_string() }
    }
}

#[derive(Debug)]
pub struct quit {
    execute: String,
}
impl quit {
    pub fn new() -> quit {
        quit { execute: "quit".to_string() }
    }
}

#[derive(Debug)]
pub struct stop {
    execute: String,
}
impl stop {
    pub fn new() -> stop {
        stop { execute: "stop".to_string() }
    }
}

#[derive(Debug)]
pub struct system_reset {
    execute: String,
}
impl system_reset {
    pub fn new() -> system_reset {
        system_reset { execute: "system_reset".to_string() }
    }
}

#[derive(Debug)]
pub struct system_powerdown {
    execute: String,
}
impl system_powerdown {
    pub fn new() -> system_powerdown {
        system_powerdown { execute: "system_powerdown".to_string() }
    }
}

#[derive(Debug)]
pub struct cpu {
    execute: String,
    pub index: String,
}
impl cpu {
    pub fn new(index: String) -> cpu {
        cpu {
            execute: "cpu".to_string(),
            index: index,
        }
    }
}

#[derive(Debug)]
pub struct cpu_add {
    execute: String,
    pub id: String,
}
impl cpu_add {
    pub fn new(id: String) -> cpu_add {
        cpu_add {
            execute: "cpu_add".to_string(),
            id: id,
        }
    }
}

#[derive(Debug)]
pub struct memsave {
    execute: String,
    pub cpu_index: String,
    pub filename: String,
    pub size: String,
    pub val: String,
}
impl memsave {
    pub fn new(cpu_index: String, filename: String, size: String, val: String) -> memsave {
        memsave {
            execute: "memsave".to_string(),
            cpu_index: cpu_index,
            filename: filename,
            size: size,
            val: val,
        }
    }
}

#[derive(Debug)]
pub struct pmemsave {
    execute: String,
    pub filename: String,
    pub size: String,
    pub val: String,
}
impl pmemsave {
    pub fn new(filename: String, size: String, val: String) -> pmemsave {
        pmemsave {
            execute: "pmemsave".to_string(),
            filename: filename,
            size: size,
            val: val,
        }
    }
}

#[derive(Debug)]
pub struct cont {
    execute: String,
}
impl cont {
    pub fn new() -> cont {
        cont { execute: "cont".to_string() }
    }
}

#[derive(Debug)]
pub struct system_wakeup {
    execute: String,
}
impl system_wakeup {
    pub fn new() -> system_wakeup {
        system_wakeup { execute: "system_wakeup".to_string() }
    }
}

#[derive(Debug)]
pub struct inject_nmi {
    execute: String,
}
impl inject_nmi {
    pub fn new() -> inject_nmi {
        inject_nmi { execute: "inject_nmi".to_string() }
    }
}

#[derive(Debug)]
pub struct set_link {
    execute: String,
    pub name: String,
    pub up: String,
}
impl set_link {
    pub fn new(name: String, up: String) -> set_link {
        set_link {
            execute: "set_link".to_string(),
            name: name,
            up: up,
        }
    }
}

#[derive(Debug)]
pub struct balloon {
    execute: String,
    pub value: String,
}
impl balloon {
    pub fn new(value: String) -> balloon {
        balloon {
            execute: "balloon".to_string(),
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct transaction {
    execute: String,
    pub actions: Vec<String>,
}
impl transaction {
    pub fn new(actions: Vec<String>) -> transaction {
        transaction {
            execute: "transaction".to_string(),
            actions: actions,
        }
    }
}

#[derive(Debug)]
pub struct human_monitor_command {
    execute: String,
    pub cpu_index: String,
    pub command_line: String,
}
impl human_monitor_command {
    pub fn new(cpu_index: String, command_line: String) -> human_monitor_command {
        human_monitor_command {
            execute: "human_monitor_command".to_string(),
            cpu_index: cpu_index,
            command_line: command_line,
        }
    }

    pub fn parse_qemu_response(response: &String) -> Result<String, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct migrate_cancel {
    execute: String,
}
impl migrate_cancel {
    pub fn new() -> migrate_cancel {
        migrate_cancel { execute: "migrate_cancel".to_string() }
    }
}

#[derive(Debug)]
pub struct migrate_set_downtime {
    execute: String,
    pub value: String,
}
impl migrate_set_downtime {
    pub fn new(value: String) -> migrate_set_downtime {
        migrate_set_downtime {
            execute: "migrate_set_downtime".to_string(),
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct migrate_set_speed {
    execute: String,
    pub value: String,
}
impl migrate_set_speed {
    pub fn new(value: String) -> migrate_set_speed {
        migrate_set_speed {
            execute: "migrate_set_speed".to_string(),
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct migrate_set_cache_size {
    execute: String,
    pub value: String,
}
impl migrate_set_cache_size {
    pub fn new(value: String) -> migrate_set_cache_size {
        migrate_set_cache_size {
            execute: "migrate_set_cache_size".to_string(),
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct query_migrate_cache_size {
    execute: String,
}
impl query_migrate_cache_size {
    pub fn new() -> query_migrate_cache_size {
        query_migrate_cache_size { execute: "query_migrate_cache_size".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<i64, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct qom_list {
    execute: String,
    pub path: String,
}
impl qom_list {
    pub fn new(path: String) -> qom_list {
        qom_list {
            execute: "qom_list".to_string(),
            path: path,
        }
    }
}

#[derive(Debug)]
pub struct qom_set {
    execute: String,
    pub path: String,
    pub property: String,
    pub value: String,
    gen: bool,
}
impl qom_set {
    pub fn new(path: String, property: String, value: String, gen: bool) -> qom_set {
        qom_set {
            execute: "qom_set".to_string(),
            path: path,
            property: property,
            value: value,
            gen: gen,
        }
    }
}

#[derive(Debug)]
pub struct set_password {
    execute: String,
    pub connected: String,
    pub password: String,
    pub protocol: String,
}
impl set_password {
    pub fn new(connected: String, password: String, protocol: String) -> set_password {
        set_password {
            execute: "set_password".to_string(),
            connected: connected,
            password: password,
            protocol: protocol,
        }
    }
}

#[derive(Debug)]
pub struct expire_password {
    execute: String,
    pub protocol: String,
    pub time: String,
}
impl expire_password {
    pub fn new(protocol: String, time: String) -> expire_password {
        expire_password {
            execute: "expire_password".to_string(),
            protocol: protocol,
            time: time,
        }
    }
}

#[derive(Debug)]
pub struct change_vnc_password {
    execute: String,
    pub password: String,
}
impl change_vnc_password {
    pub fn new(password: String) -> change_vnc_password {
        change_vnc_password {
            execute: "change_vnc_password".to_string(),
            password: password,
        }
    }
}

#[derive(Debug)]
pub struct change {
    execute: String,
    pub arg: String,
    pub device: String,
    pub target: String,
}
impl change {
    pub fn new(arg: String, device: String, target: String) -> change {
        change {
            execute: "change".to_string(),
            arg: arg,
            device: device,
            target: target,
        }
    }
}

#[derive(Debug)]
pub struct qom_list_types {
    execute: String,
    pub qemu_abstract: String,
    pub implements: String,
}
impl qom_list_types {
    pub fn new(qemu_abstract: String, implements: String) -> qom_list_types {
        qom_list_types {
            execute: "qom_list_types".to_string(),
            qemu_abstract: qemu_abstract,
            implements: implements,
        }
    }
}

#[derive(Debug)]
pub struct device_list_properties {
    execute: String,
    pub typename: String,
}
impl device_list_properties {
    pub fn new(typename: String) -> device_list_properties {
        device_list_properties {
            execute: "device_list_properties".to_string(),
            typename: typename,
        }
    }
}

#[derive(Debug)]
pub struct migrate {
    execute: String,
    pub blk: String,
    pub detach: String,
    pub inc: String,
    pub uri: String,
}
impl migrate {
    pub fn new(blk: String, detach: String, inc: String, uri: String) -> migrate {
        migrate {
            execute: "migrate".to_string(),
            blk: blk,
            detach: detach,
            inc: inc,
            uri: uri,
        }
    }
}

#[derive(Debug)]
pub struct migrate_incoming {
    execute: String,
    pub uri: String,
}
impl migrate_incoming {
    pub fn new(uri: String) -> migrate_incoming {
        migrate_incoming {
            execute: "migrate_incoming".to_string(),
            uri: uri,
        }
    }
}

#[derive(Debug)]
pub struct xen_save_devices_state {
    execute: String,
    pub filename: String,
}
impl xen_save_devices_state {
    pub fn new(filename: String) -> xen_save_devices_state {
        xen_save_devices_state {
            execute: "xen_save_devices_state".to_string(),
            filename: filename,
        }
    }
}

#[derive(Debug)]
pub struct xen_set_global_dirty_log {
    execute: String,
    pub enable: String,
}
impl xen_set_global_dirty_log {
    pub fn new(enable: String) -> xen_set_global_dirty_log {
        xen_set_global_dirty_log {
            execute: "xen_set_global_dirty_log".to_string(),
            enable: enable,
        }
    }
}

#[derive(Debug)]
pub struct device_del {
    execute: String,
    pub id: String,
}
impl device_del {
    pub fn new(id: String) -> device_del {
        device_del {
            execute: "device_del".to_string(),
            id: id,
        }
    }
}

#[derive(Debug)]
pub struct dump_guest_memory {
    execute: String,
    pub begin: String,
    pub format: String,
    pub length: String,
    pub paging: String,
    pub protocol: String,
}
impl dump_guest_memory {
    pub fn new(begin: String,
               format: String,
               length: String,
               paging: String,
               protocol: String)
               -> dump_guest_memory {
        dump_guest_memory {
            execute: "dump_guest_memory".to_string(),
            begin: begin,
            format: format,
            length: length,
            paging: paging,
            protocol: protocol,
        }
    }
}

#[derive(Debug)]
pub struct query_dump_guest_memory_capability {
    execute: String,
}
impl query_dump_guest_memory_capability {
    pub fn new() -> query_dump_guest_memory_capability {
        query_dump_guest_memory_capability {
            execute: "query_dump_guest_memory_capability".to_string(),
        }
    }

    pub fn parse_qemu_response(response: &String) -> Result<DumpGuestMemoryCapability, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct netdev_add {
    execute: String,
    pub props: String,
    pub id: String,
    pub qemu_type: String,
    gen: bool,
}
impl netdev_add {
    pub fn new(props: String, id: String, qemu_type: String, gen: bool) -> netdev_add {
        netdev_add {
            execute: "netdev_add".to_string(),
            props: props,
            id: id,
            qemu_type: qemu_type,
            gen: gen,
        }
    }
}

#[derive(Debug)]
pub struct netdev_del {
    execute: String,
    pub id: String,
}
impl netdev_del {
    pub fn new(id: String) -> netdev_del {
        netdev_del {
            execute: "netdev_del".to_string(),
            id: id,
        }
    }
}

#[derive(Debug)]
pub struct object_add {
    execute: String,
    pub props: String,
    pub id: String,
    pub qom_type: String,
    gen: bool,
}
impl object_add {
    pub fn new(props: String, id: String, qom_type: String, gen: bool) -> object_add {
        object_add {
            execute: "object_add".to_string(),
            props: props,
            id: id,
            qom_type: qom_type,
            gen: gen,
        }
    }
}

#[derive(Debug)]
pub struct object_del {
    execute: String,
    pub id: String,
}
impl object_del {
    pub fn new(id: String) -> object_del {
        object_del {
            execute: "object_del".to_string(),
            id: id,
        }
    }
}

#[derive(Debug)]
pub struct getfd {
    execute: String,
    pub fdname: String,
}
impl getfd {
    pub fn new(fdname: String) -> getfd {
        getfd {
            execute: "getfd".to_string(),
            fdname: fdname,
        }
    }
}

#[derive(Debug)]
pub struct closefd {
    execute: String,
    pub fdname: String,
}
impl closefd {
    pub fn new(fdname: String) -> closefd {
        closefd {
            execute: "closefd".to_string(),
            fdname: fdname,
        }
    }
}

#[derive(Debug)]
pub struct query_machines {
    execute: String,
}
impl query_machines {
    pub fn new() -> query_machines {
        query_machines { execute: "query_machines".to_string() }
    }
}

#[derive(Debug)]
pub struct query_cpu_definitions {
    execute: String,
}
impl query_cpu_definitions {
    pub fn new() -> query_cpu_definitions {
        query_cpu_definitions { execute: "query_cpu_definitions".to_string() }
    }
}

#[derive(Debug)]
pub struct add_fd {
    execute: String,
    pub fdset_id: String,
    pub opaque: String,
}
impl add_fd {
    pub fn new(fdset_id: String, opaque: String) -> add_fd {
        add_fd {
            execute: "add_fd".to_string(),
            fdset_id: fdset_id,
            opaque: opaque,
        }
    }

    pub fn parse_qemu_response(response: &String) -> Result<AddfdInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct remove_fd {
    execute: String,
    pub fd: String,
    pub fdset_id: String,
}
impl remove_fd {
    pub fn new(fd: String, fdset_id: String) -> remove_fd {
        remove_fd {
            execute: "remove_fd".to_string(),
            fd: fd,
            fdset_id: fdset_id,
        }
    }
}

#[derive(Debug)]
pub struct query_fdsets {
    execute: String,
}
impl query_fdsets {
    pub fn new() -> query_fdsets {
        query_fdsets { execute: "query_fdsets".to_string() }
    }
}

#[derive(Debug)]
pub struct query_target {
    execute: String,
}
impl query_target {
    pub fn new() -> query_target {
        query_target { execute: "query_target".to_string() }
    }

    pub fn parse_qemu_response(response: &String) -> Result<TargetInfo, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct send_key {
    execute: String,
    pub hold_time: String,
    pub keys: Vec<String>,
}
impl send_key {
    pub fn new(hold_time: String, keys: Vec<String>) -> send_key {
        send_key {
            execute: "send_key".to_string(),
            hold_time: hold_time,
            keys: keys,
        }
    }
}

#[derive(Debug)]
pub struct screendump {
    execute: String,
    pub filename: String,
}
impl screendump {
    pub fn new(filename: String) -> screendump {
        screendump {
            execute: "screendump".to_string(),
            filename: filename,
        }
    }
}

#[derive(Debug)]
pub struct chardev_add {
    execute: String,
    pub backend: String,
    pub id: String,
}
impl chardev_add {
    pub fn new(backend: String, id: String) -> chardev_add {
        chardev_add {
            execute: "chardev_add".to_string(),
            backend: backend,
            id: id,
        }
    }

    pub fn parse_qemu_response(response: &String) -> Result<ChardevReturn, String> {
        Ok(json::decode(&response).unwrap())
    }
}

#[derive(Debug)]
pub struct chardev_remove {
    execute: String,
    pub id: String,
}
impl chardev_remove {
    pub fn new(id: String) -> chardev_remove {
        chardev_remove {
            execute: "chardev_remove".to_string(),
            id: id,
        }
    }
}

#[derive(Debug)]
pub struct query_tpm_models {
    execute: String,
}
impl query_tpm_models {
    pub fn new() -> query_tpm_models {
        query_tpm_models { execute: "query_tpm_models".to_string() }
    }
}

#[derive(Debug)]
pub struct query_tpm_types {
    execute: String,
}
impl query_tpm_types {
    pub fn new() -> query_tpm_types {
        query_tpm_types { execute: "query_tpm_types".to_string() }
    }
}

#[derive(Debug)]
pub struct query_tpm {
    execute: String,
}
impl query_tpm {
    pub fn new() -> query_tpm {
        query_tpm { execute: "query_tpm".to_string() }
    }
}

#[derive(Debug)]
pub struct query_command_line_options {
    execute: String,
    pub option: String,
}
impl query_command_line_options {
    pub fn new(option: String) -> query_command_line_options {
        query_command_line_options {
            execute: "query_command_line_options".to_string(),
            option: option,
        }
    }
}

#[derive(Debug)]
pub struct query_rx_filter {
    execute: String,
    pub name: String,
}
impl query_rx_filter {
    pub fn new(name: String) -> query_rx_filter {
        query_rx_filter {
            execute: "query_rx_filter".to_string(),
            name: name,
        }
    }
}

#[derive(Debug)]
pub struct x_input_send_event {
    execute: String,
    pub console: String,
    pub events: Vec<String>,
}
impl x_input_send_event {
    pub fn new(console: String, events: Vec<String>) -> x_input_send_event {
        x_input_send_event {
            execute: "x_input_send_event".to_string(),
            console: console,
            events: events,
        }
    }
}

#[derive(Debug)]
pub struct query_memdev {
    execute: String,
}
impl query_memdev {
    pub fn new() -> query_memdev {
        query_memdev { execute: "query_memdev".to_string() }
    }
}

#[derive(Debug)]
pub struct query_memory_devices {
    execute: String,
}
impl query_memory_devices {
    pub fn new() -> query_memory_devices {
        query_memory_devices { execute: "query_memory_devices".to_string() }
    }
}

#[derive(Debug)]
pub struct query_acpi_ospm_status {
    execute: String,
}
impl query_acpi_ospm_status {
    pub fn new() -> query_acpi_ospm_status {
        query_acpi_ospm_status { execute: "query_acpi_ospm_status".to_string() }
    }
}

#[derive(Debug)]
pub struct rtc_reset_reinjection {
    execute: String,
}
impl rtc_reset_reinjection {
    pub fn new() -> rtc_reset_reinjection {
        rtc_reset_reinjection { execute: "rtc_reset_reinjection".to_string() }
    }
}
