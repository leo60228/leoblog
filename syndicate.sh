#!/usr/bin/env bash
zola build
webmention public/atom.xml --limit 0 --send | cargo run --bin syndicate_links
