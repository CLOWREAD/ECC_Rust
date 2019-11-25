

pub mod NetUtils
{
    use std::net::TcpStream;
    use std::io::Write;
    use std::io::Read;
    use std::time::Duration;
    use std::io;
    pub fn WS_Send()
    {
        let mut tcps=TcpStream::connect("www.baidu.com:80").unwrap();
        tcps.set_read_timeout(Some(Duration::new(20, 2000))).unwrap();
        tcps.set_write_timeout(Some(Duration::new(20, 2000))).unwrap();
        let mut buff:[u8;1800]=[0;1800];
        let send_str="GET / HTTP/1.1\r\n\r\n";
        let mut send_buff=send_str.as_bytes();
        tcps.write(&send_buff).unwrap();
        //tcps.write(&buff).unwrap();
        for  i in 1..11
        {
        let size=tcps.read( &mut buff).unwrap();
        if (size==0 )
        {
            break;
        }
        let box_v:Box<[u8]> =Box::new(buff);
        let s=box_v.into_vec();
        let res=String::from_utf8(s).unwrap();
        print!("WS_SEND");
        print!("{0}",res);
        }
    }




}