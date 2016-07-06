 QAPI block definitions QAPI event definitions Tracing commands#
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
///# @add_client
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
///#
        #[derive(Debug)]
        pub struct add_client {
            execute: String,
            pub skipauth:String,pub tls:String,pub fdname:String,pub protocol:String
        }
        impl add_client {
            pub fn new(skipauth:String,tls:String,fdname:String,protocol:String)->add_client{
                add_client{
                    execute: "add_client".to_string(),
                    skipauth:skipauth,tls:tls,fdname:fdname,protocol:protocol
                }
            }
        }
        #
/// @NameInfo:
///
/// Guest name information.
///
/// @name: #optional The name of the guest
///
/// Since 0.14.0
///#
            #[derive(Debug)]
            pub struct NameInfo{
                pub *name:String
            }
            #
/// @query-name:
///
/// Return the name information of a guest.
///
/// Returns: @NameInfo of the guest
///
/// Since 0.14.0
///#
        #[derive(Debug)]
        pub struct query_name {
            execute: String,
            #[serde(skip_serializing)]
returns:NameInfo
        }
        impl query_name {
            pub fn new(NameInfo:String)->query_name{
                query_name{
                    execute: "query_name".to_string(),
                    returns:NameInfo
                }
            }
        }
        #
/// @KvmInfo:
///
/// Information about support for KVM acceleration
///
/// @enabled: true if KVM acceleration is active
///
/// @present: true if KVM acceleration is built into this executable
///
/// Since: 0.14.0
///#
            #[derive(Debug)]
            pub struct KvmInfo{
                pub enabled:String,pub present:String
            }
            #
/// @query-kvm:
///
/// Returns information about KVM acceleration
///
/// Returns: @KvmInfo
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct query_kvm {
            execute: String,
            #[serde(skip_serializing)]
returns:KvmInfo
        }
        impl query_kvm {
            pub fn new(KvmInfo:String)->query_kvm{
                query_kvm{
                    execute: "query_kvm".to_string(),
                    returns:KvmInfo
                }
            }
        }
        #
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
///##
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
///#
            #[derive(Debug)]
            pub struct StatusInfo{
                pub running:String,pub singlestep:String,pub status:String
            }
            #
/// @query-status:
///
/// Query the run status of all VCPUs
///
/// Returns: @StatusInfo reflecting all VCPUs
///
/// Since:  0.14.0
///#
        #[derive(Debug)]
        pub struct query_status {
            execute: String,
            #[serde(skip_serializing)]
returns:StatusInfo
        }
        impl query_status {
            pub fn new(StatusInfo:String)->query_status{
                query_status{
                    execute: "query_status".to_string(),
                    returns:StatusInfo
                }
            }
        }
        #
/// @UuidInfo:
///
/// Guest UUID information.
///
/// @UUID: the UUID of the guest
///
/// Since: 0.14.0
///
/// Notes: If no UUID was specified for the guest, a null UUID is returned.
///#
            #[derive(Debug)]
            pub struct UuidInfo{
                pub UUID:String
            }
            #
/// @query-uuid:
///
/// Query the guest UUID information.
///
/// Returns: The @UuidInfo for the guest
///
/// Since 0.14.0
///#
        #[derive(Debug)]
        pub struct query_uuid {
            execute: String,
            #[serde(skip_serializing)]
returns:UuidInfo
        }
        impl query_uuid {
            pub fn new(UuidInfo:String)->query_uuid{
                query_uuid{
                    execute: "query_uuid".to_string(),
                    returns:UuidInfo
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct ChardevInfo{
                pub filename:String,pub frontend_open:String,pub label:String
            }
            #
/// @query-chardev:
///
/// Returns information about current character devices.
///
/// Returns: a list of @ChardevInfo
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct query_chardev {
            execute: String,
            
        }
        impl query_chardev {
            pub fn new()->query_chardev{
                query_chardev{
                    execute: "query_chardev".to_string(),
                    
                }
            }
        }
        #
/// @ChardevBackendInfo:
///
/// Information about a character device backend
///
/// @name: The backend name
///
/// Since: 2.0
///#
            #[derive(Debug)]
            pub struct ChardevBackendInfo{
                pub name:String
            }
            #
/// @query-chardev-backends:
///
/// Returns information about character device backends.
///
/// Returns: a list of @ChardevBackendInfo
///
/// Since: 2.0
///#
        #[derive(Debug)]
        pub struct query_chardev_backends {
            execute: String,
            
        }
        impl query_chardev_backends {
            pub fn new()->query_chardev_backends{
                query_chardev_backends{
                    execute: "query_chardev_backends".to_string(),
                    
                }
            }
        }
        #
/// @DataFormat:
///
/// An enumeration of data format.
///
/// @utf8: Data is a UTF-8 string (RFC 3629)
///
/// @base64: Data is Base64 encoded binary (RFC 3548)
///
/// Since: 1.4
///##
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
///#
        #[derive(Debug)]
        pub struct ringbuf_write {
            execute: String,
            pub format:String,pub data:String,pub device:String
        }
        impl ringbuf_write {
            pub fn new(format:String,data:String,device:String)->ringbuf_write{
                ringbuf_write{
                    execute: "ringbuf_write".to_string(),
                    format:format,data:data,device:device
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct ringbuf_read {
            execute: String,
            pub format:String,pub device:String,pub size:String,#[serde(skip_serializing)]
returns:String
        }
        impl ringbuf_read {
            pub fn new(format:String,device:String,size:String,str:String)->ringbuf_read{
                ringbuf_read{
                    execute: "ringbuf_read".to_string(),
                    format:format,device:device,size:size,returns:String
                }
            }
        }
        #
/// @EventInfo:
///
/// Information about a QMP event
///
/// @name: The event name
///
/// Since: 1.2.0
///#
            #[derive(Debug)]
            pub struct EventInfo{
                pub name:String
            }
            #
/// @query-events:
///
/// Return a list of supported QMP events by this server
///
/// Returns: A list of @EventInfo for all supported events
///
/// Since: 1.2.0
///#
        #[derive(Debug)]
        pub struct query_events {
            execute: String,
            
        }
        impl query_events {
            pub fn new()->query_events{
                query_events{
                    execute: "query_events".to_string(),
                    
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct MigrationStats{
                pub dirty_pages_rate:String,pub dirty_sync_count:String,pub duplicate:String,pub mbps:String,pub normal:String,pub normal_bytes:String,pub remaining:String,pub skipped:String,pub total:String,pub transferred:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct XBZRLECacheStats{
                pub bytes:String,pub cache_miss:String,pub cache_miss_rate:String,pub cache_size:String,pub overflow:String,pub pages:String
            }
             @MigrationStatus:
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
///##
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
///#
            #[derive(Debug)]
            pub struct MigrationInfo{
                pub *disk:String,pub *downtime:String,pub *expected_downtime:String,pub *ram:String,pub *setup_time:String,pub *status:String,pub *total_time:String,pub *xbzrle_cache:String
            }
            #
/// @query-migrate
///
/// Returns information about current migration process.
///
/// Returns: @MigrationInfo
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct query_migrate {
            execute: String,
            #[serde(skip_serializing)]
returns:MigrationInfo
        }
        impl query_migrate {
            pub fn new(MigrationInfo:String)->query_migrate{
                query_migrate{
                    execute: "query_migrate".to_string(),
                    returns:MigrationInfo
                }
            }
        }
        #
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
///##
/// @MigrationCapabilityStatus
///
/// Migration capability information
///
/// @capability: capability enum
///
/// @state: capability state bool
///
/// Since: 1.2
///#
            #[derive(Debug)]
            pub struct MigrationCapabilityStatus{
                pub capability:String,pub state:String
            }
            #
/// @migrate-set-capabilities
///
/// Enable/Disable the following migration capabilities (like xbzrle)
///
/// @capabilities: json array of capability modifications to make
///
/// Since: 1.2
///#
        #[derive(Debug)]
        pub struct migrate_set_capabilities {
            execute: String,
            pub capabilities:Vec
        }
        impl migrate_set_capabilities {
            pub fn new(capabilities:Vec)->migrate_set_capabilities{
                migrate_set_capabilities{
                    execute: "migrate_set_capabilities".to_string(),
                    capabilities:capabilities
                }
            }
        }
        #
/// @query-migrate-capabilities
///
/// Returns information about the current migration capabilities status
///
/// Returns: @MigrationCapabilitiesStatus
///
/// Since: 1.2
///#
        #[derive(Debug)]
        pub struct query_migrate_capabilities {
            execute: String,
            
        }
        impl query_migrate_capabilities {
            pub fn new()->query_migrate_capabilities{
                query_migrate_capabilities{
                    execute: "query_migrate_capabilities".to_string(),
                    
                }
            }
        }
         @MigrationParameter
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
///#
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
///#
        #[derive(Debug)]
        pub struct migrate_set_parameters {
            execute: String,
            pub compress_level:String,pub compress_threads:String,pub decompress_threads:String
        }
        impl migrate_set_parameters {
            pub fn new(compress_level:String,compress_threads:String,decompress_threads:String)->migrate_set_parameters{
                migrate_set_parameters{
                    execute: "migrate_set_parameters".to_string(),
                    compress_level:compress_level,compress_threads:compress_threads,decompress_threads:decompress_threads
                }
            }
        }
        
/// @MigrationParameters
///
/// @compress-level: compression level
///
/// @compress-threads: compression thread count
///
/// @decompress-threads: decompression thread count
///
/// Since: 2.4
///#
            #[derive(Debug)]
            pub struct MigrationParameters{
                pub compress_level:String,pub compress_threads:String,pub decompress_threads:String
            }
            #
/// @query-migrate-parameters
///
/// Returns information about the current migration parameters
///
/// Returns: @MigrationParameters
///
/// Since: 2.4
///#
        #[derive(Debug)]
        pub struct query_migrate_parameters {
            execute: String,
            #[serde(skip_serializing)]
returns:MigrationParameters
        }
        impl query_migrate_parameters {
            pub fn new(MigrationParameters:String)->query_migrate_parameters{
                query_migrate_parameters{
                    execute: "query_migrate_parameters".to_string(),
                    returns:MigrationParameters
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct MouseInfo{
                pub absolute:String,pub current:String,pub index:String,pub name:String
            }
            #
/// @query-mice:
///
/// Returns information about each active mouse device
///
/// Returns: a list of @MouseInfo for each device
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct query_mice {
            execute: String,
            
        }
        impl query_mice {
            pub fn new()->query_mice{
                query_mice{
                    execute: "query_mice".to_string(),
                    
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct CpuInfo{
                pub *PC:String,pub *nip:String,pub *npc:String,pub *pc:String,pub CPU:String,pub current:String,pub halted:String,pub qom_path:String,pub thread_id:String
            }
            #
/// @query-cpus:
///
/// Returns a list of information about each virtual CPU.
///
/// Returns: a list of @CpuInfo for each virtual CPU
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct query_cpus {
            execute: String,
            
        }
        impl query_cpus {
            pub fn new()->query_cpus{
                query_cpus{
                    execute: "query_cpus".to_string(),
                    
                }
            }
        }
        #
/// @IOThreadInfo:
///
/// Information about an iothread
///
/// @id: the identifier of the iothread
///
/// @thread-id: ID of the underlying host thread
///
/// Since: 2.0
///#
            #[derive(Debug)]
            pub struct IOThreadInfo{
                pub id:String,pub thread_id:String
            }
            #
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
///#
        #[derive(Debug)]
        pub struct query_iothreads {
            execute: String,
            
        }
        impl query_iothreads {
            pub fn new()->query_iothreads{
                query_iothreads{
                    execute: "query_iothreads".to_string(),
                    
                }
            }
        }
        #
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
///##
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
///#
            #[derive(Debug)]
            pub struct VncBasicInfo{
                pub family:String,pub host:String,pub service:String,pub websocket:String
            }
            #
/// @VncServerInfo
///
/// The network connection information for server
///
/// @auth: #optional, authentication method
///
/// Since: 2.1
///#
            #[derive(Debug)]
            pub struct VncServerInfo{
                base: VncBasicInfo,pub *auth:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct VncClientInfo{
                base: VncBasicInfo,pub *sasl_username:String,pub *x509_dname:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct VncInfo{
                pub *auth:String,pub *clients:Vec,pub *family:String,pub *host:String,pub *service:String,pub enabled:String
            }
            #
/// @VncPriAuth:
///
/// vnc primary authentication method.
///
/// Since: 2.3
///##
/// @VncVencryptSubAuth:
///
/// vnc sub authentication method with vencrypt.
///
/// Since: 2.3
///##
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
///#
            #[derive(Debug)]
            pub struct VncInfo2{
                pub *display:String,pub *vencrypt:String,pub auth:String,pub clients:Vec,pub id:String,pub server:Vec
            }
            #
/// @query-vnc:
///
/// Returns information about the current VNC server
///
/// Returns: @VncInfo
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct query_vnc {
            execute: String,
            #[serde(skip_serializing)]
returns:VncInfo
        }
        impl query_vnc {
            pub fn new(VncInfo:String)->query_vnc{
                query_vnc{
                    execute: "query_vnc".to_string(),
                    returns:VncInfo
                }
            }
        }
        #
/// @query-vnc-servers:
///
/// Returns a list of vnc servers.  The list can be empty.
///
/// Returns: a list of @VncInfo2
///
/// Since: 2.3
///#
        #[derive(Debug)]
        pub struct query_vnc_servers {
            execute: String,
            
        }
        impl query_vnc_servers {
            pub fn new()->query_vnc_servers{
                query_vnc_servers{
                    execute: "query_vnc_servers".to_string(),
                    
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct SpiceBasicInfo{
                pub family:String,pub host:String,pub port:String
            }
            #
/// @SpiceServerInfo
///
/// Information about a SPICE server
///
/// @auth: #optional, authentication method
///
/// Since: 2.1
///#
            #[derive(Debug)]
            pub struct SpiceServerInfo{
                base: SpiceBasicInfo,pub *auth:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct SpiceChannel{
                base: SpiceBasicInfo,pub channel_id:String,pub channel_type:String,pub connection_id:String,pub tls:String
            }
            #
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
///##
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
///#
            #[derive(Debug)]
            pub struct SpiceInfo{
                pub *auth:String,pub *channels:Vec,pub *compiled_version:String,pub *host:String,pub *port:String,pub *tls_port:String,pub enabled:String,pub migrated:String,pub mouse_mode:String
            }
            #
/// @query-spice
///
/// Returns information about the current SPICE server
///
/// Returns: @SpiceInfo
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct query_spice {
            execute: String,
            #[serde(skip_serializing)]
returns:SpiceInfo
        }
        impl query_spice {
            pub fn new(SpiceInfo:String)->query_spice{
                query_spice{
                    execute: "query_spice".to_string(),
                    returns:SpiceInfo
                }
            }
        }
        #
/// @BalloonInfo:
///
/// Information about the guest balloon device.
///
/// @actual: the number of bytes the balloon currently contains
///
/// Since: 0.14.0
///
///#
            #[derive(Debug)]
            pub struct BalloonInfo{
                pub actual:String
            }
            #
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
///#
        #[derive(Debug)]
        pub struct query_balloon {
            execute: String,
            #[serde(skip_serializing)]
returns:BalloonInfo
        }
        impl query_balloon {
            pub fn new(BalloonInfo:String)->query_balloon{
                query_balloon{
                    execute: "query_balloon".to_string(),
                    returns:BalloonInfo
                }
            }
        }
        #
/// @PciMemoryRange:
///
/// A PCI device memory region
///
/// @base: the starting address (guest physical)
///
/// @limit: the ending address (guest physical)
///
/// Since: 0.14.0
///#
            #[derive(Debug)]
            pub struct PciMemoryRange{
                pub base:String,pub limit:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct PciMemoryRegion{
                pub *mem_type_64:String,pub *prefetch:String,pub address:String,pub bar:String,pub size:String,pub type:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct PciBusInfo{
                pub io_range:String,pub memory_range:String,pub number:String,pub prefetchable_range:String,pub secondary:String,pub subordinate:String
            }
            #
/// @PciBridgeInfo:
///
/// Information about a PCI Bridge device
///
/// @bus: information about the bus the device resides on
///
/// @devices: a list of @PciDeviceInfo for each device on this bridge
///
/// Since: 0.14.0
///#
            #[derive(Debug)]
            pub struct PciBridgeInfo{
                pub *devices:Vec,pub bus:String
            }
            #
/// @PciDeviceClass:
///
/// Information about the Class of a PCI device
///
/// @desc: #optional a string description of the device"s class
///
/// @class: the class code of the device
///
/// Since: 2.4
///#
            #[derive(Debug)]
            pub struct PciDeviceClass{
                pub *desc:String,pub class:String
            }
            #
/// @PciDeviceId:
///
/// Information about the Id of a PCI device
///
/// @device: the PCI device id
///
/// @vendor: the PCI vendor id
///
/// Since: 2.4
///#
            #[derive(Debug)]
            pub struct PciDeviceId{
                pub device:String,pub vendor:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct PciDeviceInfo{
                pub *irq:String,pub *pci_bridge:String,pub bus:String,pub class_info:String,pub function:String,pub id:String,pub qdev_id:String,pub regions:Vec,pub slot:String
            }
            #
/// @PciInfo:
///
/// Information about a PCI bus
///
/// @bus: the bus index
///
/// @devices: a list of devices on this bus
///
/// Since: 0.14.0
///#
            #[derive(Debug)]
            pub struct PciInfo{
                pub bus:String,pub devices:Vec
            }
            #
/// @query-pci:
///
/// Return information about the PCI bus topology of the guest.
///
/// Returns: a list of @PciInfo for each PCI bus
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct query_pci {
            execute: String,
            
        }
        impl query_pci {
            pub fn new()->query_pci{
                query_pci{
                    execute: "query_pci".to_string(),
                    
                }
            }
        }
        #
/// @quit:
///
/// This command will cause the QEMU process to exit gracefully.  While every
/// attempt is made to send the QMP response before terminating, this is not
/// guaranteed.  When using this interface, a premature EOF would not be
/// unexpected.
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct quit {
            execute: String,
            
        }
        impl quit {
            pub fn new()->quit{
                quit{
                    execute: "quit".to_string(),
                    
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct stop {
            execute: String,
            
        }
        impl stop {
            pub fn new()->stop{
                stop{
                    execute: "stop".to_string(),
                    
                }
            }
        }
        #
/// @system_reset:
///
/// Performs a hard reset of a guest.
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct system_reset {
            execute: String,
            
        }
        impl system_reset {
            pub fn new()->system_reset{
                system_reset{
                    execute: "system_reset".to_string(),
                    
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct system_powerdown {
            execute: String,
            
        }
        impl system_powerdown {
            pub fn new()->system_powerdown{
                system_powerdown{
                    execute: "system_powerdown".to_string(),
                    
                }
            }
        }
        #
/// @cpu:
///
/// This command is a nop that is only provided for the purposes of compatibility.
///
/// Since: 0.14.0
///
/// Notes: Do not use this command.
///#
        #[derive(Debug)]
        pub struct cpu {
            execute: String,
            pub index:String
        }
        impl cpu {
            pub fn new(index:String)->cpu{
                cpu{
                    execute: "cpu".to_string(),
                    index:index
                }
            }
        }
        #
/// @cpu-add
///
/// Adds CPU with specified ID
///
/// @id: ID of CPU to be created, valid values [0..max_cpus)
///
/// Returns: Nothing on success
///
/// Since 1.5
///#
        #[derive(Debug)]
        pub struct cpu_add {
            execute: String,
            pub id:String
        }
        impl cpu_add {
            pub fn new(id:String)->cpu_add{
                cpu_add{
                    execute: "cpu_add".to_string(),
                    id:id
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct memsave {
            execute: String,
            pub cpu_index:String,pub filename:String,pub size:String,pub val:String
        }
        impl memsave {
            pub fn new(cpu_index:String,filename:String,size:String,val:String)->memsave{
                memsave{
                    execute: "memsave".to_string(),
                    cpu_index:cpu_index,filename:filename,size:size,val:val
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct pmemsave {
            execute: String,
            pub filename:String,pub size:String,pub val:String
        }
        impl pmemsave {
            pub fn new(filename:String,size:String,val:String)->pmemsave{
                pmemsave{
                    execute: "pmemsave".to_string(),
                    filename:filename,size:size,val:val
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct cont {
            execute: String,
            
        }
        impl cont {
            pub fn new()->cont{
                cont{
                    execute: "cont".to_string(),
                    
                }
            }
        }
        #
/// @system_wakeup:
///
/// Wakeup guest from suspend.  Does nothing in case the guest isn"t suspended.
///
/// Since:  1.1
///
/// Returns:  nothing.
///#
        #[derive(Debug)]
        pub struct system_wakeup {
            execute: String,
            
        }
        impl system_wakeup {
            pub fn new()->system_wakeup{
                system_wakeup{
                    execute: "system_wakeup".to_string(),
                    
                }
            }
        }
        #
/// @inject-nmi:
///
/// Injects a Non-Maskable Interrupt into the default CPU (x86/s390) or all CPUs (ppc64).
///
/// Returns:  If successful, nothing
///
/// Since:  0.14.0
///
/// Note: prior to 2.1, this command was only supported for x86 and s390 VMs
///#
        #[derive(Debug)]
        pub struct inject_nmi {
            execute: String,
            
        }
        impl inject_nmi {
            pub fn new()->inject_nmi{
                inject_nmi{
                    execute: "inject_nmi".to_string(),
                    
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct set_link {
            execute: String,
            pub name:String,pub up:String
        }
        impl set_link {
            pub fn new(name:String,up:String)->set_link{
                set_link{
                    execute: "set_link".to_string(),
                    name:name,up:up
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct balloon {
            execute: String,
            pub value:String
        }
        impl balloon {
            pub fn new(value:String)->balloon{
                balloon{
                    execute: "balloon".to_string(),
                    value:value
                }
            }
        }
        #
/// @Abort
///
/// This action can be used to test transaction failure.
///
/// Since: 1.6
///##
            #[derive(Debug)]
            pub struct Abort{
                
            }
            #
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
///#
        #[derive(Debug)]
        pub struct transaction {
            execute: String,
            pub actions:Vec
        }
        impl transaction {
            pub fn new(actions:Vec)->transaction{
                transaction{
                    execute: "transaction".to_string(),
                    actions:actions
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct human_monitor_command {
            execute: String,
            pub cpu_index:String,pub command_line:String,#[serde(skip_serializing)]
returns:String
        }
        impl human_monitor_command {
            pub fn new(cpu_index:String,command_line:String,str:String)->human_monitor_command{
                human_monitor_command{
                    execute: "human_monitor_command".to_string(),
                    cpu_index:cpu_index,command_line:command_line,returns:String
                }
            }
        }
        #
/// @migrate_cancel
///
/// Cancel the current executing migration process.
///
/// Returns: nothing on success
///
/// Notes: This command succeeds even if there is no migration process running.
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct migrate_cancel {
            execute: String,
            
        }
        impl migrate_cancel {
            pub fn new()->migrate_cancel{
                migrate_cancel{
                    execute: "migrate_cancel".to_string(),
                    
                }
            }
        }
        #
/// @migrate_set_downtime
///
/// Set maximum tolerated downtime for migration.
///
/// @value: maximum downtime in seconds
///
/// Returns: nothing on success
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct migrate_set_downtime {
            execute: String,
            pub value:String
        }
        impl migrate_set_downtime {
            pub fn new(value:String)->migrate_set_downtime{
                migrate_set_downtime{
                    execute: "migrate_set_downtime".to_string(),
                    value:value
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct migrate_set_speed {
            execute: String,
            pub value:String
        }
        impl migrate_set_speed {
            pub fn new(value:String)->migrate_set_speed{
                migrate_set_speed{
                    execute: "migrate_set_speed".to_string(),
                    value:value
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct migrate_set_cache_size {
            execute: String,
            pub value:String
        }
        impl migrate_set_cache_size {
            pub fn new(value:String)->migrate_set_cache_size{
                migrate_set_cache_size{
                    execute: "migrate_set_cache_size".to_string(),
                    value:value
                }
            }
        }
        #
/// @query-migrate-cache-size
///
/// query XBZRLE cache size
///
/// Returns: XBZRLE cache size in bytes
///
/// Since: 1.2
///#
        #[derive(Debug)]
        pub struct query_migrate_cache_size {
            execute: String,
            #[serde(skip_serializing)]
returns:int
        }
        impl query_migrate_cache_size {
            pub fn new(int:String)->query_migrate_cache_size{
                query_migrate_cache_size{
                    execute: "query_migrate_cache_size".to_string(),
                    returns:int
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct ObjectPropertyInfo{
                pub name:String,pub type:String
            }
            #
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
///#
        #[derive(Debug)]
        pub struct qom_list {
            execute: String,
            pub path:String
        }
        impl qom_list {
            pub fn new(path:String)->qom_list{
                qom_list{
                    execute: "qom_list".to_string(),
                    path:path
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct qom_get {
            execute: String,
            pub path:String,pub property:String,#[serde(skip_serializing)]
returns:**,gen: bool
        }
        impl qom_get {
            pub fn new(path:String,property:String,**:String)->qom_get{
                qom_get{
                    execute: "qom_get".to_string(),
                    path:path,property:property,returns:**
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct qom_set {
            execute: String,
            pub path:String,pub property:String,pub value:String,gen: bool
        }
        impl qom_set {
            pub fn new(path:String,property:String,value:String)->qom_set{
                qom_set{
                    execute: "qom_set".to_string(),
                    path:path,property:property,value:value
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct set_password {
            execute: String,
            pub connected:String,pub password:String,pub protocol:String
        }
        impl set_password {
            pub fn new(connected:String,password:String,protocol:String)->set_password{
                set_password{
                    execute: "set_password".to_string(),
                    connected:connected,password:password,protocol:protocol
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct expire_password {
            execute: String,
            pub protocol:String,pub time:String
        }
        impl expire_password {
            pub fn new(protocol:String,time:String)->expire_password{
                expire_password{
                    execute: "expire_password".to_string(),
                    protocol:protocol,time:time
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct change_vnc_password {
            execute: String,
            pub password:String
        }
        impl change_vnc_password {
            pub fn new(password:String)->change_vnc_password{
                change_vnc_password{
                    execute: "change_vnc_password".to_string(),
                    password:password
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct change {
            execute: String,
            pub arg:String,pub device:String,pub target:String
        }
        impl change {
            pub fn new(arg:String,device:String,target:String)->change{
                change{
                    execute: "change".to_string(),
                    arg:arg,device:device,target:target
                }
            }
        }
        #
/// @ObjectTypeInfo:
///
/// This structure describes a search result from @qom-list-types
///
/// @name: the type name found in the search
///
/// Since: 1.1
///
/// Notes: This command is experimental and may change syntax in future releases.
///#
            #[derive(Debug)]
            pub struct ObjectTypeInfo{
                pub name:String
            }
            #
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
///#
        #[derive(Debug)]
        pub struct qom_list_types {
            execute: String,
            pub abstract:String,pub implements:String
        }
        impl qom_list_types {
            pub fn new(abstract:String,implements:String)->qom_list_types{
                qom_list_types{
                    execute: "qom_list_types".to_string(),
                    abstract:abstract,implements:implements
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct DevicePropertyInfo{
                pub *description:String,pub name:String,pub type:String
            }
            #
/// @device-list-properties:
///
/// List properties associated with a device.
///
/// @typename: the type name of a device
///
/// Returns: a list of DevicePropertyInfo describing a devices properties
///
/// Since: 1.2
///#
        #[derive(Debug)]
        pub struct device_list_properties {
            execute: String,
            pub typename:String
        }
        impl device_list_properties {
            pub fn new(typename:String)->device_list_properties{
                device_list_properties{
                    execute: "device_list_properties".to_string(),
                    typename:typename
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct migrate {
            execute: String,
            pub blk:String,pub detach:String,pub inc:String,pub uri:String
        }
        impl migrate {
            pub fn new(blk:String,detach:String,inc:String,uri:String)->migrate{
                migrate{
                    execute: "migrate".to_string(),
                    blk:blk,detach:detach,inc:inc,uri:uri
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct migrate_incoming {
            execute: String,
            pub uri:String
        }
        impl migrate_incoming {
            pub fn new(uri:String)->migrate_incoming{
                migrate_incoming{
                    execute: "migrate_incoming".to_string(),
                    uri:uri
                }
            }
        }
         @xen-save-devices-state:
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
///#
        #[derive(Debug)]
        pub struct xen_save_devices_state {
            execute: String,
            pub filename:String
        }
        impl xen_save_devices_state {
            pub fn new(filename:String)->xen_save_devices_state{
                xen_save_devices_state{
                    execute: "xen_save_devices_state".to_string(),
                    filename:filename
                }
            }
        }
        #
/// @xen-set-global-dirty-log
///
/// Enable or disable the global dirty log mode.
///
/// @enable: true to enable, false to disable.
///
/// Returns: nothing
///
/// Since: 1.3
///#
        #[derive(Debug)]
        pub struct xen_set_global_dirty_log {
            execute: String,
            pub enable:String
        }
        impl xen_set_global_dirty_log {
            pub fn new(enable:String)->xen_set_global_dirty_log{
                xen_set_global_dirty_log{
                    execute: "xen_set_global_dirty_log".to_string(),
                    enable:enable
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct device_del {
            execute: String,
            pub id:String
        }
        impl device_del {
            pub fn new(id:String)->device_del{
                device_del{
                    execute: "device_del".to_string(),
                    id:id
                }
            }
        }
        #
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
///##
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
///#
        #[derive(Debug)]
        pub struct dump_guest_memory {
            execute: String,
            pub begin:String,pub format:String,pub length:String,pub paging:String,pub protocol:String
        }
        impl dump_guest_memory {
            pub fn new(begin:String,format:String,length:String,paging:String,protocol:String)->dump_guest_memory{
                dump_guest_memory{
                    execute: "dump_guest_memory".to_string(),
                    begin:begin,format:format,length:length,paging:paging,protocol:protocol
                }
            }
        }
        #
/// @DumpGuestMemoryCapability:
///
/// A list of the available formats for dump-guest-memory
///
/// Since: 2.0
///#
            #[derive(Debug)]
            pub struct DumpGuestMemoryCapability{
                pub formats:Vec
            }
            #
/// @query-dump-guest-memory-capability:
///
/// Returns the available formats for dump-guest-memory
///
/// Returns:  A @DumpGuestMemoryCapability object listing available formats for
///           dump-guest-memory
///
/// Since: 2.0
///#
        #[derive(Debug)]
        pub struct query_dump_guest_memory_capability {
            execute: String,
            #[serde(skip_serializing)]
returns:DumpGuestMemoryCapability
        }
        impl query_dump_guest_memory_capability {
            pub fn new(DumpGuestMemoryCapability:String)->query_dump_guest_memory_capability{
                query_dump_guest_memory_capability{
                    execute: "query_dump_guest_memory_capability".to_string(),
                    returns:DumpGuestMemoryCapability
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct netdev_add {
            execute: String,
            pub props:String,pub id:String,pub type:String,gen: bool
        }
        impl netdev_add {
            pub fn new(props:String,id:String,type:String)->netdev_add{
                netdev_add{
                    execute: "netdev_add".to_string(),
                    props:props,id:id,type:type
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct netdev_del {
            execute: String,
            pub id:String
        }
        impl netdev_del {
            pub fn new(id:String)->netdev_del{
                netdev_del{
                    execute: "netdev_del".to_string(),
                    id:id
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct object_add {
            execute: String,
            pub props:String,pub id:String,pub qom_type:String,gen: bool
        }
        impl object_add {
            pub fn new(props:String,id:String,qom_type:String)->object_add{
                object_add{
                    execute: "object_add".to_string(),
                    props:props,id:id,qom_type:qom_type
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct object_del {
            execute: String,
            pub id:String
        }
        impl object_del {
            pub fn new(id:String)->object_del{
                object_del{
                    execute: "object_del".to_string(),
                    id:id
                }
            }
        }
        #
/// @NetdevNoneOptions
///
/// Use it alone to have zero network devices.
///
/// Since 1.2
///#
            #[derive(Debug)]
            pub struct NetdevNoneOptions{
                
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetLegacyNicOptions{
                pub *addr:String,pub *macaddr:String,pub *model:String,pub *netdev:String,pub *vectors:String
            }
            #
/// @String
///
/// A fat type wrapping "str", to be embedded in lists.
///
/// Since 1.2
///#
            #[derive(Debug)]
            pub struct String{
                pub str:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetdevUserOptions{
                pub *bootfile:String,pub *dhcpstart:String,pub *dns:String,pub *dnssearch:Vec,pub *guestfwd:Vec,pub *host:String,pub *hostfwd:Vec,pub *hostname:String,pub *ip:String,pub *net:String,pub *restrict:String,pub *smb:String,pub *smbserver:String,pub *tftp:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetdevTapOptions{
                pub *downscript:String,pub *fd:String,pub *fds:String,pub *helper:String,pub *ifname:String,pub *queues:String,pub *script:String,pub *sndbuf:String,pub *vhost:String,pub *vhostfd:String,pub *vhostfds:String,pub *vhostforce:String,pub *vnet_hdr:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetdevSocketOptions{
                pub *connect:String,pub *fd:String,pub *listen:String,pub *localaddr:String,pub *mcast:String,pub *udp:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetdevL2TPv3Options{
                pub *cookie64:String,pub *counter:String,pub *dstport:String,pub *ipv6:String,pub *offset:String,pub *pincounter:String,pub *rxcookie:String,pub *rxsession:String,pub *srcport:String,pub *txcookie:String,pub *udp:String,pub dst:String,pub src:String,pub txsession:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetdevVdeOptions{
                pub *group:String,pub *mode:String,pub *port:String,pub *sock:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetdevDumpOptions{
                pub *file:String,pub *len:String
            }
            #
/// @NetdevBridgeOptions
///
/// Connect a host TAP network interface to a host bridge device.
///
/// @br: #optional bridge name
///
/// @helper: #optional command to execute to configure bridge
///
/// Since 1.2
///#
            #[derive(Debug)]
            pub struct NetdevBridgeOptions{
                pub *br:String,pub *helper:String
            }
            #
/// @NetdevHubPortOptions
///
/// Connect two or more net clients through a software hub.
///
/// @hubid: hub identifier number
///
/// Since 1.2
///#
            #[derive(Debug)]
            pub struct NetdevHubPortOptions{
                pub hubid:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetdevNetmapOptions{
                pub *devname:String,pub ifname:String
            }
            #
/// @NetdevVhostUserOptions
///
/// Vhost-user network backend
///
/// @chardev: name of a unix socket chardev
///
/// @vhostforce: #optional vhost on for non-MSIX virtio guests (default: false).
///
/// Since 2.1
///#
            #[derive(Debug)]
            pub struct NetdevVhostUserOptions{
                pub *vhostforce:String,pub chardev:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct NetLegacy{
                pub *id:String,pub *name:String,pub *vlan:String,pub opts:String
            }
            #
/// @Netdev
///
/// Captures the configuration of a network device.
///
/// @id: identifier for monitor commands.
///
/// @opts: device type specific properties
///
/// Since 1.2
///#
            #[derive(Debug)]
            pub struct Netdev{
                pub id:String,pub opts:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct InetSocketAddress{
                pub *ipv4:String,pub *ipv6:String,pub *to:String,pub host:String,pub port:String
            }
            #
/// @UnixSocketAddress
///
/// Captures a socket address in the local ("Unix socket") namespace.
///
/// @path: filesystem path to use
///
/// Since 1.3
///#
            #[derive(Debug)]
            pub struct UnixSocketAddress{
                pub path:String
            }
            #
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
///#
        #[derive(Debug)]
        pub struct getfd {
            execute: String,
            pub fdname:String
        }
        impl getfd {
            pub fn new(fdname:String)->getfd{
                getfd{
                    execute: "getfd".to_string(),
                    fdname:fdname
                }
            }
        }
        #
/// @closefd:
///
/// Close a file descriptor previously passed via SCM rights
///
/// @fdname: file descriptor name
///
/// Returns: Nothing on success
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct closefd {
            execute: String,
            pub fdname:String
        }
        impl closefd {
            pub fn new(fdname:String)->closefd{
                closefd{
                    execute: "closefd".to_string(),
                    fdname:fdname
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct MachineInfo{
                pub *alias:String,pub *is_default:String,pub cpu_max:String,pub name:String
            }
            #
/// @query-machines:
///
/// Return a list of supported machines
///
/// Returns: a list of MachineInfo
///
/// Since: 1.2.0
///#
        #[derive(Debug)]
        pub struct query_machines {
            execute: String,
            
        }
        impl query_machines {
            pub fn new()->query_machines{
                query_machines{
                    execute: "query_machines".to_string(),
                    
                }
            }
        }
        #
/// @CpuDefinitionInfo:
///
/// Virtual CPU definition.
///
/// @name: the name of the CPU definition
///
/// Since: 1.2.0
///#
            #[derive(Debug)]
            pub struct CpuDefinitionInfo{
                pub name:String
            }
            #
/// @query-cpu-definitions:
///
/// Return a list of supported virtual CPU definitions
///
/// Returns: a list of CpuDefInfo
///
/// Since: 1.2.0
///#
        #[derive(Debug)]
        pub struct query_cpu_definitions {
            execute: String,
            
        }
        impl query_cpu_definitions {
            pub fn new()->query_cpu_definitions{
                query_cpu_definitions{
                    execute: "query_cpu_definitions".to_string(),
                    
                }
            }
        }
         @AddfdInfo:
///
/// Information about a file descriptor that was added to an fd set.
///
/// @fdset-id: The ID of the fd set that @fd was added to.
///
/// @fd: The file descriptor that was received via SCM rights and
///      added to the fd set.
///
/// Since: 1.2.0
///#
            #[derive(Debug)]
            pub struct AddfdInfo{
                pub fd:String,pub fdset_id:String
            }
            #
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
///#
        #[derive(Debug)]
        pub struct add_fd {
            execute: String,
            pub fdset_id:String,pub opaque:String,#[serde(skip_serializing)]
returns:AddfdInfo
        }
        impl add_fd {
            pub fn new(fdset_id:String,opaque:String,AddfdInfo:String)->add_fd{
                add_fd{
                    execute: "add_fd".to_string(),
                    fdset_id:fdset_id,opaque:opaque,returns:AddfdInfo
                }
            }
        }
        #
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
///#
        #[derive(Debug)]
        pub struct remove_fd {
            execute: String,
            pub fd:String,pub fdset_id:String
        }
        impl remove_fd {
            pub fn new(fd:String,fdset_id:String)->remove_fd{
                remove_fd{
                    execute: "remove_fd".to_string(),
                    fd:fd,fdset_id:fdset_id
                }
            }
        }
        #
/// @FdsetFdInfo:
///
/// Information about a file descriptor that belongs to an fd set.
///
/// @fd: The file descriptor value.
///
/// @opaque: #optional A free-form string that can be used to describe the fd.
///
/// Since: 1.2.0
///#
            #[derive(Debug)]
            pub struct FdsetFdInfo{
                pub *opaque:String,pub fd:String
            }
            #
/// @FdsetInfo:
///
/// Information about an fd set.
///
/// @fdset-id: The ID of the fd set.
///
/// @fds: A list of file descriptors that belong to this fd set.
///
/// Since: 1.2.0
///#
            #[derive(Debug)]
            pub struct FdsetInfo{
                pub fds:Vec,pub fdset_id:String
            }
            #
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
///#
        #[derive(Debug)]
        pub struct query_fdsets {
            execute: String,
            
        }
        impl query_fdsets {
            pub fn new()->query_fdsets{
                query_fdsets{
                    execute: "query_fdsets".to_string(),
                    
                }
            }
        }
        #
/// @TargetInfo:
///
/// Information describing the QEMU target.
///
/// @arch: the target architecture (eg "x86_64", "i386", etc)
///
/// Since: 1.2.0
///#
            #[derive(Debug)]
            pub struct TargetInfo{
                pub arch:String
            }
            #
/// @query-target:
///
/// Return information about the target for this QEMU
///
/// Returns: TargetInfo
///
/// Since: 1.2.0
///#
        #[derive(Debug)]
        pub struct query_target {
            execute: String,
            #[serde(skip_serializing)]
returns:TargetInfo
        }
        impl query_target {
            pub fn new(TargetInfo:String)->query_target{
                query_target{
                    execute: "query_target".to_string(),
                    returns:TargetInfo
                }
            }
        }
        #
/// @QKeyCode:
///
/// An enumeration of key name.
///
/// This is used by the send-key command.
///
/// Since: 1.3.0
///
/// "unmapped" and "pause" since 2.0
///##
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
///#
        #[derive(Debug)]
        pub struct send_key {
            execute: String,
            pub hold_time:String,pub keys:Vec
        }
        impl send_key {
            pub fn new(hold_time:String,keys:Vec)->send_key{
                send_key{
                    execute: "send_key".to_string(),
                    hold_time:hold_time,keys:keys
                }
            }
        }
        #
/// @screendump:
///
/// Write a PPM of the VGA screen to a file.
///
/// @filename: the path of a new PPM file to store the image
///
/// Returns: Nothing on success
///
/// Since: 0.14.0
///#
        #[derive(Debug)]
        pub struct screendump {
            execute: String,
            pub filename:String
        }
        impl screendump {
            pub fn new(filename:String)->screendump{
                screendump{
                    execute: "screendump".to_string(),
                    filename:filename
                }
            }
        }
        #
/// @ChardevFile:
///
/// Configuration info for file chardevs.
///
/// @in:  #optional The name of the input file
/// @out: The name of the output file
///
/// Since: 1.4
///#
            #[derive(Debug)]
            pub struct ChardevFile{
                pub *in:String,pub out:String
            }
            #
/// @ChardevHostdev:
///
/// Configuration info for device and pipe chardevs.
///
/// @device: The name of the special file for the device,
///          i.e. /dev/ttyS0 on Unix or COM1: on Windows
/// @type: What kind of device this is.
///
/// Since: 1.4
///#
            #[derive(Debug)]
            pub struct ChardevHostdev{
                pub device:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct ChardevSocket{
                pub *nodelay:String,pub *reconnect:String,pub *server:String,pub *telnet:String,pub *wait:String,pub addr:String
            }
            #
/// @ChardevUdp:
///
/// Configuration info for datagram socket chardevs.
///
/// @remote: remote address
/// @local: #optional local address
///
/// Since: 1.5
///#
            #[derive(Debug)]
            pub struct ChardevUdp{
                pub *local:String,pub remote:String
            }
            #
/// @ChardevMux:
///
/// Configuration info for mux chardevs.
///
/// @chardev: name of the base chardev.
///
/// Since: 1.5
///#
            #[derive(Debug)]
            pub struct ChardevMux{
                pub chardev:String
            }
            #
/// @ChardevStdio:
///
/// Configuration info for stdio chardevs.
///
/// @signal: #optional Allow signals (such as SIGINT triggered by ^C)
///          be delivered to qemu.  Default: true in -nographic mode,
///          false otherwise.
///
/// Since: 1.5
///#
            #[derive(Debug)]
            pub struct ChardevStdio{
                pub *signal:String
            }
            #
/// @ChardevSpiceChannel:
///
/// Configuration info for spice vm channel chardevs.
///
/// @type: kind of channel (for example vdagent).
///
/// Since: 1.5
///#
            #[derive(Debug)]
            pub struct ChardevSpiceChannel{
                pub type:String
            }
            #
/// @ChardevSpicePort:
///
/// Configuration info for spice port chardevs.
///
/// @fqdn: name of the channel (see docs/spice-port-fqdn.txt)
///
/// Since: 1.5
///#
            #[derive(Debug)]
            pub struct ChardevSpicePort{
                pub fqdn:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct ChardevVC{
                pub *cols:String,pub *height:String,pub *rows:String,pub *width:String
            }
            #
/// @ChardevRingbuf:
///
/// Configuration info for ring buffer chardevs.
///
/// @size: #optional ring buffer size, must be power of two, default is 65536
///
/// Since: 1.5
///#
            #[derive(Debug)]
            pub struct ChardevRingbuf{
                pub *size:String
            }
            #
/// @ChardevBackend:
///
/// Configuration info for the new chardev backend.
///
/// Since: 1.4 (testdev since 2.2)
///#
            #[derive(Debug)]
            pub struct ChardevDummy{
                
            }
            #
/// @ChardevReturn:
///
/// Return info about the chardev backend just created.
///
/// @pty: #optional name of the slave pseudoterminal device, present if
///       and only if a chardev of type "pty" was created
///
/// Since: 1.4
///#
            #[derive(Debug)]
            pub struct ChardevReturn{
                pub *pty:String
            }
            #
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
///#
        #[derive(Debug)]
        pub struct chardev_add {
            execute: String,
            pub backend:String,pub id:String,#[serde(skip_serializing)]
returns:ChardevReturn
        }
        impl chardev_add {
            pub fn new(backend:String,id:String,ChardevReturn:String)->chardev_add{
                chardev_add{
                    execute: "chardev_add".to_string(),
                    backend:backend,id:id,returns:ChardevReturn
                }
            }
        }
        #
/// @chardev-remove:
///
/// Remove a character device backend
///
/// @id: the chardev"s ID, must exist and not be in use
///
/// Returns: Nothing on success
///
/// Since: 1.4
///#
        #[derive(Debug)]
        pub struct chardev_remove {
            execute: String,
            pub id:String
        }
        impl chardev_remove {
            pub fn new(id:String)->chardev_remove{
                chardev_remove{
                    execute: "chardev_remove".to_string(),
                    id:id
                }
            }
        }
        #
/// @TpmModel:
///
/// An enumeration of TPM models
///
/// @tpm-tis: TPM TIS model
///
/// Since: 1.5
///##
/// @query-tpm-models:
///
/// Return a list of supported TPM models
///
/// Returns: a list of TpmModel
///
/// Since: 1.5
///#
        #[derive(Debug)]
        pub struct query_tpm_models {
            execute: String,
            
        }
        impl query_tpm_models {
            pub fn new()->query_tpm_models{
                query_tpm_models{
                    execute: "query_tpm_models".to_string(),
                    
                }
            }
        }
        #
/// @TpmType:
///
/// An enumeration of TPM types
///
/// @passthrough: TPM passthrough type
///
/// Since: 1.5
///##
/// @query-tpm-types:
///
/// Return a list of supported TPM types
///
/// Returns: a list of TpmType
///
/// Since: 1.5
///#
        #[derive(Debug)]
        pub struct query_tpm_types {
            execute: String,
            
        }
        impl query_tpm_types {
            pub fn new()->query_tpm_types{
                query_tpm_types{
                    execute: "query_tpm_types".to_string(),
                    
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct TPMPassthroughOptions{
                pub *cancel_path:String,pub *path:String
            }
            #
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
///#
            #[derive(Debug)]
            pub struct TPMInfo{
                pub id:String,pub model:String,pub options:String
            }
            #
/// @query-tpm:
///
/// Return information about the TPM device
///
/// Returns: @TPMInfo on success
///
/// Since: 1.5
///#
        #[derive(Debug)]
        pub struct query_tpm {
            execute: String,
            
        }
        impl query_tpm {
            pub fn new()->query_tpm{
                query_tpm{
                    execute: "query_tpm".to_string(),
                    
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct AcpiTableOptions{
                pub *asl_compiler_id:String,pub *asl_compiler_rev:String,pub *data:String,pub *file:String,pub *oem_id:String,pub *oem_rev:String,pub *oem_table_id:String,pub *rev:String,pub *sig:String
            }
            #
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
///##
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
///#
            #[derive(Debug)]
            pub struct CommandLineParameterInfo{
                pub *default:String,pub *help:String,pub name:String,pub type:String
            }
            #
/// @CommandLineOptionInfo:
///
/// Details about a command line option, including its list of parameter details
///
/// @option: option name
///
/// @parameters: an array of @CommandLineParameterInfo
///
/// Since 1.5
///#
            #[derive(Debug)]
            pub struct CommandLineOptionInfo{
                pub option:String,pub parameters:Vec
            }
            #
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
///#
        #[derive(Debug)]
        pub struct query_command_line_options {
            execute: String,
            pub option:String
        }
        impl query_command_line_options {
            pub fn new(option:String)->query_command_line_options{
                query_command_line_options{
                    execute: "query_command_line_options".to_string(),
                    option:option
                }
            }
        }
        #
/// @X86CPURegister32
///
/// A X86 32-bit register
///
/// Since: 1.5
///##
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
///#
            #[derive(Debug)]
            pub struct X86CPUFeatureWordInfo{
                pub *cpuid_input_ecx:String,pub cpuid_input_eax:String,pub cpuid_register:String,pub features:String
            }
            #
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
///##
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
///#
///{ "struct": "RxFilterInfo",
/// "data": {
///   "name":               "str",
///   "promiscuous":        "bool",
///   "multicast":          "RxState",
///   "unicast":            "RxState",
///   "vlan":               "RxState",
///   "broadcast-allowed":  "bool",
///   "multicast-overflow": "bool",
///   "unicast-overflow":   "bool",
///   "main-mac":           "str",
///   "vlan-table":         ["int"],
///   "unicast-table":      ["str"],
///   "multicast-table":    ["str"] }}
///##
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
///#
        #[derive(Debug)]
        pub struct query_rx_filter {
            execute: String,
            pub name:String
        }
        impl query_rx_filter {
            pub fn new(name:String)->query_rx_filter{
                query_rx_filter{
                    execute: "query_rx_filter".to_string(),
                    name:name
                }
            }
        }
        #
/// @InputButton
///
/// Button of a pointer input device (mouse, tablet).
///
/// Since: 2.0
///##
/// @InputButton
///
/// Position axis of a pointer input device (mouse, tablet).
///
/// Since: 2.0
///##
/// @InputKeyEvent
///
/// Keyboard input event.
///
/// @key:    Which key this event is for.
/// @down:   True for key-down and false for key-up events.
///
/// Since: 2.0
///#
            #[derive(Debug)]
            pub struct InputKeyEvent{
                pub down:String,pub key:String
            }
            #
/// @InputBtnEvent
///
/// Pointer button input event.
///
/// @button: Which button this event is for.
/// @down:   True for key-down and false for key-up events.
///
/// Since: 2.0
///#
            #[derive(Debug)]
            pub struct InputBtnEvent{
                pub button:String,pub down:String
            }
            #
/// @InputMoveEvent
///
/// Pointer motion input event.
///
/// @axis:   Which axis is referenced by @value.
/// @value:  Pointer position.  For absolute coordinates the
///          valid range is 0 -> 0x7ffff
///
/// Since: 2.0
///#
            #[derive(Debug)]
            pub struct InputMoveEvent{
                pub axis:String,pub value:String
            }
            #
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
///#
/// @events: List of InputEvent union.
///
/// Returns: Nothing on success.
///
/// Since: 2.2
///
/// Note: this command is experimental, and not a stable API.
///
///#
        #[derive(Debug)]
        pub struct x_input_send_event {
            execute: String,
            pub console:String,pub events:Vec
        }
        impl x_input_send_event {
            pub fn new(console:String,events:Vec)->x_input_send_event{
                x_input_send_event{
                    execute: "x_input_send_event".to_string(),
                    console:console,events:events
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct NumaNodeOptions{
                pub *cpus:Vec,pub *mem:String,pub *memdev:String,pub *nodeid:String
            }
            #
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
///##
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
///#
///{ "struct": "Memdev",
/// "data": {
///   "size":       "size",
///   "merge":      "bool",
///   "dump":       "bool",
///   "prealloc":   "bool",
///   "host-nodes": ["uint16"],
///   "policy":     "HostMemPolicy" }}
///##
/// @query-memdev:
///
/// Returns information for all memory backends.
///
/// Returns: a list of @Memdev.
///
/// Since: 2.1
///#
        #[derive(Debug)]
        pub struct query_memdev {
            execute: String,
            
        }
        impl query_memdev {
            pub fn new()->query_memdev{
                query_memdev{
                    execute: "query_memdev".to_string(),
                    
                }
            }
        }
        #
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
///#
            #[derive(Debug)]
            pub struct PCDIMMDeviceInfo{
                pub *id:String,pub addr:String,pub hotpluggable:String,pub hotplugged:String,pub memdev:String,pub node:String,pub size:String,pub slot:String
            }
            #
/// @query-memory-devices
///
/// Lists available memory devices and their state
///
/// Since: 2.1
///#
        #[derive(Debug)]
        pub struct query_memory_devices {
            execute: String,
            
        }
        impl query_memory_devices {
            pub fn new()->query_memory_devices{
                query_memory_devices{
                    execute: "query_memory_devices".to_string(),
                    
                }
            }
        }
        # @ACPISlotType
///
/// @DIMM: memory slot
///# @ACPIOSTInfo
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
///#
            #[derive(Debug)]
            pub struct ACPIOSTInfo{
                pub *device:String,pub slot:String,pub slot_type:String,pub source:String,pub status:String
            }
            #
/// @query-acpi-ospm-status
///
/// Lists ACPI OSPM status of ACPI device objects,
/// which might be reported via _OST method
///
/// Since: 2.1
///#
        #[derive(Debug)]
        pub struct query_acpi_ospm_status {
            execute: String,
            
        }
        impl query_acpi_ospm_status {
            pub fn new()->query_acpi_ospm_status{
                query_acpi_ospm_status{
                    execute: "query_acpi_ospm_status".to_string(),
                    
                }
            }
        }
        #
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
///##
/// @IoOperationType
///
/// An enumeration of the I/O operation types
///
/// @read: read operation
///
/// @write: write operation
///
/// Since: 2.1
///##
/// @GuestPanicAction
///
/// An enumeration of the actions taken when guest OS panic is detected
///
/// @pause: system pauses
///
/// Since: 2.1
///##
/// @rtc-reset-reinjection
///
/// This command will reset the RTC interrupt reinjection backlog.
/// Can be used if another mechanism to synchronize guest time
/// is in effect, for example QEMU guest agent"s guest-set-time
/// command.
///
/// Since: 2.1
///#
        #[derive(Debug)]
        pub struct rtc_reset_reinjection {
            execute: String,
            
        }
        impl rtc_reset_reinjection {
            pub fn new()->rtc_reset_reinjection{
                rtc_reset_reinjection{
                    execute: "rtc_reset_reinjection".to_string(),
                    
                }
            }
        }
                      rtc_reset_reinjection{
                    execute: "rtc_reset_reinjection".to_string(),
                    
                }
            }
        }
         physical address, where device is mapped
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
///#
            #[derive(Debug, Serialize, Deserialize)]
            pub struct PCDIMMDeviceInfo{
                pub *id:String,pub addr:String,pub hotpluggable:String,pub hotplugged:String,pub memdev:String,pub node:String,pub size:String,pub slot:String
            }
            #
/// @query-memory-devices
///
/// Lists available memory devices and their state
///
/// Since: 2.1
///#
        #[derive(Debug, Serialize, Deserialize)]
        pub struct query_memory_devices {
            execute: String,
            
        }
        impl query_memory_devices {
            pub fn new()->query_memory_devices{
                query_memory_devices{
                    execute: "query_memory_devices".to_string(),
                    
                }
            }
        }
        # @ACPISlotType
///
/// @DIMM: memory slot
///# @ACPIOSTInfo
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
///#
            #[derive(Debug, Serialize, Deserialize)]
            pub struct ACPIOSTInfo{
                pub *device:String,pub slot:String,pub slot_type:String,pub source:String,pub status:String
            }
            #
/// @query-acpi-ospm-status
///
/// Lists ACPI OSPM status of ACPI device objects,
/// which might be reported via _OST method
///
/// Since: 2.1
///#
        #[derive(Debug, Serialize, Deserialize)]
        pub struct query_acpi_ospm_status {
            execute: String,
            
        }
        impl query_acpi_ospm_status {
            pub fn new()->query_acpi_ospm_status{
                query_acpi_ospm_status{
                    execute: "query_acpi_ospm_status".to_string(),
                    
                }
            }
        }
        #
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
///##
/// @IoOperationType
///
/// An enumeration of the I/O operation types
///
/// @read: read operation
///
/// @write: write operation
///
/// Since: 2.1
///##
/// @GuestPanicAction
///
/// An enumeration of the actions taken when guest OS panic is detected
///
/// @pause: system pauses
///
/// Since: 2.1
///##
/// @rtc-reset-reinjection
///
/// This command will reset the RTC interrupt reinjection backlog.
/// Can be used if another mechanism to synchronize guest time
/// is in effect, for example QEMU guest agent"s guest-set-time
/// command.
///
/// Since: 2.1
///#
        #[derive(Debug, Serialize, Deserialize)]
        pub struct rtc_reset_reinjection {
            execute: String,
            
        }
        impl rtc_reset_reinjection {
            pub fn new()->rtc_reset_reinjection{
                rtc_reset_reinjection{
                    execute: "rtc_reset_reinjection".to_string(),
                    
                }
            }
        }
        