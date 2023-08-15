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
```

## Development

I have lots of ideas for what this could be in the thing and here they are
arranged by importance

### Next
- [X] saving
- [X] pasting
- [ ] error handling (plan: bubble up errors so term can exit before they print)
- [ ] pass app as value not struct impls (get ride of double mut problems)
- [ ] use $OPENER

### For 1.0
- [ ] color
- [ ] header data from site for links
- [ ] editing in external editor
- [ ] highlights for tags
- [ ] vaults of notes
- [ ] importing text files

### Random/Long Term Ideas

- [ ] cli options
- [ ] importing binary files
- [ ] downloading files
- [ ] tagging metadata

