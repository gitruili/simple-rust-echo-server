//引用类库 io 来读写
use std::io::{self,prelude::*,BufReader,Write};
// 引用类库 net 用来完成TCP读取，
use std::net::TcpStream;
// 引入 str 库，用来转换输入的 buf 到 str 类型。
use std::str;

// 程序入口函数
fn main() -> io::Result<( )> {
	// 连接到server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
	// 建立循环，可多次输入信息
    for _ in 0..1000 {
		// 创建一个String来存储打的消息
        let mut input = String::new();
		// 读取一行输入的消息
        io::stdin().read_line(&mut input).expect("Failed to read");
		// 发送消息
        stream.write(input.as_bytes()).expect("failed to write");
		// 创建一个BufReader读取stream的数据
        let mut reader =BufReader::new(&stream);
		// 创建一个动态数组来存server返回的消息
        let mut buffer: Vec<u8> = Vec::new();
		// 读取server返回的信息，直到换行
        reader.read_until(b'\n',&mut buffer)?;
		// 打印server返回的信息
        println!("read form server:{}",str::from_utf8(&buffer).unwrap());
		// 打印换行
        println!("");

    }
	// 返回成功结果
    Ok(())
}
