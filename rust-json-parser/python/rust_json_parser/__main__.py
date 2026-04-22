import sys
import os
import json
import random
import uuid

from rust_json_parser._rust_json_parser import (
    parse_json,
    parse_json_file,
    dumps,
    benchmark_performance,
)


def generate_test_json(size_bytes: int) -> str:
    """
    Generates a valid JSON string of at least the specified size.
    Useful for 'Medium' and 'Large' benchmarking categories.
    """
    data = {
        "metadata": {"source": "benchmarking_tool", "type": "sequential_records"},
        "records": [],
    }

    while True:
        current_json = json.dumps(data)
        if len(current_json.encode("utf-8")) >= size_bytes:
            break

        record = {
            "id": len(data["records"]),
            "uuid": str(uuid.uuid4()),
            "value": random.random(),
            "payload": "X" * 100,  # Constant padding to grow size predictably
            "tags": ["bench", "test", "rust-integration"],
        }
        data["records"].append(record)

    return json.dumps(data)


def run_benchmark(size: str):
    match size:
        case "small":
            rust_time, python_json_time, simplejson_time = benchmark_performance(
                '{"key": "value"}'
            )

        case "medium":
            json_data = generate_test_json(1024)
            rust_time, python_json_time, simplejson_time = benchmark_performance(
                json_data
            )

        case "large":
            json_data = generate_test_json(100 * 1024)
            rust_time, python_json_time, simplejson_time = benchmark_performance(
                json_data
            )

    json_speedup = python_json_time / rust_time
    simplejson_speedup = simplejson_time / rust_time

    json_speedup_word = "faster" if json_speedup >= 1 else "slower"
    simplejson_speedup_word = "faster" if simplejson_speedup >= 1 else "slower"

    print(f"Rust:             {rust_time:.6f}s")
    print(
        f"Python json (C):  {python_json_time:.6f}s  (Rust is {json_speedup:.2f}x {json_speedup_word})"
    )
    print(
        f"simplejson:       {simplejson_time:.6f}s  (Rust is {simplejson_speedup:.2f}x {simplejson_speedup_word})"
    )


def main():
    if "--benchmark" in sys.argv:
        run_benchmark("small")
        run_benchmark("medium")
        run_benchmark("large")

    if len(sys.argv) > 1:
        match len(sys.argv):
            case 2:
                json = sys.argv[1]
                indent = None

            case 3:
                json = sys.argv[1]
                indent = int(sys.argv[2])
                if indent < 0:
                    raise ValueError("Indent must be greater than 0")

        if os.path.exists(json):
            try:
                data = parse_json_file(json)
                print(dumps(data, indent))
            except IOError as err:
                print(f"IO Error: {err}")
        else:
            try:
                data = parse_json(json)
                print(dumps(data, indent))
            except ValueError as err:
                print(f"Invalid JSON: {err}")
    else:
        data = sys.stdin.read()
        try:
            data = parse_json(data)
            print(dumps(data))
        except ValueError as err:
            print(f"Invalid JSON: {err}")


main()
