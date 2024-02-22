import json
import typer, rich
from typing_extensions import Annotated
from rich.console import Console
from rich.table import Table


def input_has_keys(input_keys: set, target_keys: set):
    table = Table(title="JSON Contain keys", show_lines=True)
    table.add_column("Json Keys", justify="left", style="cyan", no_wrap=True)
    table.add_column("Target Keys", justify="left", style="magenta")
    all_keys = input_keys.union(target_keys)

    for k in all_keys:
        a = k if k in input_keys else "-"
        b = k if k in target_keys else "-"
        style = "green" if a != "-" and b != "-" else "red"
        table.add_row(
            a,
            b,
            style=style,
        )

    console = Console()
    console.print(table)


def main(
    file: Annotated[
        str, typer.Argument(help="Json Input Filename", rich_help_panel="HELLO")
    ],
    has_keys: Annotated[
        str,
        typer.Option(help="Check if input has all the keys in this json"),
    ] = "",
):
    json_input = json.load(open(file, "r"))

    if has_keys:
        json_keys = set(json.load(open(has_keys, "r")).keys())
        input_has_keys(set(json_input.keys()), json_keys)


if __name__ == "__main__":
    typer.run(main)
