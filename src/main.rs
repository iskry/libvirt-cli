extern crate clap;
extern crate virt;

use clap::{App, Arg, SubCommand};
use virt::{Connect, Domain};

fn main() {
    let matches = App::new("My VM Manager")
        .version("1.0")
        .about("Manages VMs via libvirt")
        .subcommand(SubCommand::with_name("list").about("Lists all VMs"))
        .subcommand(
            SubCommand::with_name("start").about("Starts a VM").arg(
                Arg::with_name("VM_NAME")
                    .help("The name of the VM to start")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        ("list", Some(_)) => {
            list_vms();
        }
        ("start", Some(sub_m)) => {
            if let Some(vm_name) = sub_m.value_of("VM_NAME") {
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

