use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::error::Error;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread;
extern crate sysinfo;
use sysinfo::{System, SystemExt,CpuExt};

struct client{
name:String,
port:String,
candidate:bool, //my state as a server for this particular server
servers:Arc<Mutex<Vec<server>>>,
rem:i32
}

struct request_payload{
request_type: String,
sender_name: String,
receiver_name: String,
img: i32
}
struct server{
ip:String,
confirmed:bool,
port:String,
load:f32,
priority:i32 //to handle servers with the same load
}

struct dos_user{
name: String,
ip: String,
status: bool
}

lazy_static! {
static ref servers: Arc<Mutex<Vec<server>>> = Arc::new(Mutex::new(Vec::new()));
static ref clients: Arc<Mutex<Vec<client>>> = Arc::new(Mutex::new(Vec::new()));
static ref dos: Arc<Mutex<Vec<dos_user>>> = Arc::new(Mutex::new(Vec::new()));
}

//static mut load_count:i32 = 90;
static mut cpu:i32=0;
static mut server_prio:i32 = 2;
static mut server_count:i32 = 0;
static mut Candidate:bool = true;


async fn sender(msg: String, ip:String) -> Result<(), Box<dyn Error>> {
let host_name = "Secret_Agent_Wolf";
let receiver_addr = ip; // Update with the middleware's address

println!("-------------Sending a message");
let socket = UdpSocket::bind("0.0.0.0:0").await?;
//let dest_addr = receiver_addr.parse()?;

let message = msg;
socket.send_to(message.as_bytes(), receiver_addr).await?;
println!("--------> sent message! {}",message);

Ok(())
}

fn Elect_Server(sip:String,c_name:String,c:f32,priority:i32) -> bool{
println!("Electin!!");
let cpu1:f32 = unsafe{cpu as f32};
let mut candidate = true;
println!(" my cpu {}, ur cpu{},   {}",cpu1,c,c_name);
if(c<cpu1)
{
println!("Not Elected!! big load!!");
candidate = false;

}
else if(c==cpu1)
{
if(priority>unsafe{server_prio})
{
println!("Not Elected!! prio");
candidate = false;
}
}
else {
println!("Elected!!");
}

return candidate;
}

fn confirm_Server(sip:String,c_name:String) -> Result<(), Box<dyn Error>> {

println!("confirming reply!!");
for item in clients.clone().lock().unwrap().iter_mut()
{
println!("{} {}",item.name,c_name);
if(item.name.eq(&(c_name.clone())))
{
for server in item.servers.clone().lock().unwrap().iter_mut()
{
if(server.ip.eq(&sip)){
item.rem-=1;
server.confirmed=true;
println!("Item rem is {}",item.rem);
break;
}
} 
}
}

Ok(())
}


// async fn Process_Request(msg:String, sending_client_ip:String) -> Result<(), Box<dyn Error>> {
// println!("Processing"); // 0 1 2 3
// println!("msg {}", msg); // Req_type (encrypt -- ip_request) -- sender_name (@hostname)-- receiver_name -- image
// println!("received client IP: {}", sending_client_ip);
// let seperated_msg = msg.split_whitespace().collect::<Vec<_>>();

// let req_type:String = seperated_msg[0].to_string();
// let sender_name:String = seperated_msg[1].to_string();
// let receiver_name:String = seperated_msg[2].to_string();
// println!("Request Type: {}", req_type);

// // 1) add / update sending client's entry in the Directory of Service
// let mut dos_guard = dos.lock().unwrap();

// if let Some(index) = dos_guard.iter().position(|user| user.name == sender_name) {
// println!("User found. Updating user entry....");
// // let mut dos_guard = dos.lock().unwrap();
// // If the username exists, update the IP and status
// dos_guard[index].ip = sending_client_ip.clone();
// // Assuming there's a status field in DoS_user struct
// dos_guard[index].status = true;
// } else {
// println!("Not found. Inserting entry... ");
// dos_guard.push(dos_user{
// name: sender_name,
// ip: sending_client_ip.clone(),
// status: true
// });
// // let mut dos_guard = dos.lock().unwrap();

// println!("Name {}, IP {}, Status {}", dos_guard[0].name, dos_guard[0].ip, dos_guard[0].status);
// }

// // 2:
// // A: check if receiver client exists in our DoS --> return

// if let Some(index) = dos_guard.iter().position(|user| user.name == receiver_name) {
// println!("DoS Table --> Receiver Client found");
// // let receiver_client_ip:String = dos_guard[index].ip.to_string();
// let echo_back = sender(
// ["Found IP (DoS): ".to_string(), dos_guard[index].ip.clone()].concat(),
// [sending_client_ip.clone(),":8081".to_string()].concat()).await;
// echo_back;
// // st;
// // dos_guard[index].status = true;
// } else {
// //B: else it does not exist --> device not reachable 
// println!("Not found. Inserting entry... ");
// let echo_back = sender(
// "Device Not Reachable".to_string(),
// [sending_client_ip.clone(),":8081".to_string()].concat()).await;
// echo_back;

// // println!("Name {}, IP {}, Status {}", dos_guard[0].name, dos_guard[0].ip, dos_guard[0].status);
// } 


// // if(req_type == "encrypt".to_string()){
// // println!("request type == encrypt!!!")
// // }

// let mut system = System::new_all();
// system.refresh_all();
// unsafe{ cpu = system.cpus()[0].cpu_usage() as i32;} 
// //unsafe{ cpu = 10 as i32;} 
// for item in servers.clone().lock().unwrap().iter_mut()
// {
// let sender = sender([seperated_msg[0].to_string()," ".to_string(),unsafe{cpu.to_string()}].concat(),item.ip.to_string()).await?;
// sender;
// }

// // thread::sleep(Duration::from_secs(1)); //change time (in sec)

// for item in clients.clone().lock().unwrap().iter_mut()
// {
// if(item.name.eq(&(seperated_msg[0].clone())))
// {
// println!("rem :{}",item.rem);
// if(item.rem == 0)
// {
// if(item.candidate)
// {
// println!("replying to client!!");
// let st = sender("Secret Agent Wolf: bruhh, I took ur request".to_string(),"10.7.29.200:8081".to_string()).await;
// st;
// }
// }
// }
// }

// Ok(())
// }


async fn Process_Request(mm: String, sending_client_ip: String) -> Result<(), Box<dyn Error>> {
    println!("Processing");
    println!("mm {}", mm);

    let sep = mm.split_whitespace().collect::<Vec<_>>();
    println!("len{}", sep[0]);

    clients.clone().lock().unwrap().push(client {
        name: sep[0].to_string(),
        port: ":8081".to_string(),
        candidate: true,
        servers: servers.clone(),
        rem: servers.clone().lock().unwrap().len() as i32
    });
    //                                      0                           1                                   2           3
    // println!("msg {}", msg); // -- sender_name (@hostname) -- Req_type (encrypt -- ip_request) -- receiver_name -- image


    let sender_name:String = sep[0].to_string();
    let req_type:String = sep[1].to_string();
    let receiver_name:String = sep[2].to_string();
// println!("Request Type: {}", req_type);


        // 1) add / update sending client's entry in the Directory of Service
    let mut dos_guard = dos.lock().unwrap();

    if let Some(index) = dos_guard.iter().position(|user| user.name == sender_name) {
    println!("User found. Updating user entry....");
    // let mut dos_guard = dos.lock().unwrap();
    // If the username exists, update the IP and status
    dos_guard[index].ip = sending_client_ip.clone();
    // Assuming there's a status field in DoS_user struct
    dos_guard[index].status = true;
    } else {
    println!("sending client Not found. Inserting entry... ");
    dos_guard.push(dos_user{
    name: sender_name,
    ip: sending_client_ip.clone(),
    status: true
    });
    // let mut dos_guard = dos.lock().unwrap();

    println!("Name {}, IP {}, Status {}", dos_guard[0].name, dos_guard[0].ip, dos_guard[0].status);
    }

   

    let mut system = System::new_all();
    system.refresh_all();

    // let reply_to_client;

    unsafe { cpu = system.cpus()[0].cpu_usage() as i32; }

    for item in servers.clone().lock().unwrap().iter_mut() {
        let sender = sender(
            [sep[0].to_string(), " ".to_string(), unsafe { cpu.to_string() }].concat(),
            item.ip.to_string()
        ).await?;
        sender;
    }

    thread::sleep(Duration::from_secs(2)); // Send & confirm phase
    println!("Send & confirm done!!");

    // thread::sleep(Duration::from_secs(1)); // Failure Ending
    println!("Fail handle done!!");

    for item in clients.clone().lock().unwrap().iter_mut() {
        if item.name.eq(&(sep[0].clone())) {
            if item.candidate {
                for server in item.servers.clone().lock().unwrap().iter_mut() {
                    if server.confirmed == true {
                        item.candidate = Elect_Server(server.ip.clone(), item.name.clone(), server.load.clone(), server.priority.clone());
                        if !item.candidate {
                            break;
                        }
                    }
                }
            }
        }
    }

    for item in clients.clone().lock().unwrap().iter_mut() {
        if item.name.eq(&(sep[0].clone())) {
            if item.candidate {
            
                // 2:
                // A: check if receiver client exists in our DoS --> return

                if let Some(index) = dos_guard.iter().position(|user| user.name == receiver_name) {
                    println!("DoS Table --> Receiver Client found");
                    // let receiver_client_ip:String = dos_guard[index].ip.to_string();
                    let echo_back = sender(
                    ["ip_response ".to_string(), dos_guard[index].ip.clone()].concat(),
                    [sending_client_ip.clone(),":8081".to_string()].concat()).await;
                    echo_back;
                    // st;
                    // dos_guard[index].status = true;
                    } else {
                    //B: else it does not exist --> device not reachable 
                    println!("Not found. Inserting entry... ");
                    let echo_back = sender(
                    "Device Not Reachable".to_string(),
                    [sending_client_ip.clone(),":8081".to_string()].concat()).await;
                    echo_back;
                
                    // println!("Name {}, IP {}, Status {}", dos_guard[0].name, dos_guard[0].ip, dos_guard[0].status);
                    } 
                // println!("Replying to client!!");
                // let st = sender("Secret Agent Wolf: bruhh, I took ur request".to_string(), 
                // [sending_client_ip.clone(),":8081".to_string()].concat()).await;
                // st;
            }
        }
    }

    Ok(())
}


// async fn Process_Request(mm:String, sending_client_ip:String) -> Result<(), Box<dyn Error>> {
// println!("Processing");
// println!("mm {}",mm);
// let sep = mm.split_whitespace().collect::<Vec<_>>();
// println!("len{}",sep[0]);
// {
// clients.clone().lock().unwrap().push(client{
// name:sep[0].to_string(),
// port:sep[1].to_string(),
// candidate:true,
// servers:servers.clone(),
// rem:servers.clone().lock().unwrap().len() as i32

// });
// }
// let mut system = System::new_all();
// system.refresh_all();
//  unsafe{ cpu = system.cpus()[0].cpu_usage() as i32;} 
// //unsafe{ cpu = 10 as i32;} 
// for item in servers.clone().lock().unwrap().iter_mut()
// {
// let sender = sender([sep[0].to_string()," ".to_string(),unsafe{cpu.to_string()}].concat(),item.ip.to_string()).await?;
// sender;
// }




// thread::sleep(Duration::from_secs(2)); //Send & confirm phase
// println!("Send & conform done!!");
// // {
// // for item in clients.clone().lock().unwrap().iter_mut()
// // {

// //     if(item.name.eq(&(sep[0].clone()))){
// //         {
// //         for server in item.servers.clone().lock().unwrap().iter_mut(){
// //             if(server.confirmed == false)
// //             {
// //                 {
// //                 for entity in servers.clone().lock().unwrap().iter_mut(){
// //                     if(entity.ip != server.ip)
// //                     {
// //                         let sender = sender([sep[0].to_string()," ".to_string(),"drop".to_string()," ".to_string(),server.ip.clone()," ".to_string(),entity.ip.clone()].concat(),entity.ip.clone()).await?;
// //                         sender;
// //                     }
// //                 }
// //                 }
                
// //             }
        
// //         }
// //     }
// //     }
// // }

// // }
// thread::sleep(Duration::from_secs(1)); //Failure Ending
// println!("fail handle done!!");
// {
// for item in clients.clone().lock().unwrap().iter_mut()
// {
// if(item.name.eq(&(sep[0].clone())))
// {
//     if(item.candidate)
//     {
        
//         for server in item.servers.clone().lock().unwrap().iter_mut(){
//             if(server.confirmed == true)
//         {
        
//             item.candidate = Elect_Server(server.ip.clone(),item.name.clone(),server.load.clone(),server.priority.clone());
//             if(!item.candidate){
//                 break;
//             }
//         }
//     }
//     }
// }
// }
// }


// {
// for item in clients.clone().lock().unwrap().iter_mut()
// {
// if(item.name.eq(&(sep[0].clone())))
// {
// if(item.candidate)
// {
// println!("replying to client!!");
// let st = sender("Secret Agent Wolf: bruhh, I took ur request".to_string(),"10.7.57.87:8081".to_string()).await;
// st;
// }
// }
// }
// }

// Ok(())
// }



async fn listener() -> Result<(), Box<dyn Error>> {
let host_name = "Any Host";
let listener_addr = "0.0.0.0:5001"; // Update with the listener's address

let socket = UdpSocket::bind(listener_addr).await?;
println!("--------Listener to Client");

let mut buffer = vec![0u8; 1024];

if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {

let remote_ip = remote_addr.ip().to_string();
println!("IP is {}", remote_ip);

let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
Process_Request(mm.clone(),remote_ip.clone()).await;

buffer.clear();
}

Ok(())
}

async fn listener_server() -> Result<(), Box<dyn Error>> {
let host_name = "AnyHost";
let listener_addr = "0.0.0.0:5050"; // Update with the listener's address

let socket = UdpSocket::bind(listener_addr).await?;
println!("--------Listener to Server");

let mut buffer = vec![0u8; 1024];



if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
let remote_ip = remote_addr.ip().to_string();
println!("IP is {}", remote_ip);

let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
let sep = mm.split_whitespace().collect::<Vec<_>>();
println!("m is {}",mm);
// if(sep[1].eq("drop"))
// {

// }else 
if (sep[1].eq("ping"))
{
println!("Got ping");
confirm_Server(remote_ip.to_string()+":5050",sep[0].to_string());

}else if (sep[1].eq("drop")){
    for client in clients.clone().lock().unwrap().iter_mut(){
        if(client.name.eq(sep[0]))
        {
            if(sep[2] == sep[3])
            {
                client.candidate = false;
            }
            for server in client.servers.clone().lock().unwrap().iter_mut(){
                if(server.confirmed == false){
                    let sender = sender([sep[0].to_string()," ".to_string(),"drop".to_string()," ".to_string(),server.ip.clone()," ".to_string(),server.ip.clone()].concat(),server.ip.clone()).await?;
                    sender;
                }
                else {
                    let sender = sender([sep[0].to_string()," ".to_string(),"drop".to_string()," ".to_string(),remote_ip.clone(),":5050".to_string()," ".to_string(),remote_ip.clone(),":5050".to_string()].concat(),[remote_ip.clone(),":5050".to_string()].concat()).await?;
                    sender;
                }
            }
            

        }   
    } 
}else {
    for client in clients.clone().lock().unwrap().iter_mut(){
        if(client.name.eq(sep[0]))
        {
            for server in client.servers.clone().lock().unwrap().iter_mut(){
                if(server.ip.eq(&[remote_ip.to_string(),":5050".to_string()].concat()))
                {
                    server.load = sep[1].parse::<f32>().unwrap();
                    println!("replying to server!!");// will fix name issue, hardwired for now
                    let st = sender([sep[0].to_string()," ".to_string(),"ping".to_string()].concat(),server.ip.clone().to_string()).await;
                    break;
                }
            } 
        }

    }



/*Elect_Server(remote_ip.to_string(),sep[0].to_string(),sep[1].parse::<i32>().unwrap());
for server in servers.clone().lock().unwrap().iter_mut()
{
if(((remote_ip.to_string())+":5050") == server.ip)
{
println!("replying to server!!");// will fix name issue, hardwired for now
let st = sender(["Ali".to_string()," ".to_string(),"ping".to_string()].concat(),server.ip.clone().to_string()).await;
st; 
}*/

}
// if((unsafe {server_count})<servers.clone().lock().unwrap().len() as i32)
// {
// println!("Sending to server!!");
// unsafe{server_count+=1;}
// }

// Clear the buffer for the next message.
buffer.clear();
}
println!("finished listen");
Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

// PORTS
// server - to - server: 5050
// server - to - client: 8081 
// client - to -server: 5001

// update with other server's IP address
{
servers.clone().lock().unwrap().push(server{
ip:"10.7.57.31:5050".to_string(),
// ip:"10.7.29.200:5050".to_string(),
confirmed:false,
port:":5050".to_string(),
load:-1.0,
priority:3
});
servers.clone().lock().unwrap().push(server{
ip:"10.7.57.93:5050".to_string(),
// ip:"10.7.29.200:5050".to_string(),
confirmed:false,
port:":5050".to_string(),
load:-1.0,
priority:1
});
}
tokio::task::spawn_blocking(move || {
// Code executed in the new blocking task
let runtime = tokio::runtime::Runtime::new().unwrap();
runtime.block_on(async move {
while(true){

if let Err(err) = listener().await {
eprintln!("Error in listener: {}", err);
}
}

});
});

while(true){
let listen_server = listener_server().await;
listen_server;
}


Ok(())
}