#!./venv/bin/python
import enum
import optparse
import os

import screeninfo


class CompilationFeature(enum.StrEnum):
    HEADSET_ATTACHED = "headset_attached"


def has_headset_attached() -> bool:
    """
    Check if a headset is attached to the computer (screen 800x480px is present)
    :return: True if a headset is attached, False otherwise
    """

    for screen in screeninfo.get_monitors():
        if screen.width == 800 and screen.height == 480:
            return True
    return False


def main():
    """
    CLI Main function

    Options:
    -b --build: Create a production build
    -A --attached: Force headset-attached mode (default: False)
    -d --dry-run: Do not run the command, just print it
    -h --help: Show help
    """

    parser = optparse.OptionParser()
    parser.add_option("-b", "--build", action="store_true", dest="build", default=False)
    parser.add_option("-A", "--attached", action="store_true", dest="attached", default=False)
    parser.add_option("-d", "--dry-run", action="store_true", dest="dry_run", default=False)
    options, args = parser.parse_args()

    features: list[CompilationFeature] = []

    if options.attached or has_headset_attached():
        features.append(CompilationFeature.HEADSET_ATTACHED)

    cargo_command = ["cargo", "run"]
    if options.build:
        cargo_command = ["cargo", "build", "--release"]

    if features:
        cargo_command.extend(["--features", ",".join(features)])

    print(' '.join(cargo_command))
    if not options.dry_run:
        os.system(' '.join(cargo_command))


if __name__ == "__main__":
    main()
