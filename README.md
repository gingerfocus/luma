# Lynkd
    
Dead simple link viewer that interprets and outputs markdown. Much easier to use than say. In it you can veiw are collection of links in a markdown file quickly and rename, delete, open and more them around.

Note: As a technical limitation, this code expects all markdown files to start with '# ' and will add that if not present

### For Alpha
- [X] Handling non-utf8 characters
- [X] Using the file pointed to as input
- [X] Adding
- [X] Pasting
- [X] Handling bad input
- [X] Editing Links

### Serialization System
- [ ] Render any input file
- [ ] Add headers to badly formatted files
- [ ] Fold
    - [ ] Cursor moves correctly
    - [ ] Recursive Fold

### For 1.0 
- [ ] More navigation options
- [ ] Using $OPENER as oppsed to xdg-open (for handlr open)
    - [ ] Confirming/choosing browser window for opening
    - [ ] Support for links to files
- [ ] Queue prints and cursor moves instead of just printing them
    - [reference](https://docs.rs/crossterm/latest/crossterm/trait.QueueableCommand.html)
- [ ] Multi file input 
- [ ] Color
- [ ] Grab header data from site
- [ ] Tui
- [ ] Attaching Notes to links
    - [ ] Open editor in temp file
    - [ ] Tmux like pane to do in window?
- [ ] Replace Org mode for my workflow

### Overall project goal

This origonally was just a way to manage links for me but after playing with it a bit I think it can be a
great internet management tool. Ever have a link and you don't know what to do with it. Well just paste
it in here and you can have it managed. Just need to make a not about a specific website so you can go back
later. This has got you covered. Like emacs org mode but internet based instead of text

- [ ] Importing links from spotify
    - Rewriting the spt-to-torrent script
- [ ] Downloading and managing files
    - [ ] Tagging metadata
- [ ] Moving links within larger file
- [ ] System Commands inside the thing
- [ ] Marking as read
