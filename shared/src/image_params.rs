/// The underlying internal representation of the image.
#[derive(PartialEq, Eq)]
pub enum ImageFormat {
    /// Representation not known at compile time.
    Unknown,
    /// RGBA channels, 32 bit floating point integer.
    Rgba32f,
    /// RGBA channels, 16 bit floating point integer.
    Rgba16f,
    /// RGBA channels, 16 bit floating point integer.
    R32f,
    /// RGBA channels, 8 bit floating point integer.
    Rgba8,
    /// RGBA channels, 8 bit signed normalized integer.
    Rgba8Snorm,
    /// Red+Green channels, 32 bit floating point integer.
    Rg32f,
    /// Red+Green channels, 16 bit floating point integer.
    Rg16f,
    /// 32 bit unsigned integer containing two 11 bit floating point integers
    /// for the Red and Green channels, and a 10 bit floating point integer for
    /// the Blue channel.
    R11fG11fB10f,
    /// Red channel, 16 bit floating point.
    R16f,
    /// RGBA channel, 16 bit floating point.
    Rgba16,
    /// 32 bit unsigned integer containing three 10 bit unsigned normalized
    /// integers for the Red, Green, and Blue channels; with a 2 unsigned
    /// normalized integer for the Alpha channel.
    Rgb10A2,
    /// Red+Green channels, 16 bit floating point integer.
    Rg16,
    /// Red+Green channels, 8 bit floating point integer.
    Rg8,
    /// Red+Green channels, 16 bit floating point integer.
    R16,
    /// Red channel, 8 bit floating point integer.
    R8,
    /// RGBA channels, 16 bit signed normalized integer.
    Rgba16Snorm,
    /// RGB channels, 16 bit signed normalized integer.
    Rg16Snorm,
    /// Red+Green channels, 8 bit signed normalized integer.
    Rg8Snorm,
    /// Red channel, 16 bit signed normalized integer.
    R16Snorm,
    /// Red channel, 16 bit signed normalized integer.
    R8Snorm,
    /// RGBA channels, 32 bit signed integer.
    Rgba32i,
    /// RGBA channels, 16 bit signed integer.
    Rgba16i,
    /// RGBA channels, 8 bit signed integer.
    Rgba8i,
    /// Red channel, 32 bit signed integer.
    R32i,
    /// Red+Green channels, 32 bit signed integer.
    Rg32i,
    /// Red+Green channels, 16 bit signed integer.
    Rg16i,
    /// Red+Green channels, 8 bit signed integer.
    Rg8i,
    /// Red channel, 16 bit signed integer.
    R16i,
    /// Red channel, 8 bit signed integer.
    R8i,
    /// RGBA channels, 32 bit unsigned integer.
    Rgba32ui,
    /// RGBA channels, 16 bit unsigned integer.
    Rgba16ui,
    /// RGBA channels, 8 bit unsigned integer.
    Rgba8ui,
    /// Red channel, 32 bit unsigned integer.
    R32ui,
    /// 32 bit unsigned integer containing three 10 bit unsigned integers for
    /// the Red, Green, and Blue channels, and a 2 bit unsigned integer for the
    /// Alpha channel.
    Rgb10A2ui,
    /// Red+Green channels, 32 bit unsigned integer.
    Rg32ui,
    /// Red+Green channels, 16 bit unsigned integer.
    Rg16ui,
    /// Red+Green channels, 8 bit unsigned integer.
    Rg8ui,
    /// Red channel, 16 bit unsigned integer.
    R16ui,
    /// Red channel, 8 bit unsigned integer.
    R8ui,
    /// Red channel, 64 bit unsigned integer.
    R64ui,
    /// Red channel, 64 bit signed integer.
    R64i,
}
