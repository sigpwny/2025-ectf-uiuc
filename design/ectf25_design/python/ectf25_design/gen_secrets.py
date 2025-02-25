from .rust import gen_secrets
import argparse
from pathlib import Path

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--force",
        "-f",
        action="store_true",
        help="Force creation of secrets file, overwriting existing file",
    )
    parser.add_argument(
        "secrets_file",
        type=Path,
        help="Path to the secrets file to be created",
    )
    parser.add_argument(
        "channels",
        nargs="+",
        type=int,
        help="Supported channels. Channel 0 (broadcast) is always valid and will not"
        " be provided in this list",
    )
    return parser.parse_args()

def main():
    args = parse_args()
    secrets = gen_secrets(args.channels)
    with open(args.secrets_file, "wb" if args.force else "xb") as f:
        f.write(secrets)
    print(f"Wrote secrets to {str(args.secrets_file.absolute())}")

if __name__ == "__main__":
    main()