use sysinfo::{System, SystemExt, UserExt, ProcessExt, CpuExt, ComponentExt};
use colored::Colorize;
mod util;

fn main(){
    let mut sys: System = System::new_all();

    sys.refresh_all();
    println!("=> system:"); 
// RAM and swap information:


// get current process id 
let pid = sysinfo::get_current_pid().unwrap();
// get current process
let process = sys.process(pid).unwrap();

let user = sys.get_user_by_id(&util::option_to_uid(process.user_id()));
println!("       {}{}{}", user.unwrap().name().blue(),"@".blue(),util::option_to_string(sys.host_name().as_deref()).blue());
 // Display system information:
println!("---------------------------");
println!("{}           {}","OS:".blue(), util::option_to_string(sys.long_os_version().as_deref()));
println!("{}       {}","Kernel:".blue(), util::option_to_string(sys.os_version().as_deref()));
println!("{}       {}","Uptime:".blue(), util::convert_second_to_humantime(sys.uptime()));
println!("{}    {}","Processes:".blue(), sys.processes().len());
println!("{}        {}","Users:".blue(),sys.users().len());
println!("{}    {}","Boot time:".blue(), util::convert_second_to_humantime(sys.boot_time()));
for component in sys.components() {
    println!("{}    {}","Components:".blue(), component.label());
}
println!("{}   {}","Disks:".blue(), sys.disks().len());
sys.refresh_cpu(); // Refreshing CPU information.
for cpu in sys.cpus() {
println!("{}          {}","CPU:".blue(),cpu.brand());
    break;
}
println!("{}       {} Mib / {} Mib","Memory:".blue(), sys.used_memory()/1024/1024, sys.total_memory()/1024/1024);


}