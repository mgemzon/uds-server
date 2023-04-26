# Unix Domain Sockets Server

## How to run the program  
### Building and Running the server program
After cloning this repository, and going into the repository
```shell
cargo build server
cargo run server
```
### Testing the program with `socat`  
`socat` is a utility program for creating bidirectional byte streams between  
two endpoints, including the Unix domain sockets.  
! To make this next steps, work be sure that the server program is already up.  
In a separate terminal:  
```shell
$ socat - UNIX-CONNECT:/tmp/my_socket.sock  
```  
Once connected, you can send messages from `socat` to the server by typing  
text and pressing `Enter`.  