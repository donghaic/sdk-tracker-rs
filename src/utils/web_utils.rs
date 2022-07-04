use std::collections::HashMap;

use actix_web::HttpRequest;
use actix_web::web::Query;

pub fn get_string_or(query: &Query<HashMap<String, String>>, name: &str, default: &str) -> String {
    match query.get(name) {
        Some(s) => s.clone(),
        _ => default.to_string(),
    }
}

pub fn get_u32_or(query: &Query<HashMap<String, String>>, name: &str, default: u32) -> u32 {
    match query.get(name) {
        Some(v) => v.parse().unwrap_or(0),
        _ => default,
    }
}


pub fn get_client_ip(req: &HttpRequest) -> String {
    // let headers = req.headers().to_owned();

    // "X-Forwarded-For", "X-Real-Ip"
    // let ip = match headers.get("X-Forwarded-For") {
    //     Some(x) => x.to_str().unwrap().to_owned(),
    //     None => String::from(""),
    // };


    let connection_info = req.connection_info().clone();

    let ip_port = match connection_info.realip_remote_addr() {
        Some(ip) => ip.to_owned(),
        None => String::from(""),
    };

    let ip = ip_port
        .split(":")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .to_owned();

    ip
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use actix_web::test::TestRequest;

    #[test]
    fn test_client_ip() {
        let req = TestRequest::with_header("x-forwarded-for", "1.2.3.4, 9.10.11.12999, 5.6.7.8, 9.10.11.12,")
            .to_http_request();
        let headers = req.headers().to_owned();

        // let mut ip: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
        let mut trace: Vec<IpAddr> = Vec::new();
        if let Some(x_forwarded_for) = headers.get("X-Forwarded-For") {
            if let Ok(header) = x_forwarded_for.to_str() {
                let mut header_ips: Vec<IpAddr> =
                    header.split(',').flat_map(|ip| ip.trim().parse()).collect();
                header_ips.reverse();
                // ip = header_ips.get(0).unwrap().to_owned()
                trace.append(&mut header_ips);
            }
        }

        let x = trace.iter().next().take().unwrap();
        let result = trace.iter().count();
        println!("ip count => {}", result);
        println!("ip  => {:?}", x);
    }
}