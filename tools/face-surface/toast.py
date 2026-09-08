"""Static lowering of showCoronatioToast's construction, not a DOM runtime.

Read tag/attribute/text/append operations in source order. Reject unsupported
construction syntax rather than silently freezing a stale handwritten specimen.
Timer state is deliberately outside the declared pre-timer construction boundary.
"""
import ast
import html
import re


def lower(source, specimens):
    match = re.search(
        r"    function showCoronatioToast\(message, variant = ('[^']*')\) \{\n(.*?)\n    \}",
        source, re.S,
    )
    if not match:
        raise ValueError("showCoronatioToast constructor not located")
    default = ast.literal_eval(match[1])
    body = match[2]
    preamble = re.fullmatch(
        r"\s*const stack = document.querySelector\('\[data-coronatio-toast-stack\]'\);"
        r"\s*if \(!stack \|\| !message\) return;"
        r"\s*const allowed = (\[[^;]+\]);"
        r"\s*const resolvedVariant = allowed.includes\(variant\) \? variant : ('[^']*');"
        r"\s*const icons = (\{[^;]+\});\s*(const toast = .*?)\s*",
        body, re.S,
    )
    if not preamble:
        raise ValueError("Toast preamble changed: extend the static lowering explicitly")
    allowed = ast.literal_eval(preamble[1])
    fallback = ast.literal_eval(preamble[2])
    icons = ast.literal_eval(re.sub(r"\b(\w+)\s*:", r"'\1':", preamble[3]))
    # Split statements only outside string/template literals (including escapes).
    statements = []
    start, quote, escaped = 0, None, False
    for position, char in enumerate(preamble[4]):
        if quote:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == quote:
                quote = None
        elif char in "'\"`":
            quote = char
        elif char == ";":
            statements.append(preamble[4][start:position].strip())
            start = position + 1
    if quote or preamble[4][start:].strip():
        raise ValueError("Unterminated toast construction statement")

    results = []
    for specimen in specimens:
        variant = specimen.get("data-toast-variant", default)
        values = {"message": specimen["data-toast-message"],
                  "resolvedVariant": variant if variant in allowed else fallback}
        nodes, roots = {}, []

        def value(expression):
            expression = expression.strip()
            if expression in values:
                return values[expression]
            if expression.startswith("String(") and expression.endswith(")"):
                return str(value(expression[7:-1]))
            if expression == "icons[resolvedVariant]":
                return icons[values["resolvedVariant"]]
            if expression.startswith("`") and expression.endswith("`"):
                return re.sub(r"\$\{([^}]+)\}", lambda m: value(m[1]), expression[1:-1])
            if expression.startswith(("'", '"')):
                return ast.literal_eval(expression)
            raise ValueError(f"Unsupported toast expression: {expression}")

        for statement in statements:
            created = re.fullmatch(r"const (\w+) = document.createElement\((.*)\)", statement)
            assignment = re.fullmatch(r"(\w+)\.(className|textContent|dataset\.\w+) = (.*)", statement)
            attribute = re.fullmatch(r"(\w+)\.setAttribute\(('.*?'), (.*)\)", statement)
            append = re.fullmatch(r"(\w+)\.(append|appendChild)\(([^()]+)\)", statement)
            timer = re.fullmatch(r"startCoronatioToastTimer\(toast, 3000\)", statement)
            if created:
                nodes[created[1]] = {"tag": value(created[2]), "attrs": {}, "children": []}
            elif assignment:
                node, prop, expression = assignment.groups()
                if prop == "textContent":
                    nodes[node]["children"] = [html.escape(value(expression), quote=False)]
                else:
                    name = "class" if prop == "className" else "data-" + re.sub(
                        r"[A-Z]", lambda m: "-" + m[0].lower(), prop[8:])
                    nodes[node]["attrs"][name] = value(expression)
            elif attribute:
                nodes[attribute[1]]["attrs"][value(attribute[2])] = value(attribute[3])
            elif append:
                children = [nodes[name.strip()] for name in append[3].split(",")]
                if append[1] == "stack":
                    roots.extend(children)
                else:
                    nodes[append[1]]["children"].extend(children)
            elif timer:
                # The authorized bytes end before runtime timer/dataset mutations.
                continue
            else:
                raise ValueError(f"Unsupported toast construction: {statement}")

        def serialize(node):
            if isinstance(node, str):
                return node
            attrs = "".join(f' {key}="{html.escape(val, quote=True)}"'
                            for key, val in node["attrs"].items())
            return (f'<{node["tag"]}{attrs}>'
                    + "".join(serialize(child) for child in node["children"])
                    + f'</{node["tag"]}>')

        if len(roots) != 1:
            raise ValueError("Toast construction did not append exactly one root")
        results.append(serialize(roots[0]))
    if not results:
        raise ValueError("No source showcase toast messages found")
    first = source[:match.start()].count("\n") + 1
    last = source[:match.end()].count("\n")  # final construction line, before closing brace
    return "".join(results).encode(), f"src/bands/shell/document-4-tail.rs:{first}-{last}"
