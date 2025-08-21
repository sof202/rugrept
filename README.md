# Rugrept

A `grep` CLI clone written in Rust. 

Does this do everything that `grep` does?

Hell no.

## Why make this

There's this 
[really nice video by LaurieWired](https://www.youtube.com/watch?v=iQZ81MbjKpU)
that goes over the history of `grep`. Since seeing this I've been very
intrigued by `grep`. I use `grep` basically every day and never really thought
about how and why it was created. This goes for every GNU core utility. I want
to learn how all of these work, even if it takes me way longer than the legends
that developed them in the first place.

## Disclaimer

The starting point of this code base is following on from the guide given by
the [official rust book](https://doc.rust-lang.org/book/title-page.html).
However, it doesn't really implement `grep`, instead it provides the same 
output of `grep`. The project is really just there to get the user familiar
with how to create and design a CLI tool.

GNU `grep` does not simply iterate through lines in a text file and return
those that contain the search query. It instead implements the
[Boyer-Moore algorithm](https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string_search_algorithm).
