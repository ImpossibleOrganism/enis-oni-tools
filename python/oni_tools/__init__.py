import importlib.metadata

from oni_tools._rust.oni_tools import (
    sum_as_string,
)

__version__ = importlib.metadata.version("oni-tools")

__all__ = [
    "sum_as_string",
]
