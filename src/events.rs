/// #
/// @DEVICE_TRAY_MOVED
///
/// Emitted whenever the tray of a removable device is moved by the guest or by
/// HMP/QMP commands
///
/// @device: device name
///
/// @tray-open: true if the tray has been opened or false if it has been closed
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct DEVICE_TRAY_MOVED {
    pub device: String,
    pub tray_open: bool,
}
/// #
/// @BLOCK_IMAGE_CORRUPTED
///
/// Emitted when a corruption has been detected in a disk image
///
/// @device: device name. This is always present for compatibility
///          reasons, but it can be empty ("") if the image does not
///          have a device name associated.
///
/// @node-name: #optional node name (Since: 2.4)
///
/// @msg: informative message for human consumption, such as the kind of
///       corruption being detected. It should not be parsed by machine as it is
///       not guaranteed to be stable
///
/// @offset: #optional, if the corruption resulted from an image access, this is
///          the host"s access offset into the image
///
/// @size: #optional, if the corruption resulted from an image access, this is
///        the access size
///
/// fatal: if set, the image is marked corrupt and therefore unusable after this
///        event and must be repaired (Since 2.2; before, every
///        BLOCK_IMAGE_CORRUPTED event was fatal)
///
/// Since: 1.7
/// #
#[derive(Debug)]
pub struct BLOCK_IMAGE_CORRUPTED {
    pub node_name: String,
    pub offset: f64,
    pub size: f64,
    pub device: String,
    pub fatal: bool,
    pub msg: String,
}
/// #
/// @BLOCK_IO_ERROR
///
/// Emitted when a disk I/O error occurs
///
/// @device: device name
///
/// @operation: I/O operation
///
/// @action: action that has been taken
///
/// @nospace: #optional true if I/O error was caused due to a no-space
///           condition. This key is only present if query-block"s
///           io-status is present, please see query-block documentation
///           for more information (since: 2.2)
///
/// @reason: human readable string describing the error cause.
///          (This field is a debugging aid for humans, it should not
///           be parsed by applications) (since: 2.2)
///
/// Note: If action is "stop", a STOP event will eventually follow the
/// BLOCK_IO_ERROR event
///
/// Since: 0.13.0
/// #
#[derive(Debug)]
pub struct BLOCK_IO_ERROR {
    pub nospace: bool,
    pub action: String,
    pub device: String,
    pub operation: String,
    pub reason: String,
}
/// #
/// @BLOCK_JOB_COMPLETED
///
/// Emitted when a block job has completed
///
/// @type: job type
///
/// @device: device name
///
/// @len: maximum progress value
///
/// @offset: current progress value. On success this is equal to len.
///          On failure this is less than len
///
/// @speed: rate limit, bytes per second
///
/// @error: #optional, error message. Only present on failure. This field
///         contains a human-readable error message. There are no semantics
///         other than that streaming has failed and clients should not try to
///         interpret the error string
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct BLOCK_JOB_COMPLETED {
    pub error: String,
    pub device: String,
    pub len: f64,
    pub offset: f64,
    pub speed: f64,
    pub qemu_type: String,
}
/// #
/// @BLOCK_JOB_CANCELLED
///
/// Emitted when a block job has been cancelled
///
/// @type: job type
///
/// @device: device name
///
/// @len: maximum progress value
///
/// @offset: current progress value. On success this is equal to len.
///          On failure this is less than len
///
/// @speed: rate limit, bytes per second
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct BLOCK_JOB_CANCELLED {
    pub device: String,
    pub len: f64,
    pub offset: f64,
    pub speed: f64,
    pub qemu_type: String,
}
/// #
/// @BLOCK_JOB_ERROR
///
/// Emitted when a block job encounters an error
///
/// @device: device name
///
/// @operation: I/O operation
///
/// @action: action that has been taken
///
/// Since: 1.3
/// #
#[derive(Debug)]
pub struct BLOCK_JOB_ERROR {
    pub action: String,
    pub device: String,
    pub operation: String,
}
/// #
/// @BLOCK_JOB_READY
///
/// Emitted when a block job is ready to complete
///
/// @type: job type
///
/// @device: device name
///
/// @len: maximum progress value
///
/// @offset: current progress value. On success this is equal to len.
///          On failure this is less than len
///
/// @speed: rate limit, bytes per second
///
/// Note: The "ready to complete" status is always reset by a @BLOCK_JOB_ERROR
/// event
///
/// Since: 1.3
/// #
#[derive(Debug)]
pub struct BLOCK_JOB_READY {
    pub device: String,
    pub len: f64,
    pub offset: f64,
    pub speed: f64,
    pub qemu_type: String,
}
/// #
/// @BLOCK_WRITE_THRESHOLD
///
/// Emitted when writes on block device reaches or exceeds the
/// configured write threshold. For thin-provisioned devices, this
/// means the device should be extended to avoid pausing for
/// disk exhaustion.
/// The event is one shot. Once triggered, it needs to be
/// re-registered with another block-set-threshold command.
///
/// @node-name: graph node name on which the threshold was exceeded.
///
/// @amount-exceeded: amount of data which exceeded the threshold, in bytes.
///
/// @write-threshold: last configured threshold, in bytes.
///
/// Since: 2.3
/// #
#[derive(Debug)]
pub struct BLOCK_WRITE_THRESHOLD {
    pub amount_exceeded: u64,
    pub node_name: String,
    pub write_threshold: u64,
}
/// #
/// @SHUTDOWN
///
/// Emitted when the virtual machine has shut down, indicating that qemu is
/// about to exit.
///
/// Note: If the command-line option "-no-shutdown" has been specified, qemu will
/// not exit, and a STOP event will eventually follow the SHUTDOWN event
///
/// Since: 0.12.0
/// #
#[derive(Debug)]
pub struct SHUTDOWN {

            }
/// #
/// @POWERDOWN
///
/// Emitted when the virtual machine is powered down through the power control
/// system, such as via ACPI.
///
/// Since: 0.12.0
/// #
#[derive(Debug)]
pub struct POWERDOWN {

            }
/// #
/// @RESET
///
/// Emitted when the virtual machine is reset
///
/// Since: 0.12.0
/// #
#[derive(Debug)]
pub struct RESET {

            }
/// #
/// @STOP
///
/// Emitted when the virtual machine is stopped
///
/// Since: 0.12.0
/// #
#[derive(Debug)]
pub struct STOP {

            }
/// #
/// @RESUME
///
/// Emitted when the virtual machine resumes execution
///
/// Since: 0.12.0
/// #
#[derive(Debug)]
pub struct RESUME {

            }
/// #
/// @SUSPEND
///
/// Emitted when guest enters a hardware suspension state, for example, S3 state,
/// which is sometimes called standby state
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct SUSPEND {

            }
/// #
/// @SUSPEND_DISK
///
/// Emitted when guest enters a hardware suspension state with data saved on
/// disk, for example, S4 state, which is sometimes called hibernate state
///
/// Note: QEMU shuts down (similar to event @SHUTDOWN) when entering this state
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct SUSPEND_DISK {

            }
/// #
/// @WAKEUP
///
/// Emitted when the guest has woken up from suspend state and is running
///
/// Since: 1.1
/// #
#[derive(Debug)]
pub struct WAKEUP {

            }
/// #
/// @RTC_CHANGE
///
/// Emitted when the guest changes the RTC time.
///
/// @offset: offset between base RTC clock (as specified by -rtc base), and
///          new RTC clock value
///
/// Since: 0.13.0
/// #
#[derive(Debug)]
pub struct RTC_CHANGE {
    pub offset: f64,
}
/// #
/// @WATCHDOG
///
/// Emitted when the watchdog device"s timer is expired
///
/// @action: action that has been taken
///
/// Note: If action is "reset", "shutdown", or "pause" the WATCHDOG event is
/// followed respectively by the RESET, SHUTDOWN, or STOP events
///
/// Since: 0.13.0
/// #
#[derive(Debug)]
pub struct WATCHDOG {
    pub action: String,
}
/// #
/// @DEVICE_DELETED
///
/// Emitted whenever the device removal completion is acknowledged by the guest.
/// At this point, it"s safe to reuse the specified device ID. Device removal can
/// be initiated by the guest or by HMP/QMP commands.
///
/// @device: #optional, device name
///
/// @path: device path
///
/// Since: 1.5
/// #
#[derive(Debug)]
pub struct DEVICE_DELETED {
    pub device: String,
    pub path: String,
}
/// #
/// @NIC_RX_FILTER_CHANGED
///
/// Emitted once until the "query-rx-filter" command is executed, the first event
/// will always be emitted
///
/// @name: #optional, net client name
///
/// @path: device path
///
/// Since: 1.6
/// #
#[derive(Debug)]
pub struct NIC_RX_FILTER_CHANGED {
    pub name: String,
    pub path: String,
}
/// #
/// @VNC_CONNECTED
///
/// Emitted when a VNC client establishes a connection
///
/// @server: server information
///
/// @client: client information
///
/// Note: This event is emitted before any authentication takes place, thus
/// the authentication ID is not provided
///
/// Since: 0.13.0
/// #
#[derive(Debug)]
pub struct VNC_CONNECTED {
    pub client: String,
    pub server: String,
}
/// #
/// @VNC_INITIALIZED
///
/// Emitted after authentication takes place (if any) and the VNC session is
/// made active
///
/// @server: server information
///
/// @client: client information
///
/// Since: 0.13.0
/// #
#[derive(Debug)]
pub struct VNC_INITIALIZED {
    pub client: String,
    pub server: String,
}
/// #
/// @VNC_DISCONNECTED
///
/// Emitted when the connection is closed
///
/// @server: server information
///
/// @client: client information
///
/// Since: 0.13.0
/// #
#[derive(Debug)]
pub struct VNC_DISCONNECTED {
    pub client: String,
    pub server: String,
}
/// #
/// @SPICE_CONNECTED
///
/// Emitted when a SPICE client establishes a connection
///
/// @server: server information
///
/// @client: client information
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct SPICE_CONNECTED {
    pub client: String,
    pub server: String,
}
/// #
/// @SPICE_INITIALIZED
///
/// Emitted after initial handshake and authentication takes place (if any)
/// and the SPICE channel is up and running
///
/// @server: server information
///
/// @client: client information
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct SPICE_INITIALIZED {
    pub client: String,
    pub server: String,
}
/// #
/// @SPICE_DISCONNECTED
///
/// Emitted when the SPICE connection is closed
///
/// @server: server information
///
/// @client: client information
///
/// Since: 0.14.0
/// #
#[derive(Debug)]
pub struct SPICE_DISCONNECTED {
    pub client: String,
    pub server: String,
}
/// #
/// @SPICE_MIGRATE_COMPLETED
///
/// Emitted when SPICE migration has completed
///
/// Since: 1.3
/// #
#[derive(Debug)]
pub struct SPICE_MIGRATE_COMPLETED {

            }
/// #
/// @ACPI_DEVICE_OST
///
/// Emitted when guest executes ACPI _OST method.
///
/// Since: 2.1
///
/// @info: ACPIOSTInfo type as described in qapi-schema.json
/// #
#[derive(Debug)]
pub struct ACPI_DEVICE_OST {
    pub info: String,
}
/// #
/// @BALLOON_CHANGE
///
/// Emitted when the guest changes the actual BALLOON level. This value is
/// equivalent to the @actual field return by the "query-balloon" command
///
/// @actual: actual level of the guest memory balloon in bytes
///
/// Since: 1.2
/// #
#[derive(Debug)]
pub struct BALLOON_CHANGE {
    pub actual: f64,
}
/// #
/// @GUEST_PANICKED
///
/// Emitted when guest OS panic is detected
///
/// @action: action that has been taken, currently always "pause"
///
/// Since: 1.5
/// #
#[derive(Debug)]
pub struct GUEST_PANICKED {
    pub action: String,
}
/// #
/// @QUORUM_FAILURE
///
/// Emitted by the Quorum block driver if it fails to establish a quorum
///
/// @reference: device name if defined else node name
///
/// @sector-num: number of the first sector of the failed read operation
///
/// @sectors-count: failed read operation sector count
///
/// Since: 2.0
/// #
#[derive(Debug)]
pub struct QUORUM_FAILURE {
    pub reference: String,
    pub sector_num: f64,
    pub sectors_count: f64,
}
/// #
/// @QUORUM_REPORT_BAD
///
/// Emitted to report a corruption of a Quorum file
///
/// @error: #optional, error message. Only present on failure. This field
///         contains a human-readable error message. There are no semantics other
///         than that the block layer reported an error and clients should not
///         try to interpret the error string.
///
/// @node-name: the graph node name of the block driver state
///
/// @sector-num: number of the first sector of the failed read operation
///
/// @sectors-count: failed read operation sector count
///
/// Since: 2.0
/// #
#[derive(Debug)]
pub struct QUORUM_REPORT_BAD {
    pub error: String,
    pub node_name: String,
    pub sector_num: f64,
    pub sectors_count: f64,
}
/// #
/// @VSERPORT_CHANGE
///
/// Emitted when the guest opens or closes a virtio-serial port.
///
/// @id: device identifier of the virtio-serial port
///
/// @open: true if the guest has opened the virtio-serial port
///
/// Since: 2.1
/// #
#[derive(Debug)]
pub struct VSERPORT_CHANGE {
    pub id: String,
    pub open: bool,
}
/// #
/// @MEM_UNPLUG_ERROR
///
/// Emitted when memory hot unplug error occurs.
///
/// @device: device name
///
/// @msg: Informative message
///
/// Since: 2.4
/// #
#[derive(Debug)]
pub struct MEM_UNPLUG_ERROR {
    pub device: String,
    pub msg: String,
}
