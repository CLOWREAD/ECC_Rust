use std::str;
//use wasm_bindgen::prelude::*;

#[no_mangle]
static mut g_TextBuffer:[u8;4096]=[0;4096];
#[no_mangle]
static mut g_TextParam_0:[u8;1024]=[0;1024];
#[no_mangle]
static mut g_TextParam_1:[u8;1024]=[0;1024];
#[no_mangle]
static mut g_TextParam_2:[u8;1024]=[0;1024];
#[no_mangle]
static mut g_TextParam_3:[u8;1024]=[0;1024];


#[no_mangle]
pub unsafe fn StrTest() 
{
   let  s= String::from("HELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLD");

    for i in 0..12
    {
        let str_bytes=s.as_bytes();
        g_TextBuffer[i]=str_bytes[i];
    }
    
    return ;
} 

#[no_mangle]
pub fn IntTest() -> i32
{   
    return 3306;
} 

mod ECC;
#[no_mangle]
pub  unsafe fn GetkG()
{
 let mut paramstr:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_0[i]==0)
    {
        break;
    }
    paramstr.push(g_TextParam_0[i]);
}
 let res:String =ECC::GetkG(&String::from_utf8(paramstr).unwrap());
 let str_bytes=res.as_bytes();
    for i in 0..str_bytes.len()
    {        
        g_TextBuffer[i]=str_bytes[i];
    }

    g_TextBuffer[str_bytes.len()]=0;
}

#[no_mangle]
pub  unsafe fn Encrypt()
{
let mut paramstr_0:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_0[i]==0)
    {
        break;
    }
    paramstr_0.push(g_TextParam_0[i]);
}
let mut paramstr_1:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_1[i]==0)
    {
        break;
    }
    paramstr_1.push(g_TextParam_1[i]);
}
let mut paramstr_2:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_2[i]==0)
    {
        break;
    }
    paramstr_2.push(g_TextParam_2[i]);
}
let mut paramstr_3:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_3[i]==0)
    {
        break;
    }
    paramstr_3.push(g_TextParam_3[i]);
}
/////////////////////////////////////////
 let res:String =ECC::Encrypt(
     &String::from_utf8(paramstr_0).unwrap(),
     &String::from_utf8(paramstr_1).unwrap(),
     &String::from_utf8(paramstr_2).unwrap(),
     &String::from_utf8(paramstr_3).unwrap()
 );
 let str_bytes=res.as_bytes();
    for i in 0..str_bytes.len()
    {        
        g_TextBuffer[i]=str_bytes[i];
    }

    g_TextBuffer[str_bytes.len()]=0;
}
#[no_mangle]
pub  unsafe fn Decrypt()
{
let mut paramstr_0:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_0[i]==0)
    {
        break;
    }
    paramstr_0.push(g_TextParam_0[i]);
}
let mut paramstr_1:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_1[i]==0)
    {
        break;
    }
    paramstr_1.push(g_TextParam_1[i]);
}
let mut paramstr_2:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_2[i]==0)
    {
        break;
    }
    paramstr_2.push(g_TextParam_2[i]);
}
let mut paramstr_3:Vec<u8>=Vec::new();
for i in 0..1024
{
    if(g_TextParam_3[i]==0)
    {
        break;
    }
    paramstr_3.push(g_TextParam_3[i]);
}
/////////////////////////////////////////
 let res:String =ECC::Decrypt(
     &String::from_utf8(paramstr_0).unwrap(),
     &String::from_utf8(paramstr_1).unwrap(),
     &String::from_utf8(paramstr_2).unwrap(),
     &String::from_utf8(paramstr_3).unwrap()
 );
 let res_trim=res.replace("\0", "");
 let str_bytes=res_trim.as_bytes();
    for i in 0..str_bytes.len()
    {        
        g_TextBuffer[i]=str_bytes[i];
    }

    g_TextBuffer[str_bytes.len()]=0;
}
////////////////////////////////////////////
#[no_mangle]
pub fn WS_Send() -> String
{
    WS_Send_Impl();
    print!("HELLO WASM"); 
    return String::from("NULL");
} 
    use std::net::TcpStream;
    use std::io::Write;
    use std::io::Read;
    use std::time::Duration;
    use std::io;
    pub fn WS_Send_Impl()
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
