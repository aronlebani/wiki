# wiki

> Recursively query Markdown notes

## Features

Supports querying any combination of the following Markdown entities, as well as arbitrary string queries.

```
- [ ] Unchecked todo items
[Links]("https://github.com")
Lines with a custom _tag_ using _markdown_ italic syntax
```

Markdown `_italic_` syntax is borrowed to allow custom tagging of notes with automatic syntax highlighting. The alternative `*italic*` syntax can be used for adding emphasis.

## Installation

1. Ensure you have the Rust compiler installed
2. Clone repository
3. Run `./install.sh` inside the repository directory
4. Create a file `~/.config/wiki/config` with the following contents as an example

```
editor="vim"
path="/home/aron/wiki/"
```

## Usage

```
wiki [FLAGS] [OPTIONS]
```

For full usage, run:
```
wiki --help
```

## Examples

Query all lines with the tags `_tag_` and `_markdown_`: 
```
wiki query -t 'tag markdown'
```

Output:
```
README.md:12: Lines with a custom _tag_ using _markdown_ italic syntax
```

Query all pending todo items which contain the string `unchecked`:
```
wiki query -c -q 'unchecked'
```

Output:
```
README.md:10: - [ ] Unchecked todo items
```
