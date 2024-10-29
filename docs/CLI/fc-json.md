---
tags:
    - cpp
    - beta
---

# fc-json
[:simple-github: Repository](https://github.com/fcrozetta/fc.json) |
[:material-download: Releases](https://github.com/fcrozetta/fc.json/releases/latest)

## Introduction

You know when someone sends you a json file and ask you to create schemas for your application? Well, this is the solution. fc-json accepts a json file as input, and generates the schemas for you. just save the output in your project and use it as regularly.

## Usage

```bash
fc-json myFile.json
# OR
fc-json myFile.json > schemas.py
```
=== "myFile.json"
    ```json
    --8<-- "docs/file_samples/fc-json/sample.json"
    ```
=== "Output"
    ```python
    --8<-- "docs/file_samples/fc-json/sample.py"
    ```

> **Note:** myFile.json need to be a valid json

The root of the file will be called *Root*, and when creating the names of the schemas, you will get a full path, separated by *underscores*, with the suffix *Schema* attached to it.

Although this is probably not the name you will use at the end, this prevents (in most cases) duplication of schema names. This way, you only need to remember two things:

1. You can safely rename the schemas, with not fear of duplicating it.
2. you can test it right away, importing the **RootSchema**, which will never change.

the RootSchema being the initial name should be default in all schemas, changing only the casing. In python *PascalCase* is being used, but it will vary based on the language.

## Limitation on beta
This tool is not 100% production ready. For now, it only generates pydantic schemas. There are issues open to improve this tool, and expand it to different languages.

In the foreseeable future, I plan to add a python dataclass schema and a C# schema. Other languages may be added by request.

the flags were commented out, since only one option is functional, but They will be activated and used in the future. the regular usage will change.

Although the schemas layouts are being implemented manually, There is a thought in my head on how to make it generic and allow it to generate based on passed schemas. But this is just I thought for now. Don't get your hopes up. Or do, and help me implement.

### Testing

In the current state, there is no automated testing. Or test cases. Or any test suite.
I am testing manually, and things may not work in cases I don't actually tried before.

In the projecet you can see the sample file I am using to test.

### Documentation

It is also a work in progress, since I don't have anything ready yet.
Feel free to contribute

