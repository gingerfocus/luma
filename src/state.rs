use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct MarkdownFile {
    pub notes: Vec<String>,
    pub links: Vec<Link>,
    pub sections: Vec<Section>,
}

impl MarkdownFile {
    pub fn from_file(f: File) -> Self {
        // let content = fs::read_to_string(&file).unwrap();
        let mut sections: Vec<Section> = Vec::new();
        sections.push(Section::empty());

        let mut is_first = true;
        let rd = BufReader::new(f);
        let mut lines_builder = Vec::new();
        let mut top_level_content = Vec::new();

        for l in rd
            .lines()
            .filter(|l| l.is_ok())
            .map(|l| l.unwrap())
            .filter(|l| !l.is_empty())
        {
            if l.starts_with('#') {
                sections.push(Section::from_string(lines_builder.join("\n"), 1));
                lines_builder.clear();
                lines_builder.push(l);
                is_first = false;
            } else {
                if is_first {
                    top_level_content.push(l);
                } else {
                    lines_builder.push(l);
                }
            }
        }

        // somhow turn the top_level_content into links and notes

        MarkdownFile {
            notes: Vec::new(),
            links: Vec::new(),
            sections,
        }

        // if l.starts_with('[') && l.ends_with(')') {
        //                 let items: Vec<&str> = l.split("](").take(2).collect();
        //                 let title = items[0].replacen('[', "", 1);
        //                 let link = items[1].replacen(')', "", 1);
        //                 entries.push(Entry {
        //                     link,
        //                     title,
        //                     notes: Vec::new(),
        //                     folded: false,
        //                 });

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
    }

    pub fn current_section(&mut self, index: usize) -> &mut Section {
        &mut self.sections[index]
    }
}

pub struct Section {
    pub title: String,
    pub notes: Vec<String>,
    pub links: Vec<Link>,
    pub subsections: Vec<Section>,
    pub folded: bool,
    depth: usize,
}

impl Section {
    /// Note this implementation will miss things if peoople skip heading levels
    pub fn from_string(s: String, depth: usize) -> Self {
        let subsection_title = format!("{}#", "#".repeat(depth));
        let mut lines = s.lines();
        let title = lines.next().unwrap().replacen(&subsection_title, "", 1);

        let mut is_first = true;
        let mut lines_builder = Vec::new();
        let mut top_level_content = Vec::new();
        let mut subsections = Vec::new();

        for l in lines {
            if l.starts_with(&subsection_title) {
                subsections.push(Section::from_string(lines_builder.join("\n"), 1));
                lines_builder.clear();
                lines_builder.push(l);
                is_first = false;
            } else {
                if is_first {
                    top_level_content.push(l);
                } else {
                    lines_builder.push(l);
                }
            }
        }

        let (notes, links) =
            top_level_content
                .iter()
                .fold((Vec::new(), Vec::new()), |mut acc, line| {
                    if line.starts_with('[') && line.ends_with(')') {
                        let link = Link::from_string(line.to_string());
                        acc.1.push(link);
                    } else {
                        acc.0.push(line.to_string());
                    }
                    acc
                });

        let folded = depth > 2;

        Section {
            title,
            notes,
            links,
            subsections,
            folded,
            depth,
        }
    }

    pub fn empty() -> Self {
        Section {
            title: String::new(),
            notes: Vec::new(),
            links: Vec::new(),
            subsections: Vec::new(),
            folded: true,
            depth: 1,
        }
    }

    pub fn format(&self) -> String {
        let header = self.header_format();
        if self.folded {
            return header;
        }
        let notes = self
            .notes
            .iter()
            .map(|l| format!("\t{}", l))
            .collect::<Vec<String>>()
            .join("\n");
        let links = self
            .links
            .iter()
            .map(|l| l.format())
            .collect::<Vec<String>>()
            .join("\n");
        let subsections = self
            .subsections
            .iter()
            .map(|s| s.format())
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}\n{}\n{}\n{}", header, notes, links, subsections)
    }

    pub fn header_format(&self) -> String {
        format!("{}{}", "#".repeat(self.depth), self.title)
    }
}

pub struct Link {
    title: String,
    pub link: String,
    notes: Vec<String>,
}

impl Link {
    pub fn open(&self) -> anyhow::Result<()> {
        _ = std::process::Command::new("xdg-open")
            .arg(&self.link)
            .output()
            .unwrap();
        Ok(())
    }

    pub fn format(&self) -> String {
        let link = self.link_format();
        link + self
            .notes
            .iter()
            .map(|l| format!("\t- {}", l))
            .collect::<Vec<String>>()
            .join("\n")
            .as_str()
    }

    pub fn link_format(&self) -> String {
        format!("[{}]({})", self.title, self.link)
    }

    pub fn new(title: String, link: String) -> Self {
        Link {
            title,
            link,
            notes: Vec::new(),
        }
    }

    pub fn from_string(s: String) -> Self {
        let items: Vec<&str> = s.split("](").take(2).collect();
        let title = items[0].replacen('[', "", 1);
        let link = items[1].replacen(')', "", 1);
        Link {
            link,
            title,
            notes: Vec::new(),
        }
    }
}
