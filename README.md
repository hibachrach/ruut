# ruut

[![Build
Status](https://travis-ci.org/HarrisonB/ruut.svg?branch=master)][ruut-travis]
[![Crates.io](https://img.shields.io/crates/v/ruut)][ruut-crate]

## Why

I deal with folder structures a lot for work at [DocSend][docsend]. I also love the
output of [`tree(1)`][tree-wiki] for talking about folder structures.

Unfortunately, most of the time I'm not talking about folder structures on my
file-system, so in order to get the pretty output with `tree(1)`, I would have
to create the directories and/or files on my computer, which seems a bit
ridiculous.

That's why I created `ruut`. It takes a fairly easy-to-type expression like
this:
```
Parent (Child 1, Child 2 (Grandchild 1, Grandchild 2), Child 3)
```

and turns it into something pretty like

```
Parent
├── Child 1
├── Child 2
│   ├── Grandchild 1
│   └── Grandchild 2
└── Child 3
```

It's also good for pretty-printing serialized representations of trees (see the
`format` options below).

## Usage

`ruut` can either take the "structure" as an argument or from stdin:

```sh
$ ruut 'Parent (Child 1, Child 2 (Grandchild 1, Grandchild 2), Child 3)'
# Equivalent to
$ echo 'Parent (Child 1, Child 2 (Grandchild 1, Grandchild 2), Child 3)' | ruut
```

## Installation

### Download from GitHub

Grab the newest published version from the [Releases section][ruut-releases] of
this repo.

### With `cargo`

1. [Install `rust` with `rustup`][rustup-instructions]
2. Run `cargo install ruut`

## Formats

All of the examples in this section produce this as an output:
```
Parent
├── Child 1
├── Child 2
│   ├── Grandchild 1
│   └── Grandchild 2
└── Child 3
    └── Grandchild 3
```

### Parens (`-f parens`, default)

```
Parent (Child 1, Child 2 (Grandchild 1, Grandchild 2), Child 3 (Grandchild 3))
```

This is intended to be easy to type.  Note that whitespace in the middle of a
folder name is preserved.

Here's a more formal description of the syntax:

```
<name of folder> [(<name of subfolder 1> [(<name of subsubfolder1>[, <name of
subsubfolder2>[, ...]])][, <name of subfolder 2> [, ...]])]
```

Surrounding `<`,`>` means
you fill in those values yourself. Surrounding `[`,`]` means that part is
optional.

### JSON (`-f json`)

```json
{
  "Parent": {
    "Child 1": null,
    "Child 2": {
      "Grandchild 1": null,
      "Grandchild 2": {},
    },
    "Child 3": {
      "Grandchild 3": "doesn't matter",
    }
  }
}
```

Only key names are really relevant here. Note that entities other than objects
and empty objects are ignored.

Note that all [JSON5][json5] syntax is accepted. JSON5 is a superset of JSON
with support for different types of quotes, comments, etc., so you can much more
easily copy from an actual JavaScript environment. See the [JSON5
website][json5] for more details.

### JSON with properties (`-f jsonprop`)

```json
{
  "name": "Parent",
  "children": [
    {
      "name": "Child 1"
    },
    { 
      "name": "Child 2",
      "children": [
        {
          "name": "Grandchild 1"
        },
        {
          "name": "Grandchild 2",
          "children": []
        }
      ]
    },
    {
      "name": "Child 3",
      "children": [
        {
          "name": "Grandchild 3",
          "children": null
        }
      ]
    }
  ]
}
```

This is often useful if you're pulling structured trees from some external
source instead of writing them by hand. Note that `children` can also be an
object--in such a case, the properties of that object are iterated over and key
names are ignored:

```json
{
  "name": "Parent",
  "children": {
    "whatever_1": {
      "name": "Child 1"
    },
    "whatever_2": { 
      "name": "Child 2",
      "children": {
        "pls_ignore": {
          "name": "Grandchild 1"
        },
        "test_post": {
          "name": "Grandchild 2",
          "children": {}
        }
      }
    },
    "whatever_3": {
      "name": "Child 3",
      "children": {
        "it_me": {
          "name": "Grandchild 3",
          "children": null
        }
      }
    }
  }
}
```

Note that all [JSON5][json5] syntax is accepted. JSON5 is a superset of JSON
with support for different types of quotes, comments, etc., so you can much more
easily copy from an actual JavaScript environment. See the [JSON5
website][json5] for more details.

By default, this format looks for the properties `name` for what to print for
each item and `children` for what items are immediate descendants. To change
this, you can use the `--template` and `--children` options, respectively.
Missing values are filled in with the text `<missing>`--this can be overridden
with the `--raise-on-missing` flag.

#### `-t/--template <template_str>`

This option allows you to grab any properties from each JSON node using a simple
curly brace template syntax. E.g. for a node with the properties `id` = `3`,
`type` = `"Folder"`, you could write a template string of `Id: {id}, Type:
{type}` which would result in `Id: 3, Type: Folder`.

#### `-c/--children <children_prop>`

This option allows you to specify the name of the property that contains the
children JSON nodes, which is `children` by default.

#### `-r/--raise-on-missing`

This flag will cause `ruut` to immediately error out if any of the placeholders
in the template are missing.

## Versioning

This project respects [semantic versioning][semver].

[ruut-travis]: https://travis-ci.org/HarrisonB/ruut
[ruut-crate]: https://crates.io/crates/ruut
[docsend]: https://www.docsend.com/
[tree-wiki]: https://en.wikipedia.org/wiki/Tree_(command) 
[lisp-parser-python]: https://norvig.com/lispy.html
[rustup-instructions]: https://rustup.rs/
[ruut-releases]: https://github.com/HarrisonB/ruut/releases
[json5]: https://json5.org/
[semver]: https://semver.org/
