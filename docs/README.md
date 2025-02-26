# Documentation

Follow the instructions below to generate the design document.

```
docker build -t pandoc-custom .
```

Windows:
```
docker run --rm -v .\:/data pandoc-custom -s md/1-introduction.md md/2-security.md --template=sigpwny-theme.tex -o uiuc-design-doc.pdf --pdf-engine=xelatex -f gfm
```

If on Linux, be sure to set the user and group ID of the current user:
```
docker run --rm -v "$(pwd):/data" -u $(id -u):$(id -g) pandoc-custom -s md/part1.md md/part2.md md/part3.md --template=sigpwny-theme.tex -o uiuc-design-doc.pdf --pdf-engine=xelatex -f gfm
```
