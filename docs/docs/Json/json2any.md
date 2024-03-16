---
tags:
    - python
    - wip
---

# Json2any

## Quickstart

```bash
json2any input.json > out.json
```

## Original concept

The original idea of this aplicattion was to convert a json file and have the output as a sample that redacts all the data.

Running the command:

```bash
json2any input.json --format sample
```

=== "Input"
    ```json linenums="1" title="input.json"
        --8<-- "docs/file_samples/json2any/input_json.json"
    ```
=== "Output"
    ```json linenums="1"
        --8<-- "docs/file_samples/json2any/output_sample_json.json"
    ```


## Extending the output format

After the first usage, I realized I could use the "engine" that loops the file to generate different outputs, like pydantic classes.

This, however, create a challenge of settings default values for different types of data, and parsing different types of strings into their corresponding python data types, like dates, timestamps, UUIDs, etc...

On top of all of this, it is also necessary to understand that repeated items should not be considered in some cases. here we go creating flags for all of this :rofl:
## Output Formats
### Sample

This is the default format used by the application. It will create a sample of the json, converting all data to the default values.

### Pydantic
WIP