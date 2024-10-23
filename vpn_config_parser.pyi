from typing import Optional

class Constant():
    type: str
    key: str
    value: str

    def __init__(self, type: str, key: str, value: str) -> None:
        ...

class Declare():
    name: str
    constants: list[Constant]
    declarations: list[Declare]

    def __init__(self, name: str, constants: list[Constant], declarations: list[Declare]) -> None:
        ...

def parse_config(input: str) -> Optional[Declare]:
    ...
