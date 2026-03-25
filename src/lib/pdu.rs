struct pdu {
    command_length: u32,
    command_id: u32,
    command_status: u32,
    sequence_number: u32,
    pdu_data: Vec<u8>,
}

impl pdu {
    fn from_bytes(buffer: &[u8]) -> Option<Self> {
        // the header is mandatory - so at least 16 bytes are needed - each is 4 bytes
        if (buffer.len() < 16) {
            return None;
        }

        let command_length = u32::from_be_bytes(buffer[0..4].try_into().unwrap()) as usize;

        // ensure that the pdu is full
        if (buffer.len() < command_length) {
            return None;
        }

        let command_id = u32::from_be_bytes(buffer[4..8].try_into().unwrap());
        let command_status = u32::from_be_bytes(buffer[8..12].try_into().unwrap());
        let sequence_number = u32::from_be_bytes(buffer[12..16].try_into().unwrap());

        let body = buffer[16..command_length].to_vec();

        Some(SmppPdu {
            command_length: command_length as u32,
            command_id,
            command_status,
            sequence_number,
            body,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.command_length as usize);

        bytes.extend_from_slice(&self.command_length.to_be_bytes());
        bytes.extend_from_slice(&self.command_id.to_be_bytes());
        bytes.extend_from_slice(&self.command_status.to_be_bytes());
        bytes.extend_from_slice(&self.sequence_number.to_be_bytes());
        bytes.extend_from_slice(&self.body);

        bytes
    }
}
