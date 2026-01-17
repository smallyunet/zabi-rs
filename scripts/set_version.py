#!/usr/bin/env python3
import sys
import re
import os

def update_file(path, pattern, replacement):
    with open(path, 'r') as f:
        content = f.read()
    
    new_content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
    
    if content == new_content:
        print(f"Warning: No changes made to {path} with pattern {pattern}")
    else:
        print(f"Updated {path}")

    with open(path, 'w') as f:
        f.write(new_content)

def main():
    if len(sys.argv) != 2:
        print("Usage: set_version.py <version>")
        sys.exit(1)

    version = sys.argv[1]
    # Strip 'v' prefix if present
    if version.startswith('v'):
        version = version[1:]

    print(f"Setting version to {version}")

    # 1. Update zabi-derive version
    # Matches: version = "..." inside [package] usually at top, but regex needs to be reasonably specific
    # We look for ^version = "..."
    update_file(
        'zabi-derive/Cargo.toml',
        r'^version = ".*"',
        f'version = "{version}"'
    )

    # 2. Update zabi-rs version
    update_file(
        'Cargo.toml',
        r'^version = ".*"',
        f'version = "{version}"'
    )

    # 3. Update zabi-derive dependency in zabi-rs Cargo.toml
    # Matches: zabi-derive = { path = "./zabi-derive", version = "...", ... }
    # We replace the version part.
    update_file(
        'Cargo.toml',
        r'(zabi-derive = \{[^}]*version = ")[^"]*(")',
        f'\\g<1>{version}\\g<2>'
    )

if __name__ == '__main__':
    main()
