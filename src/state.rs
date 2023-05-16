use std::{fs::File, io::{BufReader, BufRead}};

pub struct MarkdownFile {
    pub sections: Vec<Section>,
}

pub struct Section {
    pub title: String,
    pub notes: Vec<String>,
    pub links: Vec<Link>,
    pub subsections: Vec<Section>,
    pub folded: bool,
}

pub struct Link {
    title: String,
    pub link: String,
    notes: Vec<String>,
}

impl MarkdownFile {
    pub fn from_file(f: File) -> anyhow::Result<Self> {
        // let content = fs::read_to_string(&file).unwrap();
        let mut sections: Vec<Section> = Vec::new();
        sections.push(Section::empty());

        let mut is_first = true;
        let rd = BufReader::new(f);
        for l in rd.lines() {
            let l = l?;
            if l.is_empty() {
                continue;
            }
            if l.starts_with('[') && l.ends_with(')') {
                if is_first {
                    entries.pop();
                }
                let items: Vec<&str> = l.split("](").take(2).collect();
                let title = items[0].replacen('[', "", 1);
                let link = items[1].replacen(')', "", 1);
                entries.push(Entry {
                    link,
                    title,
                    notes: Vec::new(),
                    folded: false,
                });
            } else {
                let last = entries.last_mut().unwrap();
                last.notes.push(l);
            }
            is_first = false;
        }

    // .fold(Vec::new(), |mut acc, line| {
    //     if line.starts_with('[') && line.ends_with(')') {
    //         acc.push(line.to_string());
    //     } else {
    //         let last = acc.last_mut().unwrap();
    //         last.push_str(line);
    //     }
    //     acc
    // })
    // .filter(|l| l.starts_with('[') && l.ends_with(')'))
    // .map(|line| {
    //     // TODO destructure using a regex
    //     let items: Vec<&str> = line.split("](").take(2).collect();
    //     let title = items[0].replacen('[', "", 1);
    //     let link = items[1].replacen(')', "", 1);
    //     Entry { link, title }
    // })

        Ok(MarkdownFile { sections })
    }

    pub fn current_section(&mut self, index: usize) -> &mut Section {
        &mut self.sections[index]
    }
}

impl Default for MarkdownFile {
    fn default() -> Self {
        log::info!("No file provided, creating empty file");
        MarkdownFile { sections: Vec::new() }
    }
}

impl Section {
    pub fn from_string(s: String) -> Self {
        todo!()
    }

    pub fn empty() -> Self {
        Section {
            title: String::new(), 
            notes: Vec::new(), 
            links: Vec::new(), 
            subsections: Vec::new(), 
            folded: true, 
        }
    }
}

impl Link {
    pub fn open(&self) -> anyhow::Result<()> {
        let _ = std::process::Command::new("xdg-open")
            .arg(&self.link)
            .output()
            .unwrap();
        Ok(())
    }
}
