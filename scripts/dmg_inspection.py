import struct

with open('/Users/jamesdavis/Documents/repos/dmg-cracker/test_resources/test.dmg', 'rb') as f:
    # Read the first 512 bytes of the file (the header)
    header = f.read(512)

    # Parse the header using the 'unpack' method of the 'struct' module
    signature, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _ = struct.unpack(
        '>QIIIIIHHHHHHHHHHHHHHHHHH', header)

    # Determine the size of the header
    header_size = 64

    # Construct the format string for the header
    format_string = '>' + 'I' * ((header_size - 8) // 4)

    # Read the entire header using the determined format string
    fields = struct.unpack(format_string, header[:header_size])

    # Print out the values of all the fields
    for i, field in enumerate(fields):
        print(f'Field {i}: {field}')
