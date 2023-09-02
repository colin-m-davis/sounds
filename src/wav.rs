#[repr(C)]
struct WAVHeader {
    riff: [u8; 4],
    chunk_size: u32,
    wave: [u8; 4],
    fmt: [u8; 4],        
    sub_chunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    data: [u8; 4],
    sub_chunk2_size: u32,
}

impl WAVHeader {
    fn new() -> Self {
        Self {
            riff: [b'R', b'I', b'F', b'F'],
            chunk_size: 0,
            wave: [b'W', b'A', b'V', b'E'],
            fmt: [b'f', b'm', b't', b' '],
            sub_chunk1_size: 16,
            audio_format: 1,
            num_channels: 1,
            sample_rate: 0,
            byte_rate: 0,
            block_align: 0,
            bits_per_sample: 0,
            data: [b'd', b'a', b't', b'a'],
            sub_chunk2_size: 0,
        }
    }
}
