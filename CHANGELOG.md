# Changelog

## 0.2.2 (unreleased)

- Fix shortcodes without arguments being ignored
- Fix shortcodes with markdown chars (_, *, etc) in name and args being ignored
- Fix subsections of index not being filled without a `_index.md`
- Fix generated index section not found in `get_section` global function
- Fix permalink generation for index page
- Add Nim syntax highlighting
- Allow static folder to be missing
- Fix shortcodes args being only passed as strings


## 0.2.1 (2017-10-17)

- Fix `base-url` argument to `gutenberg build` being called `base`
- Add syntaxes: Crystal, Elixir, Kotlin

## 0.2.0 (2017-10-05)

- Fix `section.subsections` not being filled correctly
- `section.subsections` can now be sorted by a `weight` attribute on a section front-matter
- Do nothing on directory adding/removal in livereload
- Add back `draft` on pages that was wrongly removed
- Page and Section `path` field is not starting with a `/` anymore
- All Tera global fns are now rebuilt on changes
- Use flags for port/interface in `gutenberg serve`
- Fix various issues with headers markdown rendering
- Rename `insert_anchor` in section front-matter to `insert_anchor_links`
- Remove `insert_anchor_links` from the config: it wasn't used
- Add `class` variable to `gist` shortcode
- Add reading analytics to sections content
- Add config to sitemap template
- Add `permalink` to all taxonomy items (tags & categories)
- Tags in the tags page are now sorting alphabetically instead of by number of pages in them
- Remove deprecated `link` param of `get_url`
- Add 1337 color scheme
- Defaults to compressed Sass output
- Fix regression wrt co-located assets slug detecting
- Rename `url` from page front-matter to `path` to be consistent
- Add a `base-url` flag to the `build` command to override the URL from config.toml

## 0.1.3 (2017-08-31)

- Add themes support


## 0.1.2 (2017-08-10)

- Add `redirect_to` to section front matter to redirect when landing on section
root page
- Make `title` in config optional
- Improved `gutenberg init` UX and users first experience
- Make `get_url` work for any path with optional cachebusting. 
- Deprecates `link` param of `get_url` in favour of `path` to be consistent

## 0.1.1 (2017-07-16)

- Fix RSS feed not behaving (https://github.com/Keats/gutenberg/issues/101)

## 0.1.0 (2017-07-14)

- Parallelize all the things
- Add weight sorting
- Remove `section` from the `page` rendering context: this is too expensive. Use
the global function `get_section` if you need to get it
- Put back a 20 page limit on rss feed by default (configurable)
- Remove index page getting all sections: use the `get_section` global fn instead to
only get the ones you need
- Remove pages from pagers in pagination: they were not supposed to be there
- Add built-in Sass compilation support


## 0.0.7 (2017-06-19)

- Sort individual tag/category pages by date
- Add extra builtin shortcode for Streamable videos
- `path` and `permalink` now end with a `/`
- Generate table of contents for each page
- Add `section` to a page Tera context if there is one
- Add `aliases` to pages for when you are changing urls but want to redirect
to the new one
- Name the homepage section `index` (previously empty string)

## 0.0.6 (2017-05-24)

- Fix missing serialized data for sections
- Change the single item template context for categories/tags
- Add a `get_url` and a `get_section` global Tera function
- Add a config option to control how many articles to show in RSS feed
- Move `insert_anchor_links` from config to being a section option and it can
now be insert left or right


## 0.0.5 (2017-05-15)

- Fix XML templates overriding and reloading
- `title` and `description` are now optional in the front matter
- Add GenericConfig, Vim, Jinja2 syntax
- Add `_index.md` for homepage as well and make that into a normal section
- Allow sorting by `none`, `date` and `order` for sections
- Add pagination
- Add a `get_page` global function to tera
- Revamp index page, no more `pages` variables
- Fix livereload stopping randomly
- Smarter re-rendering in `serve` command

## 0.0.4 (2017-04-23)

- Fix RSS feed link and description
- Renamed `Page::url` and `Section::url` to `Page::path` and `Section::path`
- Pass `current_url` and `current_path` to every template
- Add id to headers to allow anchor linking
- Make relative link work with anchors
- Add option to render an anchor link automatically next to headers
- Only copy the static files that changed, not the whole directory in `gutenberg serve`
- Use summary if available in RSS feed
- Add tables and footnotes support in markdown
- Add more language syntaxes
- Only load templates ending by `.html`

## 0.0.3 (2017-04-05)

- Add some colours in console
- Allow using a file other than config.toml for config
- Add sections to the index page context
- Fix page rendering not working when containing `+++`
- Add shortcodes (see README for details)
- Allow relative links to other content in markdown links
- Add `markdown`, `base64_encode` and `base64_decode` filters to the Tera instance of Gutenberg
- Work on Windows!
