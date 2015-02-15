# A subtitle lib and CLI-tool

The main idea for the library is to have something that could work like this:

`subtitle.open('path/to/file.srt').shift_by_sec(10).export_as_ssa('path/to/file.ssa')`

## The `subtitle` struct

That would be the main struct, it represents a group of subtitles
with the file/format being abstracted.

### Possible functions:

- `open`: would open a subtitle file and return a subtitle struct


## The `parsers` module

The `parsers` module could provide some basic interactions with parsers like
for example returning a list of all available parsers, which would be submodules
of the `parsers` module.
