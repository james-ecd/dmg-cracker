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



import os
from Crypto.Cipher import AES
from Crypto.Util.Padding import unpad
from Crypto.Protocol.KDF import PBKDF2

def validate_password(password, file_path):
    # Read the first 1024 bytes of the encrypted .dmg file to extract the header
    with open(file_path, "rb") as f:
        header = f.read(1024)

    # Extract the salt value from the header
    salt = header[14:30]

    # Generate the key using PBKDF2 with the extracted salt value and the user's password
    key = PBKDF2(password, salt, dkLen=32, count=10000)

    # Decrypt the encrypted .dmg file using the generated key
    iv = os.urandom(16)
    cipher = AES.new(key, AES.MODE_CBC, iv)
    with open(file_path, "rb") as f:
        f.read(1024)
        encrypted_data = f.read()
    decrypted_data = unpad(cipher.decrypt(encrypted_data), 16)

    # Check if the decrypted data is valid by looking for a known magic value
    magic_value = b'\x78\xda'
    if len(decrypted_data) < len(magic_value):
        return False
    if decrypted_data[:len(magic_value)] == magic_value:
        return True
    else:
        return False

print(validate_password("test", "/Users/jamesdavis/Documents/repos/dmg-cracker/test_resources/test.dmg"))