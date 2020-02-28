#!/usr/bin/env python3
"""
This script pulls the font awesome spec from their repo on github,
then generates a rust version of it.
Taken from fontawesome-python and adapted to Rust
"""

import yaml
import requests
import sys


def main(uri, version, include_aliases):
    icons_dict = yaml.load(requests.get(uri).text)

    out = sys.stdout
    out.write("use std::collections::HashMap;\n")
    out.write("\n")
    out.write("lazy_static! {\n")
    out.write(
        "    // Icons list generated using a modified version of fontawesome-python\n"
    )
    out.write("    pub static ref ICONS: HashMap<String, char> = {\n")
    out.write("        let mut m = HashMap::new();\n")
    # Custom icons
    out.write("        m.insert(\"alacritty\".to_string(), '\\u{f120}');\n")
    for icon_name, icon in icons_dict.items():
        names = [icon_name]
        if include_aliases:
            terms = icon["search"]["terms"]
            if terms != None:
                names += terms
        for name in names:
            # dict entry with character code
            entry = "m.insert(\"%s\".to_string(), '\\u{%s}');" % (name, icon["unicode"])
            out.write("        " + entry + "\n")
    out.write("        m\n    };\n}\n")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(
        description="Generate icons.py, containing a python mapping for font awesome icons"
    )
    parser.add_argument(
        "--revision",
        help="Version of font of font awesome to download and use. Should correspond to a git branch name.",
        default="master",
    )
    parser.add_argument(
        "--include_aliases",
        help="If enabled, also adds aliases for icons in the output.",
        default=True,
    )
    args = parser.parse_args()

    REVISION = args.revision
    URI = (
        "https://raw.githubusercontent.com"
        "/FortAwesome/Font-Awesome/%s/metadata/icons.yml" % REVISION
    )

    main(URI, args.revision, args.include_aliases)
