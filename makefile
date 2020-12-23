SHELL := zsh
.ONESHELL:
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

all: _build/.stylesheet.sentinel
.PHONY: all

_build/.stylesheet.sentinel: $(shell find assets/stylesheets -type f)
	find assets/stylesheets -type f | xargs cat > _build/assets/app.css
	touch $@

assets/stylesheets/*.css: