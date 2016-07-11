
#[derive(Debug)]
pub struct DEVICE_TRAY_MOVED {
    pub device: String,
    pub tray_open: bool,
}

#[derive(Debug)]
pub struct BLOCK_IMAGE_CORRUPTED {
    pub node_name: String,
    pub offset: f64,
    pub size: f64,
    pub device: String,
    pub fatal: bool,
    pub msg: String,
}

#[derive(Debug)]
pub struct BLOCK_IO_ERROR {
    pub nospace: bool,
    pub action: String,
    pub device: String,
    pub operation: String,
    pub reason: String,
}

#[derive(Debug)]
pub struct BLOCK_JOB_COMPLETED {
    pub error: String,
    pub device: String,
    pub len: f64,
    pub offset: f64,
    pub speed: f64,
    pub qemu_type: String,
}

#[derive(Debug)]
pub struct BLOCK_JOB_CANCELLED {
    pub device: String,
    pub len: f64,
    pub offset: f64,
    pub speed: f64,
    pub qemu_type: String,
}

#[derive(Debug)]
pub struct BLOCK_JOB_ERROR {
    pub action: String,
    pub device: String,
    pub operation: String,
}

#[derive(Debug)]
pub struct BLOCK_JOB_READY {
    pub device: String,
    pub len: f64,
    pub offset: f64,
    pub speed: f64,
    pub qemu_type: String,
}

#[derive(Debug)]
pub struct BLOCK_WRITE_THRESHOLD {
    pub amount_exceeded: u64,
    pub node_name: String,
    pub write_threshold: u64,
}

#[derive(Debug)]
pub struct SHUTDOWN {

            }

#[derive(Debug)]
pub struct POWERDOWN {

            }

#[derive(Debug)]
pub struct RESET {

            }

#[derive(Debug)]
pub struct STOP {

            }

#[derive(Debug)]
pub struct RESUME {

            }

#[derive(Debug)]
pub struct SUSPEND {

            }

#[derive(Debug)]
pub struct SUSPEND_DISK {

            }

#[derive(Debug)]
pub struct WAKEUP {

            }

#[derive(Debug)]
pub struct RTC_CHANGE {
    pub offset: f64,
}

#[derive(Debug)]
pub struct WATCHDOG {
    pub action: String,
}

#[derive(Debug)]
pub struct DEVICE_DELETED {
    pub device: String,
    pub path: String,
}

#[derive(Debug)]
pub struct NIC_RX_FILTER_CHANGED {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct VNC_CONNECTED {
    pub client: String,
    pub server: String,
}

#[derive(Debug)]
pub struct VNC_INITIALIZED {
    pub client: String,
    pub server: String,
}

#[derive(Debug)]
pub struct VNC_DISCONNECTED {
    pub client: String,
    pub server: String,
}

#[derive(Debug)]
pub struct SPICE_CONNECTED {
    pub client: String,
    pub server: String,
}

#[derive(Debug)]
pub struct SPICE_INITIALIZED {
    pub client: String,
    pub server: String,
}

#[derive(Debug)]
pub struct SPICE_DISCONNECTED {
    pub client: String,
    pub server: String,
}

#[derive(Debug)]
pub struct SPICE_MIGRATE_COMPLETED {

            }

#[derive(Debug)]
pub struct ACPI_DEVICE_OST {
    pub info: String,
}

#[derive(Debug)]
pub struct BALLOON_CHANGE {
    pub actual: f64,
}

#[derive(Debug)]
pub struct GUEST_PANICKED {
    pub action: String,
}

#[derive(Debug)]
pub struct QUORUM_FAILURE {
    pub reference: String,
    pub sector_num: f64,
    pub sectors_count: f64,
}

#[derive(Debug)]
pub struct QUORUM_REPORT_BAD {
    pub error: String,
    pub node_name: String,
    pub sector_num: f64,
    pub sectors_count: f64,
}

#[derive(Debug)]
pub struct VSERPORT_CHANGE {
    pub id: String,
    pub open: bool,
}

#[derive(Debug)]
pub struct MEM_UNPLUG_ERROR {
    pub device: String,
    pub msg: String,
}
