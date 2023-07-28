
/// Gets the color of a point with the specified value, using the default palette
/// 
/// The input is in the range 0-u16::MAX and the output is RGBA
pub fn get_color(value: u16) -> u32 {
    let blocks = MAP_COLORS_LENGTH - 1;
    let block_of_color = value as u32 * blocks as u32;
    let block_idx = (block_of_color >> u16::BITS) as usize;
    let perc_of_color = block_of_color as u16;

    let target = MAP_COLORS[block_idx];
    let next = MAP_COLORS[block_idx + 1];

    let delta_r = red(next) as i32 - red(target) as i32;
    let delta_g = green(next) as i32 - green(target) as i32;
    let delta_b = blue(next) as i32 - blue(target) as i32;

    let r = red(target) + ((delta_r * perc_of_color as i32) >> u16::BITS) as u32;
    let g = green(target) + ((delta_g * perc_of_color as i32) >> u16::BITS) as u32;
    let b = blue(target) + ((delta_b * perc_of_color as i32) >> u16::BITS) as u32;

    return pix(r, g, b);
}


/// Gets the color of a point with the specified value, using the default palette
/// 
/// The input is in the range 0-u16::MAX and the output is RGBA
/// 
/// This is a function that operates on 8 value chunks, and is easily auto vectorized
pub fn get_color_block(values: [u16; 8], buf: &mut [u32; 8]) {

    for (i, value) in values.iter().enumerate() {
        let blocks = MAP_COLORS_LENGTH - 1;
        let block_of_color = *value as u32 * blocks as u32;
        let block_idx = (block_of_color >> u16::BITS) as usize;
        let perc_of_color = block_of_color as u16;

        let target = MAP_COLORS[block_idx];
        let next = MAP_COLORS[block_idx + 1];

        let delta_r = red(next) as i32 - red(target) as i32;
        let delta_g = green(next) as i32 - green(target) as i32;
        let delta_b = blue(next) as i32 - blue(target) as i32;

        let r = red(target) + ((delta_r * perc_of_color as i32) >> u16::BITS) as u32;
        let g = green(target) + ((delta_g * perc_of_color as i32) >> u16::BITS) as u32;
        let b = blue(target) + ((delta_b * perc_of_color as i32) >> u16::BITS) as u32;

        buf[i] = pix(r, g, b);
    }
}

/// Gets the color of a point with the specified value, using a custom specified palette
/// 
/// The input is in the range 0-u16::MAX and the output is RGBA
pub fn get_color_custom<const X: usize>(value: u16, palette: [u32; X]) -> u32 {
    let blocks = X - 1;
    let block_of_color = value as u32 * blocks as u32;
    let block_idx = (block_of_color >> u16::BITS) as usize;
    let perc_of_color = block_of_color as u16;

    let target = palette[block_idx];
    let next = palette[block_idx + 1];

    let delta_r = red(next) as i32 - red(target) as i32;
    let delta_g = green(next) as i32 - green(target) as i32;
    let delta_b = blue(next) as i32 - blue(target) as i32;

    let r = red(target) + ((delta_r * perc_of_color as i32) >> u16::BITS) as u32;
    let g = green(target) + ((delta_g * perc_of_color as i32) >> u16::BITS) as u32;
    let b = blue(target) + ((delta_b * perc_of_color as i32) >> u16::BITS) as u32;

    return pix(r, g, b);
}


/// Gets the color of a point with the specified value, using a custom specified palette
/// 
/// The input is in the range 0-u16::MAX and the output is RGBA
/// 
/// This is a function that operates on 8 value chunks, and is easily auto vectorized
pub fn get_color_block_custom<const X: usize>(values: [u16; 8], buf: &mut [u32; 8], palette: [u32; X]) {

    for (i, value) in values.iter().enumerate() {
        let blocks = X - 1;
        let block_of_color = *value as u32 * blocks as u32;
        let block_idx = (block_of_color >> u16::BITS) as usize;
        let perc_of_color = block_of_color as u16;

        let target = palette[block_idx];
        let next = palette[block_idx + 1];

        let delta_r = red(next) as i32 - red(target) as i32;
        let delta_g = green(next) as i32 - green(target) as i32;
        let delta_b = blue(next) as i32 - blue(target) as i32;

        let r = red(target) + ((delta_r * perc_of_color as i32) >> u16::BITS) as u32;
        let g = green(target) + ((delta_g * perc_of_color as i32) >> u16::BITS) as u32;
        let b = blue(target) + ((delta_b * perc_of_color as i32) >> u16::BITS) as u32;

        buf[i] = pix(r, g, b);
    }
}

#[inline(always)]
fn red(pix: u32) -> u32 {
    (pix & 0xFF000000) >> 24
}

#[inline(always)]
fn green(pix: u32) -> u32 {
    (pix & 0x00FF0000) >> 16
}

#[inline(always)]
fn blue(pix: u32) -> u32 {
    (pix & 0x0000FF00) >> 8
}

#[inline(always)]
fn pix(r: u32, g: u32, b: u32) -> u32 {
    (0xFF << 24) | (b << 16) | (g << 8) | r
}

const MAP_COLORS_LENGTH: usize = 7;

const MAP_COLORS: [u32; MAP_COLORS_LENGTH] = [
    0x000000FF, // Black
    0x0000FFFF, // Blue,
    0x00FFFFFF, // Cyan,
    0x00FF00FF, // Green,
    0xFFFF00FF, // Yellow,
    0xFF0000FF, // Red,
    0xFFFFFFFF, // White
];