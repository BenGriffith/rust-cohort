import sys
import os
import json
import random
import uuid
import string

from rust_json_parser._rust_json_parser import (
    parse_json,
    parse_json_file,
    dumps,
    benchmark_performance,
)


def generate_test_json(size_bytes: int) -> str:
    """
    Generates a randomized, valid JSON string of at least size_bytes.
    Randomizes keys, values, types, and nesting.
    """
    data = {
        "metadata": {
            "source": "".join(random.choices(string.ascii_letters, k=10)),
            "seed": random.randint(0, 1000000),
            "tags": random.sample(
                ["bench", "test", "rust", "python", "simd", "json"], k=3
            ),
        },
        "records": [],
    }

    current_size = len(json.dumps(data).encode("utf-8"))

    while current_size < size_bytes:
        # Create a randomized record
        record = {
            "".join(random.choices(string.ascii_lowercase, k=8)): {
                "id": random.randint(1000, 9999),
                "active": random.choice([True, False, None]),
                "score": random.uniform(0, 100),
                "payload": "X" * random.randint(50, 200),
                "data_points": [random.random() for _ in range(random.randint(1, 5))],
            }
        }
        data["records"].append(record)

        # Check size every 10 records
        if len(data["records"]) % 10 == 0:
            current_size = len(json.dumps(data).encode("utf-8"))

    return json.dumps(data)


def run_benchmark(size: str):
    match size:
        case "small":
            rust_time, python_json_time, simplejson_time = benchmark_performance(
                '{"key": "value"}'
            )
            print("SMALL (10 to 100 bytes):")

        case "medium":
            json_data = generate_test_json(1024)
            rust_time, python_json_time, simplejson_time = benchmark_performance(
                json_data
            )
            print("MEDIUM (1 to 10 KB):")

        case "large":
            json_data = generate_test_json(100 * 1024)
            rust_time, python_json_time, simplejson_time = benchmark_performance(
                json_data
            )
            print("LARGE (100 KB):")

        case "xlarge":
            json_data = generate_test_json(1024 * 1024)
            rust_time, python_json_time, simplejson_time = benchmark_performance(
                json_data
            )
            print("X-LARGE (1 MB):")

    json_speedup = python_json_time / rust_time
    simplejson_speedup = simplejson_time / rust_time

    json_speedup_word = "faster" if json_speedup >= 1 else "slower"
    simplejson_speedup_word = "faster" if simplejson_speedup >= 1 else "slower"

    print(f"Rust:             {rust_time:.6f}s")
    print(
        f"Python json (C):  {python_json_time:.6f}s  (Rust is {json_speedup:.2f}x {json_speedup_word})"
    )
    print(
        f"simplejson:       {simplejson_time:.6f}s  (Rust is {simplejson_speedup:.2f}x {simplejson_speedup_word})\n"
    )


def main():
    if "--benchmark" in sys.argv:
        run_benchmark("small")
        run_benchmark("medium")
        run_benchmark("large")
        run_benchmark("xlarge")
        sys.exit()

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
