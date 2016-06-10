use QApiConnection::*;
use commands::*;

fn main(){
    //I want to print out all guests and their memory usage
    let qemu = QApiConnection::new();
    let get_guests_cmd = GetGuests{};
    let guest_results = qemu.get_guests();

    let dump_cmd = DumpGuestMemory{
        guest: "",
    /*params*/};
    let results = qemu.run_command(qemu);

}
