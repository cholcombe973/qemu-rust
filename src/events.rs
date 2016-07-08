
            #[derive(Debug)]
            pub struct DEVICE_TRAY_MOVED {
                execute: String,
                pub device:String,pub tray_open:String
            }
            
            #[derive(Debug)]
            pub struct BLOCK_IMAGE_CORRUPTED {
                execute: String,
                pub node_name:String,pub offset:String,pub size:String,pub device:String,pub fatal:String,pub msg:String
            }
            
            #[derive(Debug)]
            pub struct BLOCK_IO_ERROR {
                execute: String,
                pub nospace:String,pub action:String,pub device:String,pub operation:String,pub reason:String
            }
            
            #[derive(Debug)]
            pub struct BLOCK_JOB_COMPLETED {
                execute: String,
                pub error:String,pub device:String,pub len:String,pub offset:String,pub speed:String,pub qemu_type:String
            }
            
            #[derive(Debug)]
            pub struct BLOCK_JOB_CANCELLED {
                execute: String,
                pub device:String,pub len:String,pub offset:String,pub speed:String,pub qemu_type:String
            }
            
            #[derive(Debug)]
            pub struct BLOCK_JOB_ERROR {
                execute: String,
                pub action:String,pub device:String,pub operation:String
            }
            
            #[derive(Debug)]
            pub struct BLOCK_JOB_READY {
                execute: String,
                pub device:String,pub len:String,pub offset:String,pub speed:String,pub qemu_type:String
            }
            
            #[derive(Debug)]
            pub struct BLOCK_WRITE_THRESHOLD {
                execute: String,
                pub amount_exceeded:String,pub node_name:String,pub write_threshold:String
            }
            
            #[derive(Debug)]
            pub struct SHUTDOWN {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct POWERDOWN {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct RESET {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct STOP {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct RESUME {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct SUSPEND {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct SUSPEND_DISK {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct WAKEUP {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct RTC_CHANGE {
                execute: String,
                pub offset:String
            }
            
            #[derive(Debug)]
            pub struct WATCHDOG {
                execute: String,
                pub action:String
            }
            
            #[derive(Debug)]
            pub struct DEVICE_DELETED {
                execute: String,
                pub device:String,pub path:String
            }
            
            #[derive(Debug)]
            pub struct NIC_RX_FILTER_CHANGED {
                execute: String,
                pub name:String,pub path:String
            }
            
            #[derive(Debug)]
            pub struct VNC_CONNECTED {
                execute: String,
                pub client:String,pub server:String
            }
            
            #[derive(Debug)]
            pub struct VNC_INITIALIZED {
                execute: String,
                pub client:String,pub server:String
            }
            
            #[derive(Debug)]
            pub struct VNC_DISCONNECTED {
                execute: String,
                pub client:String,pub server:String
            }
            
            #[derive(Debug)]
            pub struct SPICE_CONNECTED {
                execute: String,
                pub client:String,pub server:String
            }
            
            #[derive(Debug)]
            pub struct SPICE_INITIALIZED {
                execute: String,
                pub client:String,pub server:String
            }
            
            #[derive(Debug)]
            pub struct SPICE_DISCONNECTED {
                execute: String,
                pub client:String,pub server:String
            }
            
            #[derive(Debug)]
            pub struct SPICE_MIGRATE_COMPLETED {
                execute: String,
                
            }
            
            #[derive(Debug)]
            pub struct ACPI_DEVICE_OST {
                execute: String,
                pub info:String
            }
            
            #[derive(Debug)]
            pub struct BALLOON_CHANGE {
                execute: String,
                pub actual:String
            }
            
            #[derive(Debug)]
            pub struct GUEST_PANICKED {
                execute: String,
                pub action:String
            }
            
            #[derive(Debug)]
            pub struct QUORUM_FAILURE {
                execute: String,
                pub reference:String,pub sector_num:String,pub sectors_count:String
            }
            
            #[derive(Debug)]
            pub struct QUORUM_REPORT_BAD {
                execute: String,
                pub error:String,pub node_name:String,pub sector_num:String,pub sectors_count:String
            }
            
            #[derive(Debug)]
            pub struct VSERPORT_CHANGE {
                execute: String,
                pub id:String,pub open:String
            }
            
            #[derive(Debug)]
            pub struct MEM_UNPLUG_ERROR {
                execute: String,
                pub device:String,pub msg:String
            }
            