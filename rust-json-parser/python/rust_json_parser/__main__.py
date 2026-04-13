import sys
import os

from rust_json_parser._rust_json_parser import parse_json, parse_json_file, dumps


def main():

    match len(sys.argv):
        case 2:
            json = sys.argv[1]
            indent = None

        case 3:
            json = sys.argv[1]
            indent = int(sys.argv[2])

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


main()
