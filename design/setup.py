from setuptools import setup
import sys
import subprocess
import os

# Installs the ectf25_design prebuilt wheel
def install_wheel():
    wheels_dir = os.path.join(os.path.dirname(__file__), "wheels")
    subprocess.check_call([
        sys.executable, "-m", "pip", "install", "--no-index", "--force-reinstall",
        "--find-links", wheels_dir, "ectf25_design"
    ])

install_wheel()

setup(
    name="ectf25_design_wrapper",
    version="1.0.0+uiuc",
    packages=["wrapper"],
    include_package_data=True,
)