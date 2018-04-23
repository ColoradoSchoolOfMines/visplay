# Minimal makefile for Sphinx documentation
#

# You can set these variables from the command line.
SPHINXOPTS    =
SPHINXBUILD   = sphinx-build
SPHINXPROJ    = Visplay
SOURCEDIR     = docs
BUILDDIR      = build

SPHINXDOCBUILD = sphinx-apidoc
SPHINXDOCOPTS  = -f -e -T
DOCSSOURCE     = ./docs/api/
PROJECTDIR     = visplay

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

clean:
	rm -rf docs/api

html: Makefile
	$(SPHINXDOCBUILD) $(SPHINXDOCOPTS) -o $(DOCSSOURCE) $(PROJECTDIR)
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

.PHONY: help Makefile

# Catch-all target: route all unknown targets to Sphinx using the new
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)
