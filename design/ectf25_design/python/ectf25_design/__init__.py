from . import rust

__doc__ = rust.__doc__
if hasattr(rust, "__all__"):
    __all__ = rust.__all__