# ectf25_design
This is a Python `pip` installable package which contains the deployment and encoder functionality for UIUC's eCTF 2025 design.

This package is written in Rust, using [PyO3](https://github.com/PyO3/pyo3) to create Python bindings and [maturin](https://github.com/PyO3/maturin) to build the package. This allows us to write the performance-critical parts of the code in Rust, while still providing the required Python interfaces for the eCTF host tools.

## Developing
To develop this package, you will need to have Rust and Python installed. You will also need to use Python virtual environments.

```sh
python -m venv .venv
source .venv/bin/activate
pip install maturin
```

Make sure to activate the virtual environment before running any of the commands below.

```sh
maturin develop
```

This will install the package directly into your virtual environment, allowing you to quickly run `python` and use the package immediately.

## Building
Since the Python package is expected to be installed across multiple platforms and architectures, we will need to compile the Rust code for each target (e.g. `aarch64-apple-darwin` for Apple silicon Macs, `x86_64-unknown-linux-gnu` for most Linux systems). This is done automatically using GitHub Actions, but you can also do it locally with Docker.

```sh
maturin build
```

## Python Interfaces
Below are the Python interfaces for the package. These are the functions that will be called by the eCTF host tools.

### Generate Secrets

```py
from ectf25_design.gen_secrets import gen_secrets

def gen_secrets(channels: list[int]) -> bytes:
    pass
```

### Generate Subscription

```py
from ectf25_design.gen_subscription import gen_subscription

def gen_subscription(
    secrets: bytes, device_id: int, start: int, end: int, channel: int
) -> bytes:
    pass
```

### Encoder

```py
from ectf25_design.encoder import Encoder

class Encoder:
  def __init__(self, secrets: bytes):
      pass

  def encode(self, channel: int, frame: bytes, timestamp: int) -> bytes:
      pass
```