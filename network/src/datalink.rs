pub fn transmit_crc(data: &u32) -> u32 {
    let add_data = data % 0x0904u32; 
    (data << 8) | add_data
}

pub fn recive_crc(data: &u32) -> bool {
    let check_data = (data >> 8) % 0x0904u32;
    
    check_data == (data & 0x00ff) 
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn is_crc() {
        let test_data1 = 0x1234;
        let test_send_data1 = transmit_crc(&test_data1);
        assert_eq!(true, recive_crc(&test_send_data1));
        let test_data2 = 0x12341234;
        assert_eq!(false, recive_crc(&test_data2));
    }
}
