#!/usr/bin/env python3

from __future__ import annotations

import csv
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
RAW = ROOT / "results" / "raw" / "vector_add"
OUT = ROOT / "results" / "processed"

EXPECTED_CHECKSUM = "16760315880.000"

KEY_VALUE_RE = re.compile(r"^([^=]+)=(.*)$")

REQUIRED_FIELDS = {
    "kernel",
    "language",
    "implementation",
    "elements",
    "warmup",
    "runs",
    "total_ns",
    "min_ns",
    "median_ns",
    "checksum",
}

EXPECTED_IMPLEMENTATIONS = {
    ("rust", "automatic"),
    ("rust", "explicit_simd"),
    ("zig", "scalar"),
    ("zig", "explicit_simd"),
}


def parse_result(path: Path) -> dict[str, str]:
    text = path.read_text().strip()

    if not text:
        raise ValueError(f"{path}: empty benchmark output")

    result: dict[str, str] = {}

    for raw_line in text.splitlines():
        line = raw_line.strip()

        if not line:
            continue

        match = KEY_VALUE_RE.match(line)

        if not match:
            raise ValueError(
                f"{path}: malformed line: {line!r}"
            )

        key, value = match.groups()

        if key in result:
            raise ValueError(
                f"{path}: duplicate field: {key!r}"
            )

        result[key] = value

    missing = REQUIRED_FIELDS - result.keys()

    if missing:
        raise ValueError(
            f"{path}: missing fields: {sorted(missing)}"
        )

    if result["kernel"] != "vector_add":
        raise ValueError(
            f"{path}: unexpected kernel {result['kernel']!r}"
        )

    implementation = (
        result["language"],
        result["implementation"],
    )

    if implementation not in EXPECTED_IMPLEMENTATIONS:
        raise ValueError(
            f"{path}: unexpected implementation {implementation}"
        )

    if result["checksum"] != EXPECTED_CHECKSUM:
        raise ValueError(
            f"{path}: checksum mismatch: "
            f"{result['checksum']} != {EXPECTED_CHECKSUM}"
        )

    for field in (
        "elements",
        "warmup",
        "runs",
        "total_ns",
        "min_ns",
        "median_ns",
    ):
        try:
            value = int(result[field])
        except ValueError as exc:
            raise ValueError(
                f"{path}: {field} is not an integer: "
                f"{result[field]!r}"
            ) from exc

        if value < 0:
            raise ValueError(
                f"{path}: {field} cannot be negative"
            )

    if int(result["runs"]) == 0:
        raise ValueError(
            f"{path}: runs must be greater than zero"
        )

    return result


def main() -> None:
    if not RAW.exists():
        raise SystemExit(
            f"Raw benchmark directory does not exist: {RAW}"
        )

    round_dirs = sorted(
        path
        for path in RAW.iterdir()
        if path.is_dir() and path.name.startswith("round-")
    )

    if not round_dirs:
        raise SystemExit("No benchmark rounds found.")

    rows: list[dict[str, object]] = []

    for round_dir in round_dirs:
        try:
            round_number = int(
                round_dir.name.split("-", 1)[1]
            )
        except ValueError as exc:
            raise ValueError(
                f"Invalid round directory: {round_dir.name}"
            ) from exc

        result_files = sorted(round_dir.glob("*.txt"))

        if not result_files:
            raise ValueError(
                f"{round_dir}: no benchmark result files found"
            )

        seen: set[tuple[str, str]] = set()

        for result_path in result_files:
            result = parse_result(result_path)

            key = (
                result["language"],
                result["implementation"],
            )

            if key in seen:
                raise ValueError(
                    f"{round_dir}: duplicate implementation: {key}"
                )

            seen.add(key)

            rows.append({
                "round": round_number,
                "kernel": result["kernel"],
                "language": result["language"],
                "implementation": result["implementation"],
                "elements": int(result["elements"]),
                "warmup": int(result["warmup"]),
                "runs": int(result["runs"]),
                "total_ns": int(result["total_ns"]),
                "min_ns": int(result["min_ns"]),
                "median_ns": int(result["median_ns"]),
                "checksum": result["checksum"],
            })

        if seen != EXPECTED_IMPLEMENTATIONS:
            raise ValueError(
                f"{round_dir}: expected implementations "
                f"{EXPECTED_IMPLEMENTATIONS}, got {seen}"
            )

    OUT.mkdir(parents=True, exist_ok=True)

    output_path = OUT / "vector_add.csv"

    fieldnames = [
        "round",
        "kernel",
        "language",
        "implementation",
        "elements",
        "warmup",
        "runs",
        "total_ns",
        "min_ns",
        "median_ns",
        "checksum",
    ]

    with output_path.open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=fieldnames,
        )
        writer.writeheader()
        writer.writerows(rows)

    print(
        f"Validation: PASS\n"
        f"Rounds: {len(round_dirs)}\n"
        f"Observations: {len(rows)}\n"
        f"Output: {output_path}"
    )


if __name__ == "__main__":
    main()
