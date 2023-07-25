use std::process::Child;

use markdown::mdast::Link;

trait OpenBrowser {
    fn open_in_browser(&self) -> Option<Child>;
}

impl OpenBrowser for Link {
    fn open_in_browser(&self) -> Option<Child> {
        std::process::Command::new("xdg-open")
            .arg(&self.url)
            .spawn()
            .ok()
    }
}
