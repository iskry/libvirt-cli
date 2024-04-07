extern crate clap;
extern crate virt;

use clap::{Arg, Command};
use virt::connect::Connect;
use virt::domain::Domain;
fn main() {
    let matches = Command::new("My VM Manager")
        .version("1.0")
        .about("Manages VMs via libvirt")
        .subcommand(Command::new("list").about("Lists all VMs"))
        .subcommand(
            Command::new("start").about("Starts a VM").arg(
                Arg::new("VM_NAME")
                    .help("The name of the VM to start")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("list", sub_m)) => {
            list_vms();
        }
        Some(("start", sub_m)) => {
            if let Some(vm_name) = sub_m.get_one::<String>("VM_NAME") {
                start_vm(vm_name);
            }
        }
        _ => {}
    }
}

fn list_vms() {
    let conn = Connect::open("qemu:///system").unwrap();
    let domains = conn.list_all_domains(0).unwrap();

    for domain in domains {
        println!("VM: {}", domain.get_name().unwrap());
    }
}

fn start_vm(vm_name: &str) {
    let conn = Connect::open("qemu:///system").unwrap();
    match Domain::lookup_by_name(&conn, vm_name) {
        Ok(domain) => {
            domain.create().unwrap();
            println!("VM started: {}", vm_name);
        }
        Err(_) => println!("VM not found: {}", vm_name),
    }
}
