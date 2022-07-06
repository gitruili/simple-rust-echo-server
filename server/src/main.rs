// 引用类库 net 用来完成TCP监听读取，
use std ::net::{TcpListener,TcpStream};
// 引入 thread 类库用来多线程处理
use std::thread;
//引用类库 io 来读写
use std::io::{self,Read,Write};

// 监听到客户端连接的处理函数。
/**
* @param stream: TcpStream  传入的输入流
*/
fn handle_client(mut stream: TcpStream) -> io::Result<()>{

	println!("DEBUG::client connected");
	// 定义一个存储用的数组，因为需要后续进行填充值所以声明为可变的 `mut`
    let mut buf = [0;512];
	// 建立一个循环，来反复读取客户的输入信息, 这里暂时循环1000次，也可以用loop一直循环
    for _ in 0..1000{
		// 通过read方法
        // let bytes_read = stream.read(&mut buf).expect("read error, stop the process");
		let bytes_read = stream.read(&mut buf).unwrap();
		// 如果输入流的字符长度是0直接退出循环。
        if bytes_read == 0{
            return Ok(());
        }		
		// 打印client端发过来的内容
		println!("Request: {}", String::from_utf8_lossy(&buf[..]));
		// 反馈给client相同的内容
        stream.write(&buf[..bytes_read])?;

    }
    Ok(())
}

// 程序入口函数
fn main() -> io::Result<()>{
	// 创建一个Tcp监听，通过字符串切片将addr 传入
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 调用 incoming() 方法接收客户端的链接信息，如果有新的信息进来就会返回一个Result枚举，OK(T:TcpStream)
    for stream in listener.incoming() {
        // 模式匹配
        match stream {
            // 当Result 枚举类型匹配Ok时
            Ok(stream) => {
                // 如果链接成功，开启一个新的线程，之所以用多线程的原因是TCP客户请求可能有多个。
                thread::spawn(move|| {
                    // 将客户端处理信息解耦到 handle_client 函数中，并移交 stream 变量所有权
                    handle_client(stream);
                });
            }
            // 当Result 枚举匹配错误时
            Err(e) => { 
                // 直接通过panic!宏输出错误信息，并终止程序运行。
                panic!("error {:?}", e) 
            }
        }
    }

    // 关闭Tcp监听链接
    drop(listener);
	// 返回成功结果
    Ok(())


}
