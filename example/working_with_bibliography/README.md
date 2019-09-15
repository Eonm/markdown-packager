# Working with a bibliograpy

markdown-packager allows you to embed bibliographical and any other yaml data inside your markdown document.

## Embedding a bibliography

The [bibliography](./files/bibliography.yaml) is embedded in the header of your markdown document. You can produce yaml bibliohraphy with Zotero.

```sh
markdown-packager -i ./files/article.md pack ./files/bibliography.yaml -o ./files/embedded_article.md
```

The header of [./files/embedded_article.md](./files/embedded_article.md) should look like this :

```md
---
author: John Doe
title: SKOS and knowledge organisation
documentclass: article
classoption: twoside
references:
  #
  # ... Other references
  #
  - id: ducharmeLearningSPARQL2ed2013
    type: book
    title: Learning SPARQL 2ed
    publisher: O′Reilly
    number-of-pages: "386"
    edition: 2nd Revised edition
    source: Amazon
    ISBN: 978-1-4493-7143-2
    language: Anglais
    author:
      - family: Ducharme
        given: Bob
    issued:
      - year: 2013
        month: 7
        day: 12
    publisher-place: "Sebastopol, CA"
---
# A SKOS overview

Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.[@bakerKeyChoicesDesign2013, p.35]
```

## Generating a PDF with pandoc

Pandoc-citeproc will use the header of your markdown file to produce citations and the bibliography in your [pdf document](./files/article.pdf).

```sh
pandoc ./files/embedded_article.md -o ./files/article.pdf --pdf-engine=xelatex -F pandoc-citeproc
```
