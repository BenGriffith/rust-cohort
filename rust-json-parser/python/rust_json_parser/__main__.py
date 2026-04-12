from os import path
import sys

from rust_json_parser._rust_json_parser import parse_json, parse_json_file, dumps

INDENT = 4


def main():
    if sys.argv[1]:
        target = sys.argv[1]
        data = None
        if path.exists(target):
            try:
                data = parse_json_file(target)
                print(dumps(data, INDENT))
            except Exception as err:
                print(err)
        else:
            try:
                data = parse_json(target)
                print(dumps(data, INDENT))
            except Exception as err:
                print(err)


main()
