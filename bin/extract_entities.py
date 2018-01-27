#!/usr/bin/env python3

import json
import sys
import re
from secrets import token_hex
import os


def get_a_file_path(dir_path):
    file_path = os.path.join(dir_path,
                             token_hex(16))

    if os.path.exists(file_path):
        return get_a_file_path(dir_path)

    return file_path


def save_data(entity_name, json_data):
    script_path = os.path.dirname(os.path.realpath(__file__))
    cleaned_entity_name = re.sub(r"s$", "", entity_name)
    dir_path = os.path.join(script_path,
                            "../tests/samples",
                            cleaned_entity_name)
    if not os.path.exists(dir_path):
        os.makedirs(dir_path)

    file_path = get_a_file_path(dir_path)

    with open(file_path, "w") as file_desc:
        json.dump(json_data, file_desc)

def get_entities(json_data, entity_name, index, subindex):
    if isinstance(json_data, dict):
        if entity_name is not None:
            save_data(entity_name, json_data)
        for k in json_data.keys():
            get_entities(json_data[k], k, index, subindex + 1)
    elif isinstance(json_data, list):
        for i in range(len(json_data)):
            get_entities(json_data[i], entity_name, index + 1, subindex)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("you have to provide a file name")
        sys.exit(1)

    with open(sys.argv[1], "rb") as json_file:
        get_entities(json.load(json_file), None, 0, 0)
