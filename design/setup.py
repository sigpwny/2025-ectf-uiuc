from setuptools import setup
from setuptools.command.install import install
import sys
import subprocess
import os
import glob

class InstallWithWheel(install):
    def run(self):
        # Get the Python version and platform
        py_version = f"cp{sys.version_info.major}{sys.version_info.minor}"
        platform = sys.platform
        arch = "win_amd64" if platform == "win32" else \
               "manylinux_x86_64" if "linux" in platform else \
               "macosx_10_13_x86_64"

        # Find the matching wheel
        wheels_dir = os.path.join(os.path.dirname(__file__), "wheels")
        matching_wheel = None
        # wheel files could either be .whl or .zip or .tar.gz
        wheel_files = glob.glob(os.path.join(wheels_dir, "*.whl")) + glob.glob(os.path.join(wheels_dir, "*.zip")) + glob.glob(os.path.join(wheels_dir, "*.tar.gz"))
        for wheel in wheel_files:
            if py_version in wheel and arch in wheel:
                matching_wheel = wheel
                break

        if matching_wheel:
            print(f"Installing {matching_wheel}")
            subprocess.check_call([sys.executable, "-m", "pip", "install", matching_wheel])
        else:
            raise RuntimeError("No compatible wheel found for this platform.")

        install.run(self)

setup(
    name="ectf25_design",
    version="1.0.0+uiuc",
    packages=["ectf25_design"],
    cmdclass={"install": InstallWithWheel},
    include_package_data=True,
)
