# Link Generator

Generates symbolic links from executables, into the directory where
`linkgen.exe` sits (presumably on the user's PATH).

More specifically, the idea is to make installing command-line applications
easier on Windows. Simply create a folder somewhere in your user directory, drop
in `linkgen.exe`, then add that folder to your PATH. Applications which you want
to access from your PATH can then easily be added by using
`linkgen ./path/to/exe`.

Currently only works on Windows, though adding support for other OSs is trivial.

See `linkgen --help` for more information.