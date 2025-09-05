use std::collections::HashMap;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct ScanResult {
    ip: IpAddr,
    port: u16,
    is_open: bool,
    response_time: Option<Duration>,
    service: Option<String>,
}

struct NetworkScanner {
    timeout: Duration,
    thread_count: usize,
    common_ports: Vec<u16>,
    port_services: HashMap<u16, String>,
}

impl NetworkScanner {
    fn new() -> Self {
        let mut port_services = HashMap::new();
        
        // Common port mappings
        port_services.insert(21, "FTP".to_string());
        port_services.insert(22, "SSH".to_string());
        port_services.insert(23, "Telnet".to_string());
        port_services.insert(25, "SMTP".to_string());
        port_services.insert(53, "DNS".to_string());
        port_services.insert(80, "HTTP".to_string());
        port_services.insert(110, "POP3".to_string());
        port_services.insert(143, "IMAP".to_string());
        port_services.insert(443, "HTTPS".to_string());
        port_services.insert(993, "IMAPS".to_string());
        port_services.insert(995, "POP3S".to_string());
        port_services.insert(3389, "RDP".to_string());
        port_services.insert(5432, "PostgreSQL".to_string());
        port_services.insert(3306, "MySQL".to_string());
        port_services.insert(1433, "MSSQL".to_string());
        port_services.insert(6379, "Redis".to_string());
        port_services.insert(27017, "MongoDB".to_string());

        NetworkScanner {
            timeout: Duration::from_millis(1000),
            thread_count: 50,
            common_ports: vec![21, 22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, 5432, 3306, 1433, 6379, 27017],
            port_services,
        }
    }

    fn scan_port(&self, ip: IpAddr, port: u16) -> ScanResult {
        let start_time = Instant::now();
        let socket_addr = SocketAddr::new(ip, port);
        
        match TcpStream::connect_timeout(&socket_addr, self.timeout) {
            Ok(_) => {
                let response_time = start_time.elapsed();
                let service = self.port_services.get(&port).cloned();
                
                ScanResult {
                    ip,
                    port,
                    is_open: true,
                    response_time: Some(response_time),
                    service,
                }
            }
            Err(_) => ScanResult {
                ip,
                port,
                is_open: false,
                response_time: None,
                service: None,
            },
        }
    }

    fn scan_host(&self, ip: IpAddr, ports: &[u16]) -> Vec<ScanResult> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        
        let chunk_size = (ports.len() + self.thread_count - 1) / self.thread_count;
        
        for chunk in ports.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let results_clone = Arc::clone(&results);
            let timeout = self.timeout;
            let port_services = self.port_services.clone();
            
            let handle = thread::spawn(move || {
                for &port in &chunk {
                    let start_time = Instant::now();
                    let socket_addr = SocketAddr::new(ip, port);
                    
                    let scan_result = match TcpStream::connect_timeout(&socket_addr, timeout) {
                        Ok(_) => {
                            let response_time = start_time.elapsed();
                            let service = port_services.get(&port).cloned();
                            
                            ScanResult {
                                ip,
                                port,
                                is_open: true,
                                response_time: Some(response_time),
                                service,
                            }
                        }
                        Err(_) => ScanResult {
                            ip,
                            port,
                            is_open: false,
                            response_time: None,
                            service: None,
                        },
                    };
                    
                    if scan_result.is_open {
                        results_clone.lock().unwrap().push(scan_result);
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        let mut final_results = results.lock().unwrap().clone();
        final_results.sort_by_key(|r| r.port);
        final_results
    }

    fn scan_network_range(&self, base_ip: &str, start: u8, end: u8) -> HashMap<IpAddr, Vec<ScanResult>> {
        let mut network_results = HashMap::new();
        
        for i in start..=end {
            let ip_str = format!("{}.{}", base_ip, i);
            if let Ok(ip) = IpAddr::from_str(&ip_str) {
                println!("Scanning {}...", ip);
                let results = self.scan_host(ip, &self.common_ports);
                
                if !results.is_empty() {
                    network_results.insert(ip, results);
                    println!("  ✓ {} open ports found", network_results[&ip].len());
                } else {
                    println!("  ✗ No open ports found");
                }
            }
        }
        
        network_results
    }

    fn display_results(&self, results: &[ScanResult]) {
        if results.is_empty() {
            println!("No open ports found.");
            return;
        }

        println!("\n📊 Scan Results:");
        println!("Port    Service          Status    Response Time");
        println!("------------------------------------------------");
        
        for result in results {
            let service = result.service.as_deref().unwrap_or("Unknown");
            let response_time = result.response_time
                .map(|t| format!("{:.2}ms", t.as_secs_f64() * 1000.0))
                .unwrap_or_else(|| "N/A".to_string());
            
            println!("{:<8} {:<15} {:<9} {}", 
                     result.port, 
                     service, 
                     if result.is_open { "OPEN" } else { "CLOSED" },
                     response_time);
        }
    }

    fn display_network_results(&self, network_results: &HashMap<IpAddr, Vec<ScanResult>>) {
        if network_results.is_empty() {
            println!("No hosts with open ports found.");
            return;
        }

        println!("\n🌐 Network Scan Summary:");
        println!("========================");
        
        for (ip, results) in network_results {
            println!("\n🖥️  Host: {}", ip);
            self.display_results(results);
        }

        // Summary statistics
        let total_hosts = network_results.len();
        let total_open_ports: usize = network_results.values().map(|v| v.len()).sum();
        
        println!("\n📈 Summary:");
        println!("  Hosts with open ports: {}", total_hosts);
        println!("  Total open ports found: {}", total_open_ports);
    }
}

fn parse_ip_input(input: &str) -> Result<IpAddr, String> {
    IpAddr::from_str(input.trim())
        .map_err(|_| "Invalid IP address format".to_string())
}

fn parse_port_range(input: &str) -> Result<Vec<u16>, String> {
    if input.contains('-') {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid port range format. Use: start-end".to_string());
        }
        
        let start: u16 = parts[0].trim().parse()
            .map_err(|_| "Invalid start port")?;
        let end: u16 = parts[1].trim().parse()
            .map_err(|_| "Invalid end port")?;
        
        if start > end {
            return Err("Start port must be less than or equal to end port".to_string());
        }
        
        Ok((start..=end).collect())
    } else {
        // Single port
        let port: u16 = input.trim().parse()
            .map_err(|_| "Invalid port number")?;
        Ok(vec![port])
    }
}

fn main() {
    println!("🔍 Network Port Scanner");
    println!("======================");
    println!("⚠️  Warning: Only scan networks you own or have permission to scan!");
    println!("Unauthorized network scanning may be illegal in your jurisdiction.\n");

    let scanner = NetworkScanner::new();

    loop {
        println!("Options:");
        println!("1. Scan single host");
        println!("2. Scan network range");
        println!("3. Quick scan (common ports)");
        println!("4. Custom port range scan");
        println!("5. Show scanner settings");
        println!("6. Exit");
        print!("Choose an option (1-6): ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => {
                print!("Enter IP address to scan: ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut ip_input = String::new();
                io::stdin().read_line(&mut ip_input).unwrap();
                
                match parse_ip_input(&ip_input) {
                    Ok(ip) => {
                        print!("Enter port range (e.g., '80' or '1-1000'): ");
                        io::Write::flush(&mut io::stdout()).unwrap();
                        let mut port_input = String::new();
                        io::stdin().read_line(&mut port_input).unwrap();
                        
                        match parse_port_range(&port_input) {
                            Ok(ports) => {
                                println!("Scanning {} ports on {}...", ports.len(), ip);
                                let start_time = Instant::now();
                                let results = scanner.scan_host(ip, &ports);
                                let scan_duration = start_time.elapsed();
                                
                                scanner.display_results(&results);
                                println!("\nScan completed in {:.2} seconds", scan_duration.as_secs_f64());
                            }
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                print!("Enter network base (e.g., '192.168.1'): ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut base_input = String::new();
                io::stdin().read_line(&mut base_input).unwrap();
                let base = base_input.trim();

                print!("Enter start host number (1-254): ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut start_input = String::new();
                io::stdin().read_line(&mut start_input).unwrap();
                
                print!("Enter end host number (1-254): ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut end_input = String::new();
                io::stdin().read_line(&mut end_input).unwrap();

                if let (Ok(start), Ok(end)) = (start_input.trim().parse::<u8>(), end_input.trim().parse::<u8>()) {
                    if start <= end && start >= 1 && end <= 254 {
                        println!("Scanning network range {}.{}-{}...", base, start, end);
                        let start_time = Instant::now();
                        let results = scanner.scan_network_range(base, start, end);
                        let scan_duration = start_time.elapsed();
                        
                        scanner.display_network_results(&results);
                        println!("\nNetwork scan completed in {:.2} seconds", scan_duration.as_secs_f64());
                    } else {
                        println!("Invalid range! Start and end must be between 1-254, and start <= end.");
                    }
                } else {
                    println!("Invalid host numbers!");
                }
            }
            "3" => {
                print!("Enter IP address for quick scan: ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut ip_input = String::new();
                io::stdin().read_line(&mut ip_input).unwrap();
                
                match parse_ip_input(&ip_input) {
                    Ok(ip) => {
                        println!("Quick scanning {} common ports on {}...", scanner.common_ports.len(), ip);
                        let start_time = Instant::now();
                        let results = scanner.scan_host(ip, &scanner.common_ports);
                        let scan_duration = start_time.elapsed();
                        
                        scanner.display_results(&results);
                        println!("\nQuick scan completed in {:.2} seconds", scan_duration.as_secs_f64());
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => {
                print!("Enter IP address: ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut ip_input = String::new();
                io::stdin().read_line(&mut ip_input).unwrap();
                
                print!("Enter custom port range (e.g., '8000-8100'): ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut port_input = String::new();
                io::stdin().read_line(&mut port_input).unwrap();
                
                match (parse_ip_input(&ip_input), parse_port_range(&port_input)) {
                    (Ok(ip), Ok(ports)) => {
                        println!("Scanning {} custom ports on {}...", ports.len(), ip);
                        let start_time = Instant::now();
                        let results = scanner.scan_host(ip, &ports);
                        let scan_duration = start_time.elapsed();
                        
                        scanner.display_results(&results);
                        println!("\nCustom scan completed in {:.2} seconds", scan_duration.as_secs_f64());
                    }
                    (Err(e), _) | (_, Err(e)) => println!("Error: {}", e),
                }
            }
            "5" => {
                println!("\n⚙️  Scanner Settings:");
                println!("  Timeout: {}ms", scanner.timeout.as_millis());
                println!("  Thread count: {}", scanner.thread_count);
                println!("  Common ports: {} ports", scanner.common_ports.len());
                println!("  Known services: {} mappings", scanner.port_services.len());
                
                println!("\n📋 Common ports scanned:");
                for (i, &port) in scanner.common_ports.iter().enumerate() {
                    let service = scanner.port_services.get(&port).unwrap_or(&"Unknown".to_string());
                    print!("{}:{} ", port, service);
                    if (i + 1) % 4 == 0 {
                        println!();
                    }
                }
                println!();
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice! Please enter 1-6.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ip() {
        assert!(parse_ip_input("127.0.0.1").is_ok());
        assert!(parse_ip_input("192.168.1.1").is_ok());
        assert!(parse_ip_input("invalid").is_err());
    }

    #[test]
    fn test_parse_port_range() {
        assert_eq!(parse_port_range("80").unwrap(), vec![80]);
        assert_eq!(parse_port_range("80-82").unwrap(), vec![80, 81, 82]);
        assert!(parse_port_range("invalid").is_err());
        assert!(parse_port_range("100-50").is_err()); // Invalid range
    }

    #[test]
    fn test_scanner_creation() {
        let scanner = NetworkScanner::new();
        assert!(!scanner.common_ports.is_empty());
        assert!(!scanner.port_services.is_empty());
        assert!(scanner.timeout.as_millis() > 0);
    }
}