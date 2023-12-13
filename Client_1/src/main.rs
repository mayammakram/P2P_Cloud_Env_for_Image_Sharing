use tokio::net::UdpSocket;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use std::fs:: {File, read};


//Image
use show_image::{ImageView, ImageInfo, create_window};
use image::{
    ImageBuffer,
    Rgba,
    DynamicImage,
    ImageFormat
};

//Encryption
use steganography::decoder::Decoder;
use steganography::util::{str_to_bytes, file_as_dynamic_image, file_as_image_buffer, bytes_to_str, save_image_buffer};
use base64;
use std::io::Write;

//Threads
use std::thread;




//structs
#[derive(Debug)]
struct image_details{
   owner_name: String,
   number_of_views: u32,
}




lazy_static! {
    static ref IP_ADDRESS_MAP: Arc<Mutex<HashMap<String, Vec<String>>>> = {
        let mut m = HashMap::new();
        Arc::new(Mutex::new(m))
    };

    static ref NAME_IP_ADDRESS_MAP: Arc<Mutex<HashMap<String, String>>> = {
        let mut m = HashMap::new();
        Arc::new(Mutex::new(m))
    };

    static ref IMAGE_DETAILS_MAP: Arc<Mutex<HashMap<String, image_details>>> = { //image path, image details
        let mut m = HashMap::new();
        Arc::new(Mutex::new(m))
    };
    static ref CLIENT_IP: Mutex<String> = Mutex::new("0.0.0.0".to_string());

    static ref FRIEND_NAME: Mutex<String> = Mutex::new("UNDEFINED_NAME".to_string());
}




//ENCRYPTION
//------------------------------------------------------------/

fn text_to_image(base64_string: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Decode the Base64 string into a byte array
    let image_data = base64::decode(base64_string)?;
    Ok((image_data))
}

fn decrypt_image(path: String, counter: i32) -> () {    
   
    let encoded_image = file_as_image_buffer(path);    
    let dec = Decoder::new(encoded_image);
    let out_buffer = dec.decode_alpha();
    let clean_buffer: Vec<u8> = out_buffer.into_iter()
                                    .filter(|b| {
                                        *b != 0xff_u8
                                    })
                                    .collect();
                                
    let message = bytes_to_str(clean_buffer.as_slice());

    let decoded_img = text_to_image(message ).unwrap();
    // Save the byte array as an image file
    let filename = format!("/home/aliali/Image_Sharing_DS/client/src/decoded_msg_{}.png", counter);
    let mut file = File::create(filename).unwrap();
    file.write_all(&decoded_img);

}


//COMMUNICATION -- SENDER
//------------------------------------------------------------/

async fn sender(ip:String, msg:String) -> Result<(), Box<dyn Error>> {
    // payload message to send
    let host_name = msg;

    // sends to the indicated IP, on port 5353
    let receiver_addr = ip.clone(); // Update with the receiver's address

    println!("-------------Sending a message to {}", ip);
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    
    let message = format!("{}", host_name);
    socket.send_to(message.as_bytes(), receiver_addr.clone()).await?;
    println!("--------> sent message to {} !", receiver_addr);

    Ok(())
}

async fn send_fragments(chunks: Vec<Vec<u8>>, mut i: i32, size_of_chunks: i32, serv_ip: String) {
    for image_part in chunks {
        let base64_string = base64::encode(image_part);
        let mut is_last_packet = "false ";
        if i == size_of_chunks  -1 
        {
            println!("-----------> last packet!");
            is_last_packet = "true ";
        }


        let sender_task = sender(serv_ip.clone(), ["maya".to_string()," encrypt maya ".to_string(), i.to_string(), " ".to_string(), is_last_packet.to_string() ,base64_string.to_string()].concat()).await;
        sender_task;
        time::sleep(Duration::from_millis(100)).await;
        println!("Messages are sent");
        i+=1;     
    }
}

async fn send_fragments_to_client(chunks: Vec<Vec<u8>>, mut i: i32, size_of_chunks: i32, client_ip: String, client_ip_no_port: String) {
    println!("_________________________________________________________________________________________");

    println!("Client IP in Serverr {:?}", client_ip.clone());
    println!("Client_ip_no_port in Serverr {:?}", client_ip_no_port.clone());
    
    for image_part in chunks {
        let base64_string = base64::encode(image_part);
        let mut is_last_packet = "false ";
        let access_rights = "5";
        if i == size_of_chunks  -1 
        {
            println!("-----------> last packet!");
            is_last_packet = "true ";
        }


        let sender_task = sender(client_ip.clone(), ["encrypt ".to_string(), "10.7.57.87".to_string(), " ".to_string(), is_last_packet.to_string() ,base64_string.to_string(), " ".to_string(), access_rights.to_string()," mortadella_comp1".to_string()].concat()).await;
        sender_task;
        if(is_last_packet == "false "){
            time::sleep(Duration::from_millis(100)).await;
        }
        println!("Messages are sent");
        i+=1;    

        // Wait for acknowledgement with a timeout of 5 seconds
        if is_last_packet == "true " {
            println!("LAST PKT REACHED: Waiting for acknowledgement");
            let result = tokio::time::timeout(Duration::from_secs(20), listener_to_client_ack()).await;
            match result {
                Ok(mm) => println!("Acknowledgement received: {:?}", mm),
                Err(e) => {
                    println!("No acknowledgement received within 20 seconds: {:?}", e);
                    //send to server that the client is not reachable
                    let sender_task = sender("10.7.57.93:5001" .to_string(), ["maya ".to_string(),"offline_device ".to_string(), client_ip_no_port.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string(), "ayykalam ".to_string()].concat()).await;
                    let sender_task = sender("10.7.57.31:5001" .to_string(), ["maya ".to_string(),"offline_device ".to_string(), client_ip_no_port.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string(), "ayykalam ".to_string()].concat()).await;
                    let sender_task = sender("10.7.57.77:5001" .to_string(), ["maya ".to_string(),"offline_device ".to_string(), client_ip_no_port.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string(), "ayykalam ".to_string()].concat()).await;
                },
            }
        };
 
    }
}



//COMMUNICATION -- Listener
//------------------------------------------------------------/
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
            let server_response = mm.split_whitespace().collect::<Vec<_>>();
            let response_type = server_response[0];
            println!("Received message from: {:?}", remote_ip);
            // Clear the buffer for the next message.
            buffer.clear();
            return Ok(mm);
        }
        
        Err("No message received".into())
    }

 async fn listener_to_client() -> Result<String, Box<dyn Error>> {
    let listener_addr = "0.0.0.0:8080"; // Update with the listener's address
 
    let socket = UdpSocket::bind(listener_addr).await?;
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
 }
 
 async fn listener_to_client_ack() -> Result<String, Box<dyn Error>> {
    println!("listener to CLIENT TWO started");
    let listener_addr = "0.0.0.0:8083"; // Update with the listener's address
 
    let socket = UdpSocket::bind(listener_addr).await?;
    println!("listener to CLIENT TWO started on {}",  listener_addr);
 

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
 }



 //TERMINAL
 //------------------------------------------------------------/
 fn pop_image(path: String, index: String) -> Result<(), Box<dyn Error>>
 {
    let img = image::open(path)?;
    
    // Convert the image to a byte slice.
    let bytes = img.to_bytes();
    
    // Convert the byte slice to an ImageView.
    let image_view = ImageView::new(ImageInfo::rgb8(img.width(), img.height()), &bytes);
    
    // Create a window with default options and display the image.
    let window = create_window(["image", &index].concat(), Default::default())?;
    window.set_image(["image-", &index].concat(), image_view)?;
    
    Ok(())
 }

 fn pop_image_view() -> Result<(), Box<dyn Error>>
 {
    let img = image::open("/home/aliali/Image_Sharing_DS/client/src/decoded_msg_0.png")?;
    let mask_img = image::open( "/home/aliali/Image_Sharing_DS/client/src/encoded_msg.png")?;

    let mask_img_bytes = mask_img.to_bytes();
    // Convert the image to a byte slice.
    let bytes = img.to_bytes();
    
    // Convert the byte slice to an ImageView.
    let image_view = ImageView::new(ImageInfo::rgb8(img.width(), img.height()), &bytes);
    let mask_image_view = ImageView::new(ImageInfo::rgb8(mask_img.width(), mask_img.height()), &mask_img_bytes);
    // decrement access rights from image details map given the key "mortadella_comp1"
    let mut image_details_map = IMAGE_DETAILS_MAP.lock().unwrap();
    // create bool var to check if number of views is 0
    let mut is_zero = false;
    for(key, value) in image_details_map.iter_mut()
    {
        if key.eq("mortadella_comp1")
        {
            if value.number_of_views == 0
            {
                is_zero = true;
            }

            value.number_of_views -= 1;
            
        }
    }
    
    println!("Image details map: {:?}", image_details_map);

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    // if is_zero is true, display mask image
    if is_zero == true
    {
        window.set_image("image", mask_image_view)?;
    }
    else
    {
    window.set_image("image", image_view)?;
    }
    Ok(())
 }
 async fn update_rights(index: String, rights: String, client_ip: String, client_name: String) -> Result<(), Box<dyn Error>>
 {
    let sender_task = sender([client_ip.clone(), ":8080".to_string()].concat(), ["maya ".to_string(), "10.7.57.87 ".to_string(), "update_image ".to_string(), "first ".to_string(), "0 ".to_string(), "false ".to_string() , " mortadella_comp1 ".to_string(), rights.to_string()].concat()).await;
    
    let result = tokio::time::timeout(Duration::from_secs(4), listener_to_client_ack()).await;

    match result {
        Ok(mm) => println!("Acknowledgement FOR UPDATE IMAGE received: {:?}", mm),
        Err(e) => {
            println!("No acknowledgement received within 4 seconds: {:?}", e);
            //send to server that the client is not reachable
            println!("Client IP in ay 7aga mofeda nefham menha: {}", client_ip);
            let sender_task = sender("10.7.57.93:5001" .to_string(), ["maya ".to_string(),"offline_device ".to_string(), client_ip.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string(), "ayykalam ".to_string()].concat()).await;
            let sender_task = sender("10.7.57.31:5001" .to_string(), ["maya ".to_string(),"offline_device ".to_string(), client_ip.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string(), "ayykalam ".to_string()].concat()).await;
            let sender_task = sender("10.7.57.77:5001" .to_string(), ["maya ".to_string(),"offline_device ".to_string(), client_ip.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string(), "ayykalam ".to_string()].concat()).await;
            
            thread::sleep(Duration::from_secs(1));
            let sender_task = sender("10.7.57.93:5001" .to_string(), ["maya ".to_string(),"buffer ".to_string(), client_name.to_string(), " ".to_string(), rights.to_string(), " ".to_string(), index.to_string() , " ayykalambardo".to_string()].concat()).await;
            let sender_task = sender("10.7.57.31:5001" .to_string(), ["maya ".to_string(),"buffer ".to_string(), client_name.to_string(), " ".to_string(), rights.to_string(), " ".to_string(), index.to_string() , " ayykalambardo".to_string()].concat()).await;
            let sender_task = sender("10.7.57.77:5001" .to_string(), ["maya ".to_string(),"buffer ".to_string(), client_name.to_string(), " ".to_string(), rights.to_string(), " ".to_string(), index.to_string() , " ayykalambardo".to_string()].concat()).await;
                                    
        },
    }
    
    Ok(())
 }
 async fn ask_for_input() -> String
 {
    println!("Hello ");
    println!("Welcome to Image Sharing App!");
    println!("This project allows you to view all your images in one place and share them with your friends.");
    let mut input = String::new();

        println!("To start, please select one of the following: ");
        println!("1. Request to view a friend's image");
        println!("2. Update access rights of an image for a friend");
        println!("3. View an image");

        std::io::stdin().read_line(&mut input).unwrap();
        input
 }

 async fn cli_scenarios(clients_sender_name: String) {
    if clients_sender_name.eq("1\n")
        {        
            // SEND IP_REQUEST
            println!("Please enter your friend's name you would like to view an image from: ");
            let mut input1 = String::new();
            std::io::stdin().read_line(&mut input1).unwrap();
            println!("Your friend's computer name is: {}", input1);

            let sender_task = sender("10.7.57.31:5001".to_string(), ["maya".to_string()," ip_request ".to_string(), input1.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string()].concat()).await;
            let sender_task = sender("10.7.57.77:5001".to_string(), ["maya".to_string()," ip_request ".to_string(), input1.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string()].concat()).await;
            let sender_task = sender("10.7.57.93:5001".to_string(), ["maya".to_string()," ip_request ".to_string(), input1.to_string(), " ayykalam ".to_string(), "0 ".to_string(), "false ".to_string()].concat()).await;
        }
    if clients_sender_name.eq("2\n")
        {        
            // SEND IP_REQUEST
            println!("Please enter your friend's name to update access rights: ");
            {
                let mut friend_name_guard = FRIEND_NAME.lock().unwrap();
                // clear the value in friend_name_guard
                *friend_name_guard = "".to_string();
              
                std::io::stdin().read_line(&mut friend_name_guard).unwrap();
                println!("Your friend's computer name is: {}", friend_name_guard);
            
                let sender_task = sender("10.7.57.31:5001".to_string(), ["maya".to_string()," ip_request ".to_string(), friend_name_guard.to_string(), " access_rights ".to_string(), "0 ".to_string(), "false ".to_string()].concat()).await;
                let sender_task = sender("10.7.57.77:5001".to_string(), ["maya".to_string()," ip_request ".to_string(), friend_name_guard.to_string(), " access_rights ".to_string(), "0 ".to_string(), "false ".to_string()].concat()).await;
                let sender_task = sender("10.7.57.93:5001".to_string(), ["maya".to_string()," ip_request ".to_string(), friend_name_guard.to_string(), " access_rights ".to_string(), "0 ".to_string(), "false ".to_string()].concat()).await;
            }
            
        }
        if clients_sender_name.eq("3\n"){
            println!("View Image");
            pop_image_view();
        }

}
//------------------------------------------------------------/
#[show_image::main]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    static mut path_of_image: &'static str =  "/home/aliali/Image_Sharing_DS/client/src/elkadi.png";
    let mut user_image_choice = "1";
    let mut counter = 0;

    //Thread that listens to the client
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
                    let client_response_type = server_response[2].to_string();

                    let mut client_port_w_port = [server_ip.clone(), ":8080".to_string()].concat();
                    println!("Message from client: {:?}", encoded_data);
                    // response type -- server ip -- last_packet_flag -- image_fragment    
                    if client_response_type.eq("request_view")
                    {

                        let access_rights = 5;
                        println!("We are in the request view!!!");
                        let mut map = IMAGE_DETAILS_MAP.lock().unwrap();
                        let mut _image_details = image_details {
                            owner_name: String::from("maya"),
                            number_of_views: 5,
                        };
                        map.insert("/home/aliali/Image_Sharing_DS/client/src/compressed_gallery/mortadella_comp1.jpg".to_string(), _image_details);
                        let compressed_image1 = read("/home/aliali/Image_Sharing_DS/client/src/compressed_gallery/mortadella_comp1.jpg").unwrap();
                        let base64_string_sample1 = base64::encode(compressed_image1);
                        let sender_task = sender(client_port_w_port.clone(), ["maya ".to_string(), "10.7.57.87 ".to_string(), "view_image ".to_string(), "first ".to_string(), "0 ".to_string(), "false ".to_string() , base64_string_sample1.to_string()," ".to_string(),access_rights.to_string()].concat()).await;


                        time::sleep(Duration::from_millis(500)).await;
                        _image_details = image_details {
                            owner_name: String::from("maya"),
                            number_of_views: 5,
                        };
                        map.insert("/home/aliali/Image_Sharing_DS/client/src/compressed_gallery/elkadi_comp.jpg".to_string(), _image_details);

                        let compressed_image2 = read("/home/aliali/Image_Sharing_DS/client/src/compressed_gallery/elkadi_comp.jpg").unwrap();
                        let base64_string_sample2 = base64::encode(compressed_image2);
                        let sender_task = sender(client_port_w_port.clone(), ["maya ".to_string(), "10.7.57.87 ".to_string(), "view_image ".to_string(), "last ".to_string(), "0 ".to_string(), "false ".to_string() , base64_string_sample2.to_string()," ".to_string(),access_rights.to_string()].concat()).await;
                    }           
                    if client_response_type.eq("view_image")
                    {
                        println!("We are in the view!!!");
                        let input_index = server_response[3].to_string();
                        let base64_string_sample1 = server_response[6].to_string();
                        // response type -- server ip -- response_type -- input index -- base64_string_sample1               
                        // maya             10.0.0.0         view            first/last                0               false           ayykalambardo

                        // convert base64 string to image
                        let image_data = base64::decode(base64_string_sample1).unwrap();
                        let path: String = format!("/home/aliali/Image_Sharing_DS/client/src/compressed_gallery/{}_{}.jpg",response_type,input_index);
                        
                        let mut file = File::create(path.clone()).unwrap();
                        file.write_all(&image_data);
                        pop_image(path.clone(),input_index.clone());

                        if input_index.eq("first")
                        {
                            println!("Hello ");
                            println!("Welcome to Image Sharing App!");
                            println!("Below are the list of images you can view:");
                        }

                        if input_index.eq("last")
                        {
                            println!("Please enter the index of the image you want to view: ");
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            println!("You chose image number: {}", input);
                            let image_choice = input.trim_end().to_string();
                            let sender_task = sender(server_ip.clone() + ":8080", ["maya ".to_string(), "send_image ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , image_choice.to_string()].concat()).await;
                            //push in map client name and ip 
                            {
                                let mut client_ip_guard = CLIENT_IP.lock().unwrap();
                                *client_ip_guard = server_ip.clone() + ":8080";
                                println!("Client IP: {}", client_ip_guard);
                            }
                           
                        }
                    }
                    if client_response_type.eq("update_image")
                    {
                        println!("We are in the update_image!!!");
                        let input_index = server_response[3].to_string();
                        let base64_string_sample1 = server_response[6].to_string();
                        let rights = server_response[7].to_string();
                        // response type -- server ip -- response_type -- input index -- base64_string_sample1               
                        {
                            let mut image_details_map = IMAGE_DETAILS_MAP.lock().unwrap();
                            for(key, value) in image_details_map.iter_mut()
                            {
                                if key.eq(&base64_string_sample1)
                                {
                                    value.number_of_views = rights.parse::<u32>().unwrap();
                                }
                            }
                        }
                        println!("Image details map: {:?}", *IMAGE_DETAILS_MAP.lock().unwrap());
                    }
                    if client_response_type.eq("send_image")
                    {
                        println!("We are in the send_image!!!");

                        let user_image_choice = server_response[6].to_string();


                        if user_image_choice == "first"
                        {
                            unsafe{
                            path_of_image = "/home/aliali/Image_Sharing_DS/client/src/mortadella.png".clone();
                            }
                        }
                        else if user_image_choice == "second"
                        {
                            unsafe{
                            path_of_image= "/home/aliali/Image_Sharing_DS/client/src/elkadi.png".clone();
                            }
                        }
                        else if user_image_choice == "last"
                        {
                            println!("-------------------------------------------------> LAST IMAGE SELECTED");
                            unsafe {
                            path_of_image="/home/aliali/Image_Sharing_DS/client/src/elkadi.png".clone();
                            }
                        }

                        {
                                let mut client_ip_guard = CLIENT_IP.lock().unwrap();
                                *client_ip_guard = server_ip.clone() + ":8080";
                                println!("\n send_image --> Client IP: {}", client_ip_guard);
                        }
                        let sender_task = sender("10.7.57.93:5001" .to_string(), ["maya ".to_string(),"ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                        let sender_task = sender("10.7.57.31:5001" .to_string(), ["maya ".to_string(),"ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                        let sender_task = sender("10.7.57.77:5001" .to_string(), ["maya ".to_string(),"ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                        
                    }
                    if response_type.eq("encrypt")
                    {
                        let last_packet_flag = server_response[2].to_string();
                        let encoded_image_packet = server_response[3].to_string();
                        let access_rights = server_response[4].to_string();

                        {
                            let mut map = IP_ADDRESS_MAP.lock().unwrap();
                            map.entry(server_ip.clone()).or_insert_with(Vec::new).push(encoded_image_packet.clone());
                        }

                    
                        if last_packet_flag.eq("true")
                        {
                            {
                                let senderTask = sender([server_ip.clone(), ":8083".to_string()].concat() , ["IMG_RECEIVED ".to_string(), server_ip.clone()].concat()).await;
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
                                        
                                    match text_to_image(&encoded_image_string) 
                                    {
                                        Ok(encoded_img) => 
                                        {
                                            let mut file = File::create("/home/aliali/Image_sharing_DS/client/src/encoded_msg.png").unwrap();
                                            file.write_all(&encoded_img);
                                            // handle the result of base64::decode here
                                            decrypt_image("/home/aliali/Image_sharing_DS/client/src/encoded_msg.png".to_string(),counter);
                                        
                                        },
                                        Err(err) => 
                                        {
                                        eprintln!("Error in text_to_image: {}", err);
                                        }
                                    } 
                                }
                            }
    
                            {
                            let mut image_details_map = IMAGE_DETAILS_MAP.lock().unwrap();
                            let _image_details = image_details {
                                owner_name: String::from("maya"),
                                
                                number_of_views: access_rights.parse::<u32>().unwrap(),
                            };
                            let image_name = server_response[5].to_string();
                            if !image_details_map.contains_key(image_name.clone().as_str()) {
                                image_details_map.insert(image_name.to_string(), _image_details);
                            }
                       
                            }
                            println!("Image details map: {:?}", *IMAGE_DETAILS_MAP.lock().unwrap());
    
                            let mut clients_sender_name = ask_for_input().await;

                            println!("You entered: {}-", clients_sender_name);
                            cli_scenarios(clients_sender_name).await;
                        }
                    }
                },
                Err(err) => 
                {
                    eprintln!("Error in text_to_image: {}", err);                
                }
                }
            }
        });
    });

    unsafe
    {
        let mut client_ip_no_port: String = "0.0.0.0".to_string();

        //Thread that listens to the server
        tokio::task::spawn_blocking(move ||{ 
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
                          
                            println!("Message from server: {:?}", encoded_data);
                            // response type -- server ip -- last_packet_flag -- image_fragment
                            if response_type.eq("ack-encrypt")
                            {
                                //fragemnt the image into parts of length of 1024
                                let mut i = 0;
                                let mut chunks = Vec::new();
                                let chunk_size = 30000;

                                let mut size_of_chunks = 0;

                                let image: Vec<u8> = read(path_of_image.clone()).unwrap();

                                for chunk in image.chunks(chunk_size){

                                    chunks.push(chunk.to_vec());
                                    size_of_chunks+=1;
                                }
                                println!("DURING ENCRYPTION !!!!!!!!!!!!!!!!!!!!!!!");

                                send_fragments(chunks, i, size_of_chunks, server_ip.clone()).await;
                            }
                            if response_type.eq("encrypt")
                            {
                                let last_packet_flag = server_response[2].to_string();
                                let encoded_image_packet = server_response[3].to_string();
                                {
                                    let mut map = IP_ADDRESS_MAP.lock().unwrap();
                                    map.entry(server_ip.clone()).or_insert_with(Vec::new).push(encoded_image_packet.clone());
                                }
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

                                            match text_to_image(&encoded_image_string) {
                                                Ok(encoded_img) => {
                                                    let mut file = File::create("/home/aliali/Image_Sharing_DS/client/src/encoded_msg.png").unwrap();
                                                    file.write_all(&encoded_img);
                                                    // handle the result of base64::decode here
                                                    let encoded_image_data: Vec<u8> = read("/home/aliali/Image_Sharing_DS/client/src/encoded_msg.png").unwrap();
                                                    
                                                    let mut i = 0;
                                                    let mut chunks = Vec::new();
                                                    let chunk_size = 30000;
                    
                                                    let mut size_of_chunks = 0;
                    
                                                    for chunk in encoded_image_data.chunks(chunk_size){
                    
                                                        chunks.push(chunk.to_vec());
                                                        size_of_chunks+=1;
                                                    }

                                                    {
                                                        let  client_ip_guard = CLIENT_IP.lock().unwrap();                 
                                                        println!("Client IP: {}", client_ip_guard);
                                                        send_fragments_to_client(chunks, i, size_of_chunks, client_ip_guard.to_string(),client_ip_no_port.clone()).await;
                                                    }
                                                    //DECRYPTION
                                                    decrypt_image("/home/aliali/Image_Sharing_DS/client/src/encoded_msg.png".to_string(), counter);
                                                    if counter < 10
                                                    {
                                                        time::sleep(Duration::from_secs(1)).await;

                                                        println!("Counter {}", counter);
                                                        let sender_task = sender("10.7.57.93:5001" .to_string(), ["maya".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                                        let sender_task = sender("10.7.57.31:5001" .to_string(), ["maya".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                                        let sender_task = sender("10.7.57.77:5001" .to_string(), ["maya".to_string(),counter.to_string()," ack-encrypt ".to_string(), "ayykalam ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                                        counter += 1;
                                                    }
                                                },
                                                Err(err) => {
                                                    println!("Error in text_to_image: {}", err);
                                                }
                                            }  
                                        }
                                    }
                                }
                            }
                            else if response_type.eq("ip_response")
                            {
                                if (server_ip.clone() == "Device_Not_Reachable")
                                {
                                    println!("Device_Not_Reachable");
                                    {
                                        // acquire lock for friend name guard
                                        println!("AWOOOO ----- WE THOUGHT WE WOULDNT BE HERE LOL -----");
                                        let mut friend_name_guard = FRIEND_NAME.lock().unwrap();
                                        let sender_task = sender("10.7.57.93:5001" .to_string(), ["maya ".to_string(),"buffer ".to_string(), friend_name_guard.to_string(), " 4 ".to_string(), "mortadella_comp1 ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                        let sender_task = sender("10.7.57.31:5001" .to_string(), ["maya ".to_string(),"buffer ".to_string(), friend_name_guard.to_string(), " 4 ".to_string(), "mortadella_comp1 ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                        let sender_task = sender("10.7.57.77:5001" .to_string(), ["maya ".to_string(),"buffer ".to_string(), friend_name_guard.to_string(), " 4 ".to_string(), "mortadella_comp1 ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                       
                                    }
                                    // continue;
                                }
                                else
                                {
                                    let mut path_type = server_response[2].to_string();
                                    let mut client_ip_guard = CLIENT_IP.lock().unwrap();

                                        println!("Received IP Request Response!!!");
                                        *client_ip_guard = server_ip.clone() + ":8080";
                                        client_ip_no_port = server_ip.clone();

                                        println!("Client IP: {}", client_ip_guard);
                                        
                                        if path_type.eq("image_path"){
                                            let sender_task = sender(client_ip_guard.to_string(), ["maya ".to_string(), "10.7.57.87 ".to_string(), "request_view ".to_string(), "first ".to_string(), "0 ".to_string(), "false ".to_string() , "ayykalambardo".to_string()].concat()).await;
                                        }
                                        else if path_type.eq("rights_path"){
                                            // take user input in the input user variable
                                            println!("1- Update access rights of an image\n2- Revoke access rights of an image\n");
                                            let mut input_user = String::new();
                                            std::io::stdin().read_line(&mut input_user).unwrap();
                                            
                                            if input_user.eq("1\n")
                                            {
                                                println!("Please enter the index of the image you want to update: ");
                                                
                                                let map = IMAGE_DETAILS_MAP.lock().unwrap();
                                                // Convert the hash map into a vector
                                                let vec: Vec<_> = map.iter().collect();
                                                // Iterate over the vector and keep track of the index
                                                for (i, (key, value)) in vec.iter().enumerate() {
                                                    println!("{}- {}", i, key);
                                                }
                                                let mut index = String::new();
                                                let mut new_access_rights = String::new();
                                                std::io::stdin().read_line(&mut index).unwrap();
                                                println!("Please enter the new access rights: ");
                                                std::io::stdin().read_line(&mut new_access_rights).unwrap();
                                                {
                                                    let mut friend_name_guard = FRIEND_NAME.lock().unwrap();
                                                    update_rights(index.clone(), new_access_rights.clone(),server_ip.clone(), friend_name_guard.clone()).await;

                                                }
                                    
                                            }
                                            else if(input_user.eq("2\n"))
                                            {
                                                let mut index = String::new();

                                                println!("Please enter the index of the image you want to revoke: ");
                                                std::io::stdin().read_line(&mut index).unwrap();

                                                let mut friend_name_guard = FRIEND_NAME.lock().unwrap();
                                                update_rights(index.clone(), "0".to_string(),server_ip.clone(), friend_name_guard.clone()).await;
                                            }

                                            else{
                                                println!("Invalid input. Please restart the program and try again.");
                                            }
                                    }                                        
                                        println!("Messages are sent");
                                }
                            }

                            if response_type.eq("update_image")
                            {
                                println!("We are in the update_image!!!");
                                println!("Server response: {:?}", server_response);
                                
                                let base64_string_sample1 = server_response[1].to_string();
                                let rights = server_response[2].to_string();
                                // response type -- server ip -- response_type -- input index -- base64_string_sample1               
                                // maya
                                {
                                    let mut image_details_map = IMAGE_DETAILS_MAP.lock().unwrap();
                                    for(key, value) in image_details_map.iter_mut()
                                    {
                                        if key.eq(&base64_string_sample1)
                                        {
                                            value.number_of_views = rights.parse::<u32>().unwrap();
                                        }
                                    }
                                }
                                println!("Image details map: {:?}", *IMAGE_DETAILS_MAP.lock().unwrap());
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

    }
        println!("Initial Message: IP request");
        let mut clients_sender_name = ask_for_input().await;

        println!("You entered: {}-", clients_sender_name);
        cli_scenarios(clients_sender_name).await;       
    Ok(())
}