extern crate native_tls;
extern crate serde_json;
extern crate reqwest;

use std::{env, path::Path, fs, process::Command};
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use std::io::prelude::*;
use serde_json::Value;
use chrono::Local;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        println!("Usage: ssh-alert <username> <ip> <service> <tty> <uname>");
        let path_pam_s = Path::new("/etc/pam.scripts").exists();
        if !path_pam_s{
            let lines = vec!["#!/bin/sh", "if [ ${PAM_TYPE} = \"open_session\" ]; then", "  /fern/ssh-alert/main $PAM_USER $PAM_RHOST $PAM_SERVICE $PAM_TTY `uname -a`", "fi", "exit 0"];
            // Create the directory
            let path = Path::new("/etc/pam.scripts").parent().unwrap().to_path_buf();
            fs::create_dir_all("/etc/pam.scripts").unwrap();
            println!("Creating directory: /etc/pam.scripts");
            // Set the permissions to 755
            Command::new("chmod")
                .arg("755")
                .arg("/etc/pam.scripts")
                .output()
                .expect("Failed to set permissions");
            println!("Setting permissions: /etc/pam.scripts");
            // Create the file
            fs::File::create("/etc/pam.scripts/ssh_alert.sh").unwrap();
            let mut file = fs::OpenOptions::new()
                .write(true)
                .open("/etc/pam.scripts/ssh_alert.sh")
                .unwrap();
            // Write to the file
            for line in lines {
                file.write(line.as_bytes()).unwrap();
                file.write(b"\n").unwrap();
            }
            // Set the permissions to 700
            println!("Setting permissions: /etc/pam.scripts/ssh_alert.sh");
            Command::new("chmod")
                .arg("700")
                .arg("/etc/pam.scripts/ssh_alert.sh")
                .output()
                .expect("Failed to set permissions");
            // Set the owner to root
            println!("Setting owner: /etc/pam.scripts/ssh_alert.sh");
            Command::new("chown")
                .arg("root:root")
                .arg("/etc/pam.scripts/ssh_alert.sh")
                .output()
                .expect("Failed to set owner");
            let mut file_script = fs::OpenOptions::new()
                .append(true)
                .open("/etc/pam.d/sshd")
                .unwrap();
            file_script.write(b"session required pam_exec.so /etc/pam.scripts/ssh_alert.sh").unwrap();
            println!("Writing to /etc/pam.d/sshd");
            println!("Done!");
        }
        return;
    }

    let user = std::env::args().nth(1).unwrap();
    let user_ip = std::env::args().nth(2).unwrap();
    let service = std::env::args().nth(3).unwrap();
    let tty = std::env::args().nth(4).unwrap();
    let server = std::env::args().nth(5).unwrap();

    let current_time = Local::now();

    let mut msg = format!("<p>Connection from <span style='color: #ad2c44;'>{}</span> at <b>{}</b></p><p>Connected at {}</p><p>Service: <b>{}</b></p><p>Tty: <b>{}</b></p><p>Server: <b>{}</b></p>", user_ip, current_time.format("%Y-%m-%d %H:%M:%S"), user, service, tty, server);
    let res = reqwest::blocking::get(format!("http://ip-api.com/json/{}", user_ip).as_str());
    let body = res.unwrap().text().unwrap();
    let json_body: Value = serde_json::from_str(&body).unwrap();

    if json_body["status"] == "success" {
        let location_msg = format!("<p>Country: <b>{}</b></p><p>City: <b>{}</b></p><p>ZIP: <b>{}</b></p><p>ISP: <b>{}</b></p><p>Org: <b>{}</b></p><p>As: <b>{}</b></p>", json_body["country"].as_str().unwrap(), json_body["city"], json_body["zip"], json_body["isp"], json_body["org"], json_body["as"]);
        msg.push_str(&location_msg);
    }


    let email = Message::builder()
        .from("".parse().unwrap()) 
        .to("".parse().unwrap()) 
        .subject(format!("[SSH][{}] New SSH Connection From {}", current_time.format("%H:%M"), user_ip))
        .header(ContentType::TEXT_HTML)
        .body(msg)
        .unwrap(); 
    let creds = Credentials::new("".to_string(), "".to_string());
    let mailer = SmtpTransport::relay("smtp.gmail.com") 
        .unwrap() 
        .credentials(creds) 
        .build();  
    match mailer.send(&email) {
        Ok(_) => println!("Email sent"),
        Err(e) => println!("Could not send email: {}", e),
    }

}