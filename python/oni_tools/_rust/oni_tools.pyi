"""
Python interface for the oni-tools crate.
"""

def geyser_rate(
    geyser_type: str | float,
    eruption_rate: str | float,
    eruption_duration: str | float,
    eruption_period: str | float,
    active_duration: str | float,
    active_period: str | float,
) -> str | float: ...

def sum_as_string(a: int, b: int) -> str: ...
