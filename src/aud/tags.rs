// use crate::prelude::*;

// use std::path::Path;
// use fs::File;
// use vorbis_rs::{VorbisDecoder, VorbisEncoderBuilder};
//
// pub fn add_tag(f: impl AsRef<Path>, key: &str, value: &str) -> Result<()> {
//     let mut source_ogg = File::open(f)?;
//     let mut transcoded_ogg = vec![];
//
//     let mut decoder = VorbisDecoder::new(&mut source_ogg)?;
//     let mut encoder = VorbisEncoderBuilder::new(
//         decoder.sampling_frequency(),
//         decoder.channels(),
//         &mut transcoded_ogg,
//     )?
//     .comment_tag(key, value)?
//     .build()?;
//
//     while let Some(decoded_block) = decoder.decode_audio_block()? {
//         encoder.encode_audio_block(decoded_block.samples())?;
//     }
//
//     // The encoder is automatically finished on drop, so calling finish explicitly is
//     // only needed if you want to handle any errors
//     let data = encoder.finish()?;
//
//     fs::write("test.ogg", data)?;
//
//     Ok(())
// }
