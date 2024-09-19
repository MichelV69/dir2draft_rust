# DIR2DRAFT

> *"Converts a subdirectory of MD files into a 1 MD file"*

If you're using a tool such as Obsidian, with a myriad of sub-folders containing parts and chapters of Markdown or CommonMark files, and need a quick way to tumble through the directory structure and stitch all those files together into a single Markdown/CommonMark file, this is the tool you might need.

I wrote it because that's my own predicament, and I wanted to learn Rust.  So here we are.

## What Does It Do?

### My Current Project

content
├── 00 - Frontmatter
│   ├── 00 - Artwork, Cover.md
│   ├── 00 - Copyright.md
│   ├── 01 - Content Labels and Guidelines.md
│   ├── 03 - Foreword.md
│   ├── 04 - Table of Contents.md
│   ├── 05 - The Cast
│   │   ├── the Afterliving.md
│   │   ├── the Children of the Stars.md
│   │   ├── the Fäé.md
│   │   ├── the Pillars of the Obsidian Bridge.md
│   │   └── the Stones of the Moon.md
│   └── attachments
│       └── (Bk.3)RedSeptember-52dpi-720p.v03.jpg
├── Part 1 - Fourteen Weeks Later
│   ├── Ch 1 - Nothing To See, Hear
│   │   ├── 01 == 15h03, Tuesday, 23rd September, 1997.md
│   │   ├── 02 == 16h08, Tuesday, 23rd September, 1997.md
│   │   ├── 03 == 19h42, Tuesday, 23rd September, 1997.md
│   │   ├── 04 == a short while later.md
│   │   └── 05 == 21h10, Tuesday, 23rd September, 1997.md
│   └── Ch 2 - Deja Voodoo
│       ├── 01 == 07h08, Wednesday, 24th September, 1997.md
│       ├── 02 == 08h19, Wednesday, 24th September, 1997.md
│       ├── 03 == 08h44, Wednesday, 24th September, 1997.md
│       ├── 04 == 21h26, Wednesday, 24th September, 1997.md
│       └── 05 == a few minutes later.md
└── Part 2 - Dogs of War
    └── Ch 5 - Future Falls
        ├── 12h33 Friday, 26th September, 1997.md
        ├── 13h04, Friday, 26th September, 1997.md
        ├── 20h02pm, Friday, 26th September, 1997.md
        ├── 20h12pm, Friday, 26th September, 1997.md
        ├── 20h21pm, Friday, 26th September, 1997.md
        ├── 22h07pm, Friday, 26th September, 1997.md
        ├── 23h09pm, Friday, 26th September, 1997.md
        ├── 23h31pm, Friday, 26th September, 1997.md
        └── 23h36pm, Friday, 26th September, 1997.md

### How this gets handled

***NB**: for purposes of this discussion, a folder and a directory are interchangable terminology. I will use the term 'Folder' for consistency*.

The application, when run, will tumble through the folder structure and read every entry it sees.  Officially, all it cares about are Markdown/CommonMark files, and folders. If it isn't one of those two things, it will be ignored.

The path structure it expects to see is `BOOK/PART/CHAPTER/SCENE.md`.  If you stick to that, it should work just fine.

It presumes anything ahead of an optional "`==`" in a file or folder name is for sorting purposes only, and _should not_ be displayed.  It presumes anything after an optional "`==`" _should_ be displayed.

For example, `01 == 07h08, Wednesday, 24th September, 1997.md` will be the first thing in the **Table of Contents (TOC)** for Chapter 2, but will be displayed as `07h08, Wednesday, 24th September, 1997.md`.

Any file or folder name that begins with `DNI ==` will be ignored. `DNI` == **Do Not Include**, and is for anything such as plot notes that you're keeping with your project, but you don't want added to monolith file.

Once it reads the folder and file contents of the project into memory, it will then apply a typical sorting mechanism to get everything into order, _'Human-ify'_ the various names (eg, replace `_` with a space, etc), then create a single monolithic file with a simple TOC, and then the full content of the book.

Note that it makes no attemtp to parse, filter or interpret the Markdown/CommonMark content itself. It's doing a complete 1:1 transcription of what it finds.  You may need to process the resulting monolith file afterwards.

## How To Use

## Command Line Options
