use proc_macro::TokenStream;

/// A macro for creating SPIR-V `OpTypeImage` types.
///
/// The grammar for the macro is as follows:
///
/// ```no_compile
/// Image!(
///     <dimensionality>,
///     <type|format>,
///     [sampled[=<true|false>],]
///     [multisampled[=<true|false>],]
///     [arrayed[=<true|false>],]
///     [depth[=<true|false>],]
/// )
/// ```
///
/// A basic example looks like this:
/// ```no_compile
/// #[spirv(vertex)]
/// fn main(#[spirv(descriptor_set = 0, binding = 0)] image: &Image!(2D, type=f32)) {}
/// ```
///
/// ## Arguments
///
/// - `dimensionality` — Dimensionality of an image. Accepted values: `1D`,
///   `2D`, `3D`, `rect`, `cube`, `subpass`.
/// - `type` — The sampled type of an image, mutually exclusive with `format`,
///   when set the image format is unknown.  Accepted values: `f32`, `f64`,
///   `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`.
/// - `format` — The image format of the image, mutually exclusive with `type`,
///   Accepted values: Camel case versions of [`ImageFormat`].
/// - `sampled` — Whether it is known that the image will be used with a sampler
///   at compile time, Accepted values: `true` or `false`. Default: `unknown`.
/// - `multisampled` — Whether the image contains multisampled content. Accepted
///    values: `true` or `false`. Default: `false`.
/// - `arrayed` — Whether the image contains arrayed content. Accepted
///    values: `true` or `false`. Default: `false`.
/// - `depth` — Whether it is known that the image is a depth image,
///   Accepted values: `true` or `false`. Default: `unknown`.
///
/// [`ImageFormat`]: spirv_types::image_params::ImageFormat
#[proc_macro]
#[allow(nonstandard_style)]
pub fn Image(item: TokenStream) -> TokenStream {
    item
}
