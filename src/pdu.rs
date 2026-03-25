#[derive(Debug)]
pub struct pdu {
    pub command_length: u32,
    pub command_id: u32,
    pub command_status: u32,
    pub sequence_number: u32,
    pub pdu_data: Vec<u8>,
}

impl pdu {
    pub fn from_bytes(buffer: &[u8]) -> Option<Self> {
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

        let pdu_data = buffer[16..command_length].to_vec();

        Some(pdu {
            command_length: command_length as u32,
            command_id,
            command_status,
            sequence_number,
            pdu_data,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.command_length as usize);

        bytes.extend_from_slice(&self.command_length.to_be_bytes());
        bytes.extend_from_slice(&self.command_id.to_be_bytes());
        bytes.extend_from_slice(&self.command_status.to_be_bytes());
        bytes.extend_from_slice(&self.sequence_number.to_be_bytes());
        bytes.extend_from_slice(&self.pdu_data);

        bytes
    }
}

#[derive(Debug)]
pub struct SubmitSmResp {
    pub command_length: u32,
    pub command_id: u32,
    pub command_status: u32,
    pub sequence_number: u32,
    pub message_id: String,
}

impl SubmitSmResp {
    pub fn from_bytes(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < 16 {
            return None;
        }

        let command_length = u32::from_be_bytes(buffer[0..4].try_into().ok()?);
        let command_id = u32::from_be_bytes(buffer[4..8].try_into().ok()?);
        let command_status = u32::from_be_bytes(buffer[8..12].try_into().ok()?);
        let sequence_number = u32::from_be_bytes(buffer[12..16].try_into().ok()?);

        if buffer.len() < command_length as usize {
            return None;
        }

        let body = &buffer[16..command_length as usize];

        // message_id is a null-terminated string
        let message_id = body
            .split(|&b| b == 0)
            .next()
            .unwrap_or(&[])
            .to_vec();

        let message_id = String::from_utf8_lossy(&message_id).to_string();

        Some(Self {
            command_length,
            command_id,
            command_status,
            sequence_number,
            message_id,
        })
    }
}