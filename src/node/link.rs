use crate::prelude::*;

trait OpenBrowser {
    fn open_in_browser(&self) -> Option<std::process::Child>;
}

impl OpenBrowser for mdast::Link {
    fn open_in_browser(&self) -> Option<std::process::Child> {
        std::process::Command::new("xdg-open")
            .arg(&self.url)
            .spawn()
            .ok()
    }
}
