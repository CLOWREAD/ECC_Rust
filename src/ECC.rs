extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;


use num_bigint::ParseBigIntError;
use num_bigint::BigUint;
use num_traits::{Zero, One};
//use num_traits::Pow;
use std::mem::replace;
use std::fmt::{self, Formatter, Display,UpperHex};

use crate::ECC::num_traits::Num;
use crate::ECC::num_traits::Pow;
use crate::ECC::num_traits::FromPrimitive;
use crate::ECC::num_integer::Integer;
use crate::ECC::num_traits::*;
pub struct ECC_POINT
{
  pub  X:BigUint,
  pub  Y:BigUint,
  pub  Inf:bool,

}
impl Clone for ECC_POINT
{
    fn clone(&self) -> ECC_POINT 
    {
        let mut x = &self.X+num_bigint::BigUint::zero();    
        let mut y = &self.Y+num_bigint::BigUint::zero();    
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }

    }
}

impl ECC_POINT{
    pub fn Get_G_Point() ->ECC_POINT
    { 
         
        let mut x = num_bigint::BigUint::from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16).unwrap();       
        let mut y = num_bigint::BigUint::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",16).unwrap();      
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }
        
    }
    pub fn Get_kG_Point() ->ECC_POINT
    { 
        
        let mut x = num_bigint::BigUint::from_str_radix("807BAF868A6AC6CFC192B3491C711EDC35B1E7DD7481410A52840F54C86EFB0A",16).unwrap();       
        let mut y = num_bigint::BigUint::from_str_radix("BED82BC634D0E219DDC1A0511CC56391ECA96869BC9A33231DA88D5172704A7A",16).unwrap();      
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }
        
    }

    pub fn new() ->ECC_POINT
    { 
         
        let mut x = num_bigint::BigUint::zero();       
        let mut y = num_bigint::BigUint::zero();      
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }
        
    }
    pub fn Add(p1:&ECC_POINT,p2:&ECC_POINT) ->ECC_POINT
    {  

        let mut k =num_bigint::BigUint::zero(); 
        let mut P = num_bigint::BigUint::from_str_radix("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();    
        let mut exp2 = num_bigint::BigUint::from_str_radix("2",16).unwrap(); 
        let mut exp3 = num_bigint::BigUint::from_str_radix("3",16).unwrap();    
        
        if p1.Inf && !p2.Inf
        {
        return p2.clone();
        }
        if p2.Inf && !p1.Inf
        {
        return p1.clone();
        }
        let zero_y_test =(&p1.Y+&p2.Y )%&P;
        if zero_y_test== num_bigint::BigUint::zero()
        {
            let mut inf_res=ECC_POINT::new();
            inf_res.Inf=true;
            return inf_res;
        }


        //let mut leftSide =x.modpow(& mut exp2,&mut P);
        if p1.X==p2.X
        {
            k= BigUint::from_u64(3).unwrap();
            k=k* (&p1.X)*(&p1.X);
            k= k* ECC_POINT::GetReciprocalModP(&(exp2*(&p1.Y)));
            k=k%(&P);
        }else
        {
            k= (&p2.Y)+(&P)-(&p1.Y);
            k=k%(&P);
            let mut  k_temp=(&p2.X)+(&P)-(&p1.X);
            k_temp=k_temp%(&P);
            k= k* ECC_POINT::GetReciprocalModP(&k_temp);
            k=k%(&P);
        }
       
        let mut x = (&k)*(&k)+(&P)-(&p1.X)+(&P)-(&p2.X);  
        x=x%(&P);
        let mut y = (&k)*( (&P)+(&p1.X)-(&x) )+ (&P)-(&p1.Y);     
        y=y%(&P);
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }
                
    }
    pub fn Mul( p:&ECC_POINT, x: &BigUint) ->ECC_POINT
    {
        let mut x_avatar=(x)+num_bigint::BigUint::zero();
        let mut cache:Vec<ECC_POINT>=Vec::new();

        let mut res=ECC_POINT::new();
        res.Inf=true;

        cache.push(p.clone());
        for i in (0..256)
        {
           
            if(i!=0)
            {
            let last=cache.get(i-1).unwrap();
            let mut cur= ECC_POINT::Add(&last, &last);            
            cache.push(cur)
            }
            let cur=cache.get(i).unwrap();
            if ! x_avatar.is_even()
            {
                res=ECC_POINT::Add(&res , &cur);              
               
            }
            x_avatar=&x_avatar>>1;
        }
        return res;


    }
    pub fn GetReciprocalModP(x: &BigUint)->BigUint
    {        
        let mut res=num_bigint::BigUint::from_u64(1).unwrap();
        let P = num_bigint::BigUint::from_str_radix("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();  
        let P_avatar = num_bigint::BigUint::from_str_radix("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();  
        let mut P_2=num_bigint::BigUint::from_str_radix("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2D",16).unwrap(); 
        let mut exp2 = num_bigint::BigUint::from_u64(2).unwrap(); 
        
        let mut cache:Vec<BigUint>=Vec::new();
        let x_avatar=(x)+num_bigint::BigUint::from_u64(0).unwrap();
        cache.push(x_avatar);
        for i in (0..256)
        {
            //print!("{0}\n",i);
            if(i!=0)
            {
            let last=cache.get(i-1).unwrap();
            let mut cur=last*last;
            cur=cur% (&P);
            cache.push(cur)
            }
            let cur=cache.get(i).unwrap();
            if ! P_2.is_even()
            {
                res=res*cur;              
                res=res% (&P_avatar);
            }
            P_2=P_2>>1;
        }
        return res;
    }

}

pub struct ECC
{
   pub Point:ECC_POINT,
   pub PointCache:Vec<ECC_POINT>,

}
impl ECC{

    pub fn echo()
    {


    }
    pub fn String2Vec(s:&String)->Vec<u8>
    {
        unsafe {
        let mut s_avatar=s.clone();
        let res=s_avatar.as_mut_vec();

        return res.clone();
        }
    }
    pub fn BigUint2Vec(s:&num_bigint::BigUint)->Vec<u8>
    {
        unsafe {
        
        let res=s.to_bytes_le();
        return res.clone();
        }
    }
    pub fn AttachString2BigUint(s:&String,u:&num_bigint::BigUint)->Vec<u8>
    {
        let mut s_v= ECC::String2Vec(s);
        let mut u_v= ECC::BigUint2Vec(u);
        loop
        {
            if(u_v.len()>=32)
            {
                break;
            }
            u_v.push(0);
        }
        loop
        {
            if(u_v.len()<=32)
            {
                break;
            }
            u_v.pop();
        }

        let mut res:Vec<u8>=Vec::new();
        let mut len=s_v.len();
        //len=len/u_v.len();        
        //len=len+1;
        //len=len*u_v.len();
        let mut temp:u8=0;

        for i in (0..len)
        {
            temp=u_v.get(u_v.len()-1-(i%u_v.len())).unwrap().clone();
            if(i<s_v.len())
            {
                temp=temp^s_v.get(i).unwrap();
            }
            res.push(temp);
            //print!("{0:>0width$X}",temp,width=2);

        }
        return res;

    }
    pub fn DetachString8BigUint(s:&num_bigint::BigUint,u:&num_bigint::BigUint)->String
    {
        let mut s_v= ECC::BigUint2Vec(s);
        let mut u_v= ECC::BigUint2Vec(u);
        loop
        {
            if(u_v.len()>=32)
            {
                break;
            }
            u_v.push(0);
        }
        loop
        {
            if(u_v.len()<=32)
            {
                break;
            }
            u_v.pop();
        }

        let mut res:Vec<u8>=Vec::new();
        let mut len=s_v.len();
        //len=len+u_v.len()-1;
        //len=len/u_v.len();
        //len=len*u_v.len();
        let mut temp:u8=0;

        for i in (0..len)
        {
            temp=u_v.get(32-1-(i%32)).unwrap().clone();
            if(i<s_v.len())
            {
                temp=temp^s_v.get(s_v.len()-1-i).unwrap();
            }
            res.push(temp);
            print!("\n{0:>0width$X}",temp,width=2);

        }
         res.push(0);
        //let mut res:Vec<u8>=Vec::new();
        //let mut res_str=String::from("");//String::from_utf8(res).unwrap();
        let mut res_str=String::from_utf8(res).unwrap();
        return res_str;

    }
    pub fn Byte2String(v:&Vec<u8>)->String{
        let mut res:String=String::new();
        let mut temp:u8=0;
         for i in (0..v.len())
        {
            temp=v.get(i).unwrap().clone();
           
            res+=&(format!("{0:>0width$X}",temp,width=2));

        }
        return res;
    }
    pub fn RandBigUint()->BigUint
    {
        let mut res=BigUint::zero();
        use rand::Rng;
        let one=BigUint::one();
        let mut secret_number = rand::thread_rng().gen_range(1, 1000);
        for i in (0..256)
        {
            secret_number = rand::thread_rng().gen_range(1, 1000);
            if secret_number%2==1
            {
            res+=&one;
            }
            res+=res.clone();
        }
        return res;
    }

    pub fn new (p:ECC_POINT) ->ECC
    {
        
        let mut cache:Vec<ECC_POINT>=Vec::new();
        cache.push(ECC_POINT::Get_G_Point());
        for i in (0..254)
        {

        }

        ECC{
            Point:p,
            PointCache:cache,
        }

    }
    pub  fn IsOnCurve( point :ECC_POINT) ->bool
    {
        let mut P = num_bigint::BigUint::from_str_radix("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();    
       
        let mut exp2 = num_bigint::BigUint::from_str_radix("2",16).unwrap(); 
        let mut exp3 = num_bigint::BigUint::from_str_radix("3",16).unwrap();    
        let mut x =point.X;
        let mut leftSide =x.modpow(& mut exp2,&mut P);
        let mut rightSide =x.modpow(& mut exp3,&mut P)+BigUint::from_u64(7).unwrap();
        rightSide=rightSide.mod_floor(&mut P);
        
        return leftSide.eq(&mut rightSide);
    }
    



}

pub fn Encrypt(encrypteddata:&String,kgx:&String,kgy:&String,randstr:&String) ->String
{
    let mut jsonstr:String=String::new();
    let P = num_bigint::BigUint::from_str_radix("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();  
    let mut G_point=ECC_POINT::Get_G_Point();
    let mut kG_point=ECC_POINT{
        X:num_bigint::BigUint::from_str_radix(kgx,16).unwrap(),
        Y:num_bigint::BigUint::from_str_radix(kgy,16).unwrap(),  
        Inf:false,
    }
    ;
    let ecc=ECC::new(G_point.clone());
    //let mut rand_r=ECC::RandBigUint();
    let mut rand_r=num_bigint::BigUint::from_str_radix(&randstr,10).unwrap();   
    
    rand_r=(rand_r)%(&P);
    let mut rkG=ECC_POINT::Mul(&kG_point, &rand_r);
    let mut rG=ECC_POINT::Mul(&G_point, &rand_r);
    let mut data_str=String::new();
    data_str+=&encrypteddata;
    let sec_str_vec=ECC::AttachString2BigUint(&data_str,&rkG.X);
    let sec_str=ECC::Byte2String(&sec_str_vec);
    jsonstr+="{\"rgx\":\"";
    jsonstr+= &(rG.X.to_str_radix(16));
    jsonstr+= "\",";
    jsonstr+="\"rgy\":\"";
    jsonstr+= &(rG.Y.to_str_radix(16));
    jsonstr+= "\",";
    jsonstr+="\"sec\":\"";
    jsonstr+= &(sec_str);
    jsonstr+= "\"}";
    
    return jsonstr;
   
}
pub fn Decrypt(encrypteddata:&String,rgx:&String,rgy:&String,kstr:&String) ->String
{
    let mut jsonstr:String=String::new();
    let P = num_bigint::BigUint::from_str_radix("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();  
    let mut G_point=ECC_POINT::Get_G_Point();
    let mut kG_point=ECC_POINT::Get_kG_Point();
    let ecc=ECC::new(G_point.clone());
    //let mut rand_r=ECC::RandBigUint();
    let mut rG=ECC_POINT::new();
    rG.X=num_bigint::BigUint::from_str_radix(rgx,16).unwrap(); 
    rG.Y=num_bigint::BigUint::from_str_radix(rgy,16).unwrap(); 
    let mut k=num_bigint::BigUint::from_str_radix(kstr,16).unwrap(); 
    
    let mut krG=ECC_POINT::Mul(&rG, &k);
    let mut data_str=String::new();
    data_str+=&encrypteddata;
    let mut data_num = num_bigint::BigUint::from_str_radix(&data_str,16).unwrap(); 
    print!("\n@!{0}\n",krG.X.to_str_radix(16));
    let sec_str=ECC::DetachString8BigUint(&data_num,&krG.X);
    //let sec_str=ECC::Byte2String(&sec_str_vec);
    jsonstr+="{";
    jsonstr+="\"data\":\"";
    jsonstr+= &(sec_str);
    jsonstr+= "\"}";
    
    return jsonstr;
   
}
pub fn GetkG(kstr:&String) ->String
{
    let mut jsonstr:String=String::new();
    let P = num_bigint::BigUint::from_str_radix("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();  
    let mut G_point=ECC_POINT::Get_G_Point();
    let mut k=num_bigint::BigUint::from_str_radix(kstr,16).unwrap(); 
    let mut kG=ECC_POINT::Mul(&G_point, &k);
    jsonstr+="{\"kgx\":\"";
    jsonstr+= &(kG.X.to_str_radix(16));
    jsonstr+= "\",";
    jsonstr+="\"kgy\":\"";
    jsonstr+= &(kG.Y.to_str_radix(16));
    jsonstr+= "\"}";
    return jsonstr;
}