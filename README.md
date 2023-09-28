# Luma
Link viewer to open and edit links in a yaml file.

## Building
```sh
cargo build
# OR
just build
```
## Testing
```sh
just publish
OR
cargo test
cargo clippy
```

## Development

General Idea is that it takes applications that already exists and unifies them.
It follows unix philosiphy in letter other programs do other things better.

I have lots of ideas for what this could be in the thing and here they are
arranged by importance.

### Next
- [X] saving
- [X] pasting
- [X] better error handling
- [ ] multiple input source (some static some dynamic)
- [ ] use $OPENER
- [ ] simple mpd client

### For 1.0
- [ ] color
- [ ] header data from site for links
- [ ] editing in external editor
- [ ] highlights for tags
- [ ] vaults of notes
- [ ] importing text files
- [ ] better mpd (custom client?)

### Random/Long Term Ideas

- [ ] cli options
- [ ] importing binary files
- [ ] downloading files
- [ ] tagging metadata

