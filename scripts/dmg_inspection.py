import struct

# Set the path to the encrypted .dmg file
file_path = '/Users/jamesdavis/Documents/repos/dmg-cracker/test_resources/test.dmg'

# Read the first 1024 bytes of the encrypted .dmg file to extract the header
with open(file_path, "rb") as f:
    header = f.read(1024)

# Extract the encryption algorithm identifier, key size, and salt value from the header
algorithm = header[2:6]
key_size = struct.unpack(">I", header[6:10])[0]
salt = header[14:30]

# Print the extracted values
print("Encryption algorithm:", algorithm)
print("Key size:", key_size)
print("Salt value:", salt)