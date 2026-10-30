//! Portable implementations of active assembly helpers in REDALERT/MiscAsm.cpp
//! and TIBERIANDAWN/MiscAsm.cpp. Variant-specific fixed/cardinal arithmetic is
//! kept explicit; the shared algorithms use bounded Rust slices.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MiscAsmVariant { RedAlert, TiberianDawn }

const FACING16: [u8; 32] = [
    3,2,4,255,1,2,0,255, 13,14,12,255,15,14,0,255,
    5,6,4,255,7,6,8,255, 11,10,12,255,9,10,8,255,
];
const FACING8: [u8; 16] = [1,2,1,0,7,6,7,0,3,2,3,4,5,6,5,4];

pub fn mem_copy(source: &[u8], dest: &mut [u8], count: usize) -> bool {
    if count > source.len() || count > dest.len() { return false; }
    dest[..count].copy_from_slice(&source[..count]);
    true
}

/// Dragon Strike metric for packed 16:16 coordinates.
pub fn distance_coord(a: u32, b: u32) -> u16 {
    let dx = (a as u16).wrapping_sub(b as u16) as i16;
    let dy = ((a >> 16) as u16).wrapping_sub((b >> 16) as u16) as i16;
    let ax = if dx > 0 { dx as u16 } else { dx.wrapping_neg() as u16 };
    let ay = if dy > 0 { dy as u16 } else { dy.wrapping_neg() as u16 };
    let (major, minor) = if ax >= ay { (ax, ay) } else { (ay, ax) };
    major.wrapping_add(minor >> 1)
}

fn deltas(x1: i32, y1: i32, x2: i32, y2: i32) -> (u32,u32,bool,bool) {
    let dy=y1.wrapping_sub(y2);
    let dx=x2.wrapping_sub(x1);
    (dx.unsigned_abs(),dy.unsigned_abs(),dy<0,dx<0)
}

/// Lookup-table 16-way facing, returned in 16-unit steps.
pub fn desired_facing16(x1:i32,y1:i32,x2:i32,y2:i32)->i32 {
    let (ax,ay,yn,xn)=deltas(x1,y1,x2,y2);
    let mut i=((yn as u8)<<1)|(xn as u8);
    let (major,minor,y_major)=if ax<ay {(ay,ax,true)}else{(ax,ay,false)};
    i=i.wrapping_shl(1)|(y_major as u8);
    let quarter=major.wrapping_add(1).wrapping_shr(1).wrapping_add(1).wrapping_shr(1);
    i=i.wrapping_shl(1)|(minor<quarter) as u8;
    i=i.wrapping_shl(1)|(major.wrapping_sub(quarter)<minor) as u8;
    (FACING16[i as usize] as i32)<<4
}

/// Lookup-table 8-way facing, North=0 and clockwise.
pub fn desired_facing8(x1:i32,y1:i32,x2:i32,y2:i32)->u8 {
    let (ax,ay,yn,xn)=deltas(x1,y1,x2,y2);
    let mut i=((yn as u8)<<1)|(xn as u8);
    let (major,minor,y_major)=if ax<ay {(ay,ax,true)}else{(ax,ay,false)};
    i=i.wrapping_shl(1)|(y_major as u8);
    i=i.wrapping_shl(1)|(minor<major.wrapping_add(1)>>1) as u8;
    FACING8[i as usize]<<5
}

/// 0..255 angular approximation with the source's divisor overflow reduction.
pub fn desired_facing256(sx:i32,sy:i32,dx:i32,dy:i32)->u8 {
    let mut quadrant=0u32;
    let mut x=dx.wrapping_sub(sx) as u32;
    if (dx as i64)-(sx as i64)<0 { x=0u32.wrapping_sub(x); quadrant=0xc0; }
    let mut y=sy.wrapping_sub(dy) as u32;
    if (sy as i64)-(dy as i64)<0 { quadrant^=0x40; y=0u32.wrapping_sub(y); }
    let mut base=(quadrant&0x40)^0x40;
    if y>=x { std::mem::swap(&mut y,&mut x); base^=0x40; }
    while y&0xffff_ff00==0 && x&0xffff_ff00!=0 { x>>=1; y>>=1; }
    let q=if x==0 {u32::MAX}else{(((y as u64)<<8)/x as u64) as u32};
    let mut angle=q>>3;
    if base!=0 { base=base.wrapping_sub(1); angle=0u32.wrapping_sub(angle); }
    angle.wrapping_add(base).wrapping_add(quadrant) as u8
}

pub fn cardinal_to_fixed(v:MiscAsmVariant,base:u32,cardinal:u32)->u32 {
    match v {
        MiscAsmVariant::RedAlert => if base==0 {u32::MAX}else{cardinal.wrapping_shl(16)/base},
        MiscAsmVariant::TiberianDawn => if base==0 {0xffff}else{cardinal.wrapping_shl(8)/base},
    }
}
pub fn fixed_to_cardinal(v:MiscAsmVariant,base:u32,fixed:u32)->u32 {
    let low=base.wrapping_mul(fixed);
    match v {
        MiscAsmVariant::RedAlert => low.wrapping_add(0x8000)>>16,
        MiscAsmVariant::TiberianDawn => {
            let value=low.wrapping_add(0x80);
            if value&0xff00_0000!=0 {0xffff}else{value>>8}
        }
    }
}

pub fn set_bit(words:&mut [u32],bit:i32,value:bool)->bool {
    if bit<0 {return false}
    let index=bit as usize/32;
    let Some(word)=words.get_mut(index) else{return false};
    let mask=1u32<<(bit as u32&31);
    if value {*word|=mask}else{*word&=!mask}
    true
}
pub fn get_bit(words:&[u32],bit:i32)->Option<bool> {
    if bit<0{return None}
    words.get(bit as usize/32).map(|word|word&(1u32<<(bit as u32&31))!=0)
}
/// Bounded versions of the source's unbounded BSF scans.
pub fn first_true_bit(words:&[u32])->Option<usize> {
    words.iter().enumerate().find_map(|(i,w)|(*w!=0).then(||i*32+w.trailing_zeros() as usize))
}
pub fn first_false_bit(words:&[u32])->Option<usize> {
    words.iter().enumerate().find_map(|(i,w)|(*w!=u32::MAX).then(||i*32+(!*w).trailing_zeros() as usize))
}
pub fn bound(value:i32,min:i32,max:i32)->i32 {
    let (lo,hi)=if min<max{(min,max)}else{(max,min)};
    value.max(lo).min(hi)
}
/// The original VGA CRTC screen-shake implementation was replaced by a no-op.
pub fn shake_screen(_shakes:i32) {}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum FadingTableError { PaletteTooShort,TargetColorOutOfRange }

/// Conquer_Build_Fading_Table: remap ordinary palette entries to nearest
/// reserved shade entry (240..255); index zero is transparent.
pub fn conquer_build_fading_table(palette:&[u8],color:usize,fraction:i32)->Result<[u8;256],FadingTableError>{
    if palette.len()<768{return Err(FadingTableError::PaletteTooShort)}
    if color>=256{return Err(FadingTableError::TargetColorOutOfRange)}
    let p=color*3;
    let target=[palette[p],palette[p+1],palette[p+2]];
    let raw=fraction as u32;
    let frac=(if raw>=256{255u8}else{raw as u8})>>1;
    let mut table=[0u8;256];
    for index in 1..240usize {
        let at=index*3;
        let original=[palette[at],palette[at+1],palette[at+2]];
        let mut ideal=[0u8;3];
        for ch in 0..3 {
            let diff=original[ch].wrapping_sub(target[ch]) as i8 as i32;
            let high=((diff*frac as i32*2)>>8) as u8;
            ideal[ch]=original[ch].wrapping_sub(high);
        }
        let mut best=color as u8;
        let mut best_dist=u32::MAX;
        for candidate in 240..256usize {
            let q=candidate*3;
            let r=palette[q] as i32-ideal[0] as i32;
            let g=palette[q+1] as i32-ideal[1] as i32;
            let b=palette[q+2] as i32-ideal[2] as i32;
            let dist=(r*r+g*g+b*b) as u32;
            if dist<best_dist {best_dist=dist;best=candidate as u8;}
            if dist==0 {break}
        }
        table[index]=best;
    }
    for i in 240..256 {table[i]=i as u8;}
    Ok(table)
}

pub fn reverse_long(value:u32)->u32 {value.swap_bytes()}
pub fn reverse_short(value:u16)->u16 {value.swap_bytes()}
pub fn swap_long(value:u32)->u32 {value.rotate_right(16)}
/// This helper trims only space and TAB (not CR/LF).
pub fn strtrim_ascii_space_tab(text:&[u8])->Vec<u8>{
    let trim=|b:u8|b==b' '||b==b'\t';
    let start=text.iter().position(|b|!trim(*b)).unwrap_or(text.len());
    let end=text.iter().rposition(|b|!trim(*b)).map_or(start,|i|i+1);
    text[start..end].to_vec()
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub struct FatPixelViewport {pub base_offset:usize,pub width:usize,pub height:usize,pub x_add:usize,pub pitch:usize}
/// The legacy square isn't clipped at the viewport's right/bottom edge;
/// backing-slice bounds are enforced here to avoid pointer writes past memory.
pub fn fat_put_pixel(buffer:&mut [u8],view:FatPixelViewport,x:usize,y:usize,color:u8,size:usize)->usize {
    if size==0||x>=view.width||y>=view.height{return 0}
    let Some(stride)=view.width.checked_add(view.x_add).and_then(|n|n.checked_add(view.pitch)) else{return 0};
    let Some(first)=y.checked_mul(stride).and_then(|n|view.base_offset.checked_add(n)).and_then(|n|n.checked_add(x)) else{return 0};
    let mut total=0;
    for row in 0..size {
        let Some(start)=first.checked_add(row.saturating_mul(stride)) else{break};
        let Some(end)=start.checked_add(size) else{break};
        if start>=buffer.len(){break}
        let bounded=end.min(buffer.len());
        buffer[start..bounded].fill(color);
        total+=bounded-start;
        if bounded!=end{break}
    }
    total
}

pub trait PaletteRangeBackend {fn set_direct_draw_palette(&mut self,palette:&[u8;768]);}
pub fn set_palette_range(current:&mut [u8;768],source:Option<&[u8]>,backend:&mut impl PaletteRangeBackend)->bool {
    let Some(src)=source.and_then(|s|s.get(..768)) else{return false};
    let Ok(palette)=<&[u8;768]>::try_from(src) else{return false};
    current.copy_from_slice(palette);
    backend.set_direct_draw_palette(current);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn bit_array_operations_match_x86_word_layout(){
        let mut words=[0u32;2]; assert!(set_bit(&mut words,33,true));
        assert_eq!(words,[0,2]); assert_eq!(get_bit(&words,33),Some(true));
        assert_eq!(get_bit(&words,34),Some(false)); assert_eq!(first_true_bit(&words),Some(33));
        assert_eq!(first_false_bit(&[u32::MAX,0]),Some(32)); assert!(!set_bit(&mut words,-1,true));
    }
    #[test] fn facing_algorithms_match_cardinal_axes(){
        assert_eq!(desired_facing8(0,0,0,-100),0); assert_eq!(desired_facing8(0,0,100,0),64);
        assert_eq!(desired_facing8(0,0,0,100),128); assert_eq!(desired_facing8(0,0,-100,0),192);
        assert_eq!(desired_facing256(0,0,0,-100),0); assert_eq!(desired_facing256(0,0,100,0),64);
        assert_eq!(desired_facing256(0,0,0,100),128); assert_eq!(desired_facing256(0,0,-100,0),192);
    }
    #[test] fn coordinate_distance_uses_max_plus_half_min(){
        assert_eq!(distance_coord((100u32<<16)|100,(100u32<<16)|80),20);
    }
    #[test] fn fixed_formats_are_variant_specific(){
        assert_eq!(cardinal_to_fixed(MiscAsmVariant::RedAlert,0,1),u32::MAX);
        assert_eq!(cardinal_to_fixed(MiscAsmVariant::RedAlert,100,50),0x8000);
        assert_eq!(fixed_to_cardinal(MiscAsmVariant::RedAlert,100,0x8000),50);
        assert_eq!(cardinal_to_fixed(MiscAsmVariant::TiberianDawn,0,1),0xffff);
        assert_eq!(cardinal_to_fixed(MiscAsmVariant::TiberianDawn,100,50),128);
        assert_eq!(fixed_to_cardinal(MiscAsmVariant::TiberianDawn,100,128),50);
    }
    #[test] fn fade_table_preserves_transparent_black_and_reserved_colors(){
        let mut p=vec![0u8;768]; for i in 0..256 {p[i*3]=i as u8;p[i*3+1]=i as u8;p[i*3+2]=i as u8;}
        let table=conquer_build_fading_table(&p,0,128).unwrap(); assert_eq!(table[0],0);
        for i in 240..256 {assert_eq!(table[i],i as u8);}
    }
    #[test] fn endian_and_trim_helpers_match_legacy(){
        assert_eq!(reverse_long(0x12345678),0x78563412); assert_eq!(reverse_short(0x1234),0x3412);
        assert_eq!(swap_long(0x12345678),0x56781234); assert_eq!(strtrim_ascii_space_tab(b" \t hello \t "),b"hello");
    }
    #[test] fn fat_pixel_uses_viewport_pitch(){
        let mut b=vec![0u8;128]; let v=FatPixelViewport{base_offset:4,width:8,height:8,x_add:2,pitch:2};
        assert_eq!(fat_put_pixel(&mut b,v,2,1,9,2),4);
        assert_eq!(&b[18..20],&[9,9]); assert_eq!(&b[30..32],&[9,9]);
        assert_eq!(fat_put_pixel(&mut b,v,8,1,3,2),0);
    }
    #[test] fn bound_and_mem_copy_are_bounded(){
        assert_eq!(bound(20,10,5),10);assert_eq!(bound(3,10,5),5);
        let mut d=[0u8;4];assert!(mem_copy(b"data",&mut d,4));assert_eq!(&d,b"data");
        assert!(!mem_copy(b"longer",&mut d,5));
    }
}
