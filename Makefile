## ▸▸▸ Local commands ◂◂◂

.PHONY: help
help:	## Show this help
	@fgrep -h "## " $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

.PHONY: lint
lint:		## Run cargo clippy
	cargo clippy --all-features

.PHONY: doc
doc:		## Make documantions
	cargo doc --workspace --all-features --no-deps --document-private-items

PHONY: book
book:		## Make the documantion book
	mdbook build doc

.PHONY: serve
serve:		## Open the documantion book
	mdbook serve --open doc
