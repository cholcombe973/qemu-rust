
            #[derive(Debug)]
            pub enum BiosAtaTranslation {
                auto,none,lba,large,rechs
            }
            
            #[derive(Debug)]
            pub enum BlockDeviceIoStatus {
                ok,failed,nospace
            }
            
            #[derive(Debug)]
            pub enum BlockdevOnError {
                report,ignore,enospc,stop
            }
            
            #[derive(Debug)]
            pub enum MirrorSyncMode {
                top,full,none,dirty_bitmap
            }
            
            #[derive(Debug)]
            pub enum BlockJobType {
                commit,stream,mirror,backup
            }
            
            #[derive(Debug)]
            pub enum NewImageMode {
                existing,absolute_paths
            }
            
            #[derive(Debug)]
            pub enum BlockdevDiscardOptions {
                ignore,unmap
            }
            
            #[derive(Debug)]
            pub enum BlockdevDetectZeroesOptions {
                off,on,unmap
            }
            
            #[derive(Debug)]
            pub enum BlockdevAioOptions {
                threads,native
            }
            
            #[derive(Debug)]
            pub enum BlockdevDriver {
                archipelago,blkdebug,blkverify,bochs,cloop,dmg,file,ftp,ftps,host_cdrom,host_device,host_floppy,http,https,null_aio,null_co,parallels,qcow,qcow2,qed,quorum,raw,tftp,vdi,vhdx,vmdk,vpc,vvfat
            }
            
            #[derive(Debug)]
            pub enum Qcow2OverlapCheckMode {
                none,constant,cached,all
            }
            
            #[derive(Debug)]
            pub enum BlkdebugEvent {
                l1_update,l1_grow_alloc_table,l1_grow_write_table,l1_grow_activate_table,l2_load,l2_update,l2_update_compressed,l2_alloc_cow_read,l2_alloc_write,read_aio,read_backing_aio,read_compressed,write_aio,write_compressed,vmstate_load,vmstate_save,cow_read,cow_write,reftable_load,reftable_grow,reftable_update,refblock_load,refblock_update,refblock_update_part,refblock_alloc,refblock_alloc_hookup,refblock_alloc_write,refblock_alloc_write_blocks,refblock_alloc_write_table,refblock_alloc_switch_table,cluster_alloc,cluster_alloc_bytes,cluster_free,flush_to_os,flush_to_disk,pwritev_rmw_head,pwritev_rmw_after_head,pwritev_rmw_tail,pwritev_rmw_after_tail,pwritev,pwritev_zero,pwritev_done,empty_image_prepare
            }
            
            #[derive(Debug)]
            pub enum QuorumReadPattern {
                quorum,fifo
            }
            
            #[derive(Debug)]
            pub enum BlockErrorAction {
                ignore,report,stop
            }
            
            #[derive(Debug)]
            pub enum PreallocMode {
                off,metadata,falloc,full
            }
            
            #[derive(Debug)]
            pub enum ErrorClass {
                GenericError,CommandNotFound,DeviceEncrypted,DeviceNotActive,DeviceNotFound,KVMMissingCap
            }
            
            #[derive(Debug)]
            pub enum OnOffAuto {
                auto,on,off
            }
            
            #[derive(Debug)]
            pub enum TraceEventState {
                unavailable,disabled,enabled
            }
            
            #[derive(Debug)]
            pub enum LostTickPolicy {
                discard,delay,merge,slew
            }
            
            #[derive(Debug)]
            pub enum RunState {
                debug,inmigrate,internal_error,io_error,paused,postmigrate,prelaunch,finish_migrate,restore_vm,running,save_vm,shutdown,suspended,watchdog,guest_panicked
            }
            
            #[derive(Debug)]
            pub enum DataFormat {
                utf8,base64
            }
            
            #[derive(Debug)]
            pub enum MigrationStatus {
                none,setup,cancelling,cancelled,active,completed,failed
            }
            
            #[derive(Debug)]
            pub enum MigrationCapability {
                xbzrle,rdma_pin_all,auto_converge,zero_blocks,compress
            }
            
            #[derive(Debug)]
            pub enum MigrationParameter {
                compress_level,compress_threads,decompress_threads
            }
            
            #[derive(Debug)]
            pub enum NetworkAddressFamily {
                ipv4,ipv6,unix,unknown
            }
            
            #[derive(Debug)]
            pub enum VncPrimaryAuth {
                none,vnc,ra2,ra2ne,tight,ultra,tls,vencrypt,sasl
            }
            
            #[derive(Debug)]
            pub enum VncVencryptSubAuth {
                plain,tls_none,x509_none,tls_vnc,x509_vnc,tls_plain,x509_plain,tls_sasl,x509_sasl
            }
            
            #[derive(Debug)]
            pub enum SpiceQueryMouseMode {
                client,server,unknown
            }
            
            #[derive(Debug)]
            pub enum DumpGuestMemoryFormat {
                elf,kdump_zlib,kdump_lzo,kdump_snappy
            }
            
            #[derive(Debug)]
            pub enum QKeyCode {
                unmapped,shift,shift_r,alt,alt_r,altgr,altgr_r,ctrl,ctrl_r,menu,esc,qemu_1,qemu_2,qemu_3,qemu_4,qemu_5,qemu_6,qemu_7,qemu_8,qemu_9,qemu_0,minus,equal,backspace,tab,q,w,e,r,t,y,u,i,o,p,bracket_left,bracket_right,ret,a,s,d,f,g,h,j,k,l,semicolon,apostrophe,grave_accent,backslash,z,x,c,v,b,n,m,comma,dot,slash,asterisk,spc,caps_lock,f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,num_lock,scroll_lock,kp_divide,kp_multiply,kp_subtract,kp_add,kp_enter,kp_decimal,sysrq,kp_0,kp_1,kp_2,kp_3,kp_4,kp_5,kp_6,kp_7,kp_8,kp_9,less,f11,f12,print,home,pgup,pgdn,end,left,up,down,right,insert,delete,stop,again,props,undo,front,copy,open,paste,find,cut,lf,help,meta_l,meta_r,compose,pause
            }
            
            #[derive(Debug)]
            pub enum TpmModel {
                tpm_tis
            }
            
            #[derive(Debug)]
            pub enum TpmType {
                passthrough
            }
            
            #[derive(Debug)]
            pub enum CommandLineParameterType {
                string,boolean,number,size
            }
            
            #[derive(Debug)]
            pub enum X86CPURegister32 {
                EAX,EBX,ECX,EDX,ESP,EBP,ESI,EDI
            }
            
            #[derive(Debug)]
            pub enum RxState {
                normal,none,all
            }
            
            #[derive(Debug)]
            pub enum InputButton {
                Left,Middle,Right,WheelUp,WheelDown
            }
            
            #[derive(Debug)]
            pub enum InputAxis {
                X,Y
            }
            
            #[derive(Debug)]
            pub enum HostMemPolicy {
                default,preferred,bind,interleave
            }
            
            #[derive(Debug)]
            pub enum ACPISlotType {
                DIMM
            }
            
            #[derive(Debug)]
            pub enum WatchdogExpirationAction {
                reset,shutdown,poweroff,pause,debug,none
            }
            
            #[derive(Debug)]
            pub enum IoOperationType {
                read,write
            }
            
            #[derive(Debug)]
            pub enum GuestPanicAction {
                pause
            }
            