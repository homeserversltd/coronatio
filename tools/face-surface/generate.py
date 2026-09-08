#!/usr/bin/env python3
"""Regenerate the crown's frozen face from its own offline Rust renderer."""
import argparse
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile

sys.dont_write_bytecode = True
from toast import lower as lower_toast

ROOT = Path(__file__).resolve().parents[2]
UX = Path("src/bands/shell/ux")
# Extraction selectors, not markup copies. Select all top-level specimens of the
# primitive in its dedicated showcase category, preserving production order.
SELECTORS = {
    "badge": ("badges", {"ui-badge"}),
    "breadcrumbs": ("upload-components", {"ui-breadcrumbs"}),
    "button": ("buttons", {"ui-button"}),
    "calendar": ("calendar-time", {"calendar-picker", "ui-calendar-monthly"}),
    "card": ("cards", {"ui-card"}),
    "checkbox": ("checkboxes", {"ui-checkbox"}),
    "collapsible": ("collapsible", {"collapsible"}),
    "editable-field": ("utilities", {"ui-editable-field"}),
    "file-input": ("upload-components", {"ui-file-input"}),
    "icon-button": ("upload-components", {"ui-icon-button"}),
    "input": ("inputs", {"ui-input-label", "ui-input"}),
    "ip-calendar": ("lan-ip-calendar", {"ui-lan-ip-calendar"}),
    "loading-spinner": ("loading-spinner", {"loading-spinner"}),
    "modal": ("modals", {"modal-overlay"}),
    "plus-button": ("utilities", {"ui-plus-button"}),
    "progress-bar": ("progress-bar", {"ui-progress-bar"}),
    "row-info-tile": ("row-info-tile", {"row-info-tile"}),
    "select": ("dropdowns", {"ui-select-label"}),
    "slider": ("slider", {"ui-slider__container"}),
    "table": ("table", {"ui-table-container"}),
    "tabs": ("tabs", {"ui-tab-group"}),
    "text-box": ("textbox", {"ui-text-box"}),
    "time-picker": ("calendar-time", {"time-picker"}),
    "toggle": ("toggles", {"ui-toggle"}),
    "visibility-toggle": ("visibility-system", {"ui-visibility-toggle"}),
}
VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link",
        "meta", "param", "source", "track", "wbr"}


class Elements(HTMLParser):
    """Locate source ranges without HTML normalization or attribute reordering."""
    def __init__(self, text):
        super().__init__(convert_charrefs=False)
        self.text, self.nodes, self.stack = text, [], []
        self.unmatched = []
        self.lines = [0]
        self.lines.extend(match.end() for match in re.finditer("\n", text))
        self.feed(text)
        self.close()
        # Unrelated domain-pack markup may contain unmatched closing tags. Keep
        # their offsets; only balanced, untouched selected source ranges may freeze.

    def position(self):
        line, column = self.getpos()
        return self.lines[line - 1] + column

    def handle_starttag(self, tag, attrs):
        parent = self.stack[-1] if self.stack else None
        node = {"tag": tag, "attrs": dict(attrs), "start": self.position(),
                "parent": parent, "end": None}
        self.nodes.append(node)
        if tag in VOID:
            source_tag = self.get_starttag_text()
            if source_tag is None:
                raise ValueError("Missing original start tag")
            node["end"] = self.position() + len(source_tag)
        else:
            self.stack.append(node)

    def handle_startendtag(self, tag, attrs):
        self.handle_starttag(tag, attrs)
        if tag not in VOID:
            node = self.stack.pop()
            source_tag = self.get_starttag_text()
            if source_tag is None:
                raise ValueError("Missing original start tag")
            node["end"] = self.position() + len(source_tag)

    def handle_endtag(self, tag):
        if not self.stack or self.stack[-1]["tag"] != tag:
            self.unmatched.append(self.position())
            return
        node = self.stack.pop()
        node["end"] = self.text.index(">", self.position()) + 1

    def category(self, node):
        while node:
            if "data-og-category-section" in node["attrs"]:
                return node["attrs"]["data-og-category-section"]
            node = node["parent"]
        return None

    def extract(self, category, classes):
        chosen = []
        for node in self.nodes:
            if (self.category(node) == category
                    and classes.intersection(node["attrs"].get("class", "").split())):
                if node["end"] is None or any(node["start"] <= pos < node["end"] for pos in self.unmatched):
                    raise ValueError(f"Unbalanced selected primitive in {category}")
                if not any(start <= node["start"] < end for start, end in chosen):
                    chosen.append((node["start"], node["end"]))
        if not chosen:
            raise ValueError(f"No showcase specimens for {category}: {sorted(classes)}")
        return "".join(self.text[start:end] for start, end in chosen).encode()


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args])


def snapshot(sha, paths):
    # Batch object reads avoid revision/ref-name ambiguity and one process per file.
    queries = [sha] + [f"{sha}:{path}" for path in paths]
    result = subprocess.run(["git", "-C", str(ROOT), "cat-file", "--batch"],
                            input=("\n".join(queries) + "\n").encode(),
                            capture_output=True, check=True)
    data, offset, files = result.stdout, 0, {}
    for number, query in enumerate(queries):
        end = data.index(b"\n", offset)
        header = data[offset:end].decode().split()
        if len(header) != 3:
            raise ValueError(f"Source object missing: {query}")
        oid, kind, size = header
        size = int(size)
        content = data[end + 1:end + 1 + size]
        offset = end + 2 + size
        if number == 0:
            if oid != sha or kind != "commit":
                raise ValueError(f"Not the exact source commit: {sha}")
        elif kind == "blob":
            files[paths[number - 1]] = content
        else:
            raise ValueError(f"Source input is not a blob: {query}")
    return files


def json_bytes(value):
    return (json.dumps(value, ensure_ascii=False, indent=2) + "\n").encode()


def source_identity(requested, index, primitives):
    """Pin a committed input snapshot, not the commit containing its own hash.

    Retain the previous pin only while every renderer/input byte still matches
    that commit. The metadata-only ux artifacts membership is not CSS input.
    A dirty input requires a source commit before generating its publication.
    """
    paths = ["src/bands/shell/test.rs", "src/bands/shell/test-animations.rs",
             "src/bands/shell/document-4-tail.rs", str(UX / "author-face.json")]
    paths += [str(UX / child) for child in index["children"]
              if child.startswith("library/")]
    surface_path = ROOT / UX / "face-surface.json"
    old = json.loads(surface_path.read_text()) if surface_path.exists() else {}
    candidates = [requested] if requested else [old.get("source_sha"), git("rev-parse", "HEAD").decode().strip()]
    for sha in candidates:
        if not sha or not re.fullmatch(r"[0-9a-f]{40}", sha):
            continue
        try:
            committed = snapshot(sha, [str(UX / "index.json"), *paths])
            old_index = json.loads(committed[str(UX / "index.json")])
            old_primitives = library_primitives(old_index)
            if old_primitives != primitives or old_index["children"] != index["children"]:
                continue
            if all(committed[path] == (ROOT / path).read_bytes() for path in paths):
                return sha
        except (ValueError, subprocess.CalledProcessError):
            continue
    raise ValueError("No committed source snapshot matches the rendering inputs; commit inputs first, then regenerate")


def library_primitives(index):
    names = []
    for child in index["children"]:
        if child.startswith("library/"):
            match = re.fullmatch(r"library/_([a-z0-9-]+)\.css", child)
            if not match:
                raise ValueError(f"Unsupported library entry: {child}")
            names.append(match[1])
    if len(names) != len(set(names)):
        raise ValueError("Duplicate library primitives")
    return sorted(names)


def generate(source_sha):
    index = json.loads((ROOT / UX / "index.json").read_text())
    primitives = library_primitives(index)
    if set(primitives) != set(SELECTORS) | {"toast"}:
        raise ValueError("Library membership changed: add/remove its source extraction selector")
    author = json.loads((ROOT / UX / "author-face.json").read_text())
    sha = source_identity(source_sha, index, primitives)
    # No network, server startup, JS execution, Cargo target, or runtime route.
    target = ROOT / "target"
    target.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="face-surface-", dir=target) as scratch:
        renderer = Path(scratch) / "render"
        subprocess.run(["rustc", "--edition=2021", str(Path(__file__).with_name("render.rs")),
                        "-o", str(renderer)], check=True, cwd=ROOT)
        rendered = subprocess.check_output([str(renderer)], cwd=ROOT).decode()
    elements = Elements(rendered)
    outputs, showcase = {}, {}
    for primitive in primitives:
        path = UX / "showcase" / f"{primitive}.html"
        entry = {"path": str(path)}
        if primitive == "toast":
            specimens = [node["attrs"] for node in elements.nodes
                         if elements.category(node) == "toasts"
                         and "data-coronatio-toast-spawn" in node["attrs"]]
            markup, constructor = lower_toast(
                (ROOT / "src/bands/shell/document-4-tail.rs").read_text(), specimens)
            entry.update(origin="browser-constructed", constructor=constructor)
        else:
            markup = elements.extract(*SELECTORS[primitive])
        entry["sha256"] = hashlib.sha256(markup).hexdigest()
        outputs[path] = markup
        showcase[primitive] = entry
    outputs[UX / "showcase/index.json"] = json_bytes({
        "children": [f"{primitive}.html" for primitive in primitives],
        "authority": "Source-rendered primitive specimens in production order; see tools/face-surface/README.md."
    })
    # Preserve unknown instance fields while replacing the fields this producer owns.
    surface_path = ROOT / UX / "face-surface.json"
    surface = json.loads(surface_path.read_text()) if surface_path.exists() else {}
    previous = surface.get("showcase", {})
    for name, entry in showcase.items():
        merged = dict(previous.get(name, {}))
        merged.update(entry)
        showcase[name] = merged
    surface.update(schema="coronatio.face-surface.v1", face="Coronatio", source_sha=sha,
                   primitives=primitives, author_face=sorted(author["variables"]), showcase=showcase)
    outputs[UX / "face-surface.json"] = json_bytes(surface)
    return outputs, sha


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Compare generated bytes without replacing artifacts")
    parser.add_argument("--source-sha", help="Committed rendering-input snapshot; otherwise retain a matching pin or use HEAD")
    args = parser.parse_args()
    outputs, sha = generate(args.source_sha)
    different = []
    for path, content in outputs.items():
        destination = ROOT / path
        if not destination.exists() or destination.read_bytes() != content:
            different.append(str(path))
            if not args.check:
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(content)
    # Never silently delete a predecessor's frozen output on library removal.
    stale = sorted(str(path.relative_to(ROOT)) for path in (ROOT / UX / "showcase").glob("*.html")
                   if path.relative_to(ROOT) not in outputs)
    print(json.dumps({"source_sha": sha, "artifacts": len(outputs),
                      "different": different, "stale": stale, "check": args.check}))
    return 1 if stale or (args.check and different) else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, subprocess.CalledProcessError) as error:
        sys.exit(str(error))
