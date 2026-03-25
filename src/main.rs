use std::env;

mod pdu;
mod config;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let args: Vec<String>  = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <phone_number> <message>", args[0]);
        std::process::exit(1);
    }

    let config = config::load_config("config.toml")?;

    println!("Config: {:?}", config);

    let destination_number = &args[1];
    let message = &args[2..].join(" ");
    // socket
    let address = &config.smpp_host;

    // stream
    let mut stream = TcpStream::connect(address).unwrap();
    println!("Connected to SMPP server at {}", address);
    stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();

    let pdu = build_bind_trx(
        1,
        &config.system_id,
        &config.password,
        &config.system_type,
        0x34,
        0x00,
        0x00,
        "",
    );

    stream.write_all(&pdu)?;
    println!("bind sent");

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer)?;

    if let Some(pdu) = pdu::pdu::from_bytes(&buffer[..n]) {
        println!("Parsed PDU: {:?}", pdu);

        match pdu.command_id {
            0x80000009 => {
                println!("Bind response received");
                println!("Status: {}", pdu.command_status);
            }
            _ => {
                println!("Other PDU: 0x{:X}", pdu.command_id);
            }
        }
    }

    let pdu = build_submit_sm(
        2,
        &config.sender_id,          // source address
        destination_number,   // destination number (Kenya format)
        message.as_bytes(),
    );

    stream.write_all(&pdu)?;
    println!("submit_sm sent");

    let mut buffer = [0u8; 1024];

    let n = stream.read(&mut buffer)?;

    if let Some(resp) = pdu::SubmitSmResp::from_bytes(&buffer[..n]) {
        println!("submit_sm_resp: {:?}", resp);

        if resp.command_status == 0 {
            println!("Message accepted, ID: {}", resp.message_id);
        } else {
            println!("Error, status: {}", resp.command_status);
        }
    }

    Ok(())
    
}

fn write_c_string(buf: &mut Vec<u8>, value: &str) {
    buf.extend_from_slice(value.as_bytes());
    buf.push(0); // null terminator
}

fn build_bind_trx(
    sequence_number: u32,
    system_id: &str,
    password: &str,
    system_type: &str,
    interface_version: u8,
    addr_ton: u8,
    addr_npi: u8,
    address_range: &str,
) -> Vec<u8> {
    let command_id: u32 = 0x00000009; // bind_transceiver
    let command_status: u32 = 0;

    let mut body: Vec<u8> = Vec::new();

    write_c_string(&mut body, system_id);
    write_c_string(&mut body, password);
    write_c_string(&mut body, system_type);

    body.push(interface_version);
    body.push(addr_ton);
    body.push(addr_npi);

    write_c_string(&mut body, address_range);

    let command_length = (16 + body.len()) as u32;

    let mut pdu: Vec<u8> = Vec::with_capacity(command_length as usize);

    // Header
    pdu.extend_from_slice(&command_length.to_be_bytes());
    pdu.extend_from_slice(&command_id.to_be_bytes());
    pdu.extend_from_slice(&command_status.to_be_bytes());
    pdu.extend_from_slice(&sequence_number.to_be_bytes());

    // Body
    pdu.extend_from_slice(&body);

    pdu
}

fn build_submit_sm(
    sequence_number: u32,
    source_addr: &str,
    destination_addr: &str,
    message: &[u8],
) -> Vec<u8> {
    let command_id: u32 = 0x00000004;
    let command_status: u32 = 0;

    let mut body: Vec<u8> = Vec::new();

    // Required fields (minimal example)
    write_c_string(&mut body, ""); // service_type

    body.push(0); // source_addr_ton
    body.push(0); // source_addr_npi
    write_c_string(&mut body, source_addr);

    body.push(0); // dest_addr_ton
    body.push(0); // dest_addr_npi
    write_c_string(&mut body, destination_addr);

    body.push(0); // esm_class
    body.push(0); // protocol_id
    body.push(0); // priority_flag

    write_c_string(&mut body, ""); // schedule_delivery_time
    write_c_string(&mut body, ""); // validity_period

    body.push(1); // registered_delivery (request delivery receipt)
    body.push(0); // replace_if_present_flag
    body.push(0); // data_coding
    body.push(0); // sm_default_msg_id

    body.push(message.len() as u8); // sm_length
    body.extend_from_slice(message); // short_message

    // Header length = 16 + body
    let command_length = (16 + body.len()) as u32;

    let mut pdu = Vec::with_capacity(command_length as usize);

    pdu.extend_from_slice(&command_length.to_be_bytes());
    pdu.extend_from_slice(&command_id.to_be_bytes());
    pdu.extend_from_slice(&command_status.to_be_bytes());
    pdu.extend_from_slice(&sequence_number.to_be_bytes());

    pdu.extend_from_slice(&body);

    pdu
}
