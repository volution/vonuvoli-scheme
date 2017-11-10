
https://www.tug.org/applications/tex4ht/mn-commands.html
https://github.com/michal-h21/helpers4ht/wiki/tex4ht-tutorial

## current variant
htlatex r7rs.tex "xhtml,mathml" " -cunihtf -utf8" "-cvalidate"
htlatex r7rs-ebook.tex "xhtml,mathml" " -cunihtf -utf8" "-cvalidate"

htlatex r7rs.tex "xhtml,mathml" " -cunihtf" "-cvalidate"
htlatex r7rs-ebook.tex "xhtml,mathml" " -cunihtf" "-cvalidate"

## other variants
htlatex r7rs.tex "xhtml,uni-html4" " -cunihtf"
htlatex r7rs.tex "xhtml,mozilla" " -cmozhtf" "-cvalidate"

