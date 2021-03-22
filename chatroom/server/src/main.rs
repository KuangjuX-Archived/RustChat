use std::io::{ ErrorKind, Read, Write };
use std::net::{ TcpListener};
use std::sync::mpsc;
use std::thread;

fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    listener.set_nonblocking(true)?;
    let mut clients = vec![];

    if let Ok((mut socket, _)) = listener.accept(){
        clients.push(socket.try_clone().expect("Failed to clone clients"));

        let (sender, receiver) = mpsc::channel::<String>();
        let sd = sender.clone();

        thread::spawn(move || loop{
            //创建一个指定大小的信息缓存区
            let mut buffer = vec![0; 1024];
            //socket是指TCPListener的accept获取到的连接的客户端TCP流
            match socket.read_exact(&mut buffer){//读取TCP流中的消息
                Ok(_) =>{//获取成功
                    let message = buffer.into_iter().take_while(|&x| x!=0).collect::<Vec<_>>();//从缓冲区中读取信息
                    let message = String::from_utf8(message).expect("Invalid utf8 message");//将信息转换为utf8格式
                    sd.send(message).expect("Failed to send message to receiver");//将消息发送到消息队列

                    if let Ok(message) = receiver.try_recv(){//从队列获取信息
                        let msg = message.clone();
                        println!("Message [{}]  is received.",msg);
                        //转发给每一个客户端
                        clients = clients.into_iter().filter_map(|mut client|{
                            let mut buffer = message.clone().into_bytes();//将消息放入缓冲区
                            buffer.resize(1024, 0);
                            client.write_all(&buffer).map(|_| client).ok()
                        }).collect::<Vec<_>>();
                        }
                    },
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),//阻塞错误
                Err(_) => {//发生错误
                    //处理错误
                    break;//结束线程
                }
            }
            //线程休眠
            thread::sleep(::std::time::Duration::from_millis(100));
        });
    }

    Ok(())

}