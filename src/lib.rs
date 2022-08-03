//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

extern crate procfs;
use procfs::{process::{FDTarget, Stat}, net::TcpNetEntry};
use std::collections::HashMap;

//collection of IO streams
pub struct Observer {
    pub data_streams: Vec<Box<dyn NetIO>>,
    pub data: Vec<&'static str>
}

impl  Observer {
    pub fn add_data_stream(&mut self, data_stream: Box<dyn NetIO>){

        let _ = &self.data_streams.push(data_stream);
        
    }
    pub fn delete_data_stream(){

    }

    //TODO get table data from data_streams
    pub fn get_data(&self) -> String{

        for x in &self.data_streams  {
            x.get_data()
        }

        let mut s: String = "".to_owned();
        let b = s.as_str();
        return b.to_string();
    }

    pub fn new() -> Observer  {
        Observer{
            data_streams: Vec::new(),
            data: Vec::new()
        }
    }
}

trait TObserver<T>{
    fn add_data_stream(&mut self, data_stream: &IOStream<T>);
    fn remove_data_stream(&mut self, data_stream: &IOStream<T>);
    fn get_data(&mut self);
}

trait IOStreams<T> {
    fn get_data_one(&self) -> &Vec<T>;
}

//get network data and send to observer
pub struct IOStream<T> {
    id: i32,
    data_source: T,
}


pub enum NetType {
    TCP4,
    TCP6,
    UDP4,
    UDP6
}

pub trait NetIO {
    fn get_data(&self);
}

struct table {
    header: Vec<&'static str>,
    rows: Vec<Vec<String>>,
}


struct TCP4{}

impl NetIO for TCP4{
    fn get_data(&self) {
        let mut data = table{header: Vec::new(), rows: Vec::new()};

        let mut map = get_proc_inode_table();
        let tcp = procfs::net::tcp().unwrap();

        for entry in tcp.into_iter() {
            // find the process (if any) that has an open FD to this entry's inode
            let local_address = format!("{}", entry.local_address);
            let remote_addr = format!("{}", entry.remote_address);
            let state = format!("{:?}", entry.state);
            if let Some(stat) = map.get(&entry.inode) {
                println!(
                    "{:<26} {:<26} {:<15} {:<12} {}/{}",
                    local_address, remote_addr, state, entry.inode, stat.pid, stat.comm
                );
            }
            data.rows.push(vec![entry.local_address.to_string().clone()])
        }
    }
}

pub struct NetFactory;
impl NetFactory{
    pub fn new_net_io(t: &NetType) -> Box<dyn NetIO>{
        match t {
            NetType::TCP4 => Box::new(TCP4{}),
            NetType::TCP6 => todo!(),
            NetType::UDP4 => todo!(),
            NetType::UDP6 => todo!(),
        }
    }
}

fn get_proc_inode_table() -> HashMap<u64, Stat>{
    let all_procs = procfs::process::all_processes().unwrap();
    // build up a map between socket inodes and processes:
    let mut map: HashMap<u64, Stat> = HashMap::new();

    for p in &all_procs{
        
        if let (Ok(stat) ,Ok(fds)) = (p.stat(), p.fd()){
            for fd in fds{
                
                if let FDTarget::Socket(inode) = fd.target{
                    //TODO swap out clone?
                    map.insert(inode, stat.clone());
                };
            }
        }
    }

    return map;
}

fn _te(){

    let mut map = get_proc_inode_table();
        // get the tcp table
    let tcp = procfs::net::tcp().unwrap();
    let tcp6 = procfs::net::tcp6().unwrap();
    let udp = procfs::net::udp().unwrap();
    
    println!(
        "{:<26} {:<26} {:<15} {:<8} {}",
            "Local address", "Remote address", "State", "Inode", "PID/Program name"
        );
    
        for entry in tcp.into_iter().chain(tcp6) {
            // find the process (if any) that has an open FD to this entry's inode
            let local_address = format!("{}", entry.local_address);
            let remote_addr = format!("{}", entry.remote_address);
            let state = format!("{:?}", entry.state);
            if let Some(stat) = map.get(&entry.inode) {
                println!(
                    "{:<26} {:<26} {:<15} {:<12} {}/{}",
                    local_address, remote_addr, state, entry.inode, stat.pid, stat.comm
                );
            } else {
                // We might not always be able to find the process assocated with this socket
                println!(
                    "{:<26} {:<26} {:<15} {:<12} -",
                    local_address, remote_addr, state, entry.inode
                );
            }
        }
}