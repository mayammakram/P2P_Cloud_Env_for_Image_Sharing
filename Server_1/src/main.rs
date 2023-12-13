use chrono::Local;
use tokio::net::UdpSocket;
use std::error::Error;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::time::{Duration, self};
use std::thread;
use std::io::Write;

//image encryption/decryption 
use image::{
    ImageBuffer,
    Rgba,
};
use steganography::encoder::Encoder;
use steganography::decoder::Decoder;
use steganography::util::{str_to_bytes, file_as_dynamic_image, bytes_to_str, save_image_buffer, file_as_image_buffer};
use base64;
use std::fs:: {File, read};
use std::collections::HashMap;

extern crate sysinfo;
use sysinfo::{System, SystemExt};

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
    load:f64,
    priority:i32, //to handle servers with the same load
}

struct dos_user{
    name: String,
    ip: String,
    status: bool
}

//make vector of structs to store the servers
struct buffer{
    name:String,
    access_right:String,
    image_nameP:String
}


lazy_static! {
    static ref servers: Arc<Mutex<Vec<server>>> = Arc::new(Mutex::new(Vec::new()));
    static ref clients: Arc<Mutex<Vec<client>>> = Arc::new(Mutex::new(Vec::new()));
    static ref dos: Arc<Mutex<Vec<dos_user>>> = Arc::new(Mutex::new(Vec::new()));
    static ref buffers: Arc<Mutex<Vec<buffer>>> = Arc::new(Mutex::new(Vec::new()));

    static ref pipe_ss: (Arc<crossbeam_channel::Sender<String>>, Arc<crossbeam_channel::Receiver<String>>) = {
        let (tx, rx) = crossbeam_channel::unbounded();
        (Arc::new(tx), Arc::new(rx))
    };

    static ref pipe_cs: (Arc<crossbeam_channel::Sender<String>>, Arc<crossbeam_channel::Receiver<String>>) = {
        let (tx, rx) = crossbeam_channel::unbounded();
        (Arc::new(tx), Arc::new(rx))
    };
    static ref pipe_EI: (Arc<crossbeam_channel::Sender<String>>, Arc<crossbeam_channel::Receiver<String>>) = {
        let (tx, rx) = crossbeam_channel::unbounded();
        (Arc::new(tx), Arc::new(rx))
    };

    static ref IP_ADDRESS_MAP: Arc<Mutex<HashMap<String, Vec<String>>>> = {
        let mut m = HashMap::new();
        Arc::new(Mutex::new(m))
    };
}

static mut img_en:bool = false;
static mut cpu:i32=0;
static mut server_status: bool = true;
static mut server_prio:i32 = 3;
static mut percent_mem_usage:f64=999.0;

static mut server_count:i32 = 0;
static mut Candidate:bool = true;



async fn sender(msg: String, ip:String) -> Result<(), Box<dyn Error>> {
let host_name = "Secret_Agent_Wolf";
let receiver_addr = ip; // Update with the middleware's address

println!("-------------Sending a message");
let socket = UdpSocket::bind("0.0.0.0:0").await?;


println!("Sending to: {}", receiver_addr);

socket.send_to(msg.as_bytes(), receiver_addr).await?;


Ok(())
}

fn Elect_Server(sip:String,c_name:String, c:f64, priority:i32) -> bool{
            // c is the load of the other server
            println!("Electing Server...");
            println!("Got Server name {} with priority: {}", c_name, priority);

            let mut sys = System::new_all(); // refresh memory values
            sys.refresh_all();

            let used_memory = sys.used_memory();
            let total_memory = sys.total_memory();

            let mut candidate = true;
            println!("--------------------------------------Performing Election----------------------");
            println!(" My CPU {}, Other Server's CPU {}",unsafe{percent_mem_usage},c);
            if c < unsafe{percent_mem_usage}
            {
                println!("Not Elected!! big load!!");
                candidate = false; 
            }
            else if c==unsafe{percent_mem_usage}
            {
                if priority>unsafe{server_prio}
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

        println!("Confirming Reply from Server!!");
        for item in clients.clone().lock().unwrap().iter_mut()
        {
    `       if item.name.eq(&(c_name.clone()))
            {
                for server in item.servers.clone().lock().unwrap().iter_mut()
                {
                    if server.ip.eq(&sip) 
                    {
                        item.rem-=1;
                        server.confirmed=true;
                        break;
                    }
                } 
            }
`        }

        Ok(())
}

async fn send_frag_image(chunks:Vec<Vec<u8>>, mut i: i32, size_of_chunks: i32, sending_client_ip: String) {
    println!("-----------> Num of Chunks {}",size_of_chunks);
    for image_part in chunks {
        let base64_string = base64::encode(image_part);
        let mut is_last_packet = "false ";
        if i == size_of_chunks  -1 
        {
            println!("-----------> last packet!");
            is_last_packet = "true ";
        }

        let sender_task = sender( [" encrypt ".to_string(), sending_client_ip.clone(), " ".to_string(), is_last_packet.to_string() ,base64_string.to_string()].concat(),[sending_client_ip.to_string(),":8081".to_string()].concat()).await;
        sender_task;
        thread::sleep(Duration::from_millis(100)); // Send & confirm phase
        unsafe{img_en = false};
        println!("Messages are sent");
        i+=1;     
    }
}

async fn Process_Request(mm: String, sending_client_ip: String) -> Result<(), Box<dyn Error>> {
    println!("");
    println!("-----------------------------------Processing Request in Server ---------------------------");

    let sep = mm.split_whitespace().collect::<Vec<_>>();
    let mut Temp_servers = Arc::new(Mutex::new(Vec::new()));
    {
        let mut servers_lock = Temp_servers.lock().unwrap();
        servers_lock.push(server {
            ip: "10.7.57.93:5050".to_string(),
            confirmed: false,
            port: ":5050".to_string(),
            load: 999.0,
            priority: 2,
        });
        servers_lock.push(server {
            ip: "10.7.57.77:5050".to_string(),
            confirmed: false,
            port: ":5050".to_string(),
            load: 999.0,
            priority: 1,
        });
    }

    let width = 800;
    let height = 600;
    let mut based_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut encoded_msg: Vec<u8> ;
    let sender_name:String = sep[0].to_string();
    let req_type:String = sep[1].to_string();
    let receiver_name:String = sep[2].to_string();
    let packet_number:String = sep[3].to_string();
    let last_packet_flag:String = sep[4].to_string();
    let hidden_image_packet:String = sep[5].to_string();

    println!("alis yo");
    println!("mm {}", mm);
    // 2. server encrypts the image 
    if req_type.eq("encrypt")
    {
        println!("Encrypting Image");
        {
            let mut map = IP_ADDRESS_MAP.lock().unwrap();
            map.entry(sending_client_ip.clone()).or_insert_with(Vec::new).push(hidden_image_packet.clone());
        }

        if last_packet_flag.eq("true")
        {
            {
            let mut map = IP_ADDRESS_MAP.lock().unwrap();

            if let Some(hidden_image) = map.get(&sending_client_ip.clone()) 
            {
                // convert hiden image to string
                let mut hidden_image_string = String::new();
                for item in hidden_image.clone() {
                    hidden_image_string.push_str(&item);
                }

                //text to image
                let hidden_img_out = text_to_image(&hidden_image_string).unwrap();
                let filename = format!("/home/aliali/Documents/Image_Sharing_DS/server/src/hidden_img_out{}.png",sending_client_ip.clone());
                let mut file = File::create(filename).unwrap();
                file.write_all(&hidden_img_out); 
            
                // clear map given sending_client_ip
                map.remove(&sending_client_ip.clone());
                let filename_en = format!("/home/aliali/Documents/Image_Sharing_DS/server/src/encoded_msg{}.png",sending_client_ip.clone());
                based_image = encrypt_image(&hidden_image_string.clone(),filename_en.clone());
                encoded_msg = read(filename_en).unwrap();
                
                //fragemnt the image into parts of length of 1024
                let mut i = 0;
                let mut chunks = Vec::new();
                let chunk_size = 30000;

                let mut size_of_chunks = 0;
                //   let mut based_image_raw = based_image.into_raw();
                for chunk in encoded_msg.chunks(chunk_size){

                chunks.push(chunk.to_vec());
                size_of_chunks+=1;
                }
                send_frag_image(chunks, i, size_of_chunks, sending_client_ip).await;

            }
            }

        }
    }
    else if req_type.eq("ip_request") || req_type.eq("ack-encrypt") || req_type.eq("offline_device")
    {
        clients.clone().lock().unwrap().push(client {
            name: sep[0].to_string(),
            port: ":8081".to_string(),
            candidate: true,
            servers: Temp_servers.clone(),
            rem: Temp_servers.lock().unwrap().len() as i32,
        });
        unsafe{img_en = true};
        println!("");
        println!("----------------------------------Checking for IP Address in Discovery Service ;) ---------------------------");
        // 1) add / update sending client's entry in the Directory of Service
        let mut dos_guard = dos.lock().unwrap();
        if req_type.eq("ip_request")
        {
            // 1:
            // A: check if sender client exists in our DoS --> return
            if let Some(index) = dos_guard.iter().position(|user| user.name == sender_name) 
            {
                println!("User found. Updating user entry....");
                // If the username exists, update the IP and status
                dos_guard[index].ip = sending_client_ip.clone();
                // Assuming there's a status field in DoS_user struct
                dos_guard[index].status = true;
            } 

            // B: else it does not exist --> insert entry
            else 
            {
                println!("Client IP Address Not found. Inserting entry on DoS ... ");
                dos_guard.push(dos_user{
                name: sender_name.clone(),
                ip: sending_client_ip.clone(),
                status: true
                });
                println!("");
                println!("----------------------------------Data Added in Directory of Service ---------------------");
                println!("Name: {}, IP: {}, Status: {}", dos_guard[0].name, dos_guard[0].ip, dos_guard[0].status);
                //iterate over the buffer and check if the name of the client is in the buffer
                
            }
        }

        else if req_type.eq("offline_device")
        {
            println!("Offline Device");
            //remove entry from DoS
            if let Some(index) = dos_guard.iter().position(|user| user.ip == receiver_name) 
            {
                println!("User found. Removing user entry....");
                dos_guard.remove(index);
            }
        }
    
        thread::sleep(Duration::from_millis(1000)); // Send & confirm phase
        println!("Send & confirm done!!");

        println!("---------------------------------- INITIATING ELECTION-----------------------------------");
        for item in clients.clone().lock().unwrap().iter_mut() 
        {
            if item.name.eq(&(sep[0].clone())) 
            {
                let mut temp = unsafe { percent_mem_usage.clone() };
                let mut elected_server = "10.7.57.31".to_string();
                println!("This server has load {}", temp);
                // compare temp to server in servers' load
                for server in servers.clone().lock().unwrap().iter_mut() {
                    println!("Server {} has load {}", server.ip, server.load);
                    if server.load < temp {
                        
                        temp = server.load;
                        elected_server = server.ip.clone();
                        println!("Found server.ip {} with lower load", elected_server);
                    }
                }
                println!("=============> ELECTED SERVER {}", elected_server);

                if elected_server.eq("10.7.57.31") 
                {
                    for item in buffers.clone().lock().unwrap().iter_mut(){
                        if item.name.eq(&sender_name){
                            //send the image to the client
                            println!("Sending the image to the client");
                            let mut image_name = item.image_nameP.clone();
                            let mut access_right = item.access_right.clone();
                            //sleep for 1 sec
                            thread::sleep(time::Duration::from_secs(1));

                            let sender_task = sender( ["update_image ".to_string(), image_name.clone(), " ".to_string(), access_right.to_string() ].concat(),[sending_client_ip.to_string(),":8081".to_string()].concat()).await;
                            sender_task;
                        }
                    }
                }
                    if req_type.eq("ack-encrypt")
                    { let echo_back_reply = sender(
                        "ack-encrypt 10.7.57.31:5001 awoo helo".to_string(),
                        [sending_client_ip.clone(),":8081".to_string()].concat()).await;
                        echo_back_reply;
                    }
                    else if req_type.eq("ip_request")
                    {
                        // 2:
                        // A: check if receiver client exists in our DoS --> return
                        if let Some(index) = dos_guard.iter().position(|user| user.name == receiver_name.clone()) 
                        {
                            let call_back = packet_number.clone();
                            println!("DoS Table --> Receiver Client found");
                            let receiver_client_ip:String = dos_guard[index].ip.to_string();
                            if call_back.eq("access_rights"){
                                let echo_back: Result<(), Box<dyn Error>> = sender(
                                ["ip_response ".to_string(), receiver_client_ip," rights_path".to_string()].concat(),
                                [sending_client_ip.clone(),":8081".to_string()].concat()).await;
                                echo_back?;

                            }
                            else {
                                let echo_back: Result<(), Box<dyn Error>> = sender(
                                    ["ip_response ".to_string(), receiver_client_ip," image_path".to_string()].concat(),
                                    [sending_client_ip.clone(),":8081".to_string()].concat()).await;
                                    echo_back?;
                            }

                            dos_guard[index].status = true;
                        } 

                        else 
                        {
                            // B: else it does not exist --> device not reachable 
                            println!("Not found. Inserting entry... ");

                            let echo_back = sender(
                            "ip_response Device_Not_Reachable".to_string(),
                            [sending_client_ip.clone(),":8081".to_string()].concat()).await;
                            echo_back?;
                        }
                    } 
                
                else
                {
                    println!("=============> DID NOT ELECT THIS SERVER ----> elected server is {}", elected_server);

                }
                
            }
            

        }


        {
            let _clients = &mut clients.lock().unwrap();
            if let Some(index) = _clients.iter().position(|client| client.name == sep[0]) 
            {
                println!("Successfully removed client {}", _clients[index].name);
                _clients.remove(index);
                println!("AFTER REMOVING CLIENT");
            }
            else
            {
                println!("You screwed up");
            }
        }
    }
    else if (req_type.eq("buffer"))
    {
        let name = sep[2].to_string();
        let access_right = sep[3].to_string();
        let image_name = sep[4].to_string();
        buffers.clone().lock().unwrap().push(buffer{
            name: name,
            access_right: access_right,
            image_nameP: image_name
        });

    }
      
    Ok(())
}

async fn Process_Server(mm: String, remote_ip: String) -> Result<(), Box<dyn Error>> {
    let sep = mm.split_whitespace().collect::<Vec<_>>();
    if(sep[0].eq("dos_update"))
    {
        //add recieved msg to dos table'
        //print that we recieved the msg
        println!("Updating DoS Table");
        let mut dos_guard = dos.lock().unwrap();
        if let Some(index) = dos_guard.iter().position(|user| user.name == sep[1]) 
        {
            println!("User found. Updating user entry....");
            // If the username exists, update the IP and status
            dos_guard[index].ip = sep[2].to_string();
            dos_guard[index].status = sep[3].parse::<bool>().unwrap();
        }
        else {
            println!("Client IP Address Not found. Inserting entry on DoS ... ");
            dos_guard.push(dos_user{

            name: sep[1].to_string(),
            ip: sep[2].to_string(),
            status: sep[3].parse::<bool>().unwrap()
            });
        }

    }
    else if (sep[1].eq("ping"))
    {
        println!("Got Ping");
        confirm_Server(remote_ip.to_string()+":5050",sep[0].to_string());
    }
    
    else if (sep[1].eq("drop"))
    {
        for client in clients.clone().lock().unwrap().iter_mut(){
            if(client.name.eq(sep[0]))
            {
                if(sep[2] == sep[3])
                {
                    client.candidate = false;
                }
                for server in client.servers.clone().lock().unwrap().iter_mut(){
                    if (server.confirmed == false){
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
    } 
    else 
    {
        for client in clients.clone().lock().unwrap().iter_mut(){
            if(client.name.eq(sep[0]))
            {
                for server in client.servers.clone().lock().unwrap().iter_mut(){
                    if(server.ip.eq(&[remote_ip.to_string(),":5050".to_string()].concat()))
                    {
                        println!("PROCESS_SERVER: {}",mm);
                        server.load = sep[1].parse::<f64>().unwrap();
                        println!("Replying to server!!");// will fix name issue, hardwired for now
                        let st: Result<(), Box<dyn Error>> = sender([sep[0].to_string()," ".to_string(),"ping".to_string()].concat(),server.ip.clone().to_string()).await;
                        break;
                    }
                } 
            }
        }
    }   
    Ok(())
}

async fn listener() -> Result<(), Box<dyn Error>> {
    let host_name = "Any Host";
    let listener_addr = "0.0.0.0:5001"; // Update with the listener's address

    let socket = UdpSocket::bind(listener_addr).await?;
    let mut buffer = vec![0u8; 65507];
    if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {

        let remote_ip = remote_addr.ip().to_string();
        println!("Client IP is {}", remote_ip);
        let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
        let tx = &*pipe_cs.0;

        if( unsafe { server_status == true} || unsafe{img_en == true}){
            tx.send([remote_ip,"?".to_string() , mm.clone()].concat()).unwrap();
        }

        buffer.clear();
    }

    Ok(())
}

async fn listener_2() -> Result<(), Box<dyn Error>> {
    let host_name = "Any Host";
    let listener_addr = "0.0.0.0:5002"; // Update with the listener's address

    let socket = UdpSocket::bind(listener_addr).await?;
    let mut buffer = vec![0u8; 65507];
    if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await 
    {
        let remote_ip = remote_addr.ip().to_string();
        println!("Client IP is {}", remote_ip);

        let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
        let tx = &*pipe_cs.0;

        if( unsafe { server_status == true} || unsafe{img_en == true}){
            tx.send([remote_ip,"?".to_string() , mm.clone()].concat()).unwrap();
        }

        buffer.clear();
    }

    Ok(())
}

async fn listener_3() -> Result<(), Box<dyn Error>> {
    let host_name = "Any Host";
    let listener_addr = "0.0.0.0:5003"; // Update with the listener's address

    let socket = UdpSocket::bind(listener_addr).await?;
    let mut buffer = vec![0u8; 65507];

    if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
        let remote_ip = remote_addr.ip().to_string();
        println!("Client IP is {}", remote_ip);
        let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
        let tx = &*pipe_cs.0;

        if( unsafe { server_status == true} || unsafe{img_en == true}){
            tx.send([remote_ip,"?".to_string() , mm.clone()].concat()).unwrap();
        }

        buffer.clear();
    }
    Ok(())
}

async fn listener_server() -> Result<(), Box<dyn Error>> {
    let host_name = "AnyHost";
    let listener_addr = "0.0.0.0:5050"; // Update with the listener's address

    let socket = UdpSocket::bind(listener_addr).await?;
    let mut buffer = vec![0u8; 65507];

    if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
        let remote_ip = remote_addr.ip().to_string();
        println!("Server IP is {}", remote_ip);

        let mm = String::from_utf8_lossy(&buffer[..len]).to_string();

        println!("Message from Server is {}",mm); 
        let tx = &*pipe_ss.0;

        if( unsafe { server_status == true || unsafe{img_en == true}}){
            tx.send([remote_ip,"?".to_string() , mm.clone()].concat()).unwrap();
        }
        // Clear the buffer for the next message.
        buffer.clear();
    }

    Ok(())
}

async fn listener_to_server_loads() -> Result<(), Box<dyn Error>> {
    let host_name = "AnyHost";
    let listener_addr = "0.0.0.0:5053"; // Update with the listener's address
 
    let socket = UdpSocket::bind(listener_addr).await?;
 
    let mut buffer = vec![0u8; 65507];
 
    if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
        let remote_ip = remote_addr.ip().to_string();
        println!("Server IP is {}", remote_ip);
 
        let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
        println!("Message from Server is {}", mm);
 
        let tx_loads = &*pipe_EI.0;

 
        if (unsafe { server_status == true} || unsafe { img_en == true }) {  
            println!("Sending to Server Loads");
            tx_loads.send(mm).unwrap();
        }
        // Clear the buffer for the next message.
        buffer.clear();
    }
 
    Ok(())
 }

fn image_to_text(image_data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // Encode the byte array to a Base64 string
    let base64_string = base64::encode(image_data);
   
    Ok(base64_string)
} 
   
fn text_to_image(base64_string: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Decode the Base64 string into a byte array
    let image_data = base64::decode(base64_string)?;
    Ok((image_data))
}

fn encrypt_image(message: &String,filename_en:String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Convert our string to bytes
    let payload = str_to_bytes(&message);
    // Load the image where we want to embed our secret message
    let mask_image = file_as_dynamic_image("/home/aliali/Documents/Image_Sharing_DS/server/src/mask_960.jpg".to_string()); 
    // Create an encoder
    let enc = Encoder::new(payload, mask_image);
    // Encode our message into the alpha channel of the image
    let result = enc.encode_alpha();
    let result1 = enc.encode_alpha();
    // Save the new image
    save_image_buffer(result, filename_en);
    println!("Finished Encryption!!");

    return result1;
  }

fn decrypt_image(image: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let dec = Decoder::new(image);
    let out_buffer = dec.decode_alpha();
    let clean_buffer: Vec<u8> = out_buffer.into_iter()
    .filter(|b| {
        *b != 0xff_u8
    })
    .collect();
    println!("Before decoding {:?}", clean_buffer.len());
    let message = match std::str::from_utf8(&clean_buffer) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid UTF-8 sequence found: {:?}", e);
            return;
        }
     };

    let decoded_img: Vec<u8> = text_to_image(message ).unwrap();
    // Save the byte array as an image file
    let mut file = File::create("/home/aliali/Documents/Image_Sharing_DS/server/src/decoded_msg.png").unwrap();
    file.write_all(&decoded_img);
}

async fn fault_tolerance() -> Result<(), Box<dyn Error>> { 
    while(true){
        if( unsafe { server_status == true}){
            thread::sleep(Duration::from_secs(50));
            unsafe{server_status = false};
            sender("10.7.57.31 999".to_string(), "10.7.57.93:5053".to_string()).await;
            sender("10.7.57.31 999".to_string(), "10.7.57.77:5053".to_string()).await;

            println!("Server is down!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            thread::sleep(Duration::from_secs(10)); // we can change it any time 
            
            println!("Server is up :)>??????????????????????????????????????????????????????????????/");
            unsafe { server_status = true};
        }
    }
    Ok(())
}

fn calc_machine_load() -> f64 {
    let mut sys = System::new_all(); // refresh memory values
    sys.refresh_all();
 
    let used_memory = sys.used_memory();
    let total_memory = sys.total_memory();
    //return 999 if server is down
    if(unsafe{server_status == false})
    {
        return 999.0;
    }
    return (used_memory as f64 / total_memory as f64) * 100.0
 }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_pipe = &*pipe_cs;
    let (tx ,rx) = my_pipe;
    let my_pipe_server = &*pipe_ss;
    let (stx ,srx) = my_pipe_server;
    let my_pipe_server_loads = &*pipe_EI;
    let (stx_loads ,srx_loads) = my_pipe_server_loads;

    for _x in 0..2 {
        thread::spawn(move || {
            loop {
                let thread_no = _x;
                let received = srx_loads.recv().unwrap();
                println!("");
                println!("-----------------------------------Creating a Server LOADS Thread -----------------------------");
                println!("Server Loads ======> Recieved Data: {} from thread #{}", received, thread_no);
                         
                let mut parts = received.split(' ');        
                let sending_server_ip = parts.next().unwrap();
                let load: f64= parts.next().unwrap().parse().unwrap();

                let rt = tokio::runtime::Runtime::new().unwrap();
                if( unsafe { server_status == true}){
                    rt.block_on(async {
                        println!("sending_server_ip: {:?} from thread #{:?}", sending_server_ip, thread_no);
                        println!("Received load {:?}", load);

                        // update servers with load
                        for server in servers.clone().lock().unwrap().iter_mut() {
                            if server.ip.eq(&[sending_server_ip.to_string(),":5050".to_string()].concat())
                            {
                                if(server.load == 999.0)
                                {
                                    //send all dos entries to server ip
                                    for item in dos.clone().lock().unwrap().iter_mut(){
                                        let echo_back: Result<(), Box<dyn Error>> = sender(
                                            ["dos_update ".to_string(), item.name.clone()," ".to_string(),item.ip.clone()," ".to_string(),item.status.to_string().clone()].concat(),
                                            [sending_server_ip.clone(),":5050"].concat()).await;
                                            echo_back;
                                    }                            
                                }
                                server.load = load;
                                println!("Updated server {:?} with load {:?}", sending_server_ip, server.load);
                            }
                        }
                    });
                }       
            }
        });
    }
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
                loop {
                    if(unsafe { server_status == true}){
                        //print that we send load
                        println!("Sending load to other servers");
                        unsafe {percent_mem_usage = calc_machine_load();}
                        let now = Local::now();
                        println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
                        println!("Machine Load is {}", unsafe{percent_mem_usage});
                        let _sender = sender(["10.7.57.31 ", unsafe{&percent_mem_usage.to_string()}].concat(),
                            "10.7.57.93:5053".to_string()).await;
                    
                        _sender;
                        let _sender_two = sender(["10.7.57.31 ", unsafe{&percent_mem_usage.to_string()}].concat(),
                        "10.7.57.77:5053".to_string()).await;
                        _sender_two;
                        thread::sleep(Duration::from_secs(15));
                    }
                }

            // let tolerate = fault_tolerance().await;
        });
    });
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let tolerate = fault_tolerance().await;
        });
    });

    for _i in 0..12 {
        thread::spawn(move || {
                loop {
                let thread_no = _i;                
                let received = rx.recv().unwrap();
                println!("");
                println!("-----------------------------------Creating a Client Thread -----------------------------");
                
                let mut parts = received.split('?');

                let sending_client_ip = parts.next().unwrap();
                let mm = parts.next().unwrap();
                if( unsafe { server_status == true}){
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                    Process_Request(mm.to_string().clone(), sending_client_ip.to_string().clone()).await;
                    });
                }
            }
        });
    }
    let no_threads = 4;
    for _i in 0..no_threads {
        thread::spawn(move || {
            loop {
                let thread_no = _i;
                let received = srx.recv().unwrap();
                println!("");
                println!("-----------------------------------Creating a Server Thread -----------------------------");
                println!("Recieved Data: {} from thread #{}", received, thread_no);
                let mut parts = received.split('?');

                let sending_client_ip = parts.next().unwrap();
                let mm = parts.next().unwrap();
                let rt = tokio::runtime::Runtime::new().unwrap();
                if( unsafe { server_status == true}){
                    rt.block_on(async {   
                        Process_Server(mm.to_string().clone(), sending_client_ip.to_string().clone()).await;
                        // Process_Request(mm.to_string().clone(), remote_ip.to_string().clone()).await;
                    });
                }
            }
        });
    }
 

// PORTS
// server - to - server: 5050
// server - to - client: 8081 
// client - to -server: 5001
    // update with other server's IP address
    {
        servers.clone().lock().unwrap().push(server{
            ip:"10.7.57.93:5050".to_string(),
            // ip:"10.7.29.200:5050".to_string(),
            confirmed:false,
            port:":5050".to_string(),
            load:999.0,
            priority:2,
            // failure_token
        });
        servers.clone().lock().unwrap().push(server{
            ip:"10.7.57.77:5050".to_string(),
            // ip:"10.7.29.200:5050".to_string(),
            confirmed:false,
            port:":5050".to_string(),
            load:999.0,
            priority:1,
            // failure_token: false,
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
    tokio::task::spawn_blocking(move || {
        // Code executed in the new blocking task
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            while(true){
                if let Err(err) = listener_2().await {
                    eprintln!("Error in listener: {}", err);
                }
            }
        });
    });
    tokio::task::spawn_blocking(move || {
        // Code executed in the new blocking task
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            while(true){
                if let Err(err) = listener_3().await {
                    eprintln!("Error in listener: {}", err);
                }
            }
        
        });
    });
    tokio::task::spawn_blocking(move || {
        // Code executed in the new blocking task
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            while(true){
                if let Err(err) = listener_server().await {
                    eprintln!("Error in server listener: {}", err);
                }
            }
        });
    });
    tokio::task::spawn_blocking(move || {
        // Code executed in the new blocking task
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            while(true){
                if let Err(err) =  listener_to_server_loads().await {
                    eprintln!("Error in server listener: {}", err);
                }
            }
        });
    });

Ok(())
}