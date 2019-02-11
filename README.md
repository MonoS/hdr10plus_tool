# hdr10plus_parser
Tool to check if a HEVC file contains SMPTE 2094-40 metadata in SEI 
messages.

If dynamic metadata is found, the whole file is parsed through and a 
metadata JSON file is generated for use with x265/other encoders.

# Usage, in CLI:

* `hdr10plus_parser.exe "path/to/file.hevc"`
* `ffmpeg -i "input.mkv" -c:v copy -vbsf hevc_mp4toannexb -f hevc - | hdr10plus_parser.exe -`

options:
* `-i`, `--input <INPUT>` Sets the input file to use.
* `-o`, `--output <OUTPUT>` Sets the output JSON file to use.

* `--verify` Checks for dynamic metadata only.

# Sample files
Tears of Steel samples encoded with x265 using `--dhdr10-info` for tests.

Sample JSON metadata available here: https://bitbucket.org/multicoreware/x265/downloads/
