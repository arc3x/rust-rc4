#[cfg(test)]
pub mod tests {
    use super::keystream;
    
    #[test]
    fn same_input_same_output() {
        let ks1:Vec<u8> =  keystream(100, 10000, "fish");
        let ks2:Vec<u8> =  keystream(100, 10000, "fish");
        assert_eq!(ks1, ks2);
    }
    
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn diff_input_diff_output() {
        let ks1:Vec<u8> =  keystream(100, 10000, "fish");
        let ks2:Vec<u8> =  keystream(100, 10000, "fishs");
        assert_eq!(ks1, ks2);
    }
}


/// Produces a RC4 Keystream
/// # Arguments
/// * 'n' - length of keystream
/// * 'r' - rounds of key scheduling
/// * 'k' - key
///
/// # Returns
/// * 'keystream' - a vector of u8 bytes
pub fn keystream(n:i32, r:i32, k:&str) -> Vec<u8> {    
    let mut keystream:Vec<u8> = Vec::new();
    //put key in u8 vec    
    let mut key:Vec<u8> = Vec::new();
    for c in k.chars() {
        assert_eq!(c.len_utf8(), 1);        
        key.push(c as u8)
    }
        
    //init scheduling array 
    let mut sch:[u8; 256] = [0; 256];
    for i in 0..256 {
        sch[i]=i as u8;
    }
    
    //do key scheduling
    let mut j:u8 = 0;
    for _ in 0..r {
        for i in 0..256 {
            //j = (j + sch[i] + key[i%key.len()]) % 255;
            j = j.wrapping_add(j).wrapping_add(sch[i]).wrapping_add(key[i%key.len()]);
            //swap
            let t:u8 = sch[i];
            sch[i]=sch[j as usize];
            sch[j as usize]=t;
        }
    }   
    
    //produce the keystream
    j=0;
    let mut ip:u8 = 0;
    for i in 0..n {
        ip = ip.wrapping_add(i as u8+1);
        j = j.wrapping_add(j).wrapping_add(sch[ip as usize]);
        //swap
        let t:u8 = sch[i as usize];
        sch[i as usize]=sch[j as usize];
        sch[j as usize]=t;
        //write keystream
        keystream.push(sch[(sch[ip as usize].wrapping_add(sch[j as usize])) as usize]);
    }  

    //return
    keystream
}