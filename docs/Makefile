default:
	@echo "Building the Design Document"
	@pandoc -F mermaid-filter -F pandoc-crossref \
	1-introduction.md \
	2-security.md \
	3-functional-requirements.md \
	4-security-requirements.md \
	--template=sigpwny-theme.tex \
	-o uiuc-design-doc.pdf \
	--pdf-engine=xelatex \
	-f gfm