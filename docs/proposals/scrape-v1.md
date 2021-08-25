# Scrape API Spec

Yaml File directive-based API to a scrape job(s)

## Problem Statement

when scraping you know the structure of an HTML document well enough to write basic basic selectors that target any and all web pages

## User Stories

As a Person Interested with published web content I would like to define selectors in a yaml file to declaratively create selectors for a scrape job

## Workflow

1. install or have crawlrs
2. create a directory for your scrape project
3. define yaml
4. run the cli tool with the yaml as input

scenario:

have this cli tool dockerized, with the Dockerfile in the repo as well as like committed json/yaml files for your data and declarative yaml descriptions of the selectors/transformers
to update your data, re-run the exact same scrape job with the tool
if the website changes, you can fairly trivially tweak your declarative selectors
commit the whole thing, and then you have a fairly manageable scaped data set

## Job Spec

A scrape job, loosely modeled after https://kubernetes.io/docs/concepts/overview/working-with-objects/kubernetes-objects/

kind correspond to node types: html, script, style, text, image, video, bindata

### Html

an Html scrap job

spec for defining a target page (or a pattern to follow given a sitemap or an index page, i.e. posts page, tags page, etc)

```yaml
---
apiVersion: scrape/v1
kind: html
metadata:
  name: string # job name
  dir: string # PathBuf directory path to write
spec:
  url: string #starting url
  options:
    follow: regex #url or pattern (if follow) to scrape job against
  output:
    - name: string #object name
      properties:
        - name: #property name that will be parsed into output object
          type: string # string, int, floating point -- type that value should be treated as
          selector: string #html tag, class, or ID
```
