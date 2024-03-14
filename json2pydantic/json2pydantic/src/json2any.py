from typing_extensions import Annotated
from enum import StrEnum
import json
import typer
from rich import print
from rich.console import Console

app = typer.Typer()
console = Console()


class Exporters(StrEnum):
    SAMPLE = ("sample",)
    PYDANTIC = "pydantic"


def get_type_sample(value):
    if type(value) == str:
        return "string"
    if type(value) == int:
        return 1
    if type(value) == float:
        return 2.3
    if type(value) == dict:
        return {k: get_type_sample(v) for k, v in value.items()}
    if type(value) == type([]):
        return [get_type_sample(x) for x in value]
    return value


@app.command()
def convert(
    input_file: str, format: Annotated[Exporters, typer.Option()] = Exporters.SAMPLE
):
    """Convert input file to specified output"""
    with open(input_file, "r") as f:
        data: dict = json.load(f)
        if format == Exporters.SAMPLE:
            print(json.dumps(get_type_sample(data), indent=4))
        if format == Exporters.PYDANTIC:
            print("[bold red] NOT IMPLEMENTED YET")


if __name__ == "__main__":
    app()
