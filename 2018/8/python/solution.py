#!/usr/bin/env python3

def get_length(node):
    length = 2
    length += len(node['metadata'])
    for child_node in node['children']:
        length += get_length(child_node)
    return length


def parse_node(input_list: list) -> dict:
    metadata_sum = 0
    value = 0
    child_node_count = input_list[0]
    children = []
    metadata_count = input_list[1]
    rest = input_list[2:]
    offset = 2
    for i in range(child_node_count):
        child_node = parse_node(input_list[offset:])
        children.append(child_node)
        offset += get_length(child_node)
        metadata_sum += child_node['metadata_sum']
    metadata = input_list[offset:offset + metadata_count]
    for metadatum in metadata:
        metadata_sum += metadatum
    if child_node_count == 0:
        value = metadata_sum
    else:
        for metadatum in metadata:
            if len(children) > metadatum - 1:
                value += children[metadatum - 1]['value']

    return {'metadata': metadata, 'children': children, 'metadata_sum': metadata_sum, 'value': value}


with open('../input', 'r') as input_file:
    input_list = [int(x) for x in input_file.read().strip().split(' ')]
    print("Part 1:", parse_node(input_list)['metadata_sum'])
    print("Part 2:", parse_node(input_list)['value'])

