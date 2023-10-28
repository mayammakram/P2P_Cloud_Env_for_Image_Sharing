// use tokio::net::{TcpListener, TcpStream};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use rand::Rng;
// use std::net::SocketAddr;
// use std::error::Error;




//   fn selectServer() -> String{
//     println!("bruhhh");
//     let mut rng = rand::thread_rng();
//     let random_number: u32 = rng.gen_range(0..=1);
//     println!("Random number between 0 and 1: {}", random_number);
//     let mut s_port: &str = "";
//     if(random_number == 0){
//         s_port = "5050"
//     }
//     else{
//         s_port = "5051"
//     }
//     let s_msg : String= "10.7.29.200:".to_string() + &s_port.to_string();
//     return s_msg;
// }



// async fn sender(msg: &str) -> Result<(), Box<dyn Error>>  {

//     let ip_selected = selectServer();
//     println!("the ip selected is {}",ip_selected );
//     // payload message to send
//     let host_name = "Ali";
//     // sends to the indicated IP, on port 5353
//     let receiver_addr = ip_selected; // Update with the receiver's address



//     println!("-------------Sending a message");
//     let dest_addr: SocketAddr = receiver_addr.parse()?;
//     let mut stream = TcpStream::connect(dest_addr).await?;
//     println!("Connected to {}", receiver_addr);
    

//    let message = format!("{}", msg);
//     stream.write_all(message.as_bytes()).await?;
//     println!("--------> sent message!");


//     Ok(())
// }

// async fn listener() -> Result<(), Box<dyn Error> > {
//     let host_name = "Any Host";
//     // 0.0.0.0 means this machine accepts incoming requests from any IP
//     // listens on port 49278
//     let listener_addr = "0.0.0.0:8080"; // Update with the listener's address

//     let listener = TcpListener::bind(listener_addr).await?;
//     println!("--------Listener");
//     println!("{} has sent a message on broadcast addrs: {}", host_name, listener_addr);
//     while let Ok((mut stream, _)) = listener.accept().await {
//         let mut buffer = vec![0u8; 1024];
//         let n = stream.read(&mut buffer).await?;
        
//         if n == 0 {
//             break;
//         }

//         let message = String::from_utf8_lossy(&buffer[..n]);
//         println!("Message received from {}: {}", host_name, message);

//         // let response = format!("Hello from {}", host_name);
//         // stream.write_all(response.as_bytes()).await?;
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // let sender_task = sender().await?;
    
//     // sender_task; // Ensure the sender completes first

//     // Sends one message to three servers
//     for _ in 0..10 {
//         let sender_task = sender("Request access").await?;
//         sender_task; // Ensure the sender completes first
//     }
    
    
//     let receiver_task = listener().await?;
//     receiver_task;

//     Ok(())


// }


















use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::error::Error;
use std::time::Duration;
use tokio::time;

use std::thread;

// async fn sender() -> Result<(), Box<dyn Error>> {
//     // payload message to send
//     let host_name = "Ali";
//     // sends to the indicated IP, on port 5353
//     let receiver_addr = "10.7.29.200:8081"; // Update with the receiver's address

//     println!("-------------Sending a message");
//     let dest_addr: SocketAddr = receiver_addr.parse()?;
//     let mut stream = TcpStream::connect(dest_addr).await?;
//     println!("Connected to {}", receiver_addr);

//    let message = format!("{}", host_name);
//     stream.write_all(message.as_bytes()).await?;
//     println!("--------> sent message!");


//     Ok(())
// }


//**UDP

async fn sender(ip:&str) -> Result<(), Box<dyn Error>> {
    // payload message to send
    let host_name = "encrypt @aliali @mayamakram 1";
    // sends to the indicated IP, on port 5353
    let receiver_addr = ip; // Update with the receiver's address

    println!("-------------Sending a message");
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    // let dest_addr = receiver_addr.parse()?;
    
    let message = format!("{}", host_name);
    socket.send_to(message.as_bytes(), receiver_addr).await?;
    println!("--------> sent message!");

    Ok(())
}
//------------------------------------------------------------/

// async fn listener() -> Result<(), Box<dyn Error> > {
//     let host_name = "Any Host";
//     // 0.0.0.0 means this machine accepts incoming requests from any IP
//     // listens on port 49278
//     let listener_addr = "0.0.0.0:8080"; // Update with the listener's address

//     let listener = TcpListener::bind(listener_addr).await?;
//     println!("--------Listener");
//     println!("{} has sent a message on broadcast addrs: {}", host_name, listener_addr);
//     while let Ok((mut stream, _)) = listener.accept().await {
//         let mut buffer = vec![0u8; 1024];
//         let n = stream.read(&mut buffer).await?;
        
//         if n == 0 {
//             break;
//         }

//         let message = String::from_utf8_lossy(&buffer[..n]);
//         println!("Message received from {}: {}", host_name, message);

//         // let response = format!("Hello from {}", host_name);
//         // stream.write_all(response.as_bytes()).await?;
//     }

//     Ok(())
// }

//**UDP

// async fn listener() -> Result<(), Box<dyn Error>> {
//     let host_name = "Any Host";
//     let listener_addr = "0.0.0.0:8081"; // Update with the listener's address

//     let socket = UdpSocket::bind(listener_addr).await?;
//     println!("--------Listener");
//     println!("{} has sent a message on broadcast addrs: {}", host_name, listener_addr);
//     let mut buffer = vec![0u8; 1024];
//     while let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
//         let remote_ip = remote_addr.ip().to_string();
//         println!("IP is {}", remote_ip);
        
//         let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
        
//         // Clear the buffer for the next message.
//         buffer.clear();
//     }

//     Ok(())
// }
async fn listener() -> Result<(), Box<dyn Error>> {
    let host_name = "Any Host";
    let listener_addr = "0.0.0.0:8081"; // Update with the listener's address

    let socket = UdpSocket::bind(listener_addr).await?;
    println!("--------Listener");
    println!("{} has started listening on address: {}", host_name, listener_addr);

  
        let mut buffer = vec![0u8; 1024];
        if let Ok((len, remote_addr)) = socket.recv_from(&mut buffer).await {
            let remote_ip = remote_addr.ip().to_string();
            println!("IP is {}", remote_ip);
            
            let mm = String::from_utf8_lossy(&buffer[..len]).to_string();
            println!("message is {}", mm);
            // Clear the buffer for the next message.
            buffer.clear();
        }
        
        // Add a delay between iterations of the loop.
        time::sleep(Duration::from_secs(1)).await;
        Ok(())
}




//------------------------------------------------------------/


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let sender_task = sender().await?;
    
    // sender_task; // Ensure the sender completes first
    // let handle = thread::spawn(|| {
        // Code executed in the new thread

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

        println!("Thread1");
        for _ in 0..1 {
            let sender_task = sender("10.7.57.213:5001").await;
            let sender_task = sender("10.7.57.51:5001").await;
            // Ensure the sender completes first
        }


    Ok(())
}