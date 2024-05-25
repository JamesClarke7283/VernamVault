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

## Command-Line Interface (CLI)

I now want to lay out the foundations for the CLI tool, the following are the subcommands of the `vernamvault` command.

### Manage 

the `manage` sub command lets you manage the keys you have, you can `generate`, `list` and `delete` keys by identifier.

#### Generate
`generate` takes positional arguments `<LENGTH_OF_KEY_IN_N_CHARACTERS> <NUMBER_OF_KEYS> <KEY_DIRECTORY>`
this takes into account the 16 byte part of the key that is used as the identifer as well, so its really `LENGTH_OF_KEY_IN_NUMBER_OF_UTF8_CHARACTERS` plus 16 bytes.

The keys are generated as single files per key as `.pad` files, named by the first 16 bytes of the key as hexadecimal, and are stored in the `KEY_DIRECTORY`. The user is prompted whether they want to fill that directory with the keys, as it will create a lot of files, this is displayed all in UPPERCASE in YELLOW as to warn the user.

### Encrypt
The `encrypt` subcommand, takes a mandatory `-m/--message`/`-i/--input-file` flag which can either be inline text or a text file which is full of utf-8 characters only, it also takes a mandatory `-k/--key-dir` flag which points to the directory of where the keys are stored which will be used, a random key is picked from the list of keys. the last flag is the `-o/--output-file` flag, which is the full path to the output file where the encrypted message will be sent, if this flag is not specified, then the armored/base64 text will be output to the screen for them to copy.

### Decrypt
This `decrypt` subcommand, takes two mandatory flags: `-k/--key-dir` which is the directory which contains the keys, and `-i/--input-file` which contains the path to the `armored/base64` ciphertext file, if this is not specified then it prompts the user for input to paste in the content of that file, it then outputs the message to the screen or gives a decrpyt error if it can't find the key. if the key is found, it deletes the key after the message is output and informs the user the key was deleted securely.


