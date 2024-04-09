pub const ACK: u8 = 0x01;

pub const CPU_PERCENT: u8 = 0x05; // Followed by big-endian u16 which is cpu utilization % * 100 and truncated to an integer
