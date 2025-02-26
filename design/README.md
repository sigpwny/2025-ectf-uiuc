# ectf25_design_wrapper
This top-level folder for the `ectf25_design` Python package contains the precompiled wheels for the `ectf25_design` package. The wheels are stored in the `wheels` folder and the `setup.py` file is used to install the correct wheel for the current system.

Wheels are built using CI/CD on GitHub Actions via this [workflow file](../.github/workflows/ci-python.yml) and pushed to the main branch.

The `ectf25_design` folder contains the Rust source code for the actual package. See the [README](ectf25_design/README.md) for more information on how to develop and build the package.