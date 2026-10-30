//! PCX writer from REDALERT/WIN32LIB/WRITEPCX.CPP.
//! Produces the original 8-bit, single-plane, 256-colour PCX layout.

use std::fmt;

pub const PCX_HEADER_LEN: usize = 128;
pub const PCX_PALETTE_LEN: usize = 768;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PcxError {
    InvalidDimensions,
    InvalidStride,
    PixelDataTooShort,
    PaletteTooShort,
}
impl fmt::Display for PcxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for PcxError {}

fn put_u16(header: &mut [u8; PCX_HEADER_LEN], offset: usize, value: u16) {
    header[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

/// Encode one PCX scanline using the source's run-length rules.
pub fn encode_scanline(line: &[u8], output: &mut Vec<u8>) {
    if line.is_empty() {
        return;
    }
    let mut last = line[0];
    let mut run = 1usize;
    for &color in &line[1..] {
        if color == last {
            run += 1;
            if run == 63 {
                output.push(0xff);
                output.push(last);
                run = 0;
            }
        } else {
            if run != 0 {
                if run == 1 && last & 0xc0 != 0xc0 {
                    output.push(last);
                } else {
                    output.push((run as u8) | 0xc0);
                    output.push(last);
                }
            }
            last = color;
            run = 1;
        }
    }
    if run != 0 {
        if run == 1 && last & 0xc0 != 0xc0 {
            output.push(last);
        } else {
            output.push((run as u8) | 0xc0);
            output.push(last);
        }
    }
}

/// Encode an indexed viewport and 6-bit DAC palette as a complete PCX file.
/// pixels is the full backing buffer. stride, x, and y reproduce the viewport
/// offset calculation. The palette is copied and each channel is shifted left
/// by two, leaving the caller's bytes unchanged.
pub fn write_pcx(
    pixels: &[u8],
    stride: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    palette: &[u8],
) -> Result<Vec<u8>, PcxError> {
    if width == 0 || height == 0 || width > u16::MAX as usize
        || height > u16::MAX as usize {
        return Err(PcxError::InvalidDimensions);
    }
    if stride < x.checked_add(width).ok_or(PcxError::InvalidStride)? {
        return Err(PcxError::InvalidStride);
    }
    if palette.len() < PCX_PALETTE_LEN {
        return Err(PcxError::PaletteTooShort);
    }
    let start = y.checked_mul(stride).and_then(|v| v.checked_add(x))
        .ok_or(PcxError::PixelDataTooShort)?;
    let last_end = start
        .checked_add((height - 1).checked_mul(stride).ok_or(PcxError::PixelDataTooShort)?)
        .and_then(|v| v.checked_add(width))
        .ok_or(PcxError::PixelDataTooShort)?;
    if last_end > pixels.len() {
        return Err(PcxError::PixelDataTooShort);
    }

    let mut header = [0u8; PCX_HEADER_LEN];
    header[0] = 10; // manufacturer
    header[1] = 5;  // PCX version 3.0
    header[2] = 1;  // RLE encoding
    header[3] = 8;  // bits per pixel
    put_u16(&mut header, 4, 0);
    put_u16(&mut header, 6, 0);
    put_u16(&mut header, 8, (width - 1) as u16);
    put_u16(&mut header, 10, (height - 1) as u16);
    put_u16(&mut header, 12, 320);
    put_u16(&mut header, 14, 200);
    header[65] = 1; // one colour plane
    put_u16(&mut header, 66, width as u16);
    put_u16(&mut header, 68, 1); // colour palette

    let mut output = Vec::with_capacity(PCX_HEADER_LEN + width * height + 769);
    output.extend_from_slice(&header);
    for row in 0..height {
        let offset = start + row * stride;
        encode_scanline(&pixels[offset..offset + width], &mut output);
    }
    output.push(0x0c);
    for channel in &palette[..PCX_PALETTE_LEN] {
        output.push(channel.wrapping_shl(2));
    }
    Ok(output)
}
