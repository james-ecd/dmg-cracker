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

import struct
import io

def extract_salt_and_encryption_key(dmg_path):
    with open(dmg_path, 'rb') as dmg_file:
        data = dmg_file.read()

    # Check the DMG signature (koly)
    signature = data[:4].decode()
    if signature != 'koly':
        raise ValueError("Invalid DMG signature")

    # Extract the plist offset and size
    plist_offset, plist_size = struct.unpack(">QQ", data[48:64])

    # Read the plist data
    plist_data = data[plist_offset : plist_offset + plist_size]

    try:
        import plistlib
        plist = plistlib.loads(plist_data)
    except Exception as e:
        raise ValueError("Error parsing DMG plist data") from e

    encrypted_key = None
    salt = None

    # Search for the encrypted key and salt in the plist
    for resource in plist.get("resource-forks", []):
        if "blkx" in resource:
            blkx_data = plist["resource-forks"][resource]["blkx"]
            encrypted_key = blkx_data.get("encrypted-key")
            salt = blkx_data.get("salt")
            break

    if encrypted_key is None or salt is None:
        raise ValueError("Failed to extract the encrypted key and salt from the DMG file")

    return encrypted_key, salt


dmg_path = '/Users/jamesdavis/Documents/repos/dmg-cracker/test_resources/test.dmg'
encrypted_key, salt = extract_salt_and_encryption_key(dmg_path)
print(f"Encrypted Key: {encrypted_key}")
print(f"Salt: {salt}")

print(validate_password("test", "/Users/jamesdavis/Documents/repos/dmg-cracker/test_resources/test.dmg"))