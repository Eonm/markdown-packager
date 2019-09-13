[![Build Status](https://travis-ci.org/Eonm/markdown-packager.svg?branch=master)](https://travis-ci.org/Eonm/markdown-packager)
[![Coverage Status](https://coveralls.io/repos/github/Eonm/markdown-packager/badge.svg)](https://coveralls.io/github/Eonm/markdown-packager)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# 📦 Markdown Packager

Link and embed files inside your markdown documents

## Install

**On linux**

```sh
curl -L https://github.com/Eonm/markdown-packager/releases/latest/download/markdown-packager --output markdown-packager
chmod +x markdown-packager
sudo mv -u markdown-packager /usr/bin/
```

## Pack

The `pack` sub command allows you to embed files inside your markdown file.

```sh
markdown-packager -i input_file.md pack style.css metadata.yml
```

**External files**

* `.yml` : The content of the external yaml file is added to the header of your markdown file.
* `.md` : The header of the external markdown document won't be added to your markdown file. The content of the .md file is appended at the end of your markdown file.

Other file formats are appended at the end of your markdown file.

**Images**

Images are embedded as base64 links. Remote images are downloaded before being embedded.

```md
![Example image](./test/files/image.gif)

<!-- will be rendered as : -->

![Example image](data:image/gif;base64,R0lGODlhAQABAIABAAAAAP///yH+EUNyZWF0ZWQgd2l0aCBHSU1QACwAAAAAAQABAAACAkQBADs=)
```

**Images format**

Supported image formats are :

* .gif
* .png
* .jpeg
* .svg

## Link

The `link` sub command allows you to link images inside your markdown file. Remote images are downloaded before being linked.

```md
![Example image](https://upload.wikimedia.org/wikipedia/commons/4/48/Markdown-mark.svg)

<!-- will be rendered as : -->

![Example image](./Markdown-mark.svg)
```

## Global options

* `--image-dir` : This flag allows you to specify the destination and source folder for images. Remote images are downloaded in this directory.

* `--log` : Display useful informations

## Use cases

**Embed bibliographical references** :

See how embeded references work : https://rmarkdown.rstudio.com/authoring_bibliographies_and_citations.html#inline_references

```sh
markdown-packager -i input_file.md pack my_bibliography.yaml -o output_file.md
pandoc output_file.md -o output_file.pdf -F pandoc-citeproc
```

**Embed images** :

```sh
markdown-packager -i input_file.md pack -o output_file.md
```

**Stdin and Stdout :**

Read content from stdin and write content to stdout.

```sh
cat input_file.md | markdown-packager pack > output_file.md
```
