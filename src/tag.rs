use std::{io::Write, path::PathBuf};

use crate::prelude::*;

#[derive(Debug)]
pub enum Programs {
    Editor(EditorData),
    Tagger { editor: EditorData },
}

#[derive(Debug)]
pub struct EditorData {
    text: String,
    resp: oneshot::Sender<String>,
}

const LUMA_TEMPFILE: &str = "/tmp/luma-edit.json";

impl Programs {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Editor(data) => {
                {
                    let mut f = fs::File::create(LUMA_TEMPFILE).unwrap();
                    f.write_all(data.text.as_bytes()).unwrap();
                    log::debug!("wrote inital data to buffer");
                }

                let _ = tokio::process::Command::new("nvim")
                    .arg(LUMA_TEMPFILE)
                    .spawn()
                    .unwrap()
                    .wait()
                    .await
                    .unwrap()
                    .success();

                log::debug!("asked question");

                let res = fs::read(LUMA_TEMPFILE).unwrap();
                if let Ok(s) = String::from_utf8(res) {
                    data.resp.send(s).unwrap();
                }
                Ok(())
            }
            Self::Tagger { editor } => {
                Err(format!("Unimplliented handle of tagger: {:?}", editor).into())
            }
        }
    }

    pub async fn editor(text: String, resp: oneshot::Sender<String>) -> Programs {
        Programs::Editor(EditorData { text, resp })
    }

    pub fn tagger(_file: impl AsRef<PathBuf>) -> Programs {
        todo!()
        // err() { echo "Usage:
        //     tag [OPTIONS] file
        // Options:
        //     -I: dont hide the image
        // " && exit 1 ;
        // }
        //
        // file="$1"
        //
        // temp="$(mktemp "/tmp/$1.XXXXXX")"
        // trap 'rm -f $temp' HUP INT QUIT TERM EXIT
        //
        // [ ! -f "$file" ] && echo 'Provide file to tag.' && exit 1
        //
        // vorbiscomment -l "$file" -c "$temp"
        //
        // if [ ! -n "$donthide" ]; then
        //     image="$(grep "METADATA_BLOCK_PICTURE" "$temp")"
        //     sed -i -e '/METADATA_BLOCK_PICTURE/d' "$temp"
        // fi
        //
        // grep "language=" $temp || echo "language=" >> "$temp"
        // grep "title=" $temp || echo "title=" >> "$temp"
        // grep "artist=" $temp || echo "artist=" >> "$temp"
        // grep "album=" $temp || echo "album=" >> "$temp"
        // # grep "TRACKNUMBER=" $temp || echo "TRACKNUMBER=" >> $temp
        // # grep "total=" $temp || echo "total=" >> $temp
        // grep "year=" $temp || echo "year=" >> "$temp"
        // grep "tags=" $temp || echo "tags=" >> "$temp"
        // grep "rating=" $temp || echo "rating=" >> "$temp"
        // grep "comment=" $temp || echo "comment=" >> "$temp"
        //
        // $EDITOR $temp
        //
        // [ -n "$image" ] && echo "$image" >> "$temp"
        // vorbiscomment -w "$file" -c "$temp"
        //
        // rm "$temp"
        //
    }
}
