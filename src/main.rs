mod ModA{
    pub struct K {
    pub m_I: i64,
    }
    impl K {
        pub fn New()-> K
        {
            let mut k =K{
                m_I:0
                };
           return k;
        }
        pub fn clone(&self)->K
        {
            print!("CLONE!");
            let mut k =K{
                m_I:self.m_I
                };
           return k;
        }
        // add code here
    }
}

use std::string;
use std::str;
mod net_util;
use net_util::NetUtils;
mod ECC;

fn main() {
    
    //let s=signatory_secp256k1::EcdsaSigner{};
   


}
