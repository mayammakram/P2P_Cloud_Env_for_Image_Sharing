# Overview
The project at hand is a comprehensive cloud infrastructure that places a significant emphasis on transparency, load balancing, and fault tolerance. Leveraging distributed election algorithms, the system is designed to support key cloud qualities. The primary services offered by this cloud include image encryption through steganography and a discovery service.

# FEATURES
## A. Image Encryption and Decryption through Steganography
**1) User Registration:** Users are provided with a registration process, allowing them to encrypt their images using steganography. Users are automatically registered into the cloud when they decide to request an image from another user. The steganographic technique ensures the secure embedding of information within the images. Users, upon registering with the cloud’s discovery service, will have to encrypt their images before sharing them with their peers.

**2) Gallery Viewing and Image Compression:** Upon requesting to view images from a certain client, an entire gallery of compressed low-res images is sent to the user to choose the desired image.

**3) Image Encryption and Decryption:** A high-resolution copy of the desired image is then sent for encryption at the server’s end and sent back to the client, where its access rights are encapsulated. The desired image is then sent to its destination client, where it is decrypted and viewed according to its number of views.

**4) Viewing Permissions:** Images are accompanied by specified viewing permissions, and users can set the number of
permitted views. The cloud ensures that users can only see
their images with restricted views. If the user exceeds that
number, they have no access to view the decrypted image.
Unless the user has newly updated rights, the encrypted
image pops up whenever the user requests to view that image
(assuming they have completed their number of views).

**5) Ownership Control and Offline Operation Support:**
Owners retain control over their images, being able to update
and modify viewing quotas. To support offline operations, the
system follows an acknowledgment reply to detect if a client
is offline. Whenever a client is sent an image, it has to send
to the owner client an acknowledgment back saying that the
image has been received. If an acknowledgment has not been
received after a specified period, after a couple of attempts,
the receiver user is considered offline. This feature aims to
provide a robust mechanism for reconstructing permissions in
case of failures.

## B. Discovery Service
**1) Peer Registration and Online Visibility:** The discovery
service serves as a fundamental component where users register to identify online peers. This registration allows users
to understand which peers are currently online and how to
establish direct P2P communication with them. The table
of online users is consistently maintained within the cloud,
promoting reliable and up-to-date peer discovery.

**2) P2P Communication:** The P2P architecture plays a crucial role in the project, enabling users to communicate directly
without relying on the central cloud infrastructure when both
parties are online. This design choice enhances the efficiency
of communication and reduces reliance on cloud resources,
contributing to a more resilient and scalable system.

**3) Offline Support for Consistency:** Offline support is integrated into the discovery service to handle scenarios where
users or image owners are offline. After a client goes offline
and the owner has been notified, the cloud is notified, and
the offline host is removed from the directory of service.
If an owner wants to modify the access rights to any of
the offline clients, it sends the updated access rights to the
cloud, the cloud buffers it, and whenever the offline client
registers itself once again as active, the cloud directly sends the
buffered rights to the designated host. This proactive approach
helps mitigate the impact of failures and provides a seamless
experience for users.

# USECASES
## A. Case I: Request an Image
1) Client B requests to view Client A’s gallery.
2) The server does an IP look-up on Client A’s hostname.
3) If it exists, the server sends back the associated IP
address. Otherwise, notify Client B that Client A is not
registered within the cloud.
4) Client B requests from Client A its gallery and sends
back to Client A the index of the desired image.
5) Client A sends the high-resolution image for encryption
and sends it to Client B, encapsulated with its access
rights.
6) Client B can only view the image any time within its
limited number of views. Upon reaching the limit, only
the encrypted image appears.
## B. Case II: Update Access Permissions
1) Client B has requested to view an image with all the
previous steps.
2) Client A has decided to update Client B’s access rights.
3) If Client B is online, it automatically receives its newly
updated rights
## C. Case III: Offline Client
1) If client B goes offline after receiving an encrypted
image.
2) Client A is notified and sends to the server to update its
directory of service.
## D. Update Permissions of an Offline Client
1) If client A wants to update the access rights of a specific
image on an offline client.
2) Client A sends the required information along with the
new viewing permissions to the cloud
3) The cloud then buffers the request in its servers and
waits for the offline client to become online
4) Once the client registers itself onto the cloud, the cloud
automatically sends the updated access rights, and the
viewing permissions are updated
5) The current online client is able to view the image at
any time with its new permissions.
It is good to note that a new leader is always elected based
on CPU Usage to handle any request.

# PERFORMANCE EVALUATION
It is estimated that each request takes about 18 sec. How-
ever, this differs when there is multiple clients requesting at
the same time. In the election, it was reduced a lot to finish all
client requests. It balanced and handled the requests smoothly
compared to when no election happened. The tables below
will show 1000 requests from each client and how much time
is needed to finish all the requests.

**With Balance Loading:**
| Clients  | Time Estimate to finish 1000 requests | Average Time Estimate by each request |
|----------|---------------------------------------|---------------------------------------|
| Client 1 | 6.03 hours                            | 21.72 sec                             |
| Client 2 | 5.6 hours                             | 20.52 sec                             |
| Client 3 | 5.95 hours                            | 21.42 sec                             |

**Without Balance Loading:**
| Clients  | Time Estimate to finish 1000 requests | Average Time Estimate by each request |
|----------|---------------------------------------|---------------------------------------|
| Client 1 | 8.7 hours                             | 31.32 sec                             |
| Client 2 | 8.4 hours                             | 30.24 sec                             |
| Client 3 | 8.64 hours                            | 31.104 sec                            |

Load balancing proves to be a game-changer in enhancing
server performance, as evidenced by a comparative analysis of two scenarios. In the balanced loading setup, where
requests are smartly distributed among servers, the system
operates more efficiently and responsively. The key difference
in processing times between the load-balanced and non-load balanced scenarios boils down to how server resources are
allocated.

When servers share the load more evenly, the system’s
response to requests is better coordinated, leading to shorter
waiting times and overall improved performance. Distributing
requests across multiple servers prevent bottlenecks and allow for parallel processing, resulting in a noticeable decrease
in the average time it takes to handle each request, as shown
in the data.

For example, in the load-balanced configuration, Client
1 estimates 6.03 hours to complete 1000 requests, with an
average time of 21.72 seconds per request. Clients 2 and 3
also experience significant time savings and reduced average
processing times. This efficiency is a result of smartly balancing the workload among servers.

On the other hand, the non-load-balanced scenario reveals
inefficiencies due to uneven request distribution. Some servers
may sit idle while others are swamped, leading to longer waiting times and delayed processing. The substantial differences
in time estimates and average processing times for each client
in the non-load-balanced setup highlight suboptimal resource
use.

# COMPILATION PROCEDURE
To compile the project, follow the steps outlined below:
Simply run the following using Rust V. 1.66.1
```
cargo build
cargo run
```
*Make sure to change all the directory image paths as they
are the complete paths.*
# CONCLUSION
In conclusion, this project represents a significant advancement in cloud computing, meticulously addressing transparency, load balancing, fault tolerance, and user-centric design. The integration of distributed election algorithms en-
sures equitable workload distribution, optimizing resource
utilization and responsiveness. The fault-tolerant server clus-
ter exhibits resilience through simulated failures, promoting
system reliability and consistency. Leveraging steganogra-
phy for image encryption adds a layer of security, ensuring
confidentiality and integrity. The user-centric approach em-
powers owners with control, offline operation support, and
view updates. Looking ahead, potential enhancements include
machine learning for predictive load balancing and exploring
advanced encryption methods. The collaborative efforts of
the development team, coupled with effective communication,
have yielded a robust cloud infrastructure that not only meets
academic criteria but also demonstrates practical application
in addressing real-world challenges
The performance metrics underscore the positive impact of
load balancing on the cloud system. Beyond just speeding
up processing times, load balancing ensures fair resource
use, making it a vital element for efficiently handling multiple clients and requests. The data presented emphasizes the
tangible advantages of load balancing in enhancing overall
performance and responsiveness in the cloud infrastructure

***More details can be found in the Technical Report.***
