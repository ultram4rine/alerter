#!/usr/bin/env python3

import argparse
import re
from datetime import datetime, timezone

parser = argparse.ArgumentParser(
    description="Script for transforming 'keep a changelog' to 'debian/changelog'."
)
parser.add_argument("-i", "--input", type=str, default="CHANGELOG.md",
                    help="Input changelog in 'keep a changelog' format. Default: 'CHANGELOG.md'")
parser.add_argument("-o", "--output", type=str,
                    help="Output file.", required=True)
parser.add_argument('-p', "--package-name", type=str,
                    help="Name of package.", required=True)
parser.add_argument("-a", "--author", type=str,
                    help="Author's name.", required=True)
parser.add_argument("-e", "--email", type=str,
                    help="Author's email.", required=True)
args = parser.parse_args()

changelog_file = open(args.input, "r")
debian_changelog_file = open(args.output, "w")
package_name = args.package_name
author = args.author
email = args.email


def remove_md_link(line: str):
    return re.sub(r"(.*)\[(.*)]\(.*\)(.*)", r"\1\2\3", line)


lines = changelog_file.readlines()
first_ver_entry = True  # Flag for first version occurence
ver_and_date = []
for line in lines:
    if re.match("^## \[\d.*]", line):
        if first_ver_entry:
            first_ver_entry = False
        else:
            new_line = "\n -- {} <{}>  {}\n\n".format(author, email,
                                                      datetime.strptime(ver_and_date[1], "%Y-%m-%d").replace(tzinfo=timezone.utc).strftime("%a, %d %b %Y %H:%M:%S %z"))
            debian_changelog_file.write(new_line)

        ver_and_date = line[3:].strip().split(" - ")
        new_line = "{} ({}) unstable; urgency=medium\n\n".format(package_name,
                                                                 ver_and_date[0].replace('[', '').replace(']', ''))
        debian_changelog_file.write(new_line)
    elif re.match("^- .*", line):
        new_line = "  {}\n".format(
            remove_md_link(line).strip().replace("- ", "* ", 1))
        debian_changelog_file.write(new_line)

new_line = "\n -- {} <{}>  {}\n".format(author, email,
                                        datetime.strptime(ver_and_date[1], "%Y-%m-%d").replace(tzinfo=timezone.utc).strftime("%a, %d %b %Y %H:%M:%S %z"))
debian_changelog_file.write(new_line)
