#[repr(align(4))]
pub struct WordAligned<Bytes: ?Sized>(pub Bytes);

/// Treat the given by slice as a SPIR-V module.
/// The pointer has to be aligned to 32-bit boundary and be a valid SPIR-V binary.
pub fn make_spirv<'a>(data: &'a [u8]) -> super::ShaderModuleSource<'a> {
    const MAGIC_NUMBER: u32 = 0x0723_0203;

    let (pre, words, post) = unsafe { data.align_to::<u32>() };
    assert_eq!(pre, &[], "data offset is not aligned to words!");
    assert_eq!(post, &[], "data size is not aligned to words!");
    assert_eq!(
        words[0], MAGIC_NUMBER,
        "wrong magic word {:x}. Make sure you are using a binary SPIRV file.",
        words[0]
    );
    super::ShaderModuleSource::SpirV(words)
}
