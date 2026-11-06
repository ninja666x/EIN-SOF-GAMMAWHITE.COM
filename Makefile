PDF      := ein-sof-whitepaper-v1.1.pdf
TYPST    := whitepaper/main.typ
PANDOC_MD:= whitepaper/main.md
META     := whitepaper/metadata.yaml

all: pdf

pdf: $(TYPST)
	@command -v typst >/dev/null || (echo "Asenna: cargo install typst-cli" && exit 1)
	typst compile $(TYPST) $(PDF)
	@echo "✅ $(PDF)"

pdf-pandoc: $(PANDOC_MD) $(META)
	pandoc $(PANDOC_MD) \
		--metadata-file=$(META) \
		--pdf-engine=xelatex \
		--toc --toc-depth=2 \
		--highlight-style=tango \
		-V geometry:margin=2.5cm \
		-V fontsize=11pt \
		-V mainfont="DejaVu Serif" \
		-V monofont="DejaVu Sans Mono" \
		-o $(PDF)

clean:
	rm -f $(PDF)

.PHONY: all pdf pdf-pandoc clean
