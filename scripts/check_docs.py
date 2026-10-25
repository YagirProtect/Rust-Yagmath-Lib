"""Validate local documentation links, explicit/heading anchors, and SVG XML.

Run from any working directory with Python 3.10+. No dependencies or network.
Rust code blocks are separately validated by cargo test --doc.
"""

from pathlib import Path
import re
from urllib.parse import unquote, urlsplit
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[1]


def anchors(content: str) -> set[str]:
    result = set(re.findall(r'<a\s+id="([^"]+)"', content))
    headings: dict[str, int] = {}
    for heading in re.findall(r"^#{1,6}\s+(.+)$", content, re.MULTILINE):
        slug = re.sub(r"[^\w\- ]", "", heading.lower()).replace(" ", "-")
        count = headings.get(slug, 0)
        result.add(slug if count == 0 else f"{slug}-{count}")
        headings[slug] = count + 1
    return result


def main() -> int:
    pages = [ROOT / "README.md", *sorted((ROOT / "docs").rglob("*.md"))]
    errors: list[str] = []
    checked = 0
    for page in pages:
        content = page.read_text(encoding="utf-8-sig")
        for match in re.finditer(r"!?\[[^\]]*\]\(([^\s)]+)\)", content):
            target = urlsplit(match.group(1))
            if target.scheme or target.netloc:
                continue
            checked += 1
            destination = page.parent / unquote(target.path) if target.path else page
            line = content.count("\n", 0, match.start()) + 1
            label = f"{page.relative_to(ROOT)}:{line}: {match.group(1)}"
            if not destination.is_file():
                errors.append(f"Missing file: {label}")
            elif target.fragment and destination.suffix == ".md":
                if unquote(target.fragment) not in anchors(destination.read_text(encoding="utf-8-sig")):
                    errors.append(f"Missing anchor: {label}")

    assets = sorted((ROOT / "docs" / "assets").glob("*.svg"))
    for asset in assets:
        try:
            tree = ET.parse(asset)
            root = tree.getroot()
            if root.tag != "{http://www.w3.org/2000/svg}svg":
                errors.append(f"Not an SVG document: {asset.name}")
            ids = [node.attrib["id"] for node in root.iter() if "id" in node.attrib]
            if len(ids) != len(set(ids)):
                errors.append(f"Duplicate SVG IDs: {asset.name}")
        except ET.ParseError as error:
            errors.append(f"Invalid SVG {asset.name}: {error}")

    for error in errors:
        print(error)
    print(f"Checked {len(pages)} Markdown pages, {checked} local links, {len(assets)} SVG files.")
    if errors:
        print(f"FAILED: {len(errors)} documentation errors.")
        return 1
    print("Documentation links and SVG XML are valid.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
