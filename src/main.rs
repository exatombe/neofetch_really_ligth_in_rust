use sysinfo::{System, SystemExt, UserExt, ProcessExt, Uid, CpuExt};
use colored::{Colorize, Color};
fn main(){
    let mut sys: System = System::new_all();

    sys.refresh_all();
    println!("=> system:"); 
// RAM and swap information:


fn option_to_string(option: Option<&str>) -> &str{
    match option {
        Some(s) => s.clone(),
        None => "unknown",
    }
}  

fn option_to_uid(option: Option<&Uid>) -> Uid {
    match option {
        Some(s) => s.clone(),
        None => todo!(),
    }
}

fn convert_second_to_humantime(seconds: u64) -> String{
    let mut seconds = seconds;
    let mut minutes = 0;
    let mut hours = 0;
    let mut days = 0;
    let mut years = 0;
    let mut months = 0;
    let mut weeks = 0;
    let mut result = String::new();
    if seconds >= 60 {
        minutes = seconds / 60;
        seconds = seconds % 60;
    }
    if minutes >= 60 {
        hours = minutes / 60;
        minutes = minutes % 60;
    }
    if hours >= 24 {
        days = hours / 24;
        hours = hours % 24;
    }
    if days >= 7 {
        weeks = days / 7;
        days = days % 7;
    }
    if weeks >= 4 {
        months = weeks / 4;
        weeks = weeks % 4;
    }
    if months >= 12 {
        years = months / 12;
        months = months % 12;
    }
    if years > 0 {
        result.push_str(&format!("{} years ", years));
    }
    if months > 0 {
        result.push_str(&format!("{} months ", months));
    }
    if weeks > 0 {
        result.push_str(&format!("{} weeks ", weeks));
    }
    if days > 0 {
        result.push_str(&format!("{} days ", days));
    }
    if hours > 0 {
        result.push_str(&format!("{} hours ", hours));
    }
    if minutes > 0 {
        result.push_str(&format!("{} minutes ", minutes));
    }
    if seconds > 0 {
        result.push_str(&format!("{} seconds ", seconds));
    }
    return result;
}

// get current process id 
let pid = sysinfo::get_current_pid().unwrap();
// get current process
let process = sys.process(pid).unwrap();

let user = sys.get_user_by_id(&option_to_uid(process.user_id()));
println!("{}{}{}", user.unwrap().name().blue(),"@".blue(),option_to_string(sys.host_name().as_deref()).blue());
 // Display system information:
println!("{}           {}","OS:".blue(), option_to_string(sys.long_os_version().as_deref()));
println!("{}       {}","Kernel:".blue(), option_to_string(sys.os_version().as_deref()));
println!("{}       {}","Uptime:".blue(), convert_second_to_humantime(sys.uptime()));
println!("{}    {}","Processes:".blue(), sys.processes().len());
println!("{}        {}","Users:".blue(),sys.users().len());
println!("{}    {}","Boot time:".blue(), convert_second_to_humantime(sys.boot_time()));
sys.refresh_cpu(); // Refreshing CPU information.
for cpu in sys.cpus() {
    println!("{}       {}","CPU:".blue(),cpu.brand());
    break;
}
println!("{} {} Mib / {} Mib","Memory:".blue(), sys.used_memory()/1024/1024, sys.total_memory()/1024/1024);


}