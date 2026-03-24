#!/usr/bin/env python3
from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path


REQUIRED_FEATURES: dict[str, set[str]] = {
    "web-preview": {"wry"},
}


def discover_examples(examples_dir: Path) -> list[str]:
    names: list[str] = []
    for child in sorted(examples_dir.iterdir()):
        if child.is_dir() and (child / "main.rs").is_file():
            names.append(child.name)
    return names


def main() -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Run iced-shadcn examples one by one. "
            "Close the current example window to start the next one."
        )
    )
    parser.add_argument(
        "--features",
        default="",
        help="Comma-separated cargo features to pass, e.g. 'wry,rfd'",
    )
    parser.add_argument(
        "--start-from",
        default="",
        help="Start from this example name (inclusive).",
    )
    args = parser.parse_args()

    crate_dir = Path(__file__).resolve().parent.parent
    examples_dir = crate_dir / "examples"
    all_examples = discover_examples(examples_dir)
    if not all_examples:
        print("No examples found.")
        return 1

    feature_set = {f.strip() for f in args.features.split(",") if f.strip()}

    if args.start_from:
        if args.start_from not in all_examples:
            print(
                f"Example '{args.start_from}' not found. "
                f"Available: {', '.join(all_examples)}"
            )
            return 1
        start_idx = all_examples.index(args.start_from)
        all_examples = all_examples[start_idx:]

    runnable = all_examples

    print(f"Found {len(all_examples)} examples.")
    if feature_set:
        print(f"Global features: {sorted(feature_set)}")

    print(
        "Starting sequential run. "
        "After visual inspection, close each app window to continue.\n"
    )

    for i, example in enumerate(runnable, start=1):
        cmd = ["cargo", "run", "--example", example]
        example_features = set(feature_set)
        example_features.update(REQUIRED_FEATURES.get(example, set()))
        if example_features:
            cmd.extend(["--features", ",".join(sorted(example_features))])

        print(f"[{i}/{len(runnable)}] Running: {example}")
        print(f"Command: {' '.join(cmd)}")
        result = subprocess.run(cmd, cwd=crate_dir)
        if result.returncode != 0:
            print(f"Example '{example}' failed with exit code {result.returncode}.")
            answer = input("Continue with next example? [y/N]: ").strip().lower()
            if answer != "y":
                return result.returncode
        print("")

    print("All selected examples have been processed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
