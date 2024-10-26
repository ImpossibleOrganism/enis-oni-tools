from numbers import Number
from collections.abc import Mapping
from functools import singledispatchmethod


class Reaction(dict, Number):
    ATOL = 0.1

    # --- Reaction Math ----------------------------------------------------------------
    @singledispatchmethod
    def __add__(self, other):
        raise NotImplementedError(
            f"Cannot add '{type(self).__qualname__}' and '{type(other).__qualname__}'."
        )

    @__add__.register(Mapping)
    def _(self, other):
        return type(self)(
            {
                key: self.get(key, 0.0) + other.get(key, 0.0)
                for key in self.keys() | other.keys()
            }
        )

    # TODO: rmul and radd aren't getting called when I do 2 * reaction because
    #  int.__mul__ raises a TypeError instead of NotImplemented.
    def __radd__(self, other):
        return other.__add__(self)

    @singledispatchmethod
    def __mul__(self, other):
        raise NotImplementedError(
            f"Cannot multiply '{type(self).__qualname__}' and "
            f"'{type(other).__qualname__}'."
        )

    def __rmul__(self, other):
        return other.__mul__(self)

    @__mul__.register(Number)
    def _(self, other):
        return type(self)({key: value * other for key, value in self.items()})

    # --- Representation ---------------------------------------------------------------
    def __format__(self, format_spec: str):
        match format_spec:
            case "":
                return str(self)
            case "n":
                return self._format_in_lines()
            case _:
                raise TypeError(
                    f"Unsupported format string '{format_spec}' passed to "
                    f"{self.__format__.__qualname__}"
                )

    def _get_string_groups(self):
        """Get a string representation of each element with its quantity, organized into
        the left and right hand of the reaction."""

        left_strings = []
        right_strings = []

        for element, amount in self.items():
            if amount > self.ATOL:
                strings = right_strings
            elif amount < -self.ATOL:
                strings = left_strings
            else:
                continue

            strings.append(f"{abs(amount):.1f} {element}")

        return left_strings, right_strings

    def _format_in_lines(self):
        """Format the reaction to have each element with its quantity on its own
        line."""
        left, right = self._get_string_groups()
        return "\n".join([f"- {st}" for st in left] + [f"+ {st}" for st in right])

    def __str__(self):
        left, right = self._get_string_groups()
        return f"{" + ".join(left)} -> {" + ".join(right)}"
