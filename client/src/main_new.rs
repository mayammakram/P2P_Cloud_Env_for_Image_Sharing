// // use tokio::net::{TcpListener, TcpStream};
// // use tokio::io::{AsyncReadExt, AsyncWriteExt};
// // use rand::Rng;
// // use std::net::SocketAddr;
// // use std::error::Error;




// // fn selectServer() -> String{
// // println!("bruhhh");
// // let mut rng = rand::thread_rng();
// // let random_number: u32 = rng.gen_range(0..=1);
// // println!("Random number between 0 and 1: {}", random_number);
// // let mut s_port: &str = "";
// // if(random_number == 0){
// // s_port = "5050"
// // }
// // else{
// // s_port = "5051"
// // }
// // let s_msg : String= "10.7.29.200:".to_string() + &s_port.to_string();
// // return s_msg;
// // }



// // async fn sender(msg: &str) -> Result<(), Box<dyn Error>> {

// // let ip_selected = selectServer();
// // println!("the ip selected is {}",ip_selected );
// // // payload message to send
// // let host_name = "Ali";
// // // sends to the indicated IP, on port 5353
// // let receiver_addr = ip_selected; // Update with the receiver's address



// // println!("-------------Sending a message");
// // let dest_addr: SocketAddr = receiver_addr.parse()?;
// // let mut stream = TcpStream::connect(dest_addr).await?;
// // println!("Connected to {}", receiver_addr);
 

// // let message = format!("{}", msg);
// // stream.write_all(message.as_bytes()).await?;
// // println!("--------> sent message!");


// // Ok(())
// // }

// // async fn listener() -> Result<(), Box<dyn Error> > {
// // let host_name = "Any Host";
// // // 0.0.0.0 means this machine accepts incoming requests from any IP
// // // listens on port 49278
// // let listener_addr = "0.0.0.0:8080"; // Update with the listener's address

// // let listener = TcpListener::bind(listener_addr).await?;
// // println!("--------Listener");
// // println!("{} has sent a message on broadcast addrs: {}", host_name, listener_addr);
// // while let Ok((mut stream, _)) = listener.accept().await {
// // let mut buffer = vec![0u8; 1024];
// // let n = stream.read(&mut buffer).await?;
 
// // if n == 0 {
// // break;
// // }

// // let message = String::from_utf8_lossy(&buffer[..n]);
// // println!("Message received from {}: {}", host_name, message);

// // // let response = format!("Hello from {}", host_name);
// // // stream.write_all(response.as_bytes()).await?;
// // }

// // Ok(())
// // }

// // #[tokio::main]
// // async fn main() -> Result<(), Box<dyn Error>> {
// // // let sender_task = sender().await?;
 
// // // sender_task; // Ensure the sender completes first

// // // Sends one message to three servers
// // for _ in 0..10 {
// // let sender_task = sender("Request access").await?;
// // sender_task; // Ensure the sender completes first
// // }
 
 
// // let receiver_task = listener().await?;
// // receiver_task;

// // Ok(())


// // }


















// use tokio::net::{TcpListener, TcpStream};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::UdpSocket;
// use std::net::SocketAddr;
// use std::error::Error;
// use std::time::Duration;
// use tokio::time;
// use std::fs:: {File, read};
// use image::{
// ImageBuffer,
// Rgba,
// DynamicImage,
// ImageFormat
// };


// use steganography::decoder::Decoder;
// use steganography::util::{str_to_bytes, file_as_dynamic_image, file_as_image_buffer, bytes_to_str, save_image_buffer};
// use base64;
// use std::io::Write;
// // use image::ImageFormat;



// use std::thread;

// // async fn sender() -> Result<(), Box<dyn Error>> {
// // // payload message to send
// // let host_name = "Ali";
// // // sends to the indicated IP, on port 5353
// // let receiver_addr = "10.7.29.200:8081"; // Update with the receiver's address

// // println!("-------------Sending a message");
// // let dest_addr: SocketAddr = receiver_addr.parse()?;
// // let mut stream = TcpStream::connect(dest_addr).await?;
// // println!("Connected to {}", receiver_addr);

// // let message = format!("{}", host_name);
// // stream.write_all(message.as_bytes()).await?;
// // println!("--------> sent message!");


// // Ok(())
// // }


// //**UDP

// async fn sender(ip:String,msg:String) -> Result<(), Box<dyn Error>> {
// // payload message to send
// let host_name = msg;
// // let host_name = "encrypt @aliali @mayamakram 1";
// // sends to the indicated IP, on port 5353
// let receiver_addr = ip.clone(); // Update with the receiver's address

// println!("-------------Sending a message to {}", ip);
// let socket = UdpSocket::bind("0.0.0.0:0").await?;
// // let dest_addr = receiver_addr.parse()?;
 
// let message = format!("{}", host_name);
// socket.send_to(message.as_bytes(), receiver_addr.clone()).await?;
// println!("--------> sent message to {} !", receiver_addr);

// Ok(())
// }


// fn text_to_image(base64_string: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
// // Decode the Base64 string into a byte array
// let image_data = base64::decode(base64_string)?;

// Ok((image_data))
// }

// //------------------------------------------------------------/
// // fn decrypt_image(image: ImageBuffer<Rgba<u8>, Vec<u8>>) {
// fn decrypt_image(path: String) -> () { 
// //` let encoded_image = file_as_image_buffer("/home/aliali/Documents/Image_Sharing_DS/steg/ammora_out.png".to_string());
// let encoded_image = file_as_image_buffer(path); 
// let dec = Decoder::new(encoded_image);
// let out_buffer = dec.decode_alpha();
// let clean_buffer: Vec<u8> = out_buffer.into_iter()
// .filter(|b| {
// *b != 0xff_u8
// })
// .collect();
// let message = bytes_to_str(clean_buffer.as_slice());
// println!("decoded message: {:?}", message);

// let decoded_img = text_to_image(message ).unwrap();
// // Save the byte array as an image file
// let mut file = File::create("/home/aliali/Image_Sharing_DS/new_client/src/decoded_msg.png").unwrap();
// file.write_all(&decoded_img);

// }
// // async fn listener() -> Result<(), Box<dyn Error> > {
// // let host_name = "Any Host";
// // // 0.0.0.0 means this machine accepts incoming requests from any IP
// // // listens on port 49278
// // let listener_addr = "0.0.0.0:8080"; // Update with the listener's address
// // let listener = TcpListener::bind(listener_addr).await?;
// // println!("--------Listener");
// // println!("{} has sent a message on broadcast addrs: {}", host_name, listener_addr);
// // while let Ok((mut stream, _)) = listener.accept().await {
// // let mut buffer = vec![0u8; 1024];
// // let n = stream.read(&mut buffer).await?;
// // if n == 0 {
// // break;
// // }
// // let message = String::from_utf8_lossy(&buffer[..n]);
// // println!("Message received from {}: {}", host_name, message);
// // // let response = format!("Hello from {}", host_name);
// // // stream.write_all(response.as_bytes()).await?;
// // }
// // Ok(())
// // }

// //**UDP

// // async fn listener() -> Result<(), Box<dyn Error>> {
// // let host_name = "Any Host";
// // let listener_addr = "0.0.0.0:8081"; // Update with the listener's address
// // let socket = UdpSocket::bind(listener_addr).await?;
// // println!("--------Listener");
// // println!("{} has sent a message on broadcast addrs: {}", host_name, listener_addr);
// // let mut buffer = vec![0u8; 1024];
// // while let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
// // let remote_ip = remote_addr.ip().to_string();
// // println!("IP is {}", remote_ip);
// // let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
// // // Clear the buffer for the next message.
// // buffer.clear();
// // }
// // Ok(())
// // }
// async fn listener() -> Result<String, Box<dyn Error>> {
// let host_name = "Any Host";
// let listener_addr = "0.0.0.0:8081"; // Update with the listener's address

// let socket = UdpSocket::bind(listener_addr).await?;
// println!("--------Listener");
// println!("{} has started listening on address: {}", host_name, listener_addr);

 
// let mut buffer = vec![0u8; 65507];
// if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
// let remote_ip = remote_addr.ip().to_string();
// println!("IP is {}", remote_ip);
 
// let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
// println!("Message Image {}", mm);
// let server_response = mm.split_whitespace().collect::<Vec<_>>();
// let response_type = server_response[0];
// // let response_payload = server_response[1];
// // if response_type == "ip_response" {
// // println!("IP Request Response!!!");
// // let send_to_client = sender([response_payload.clone().to_string(),":8080".to_string()].concat(),"Hellooooo
// // ".to_string()).await;
// // send_to_client;
// // }
// // let sep = mm.to_string().split_whitespace().collect()::<Vec<_>>();
// // Clear the buffer for the next message.
// buffer.clear();
// return Ok(mm);
// }
 
// // Add a delay between iterations of the loop.
// // time::sleep(Duration::from_secs(1)).await;
// Err("No message received".into())
// }




// //------------------------------------------------------------/

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
// // let sender_task = sender().await?;
 
// // sender_task; // Ensure the sender completes first
// // let handle = thread::spawn(|| {
// // Code executed in the new thread

// tokio::task::spawn_blocking(move || {
// // Code executed in the new blocking task
// let runtime = tokio::runtime::Runtime::new().unwrap();
// runtime.block_on(async move {
// while(true){
// // if let Err(err) = listener().await {
// // eprintln!("Error in listener: {}", err);
// // }

// match listener().await {
// Ok(encoded_data) => {
// match text_to_image(&encoded_data) {
// Ok(encoded_img) => {
// let mut file = File::create("/home/aliali/Image_Sharing_DS/new_client/src/encoded_msg.png").unwrap();
// file.write_all(&encoded_img);
// // handle the result of base64::decode here
// // let image_data: Vec<u8> = read("/home/aliali/Documents/Image_Sharing_DS/steg/src/dog.png").unwrap();
// let image_path = "/home/aliali/Documents/Image_Sharing_DS/new_client/src/encoded_msg.png".to_string();
 
// decrypt_image("/home/aliali/Image_Sharing_DS/new_client/src/encoded_msg.png".to_string());
// },
// Err(err) => {
// eprintln!("Error in text_to_image: {}", err);
// }
// }
// },
// Err(err) => {
// eprintln!("Error in listener: {}", err);
// }
// }
// }
// });
// });

// println!("Thread1");

// let image: Vec<u8> = read("/home/aliali/Image_Sharing_DS/new_client/src/apple.png").unwrap();
// let base64_string = base64::encode(image);
// // let image_string_arr: Vec<String> = image.iter().map(|num| num.to_string()).collect();
// // let image_string = image_string_arr.join(", ");
// // println!("MAGE {}", image_string);
// // for i in 0..1 {
// let sender_task = sender("10.7.57.31:5001".to_string(), ["maya".to_string()," ip_request maya ".to_string(), base64_string.to_string()].concat()).await;
// let sender_task = sender("10.7.57.93:5001".to_string(), ["maya".to_string()," ip_request maya ".to_string(), base64_string.to_string()].concat()).await;

// // let sender_task = sender("10.7.57.93:5001".to_string(), ["maya".to_string()," ip_request ali 11".to_string()].concat()).await;
// //let sender_task = sender("10.7.57.213:5001".to_string(), ["maya".to_string()," ip_request ali 11".to_string()].concat()).await;

// // thread::sleep(Duration::from_millis(20));
// // let sender_task = sender("10.7.57.31:5001".to_string(), ["maya".to_string()," ip_request ali 11".to_string()].concat()).await;
// // let sender_task = sender("10.7.57.93:5001".to_string(), ["maya".to_string()," ip_request ali 11".to_string()].concat()).await;
// // let sender_task = sender("10.7.57.213:5001", ["ali".to_string(),i.to_string(),":8081 yasta".to_string()].concat()).await;
// // let sender_task = sender("10.7.57.31:5001", ["ali".to_string(),i.to_string(),":8081 yasta".to_string()].concat()).await;
// // let sender_task = sender("10.7.57.93:5001", ["ali".to_string(),i.to_string(),":8081 yasta".to_string()].concat()).await;
// // thread::sleep(Duration::from_secs(1)); //Failure Ending
// // // Ensure the sender completes first
// // }


// Ok(())
// }

































use tokio::net::UdpSocket;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use std::fs:: {File, read};
use image::{
 ImageBuffer,
 Rgba,
 DynamicImage,
 ImageFormat
};

use steganography::decoder::Decoder;
use steganography::util::{str_to_bytes, file_as_dynamic_image, file_as_image_buffer, bytes_to_str, save_image_buffer};
use base64;
use std::io::Write;
// use image::ImageFormat;
// use lazy_static::lazy_static;


use std::thread;



//**UDP

//struct
#[derive(Debug)]
struct image_details{
   owner_name: String,
   // image_path: String,
   number_of_views: u32,
}


lazy_static! {
   static ref IP_ADDRESS_MAP: Arc<Mutex<HashMap<String, Vec<String>>>> = {
   let mut m = HashMap::new();
   // m.insert("192.168.0.1".to_string(), vec!["pkt1".to_string(), "pkt2".to_string()]);
   Arc::new(Mutex::new(m))
   };

   static ref IMAGE_DETAILS_MAP: Arc<Mutex<HashMap<String, image_details>>> = { //image path, image details
   let mut m = HashMap::new();
   Arc::new(Mutex::new(m))
   };

}



async fn sender(ip:String,msg:String) -> Result<(), Box<dyn Error>> {
 // payload message to send
 let host_name = msg;

 // sends to the indicated IP, on port 5353
 let receiver_addr = ip.clone(); // Update with the receiver's address

 println!("-------------Sending a message to {}", ip);
 let socket = UdpSocket::bind("0.0.0.0:0").await?;
 // let dest_addr = receiver_addr.parse()?;
 
 let message = format!("{}", host_name);
 socket.send_to(message.as_bytes(), receiver_addr.clone()).await?;
 println!("--------> sent message to {} !", receiver_addr);

 Ok(())
}


fn text_to_image(base64_string: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
 // Decode the Base64 string into a byte array
 let image_data = base64::decode(base64_string)?;

 Ok((image_data))
}

//------------------------------------------------------------/
// fn decrypt_image(image: ImageBuffer<Rgba<u8>, Vec<u8>>) {
fn decrypt_image(path: String, counter :i32) -> () { 
 //` let encoded_image = file_as_image_buffer("/home/aliali/Documents/Image_Sharing_DS/steg/ammora_out.png".to_string());
 let encoded_image = file_as_image_buffer(path); 
 let dec = Decoder::new(encoded_image);
 let out_buffer = dec.decode_alpha();
 let clean_buffer: Vec<u8> = out_buffer.into_iter()
 .filter(|b| {
 *b != 0xff_u8
 })
 .collect();
 
 let message = bytes_to_str(clean_buffer.as_slice());
 // println!("decoded message: {:?}", message);

 let decoded_img = text_to_image(message ).unwrap();
 let filename = format!("/home/aliali/Image_sharing_DS/new_client/src/decoded_msg_{}.png",counter);
 // Save the byte array as an image file
 let mut file = File::create(filename).unwrap();
 file.write_all(&decoded_img);

}

async fn listener() -> Result<String, Box<dyn Error>> {
 let host_name = "Any Host";
 let listener_addr = "0.0.0.0:8081"; // Update with the listener's address

 let socket = UdpSocket::bind(listener_addr).await?;
 println!("--------Listener");
 println!("{} has started listening on address: {}", host_name, listener_addr);

 
 let mut buffer = vec![0u8; 65507];
 if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
 let remote_ip = remote_addr.ip().to_string();
 println!("IP is {}", remote_ip);
 
 let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
 // println!("Message Image {}", mm);
 let server_response = mm.split_whitespace().collect::<Vec<_>>();
 let response_type = server_response[0];
 println!("Received message from: {:?}", remote_ip);
 // Clear the buffer for the next message.
 buffer.clear();
 return Ok(mm);
 }
 
 // Add a delay between iterations of the loop.
 // time::sleep(Duration::from_secs(1)).await;
 Err("No message received".into())
 }

 async fn listener_to_client() -> Result<String, Box<dyn Error>> {
   // let host_name = "Any Host";
   let listener_addr = "0.0.0.0:8080"; // Update with the listener's address

   let socket = UdpSocket::bind(listener_addr).await?;
   // println!("--------Listener");
   println!("listener to CLIENT started on {}",  listener_addr);

 
       let mut buffer = vec![0u8; 66507];
       if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
           let remote_ip = remote_addr.ip().to_string();
           println!("Received msg from CLIENT; IP = {}", remote_ip);
           
           let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
           println!("msg is {}", mm);
           // Clear the buffer for the next message.
           buffer.clear();
           return Ok(mm);

       }
       Err("No message received".into())

      //  Ok(())
       // Add a delay between iterations of the loop.
       //time::sleep(Duration::from_secs(1)).await;
}


//------------------------------------------------------------/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> 
{
   // let sender_task = sender().await?;
   let image: Vec<u8> = read("/home/aliali/Image_sharing_DS/new_client/src/elkadi.png").unwrap();
   let mut counter = 0;
   // sender_task; // Ensure the sender completes first
   // let handle = thread::spawn(|| {
   // Code executed in the new thread
 
   tokio::task::spawn_blocking(move || 
   {
      // Code executed in the new blocking task
      let runtime = tokio::runtime::Runtime::new().unwrap();
      runtime.block_on(async move 
      {
         while(true)
         {
            match listener_to_client().await
            {
               Ok(encoded_data) => 
               {
                  let server_response = encoded_data.split_whitespace().collect::<Vec<_>>();
                  let response_type = server_response[0].to_string();
                  let server_ip = server_response[1].to_string();

                  if response_type.eq("encrypt")
                  {
                     let last_packet_flag = server_response[2].to_string();
                     let encoded_image_packet = server_response[3].to_string();
                     let access_rights = server_response[4].to_string();
                     // println!("Dencrypting Image");
                     {
                        let mut map = IP_ADDRESS_MAP.lock().unwrap();
                        // map.insert(sending_client_ip, vec![hidden_image.clone()]);
                        map.entry(server_ip.clone()).or_insert_with(Vec::new).push(encoded_image_packet.clone());
                     }
                  
                     // if last_packet_flag.eq("1")
                     if last_packet_flag.eq("true")
                     {
                        {
                           let mut map = IP_ADDRESS_MAP.lock().unwrap();
                        
                           if let Some(hidden_image) = map.get(&server_ip.clone()) 
                           {
                              // convert hiden image to string
                              let mut encoded_image_string = String::new();
                              for item in hidden_image.clone() 
                              {
                                 encoded_image_string.push_str(&item);
                              }
                              map.remove(&server_ip.clone());
                           
                              // println!("Hidden image string: {}", hidden_image_string);
                              
                              match text_to_image(&encoded_image_string) 
                              {
                                 Ok(encoded_img) => 
                                 {
                                    let mut file = File::create("/home/aliali/Image_sharing_DS/new_client/src/encoded_msg.png").unwrap();
                                    file.write_all(&encoded_img);
                                    // handle the result of base64::decode here
                                    // let image_data: Vec<u8> = read("/home/aliali/Documents/Image_Sharing_DS/steg/src/dog.png").unwrap();
                                 //  let image_path = "/home/aliali/Image_sharing_DS/new_client/src/encoded_msg.png".to_string();
                                    
                                    decrypt_image("/home/aliali/Image_sharing_DS/new_client/src/encoded_msg.png".to_string(),counter);
                                    // if counter < 10 
                                    // {
                                    // println!("Counter {}", counter);
                                    //    let sender_task = sender("10.7.57.93:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                    //    let sender_task = sender("10.7.57.31:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                    //    let sender_task = sender("10.7.57.77:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                    // counter += 1;
                                    // }
                                 },
                                 Err(err) => 
                                 {
                                 eprintln!("Error in text_to_image: {}", err);
                                    // let sender_task = sender("10.7.57.93:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                    // let sender_task = sender("10.7.57.31:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                    // let sender_task = sender("10.7.57.77:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                 }
                              } 
                           }
                        }

                        {
                           // let mut map = IMAGE_DETAILS_MAP.lock().unwrap();
                           let mut image_details_map = IMAGE_DETAILS_MAP.lock().unwrap();
                           let _image_details = image_details {
                              owner_name: String::from("maya"),
                              number_of_views: access_rights.parse::<u32>().unwrap(),
                           };
                           

                           if !image_details_map.contains_key("/home/aliali/Image_sharing_DS/new_client/src/decoded_msg_0.png") {
                              image_details_map.insert("/home/aliali/Image_sharing_DS/new_client/src/decoded_msg_0.png".to_string(), _image_details);
                           }
                        }
                           println!("Image details map: {:?}", *IMAGE_DETAILS_MAP.lock().unwrap());

                           // if let Some(image_details) = map.get("/home/aliali/Image_sharing_DS/new_client/src/encoded_msg.png")
                           // {
                              
                           //    // push the image details to the vector
                           //    let mut image_details_vec = Vec::new();
                           //    image_details.owner_name = server_ip.clone();
                           //    //convert from string to u32
                           //    let access_rights = access_rights.parse::<u32>().unwrap();
                           //    image_details.number_of_views = access_rights.clone();
                           //    // image_details.number_of_views = access_rights.clone();

                           //    image_details_vec.push(image_details.clone());
                           //    map.remove(&server_ip.clone());
   
                           // }
                        
                     }

                     
                  }
               
               },
               Err(err) => 
               {
                  eprintln!("Error in text_to_image: {}", err);
                  // let sender_task = sender("10.7.57.93:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                  // let sender_task = sender("10.7.57.31:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                  // let sender_task = sender("10.7.57.77:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
               
               }
            }
         }
      });
   });

   tokio::task::spawn_blocking(move || 
   {
      // Code executed in the new blocking task
      let runtime = tokio::runtime::Runtime::new().unwrap();
      runtime.block_on(async move 
      {
         while(true)
         {
            match listener().await
            {
               Ok(encoded_data) => 
               {
                  let server_response = encoded_data.split_whitespace().collect::<Vec<_>>();
                  let response_type = server_response[0].to_string();
                  let server_ip = server_response[1].to_string();
                  //  let last_packet_flag = server_response[2].to_string();
                  //  let encoded_image_packet = server_response[3].to_string();
                  // response type -- server ip -- last_packet_flag -- image_fragment
                  if response_type.eq("ack-encrypt")
                  {
                     //fragemnt the image into parts of length of 1024
                     // let mut image_parts: Vec<Vec<u8>> = Vec::new();
                     let mut i = 0;
                     let mut chunks = Vec::new();
                     let chunk_size = 30000;

                     let mut size_of_chunks = 0;

                     for chunk in image.chunks(chunk_size)
                     {
                        chunks.push(chunk.to_vec());
                        size_of_chunks+=1;
                     }

                     send_fragments(chunks, i, size_of_chunks, server_ip.clone()).await;
                  }

                  if response_type.eq("encrypt")
                  {
                     let last_packet_flag = server_response[2].to_string();
                     let encoded_image_packet = server_response[3].to_string();
                     println!("Dencrypting Image");
                     {
                        let mut map = IP_ADDRESS_MAP.lock().unwrap();
                        // map.insert(sending_client_ip, vec![hidden_image.clone()]);
                        map.entry(server_ip.clone()).or_insert_with(Vec::new).push(encoded_image_packet.clone());
                     }

                     // if last_packet_flag.eq("1")
                     if last_packet_flag.eq("true")
                     {
                        {
                           let mut map = IP_ADDRESS_MAP.lock().unwrap();

                           if let Some(hidden_image) = map.get(&server_ip.clone()) 
                           {
                              // convert hiden image to string
                              let mut encoded_image_string = String::new();
                              for item in hidden_image.clone() 
                              {
                                 encoded_image_string.push_str(&item);
                              }
                              map.remove(&server_ip.clone());

                              // println!("Hidden image string: {}", hidden_image_string);
                              
                              match text_to_image(&encoded_image_string) 
                              {
                                 Ok(encoded_img) => 
                                 {
                                    let mut file = File::create("/home/aliali/Image_sharing_DS/new_client/src/encoded_msg.png").unwrap();
                                    file.write_all(&encoded_img);
                                    // handle the result of base64::decode here
                                    // let image_data: Vec<u8> = read("/home/aliali/Documents/Image_Sharing_DS/steg/src/dog.png").unwrap();
                                    //  let image_path = "/home/aliali/Image_sharing_DS/new_client/src/encoded_msg.png".to_string();
                                    
                                    decrypt_image("/home/aliali/Image_sharing_DS/new_client/src/encoded_msg.png".to_string(),counter);
                                    if counter < 10 
                                    {
                                       println!("Counter {}", counter);
                                       let sender_task = sender("10.7.57.93:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                       let sender_task = sender("10.7.57.31:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                       let sender_task = sender("10.7.57.77:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                       counter += 1;
                                    }
                                 },
                                 Err(err) => 
                                 {
                                    eprintln!("Error in text_to_image: {}", err);
                                    // let sender_task = sender("10.7.57.93:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                    // let sender_task = sender("10.7.57.31:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                    // let sender_task = sender("10.7.57.77:5001" .to_string(), ["ali".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                 }
                              }
                           }
                        } 
                     }
                  }

                  else if response_type.eq("ip_response")
                  {
                     println!("SERVER RESPONSE: {}", server_ip);
                  }
               },
               Err(err) =>
               {
               eprintln!("Error in listener: {}", err);
               }
            }
         }
      });
   });

   println!("Thread1");


   // for i in 0..1 {
   let sender_task = sender("10.7.57.93:5001" .to_string(), ["ali ".to_string(),"ip_request maya ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() ].concat()).await;
   let sender_task = sender("10.7.57.31:5001" .to_string(), ["ali ".to_string(),"ip_request maya ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() ].concat()).await;
   let sender_task = sender("10.7.57.77:5001" .to_string(), ["ali ".to_string(),"ip_request maya ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() ].concat()).await;
   // thread::sleep(Duration::from_secs(1)); //Failure Ending

   // }
   // let image: Vec<u8> = read("/home/aliali/Image_Sharing_DS/new_client/src/vienna.png").unwrap();
   // //fragemnt the image into parts of length of 1024
   // // let mut image_parts: Vec<Vec<u8>> = Vec::new();
   // let mut i = 0;
   // let mut chunks = Vec::new();
   // let chunk_size = 30000;

   // let mut size_of_chunks = 0;

   // for chunk in image.chunks(chunk_size){

   // chunks.push(chunk.to_vec());
   // size_of_chunks+=1;
   // }
   // // let mut size_of_chunks = chunks
   
   // // create a loop to send the image parts
   // // make election (ack)

   // // query servers --> initiate election --> receive server IP -> invoke send_fragments with server IP;


   
   // // let sender_task = sender("10.7.57.31:5001" .to_string(), ["maya ".to_string(),"ack-encrypt ".to_string(), "ayykalam ".to_string(), i.to_string(), " ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;


   // send_fragments(chunks, i, size_of_chunks).await;

   
   
   // let base64_string = base64::encode(image);

   // let sender_task = sender("10.7.57.31:5001".to_string(), ["maya".to_string()," ip_request maya ".to_string(), base64_string.to_string()].concat()).await;
   // let sender_task = sender("10.7.57.93:5001".to_string(), ["maya".to_string()," ip_request maya ".to_string(), base64_string.to_string()].concat()).await;


   Ok(())
}

async fn send_fragments(chunks: Vec<Vec<u8>>, mut i: i32, size_of_chunks: i32, serv_ip: String) 
{
   for image_part in chunks 
   {
      let base64_string = base64::encode(image_part);
      // println!("base64_string: {:?}",base64_string);
      let mut is_last_packet = "false ";
      if i == size_of_chunks -1 
      {
         println!("-----------> last packet!");
         is_last_packet = "true ";
      }


      let sender_task = sender(serv_ip.clone(), ["ali ".to_string(),"encrypt ali ".to_string(), i.to_string(), " ".to_string(), is_last_packet.to_string() ,base64_string.to_string()].concat()).await;
      sender_task;
      // let sender_task = sender("10.7.57.93:5001" .to_string(), ["maya".to_string()," ip_request maya ".to_string(), base64_string.to_string()].concat()).await;
      time::sleep(Duration::from_millis(200)).await;
      println!("Messages are sent");
      i+=1; 
   }
}