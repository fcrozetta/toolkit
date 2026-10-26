from typing import Union
from pydantic import BaseModel

class Root_myObjectSchema(BaseModel):
    innerString: str
    innerNumber: float

class RootSchema(BaseModel):
    myString: str
    myNumber: float
    myObject: Root_myObjectSchema

