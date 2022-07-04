#[cfg(test)]
mod tests {
    use ipdb_rs as ipdb;
    use std::time::Instant;

    #[test]
    fn test_client_ip() {
        let ipadd = ipdb::find("113.67.126.164", "CN");
        println!("ip addr {:?}", ipadd);
        let now = Instant::now();
        let mut sum: usize = 0;
        for i in 0..1000000 {
            if let Ok(v) = ipdb::find("113.67.126.164", "CN") {
                sum += v.len();
            }
        }
        println!("cost {} ms, total {}", now.elapsed().as_millis(), sum / 3);
    }
}