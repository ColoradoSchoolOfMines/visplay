SVGFILES = $(wildcard *.svg)
PDFFILES = $(patsubst %.svg, %.pdf, $(SVGFILES))
PNGFILES = $(patsubst %.svg, %.png, $(SVGFILES))

all: $(PDFFILES) $(PNGFILES)

%.pdf: %.svg
	inkscape --export-pdf $@ $<

%.png: %.svg
	inkscape --export-png $@ $<

.PHONY: clean
clean:
	rm -f $(PDFFILES)
