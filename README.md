# Lynkd
    
    Dead simple link viewer that interprets and outputs markdown. Much easier to use than say. In it you can veiw are collection of links in a markdown file quickly and rename, delete, open and more them around.

### For 1.0
- [X] Handling non-utf8 characters
    - [ ] Add testing to prove it works
    - [ ] Show invisable characters
    - [ ] Place cursor better
- [ ] Using the file pointed to as input
- [ ] Adding
- [ ] Pasting
- [ ] Handling bad input


### Long Term
- [ ] More navigation options
- [ ] Using $OPENER as oppsed to xdg-open (for handlr open)
    - [ ] Confirming/choosing browser window for opening
    - [ ] Support for links to files
- [ ] Cursor moves better
- [ ] Moving links within larger file
- [ ] Queue prints and cursor moves instead of just printing them
    - [reference](https://docs.rs/crossterm/latest/crossterm/trait.QueueableCommand.html)
- [ ] Using multi file input with options to concat or keep seperate
- [ ] Marking as read
- [ ] System Commands inside the thing
- [ ] Color
- [ ] Grab header data from site
- [ ] Use a tui instead of just raw term?
