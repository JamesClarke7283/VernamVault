# Vernam Vault Plan

I would like vernam vault to have 3 major features:
Encryption, Decryption and Key Management.

## Encryption

When we encrypt a message the first 16 bytes of the key is used as a identifier to be sent with the armored Base64 encoded message, a bit like PGP/GPG, this enables us to tell the difference between the keys and the recipient knows which one to use for decyption. Each ciphertext is the same length in order to ensure that an adversary can't perform any cryptoanalytic techneques on the ciphertext, any non message parts will just be filled with randomized bits.

## Decryption

Once we match the first 16 bytes of the ciphertext in the armored/base64 encoded encrypted OTP message (which is the first 16 bytes of the key), we choose the correct key which starts with that IDENTIFIER component, and perform the decryption, the message is then revealed.

## Key Management

Both parties when they generate their keys, will choose a key/message size, which is represented as characters, but will later be converted as bytes, as the keys will be stored as binary data in a folder, where each key is a file with the name being the IDENTIFIER(represented as hexedecimal, which will be converted to binary to perform the comparison with the correct key on encryption). Once a message is encrypted/decrypted, the key will automatically be deleted for security, in a secure manner, overwriting the file with new data so its harder to recover(a hard drive is reccomended to the user to store the keys as its easier to wipe). The user will also need to specify the number of keys/pads they would like to have, when generating the keys inside the key directory of their choice.

## Message Format

Here is an example of what the message will look like, that is output by the tool:

```plaintext
-----BEGIN ONE TIME PAD MESSAGE-----
Identifier: CgQ7JFLX
U29tZSBleGFtcGxlIGVuY3J5cHRlZCBtZXNzYWdlIGJ5dGVzIHdpbGwgaGVyZQ==
-----END ONE TIME PAD MESSAGE-----
```
