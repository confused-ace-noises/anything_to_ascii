# API documentation
The API for anything_to_ascii is quite easy to use, but quite complex behind the scenes.
Every endpoint documented will be structured like so (the words surrounded by "#" are the ones subject to change):

### #Name#
**endpoint:** `/api/#endpoint_url#`

**data:** #data_kind#

**parameters:**
- #**parameter_name**# **(**#**values_accepted**#**)**: #usage#
- ...
- ...

**example:** `curl example`

for any confusion refer to the footnotes section.

## Endpoints

### Image to ASCII
**description**: given an image as binary data, processes into ASCII art.

**endpoint:** `/api/img_to_ascii`

**data:** image

**parameters:**
- **height (not present; positive integer)**: defines the height in characters of the resulting ASCII art; if not present, it'll default to the height of original image.
- **width (not present; positive integer)**: defines the width in characters of the resulting ASCII art; if not present, it'll default to the height of original image.
- **invert (not present; present with no associated value; bool)**: defines whether the ASCII art luminosity should be inverted: the most luminous spots will use the least dense characters, and vice versa; default: false; present with no associated value: true.
- **colored (not present; present with no associated value; bool)**:
defines whether the ASCII art should have ANSI encoded colors or not; default: false; present with no associated value: true.

- **uniform (not present; present with no associated value; bool)**: defines whether the ASCII art should be made all out of the densest character; when paired with **invert**, only uses the least dense character; generally only useful when paired with **colored**; default: false; present with no associated value: true.

**example**: `curl -X POST http://localhost:8000/api/img_to_ascii?&width=150&invert=false&colored --data-binary @image.png`

***

### Video to ASCII
**description**: given a video as binary data, processes into ASCII art. Each frame is separated in the response with a `###`

**endpoint:** `/api/video_to_ascii`

**data:** video

**parameters:**
- **height (not present; positive integer)**: defines the height in characters of the resulting ASCII arts; if not present, it'll default to the height of original video frame.
- **width (not present; positive integer)**: defines the width in characters of the resulting ASCII arts; if not present, it'll default to the height of original video frame.
- **nframes (not present; positive integer lower than the original video's number of frames)**: the total number of ASCII art frames that will be produced; they get taken as evenly spaced-out as possible; default: the original video's number of frames.

- **invert (not present; present with no associated value; bool)**: defines whether the ASCII arts' luminosity should be inverted: the most luminous spots will use the least dense characters, and vice versa; default: false; present with no associated value: true.
- **colored (not present; present with no associated value; bool)**:
defines whether the ASCII arts should have ANSI encoded colors or not; default: false; present with no associated value: true.

- **uniform (not present; present with no associated value; bool)**: defines whether the ASCII arts should be made all out of the densest character; when paired with **invert**, only uses the least dense character; generally only useful when paired with **colored**; default: false; present with no associated value: true.

**example**: `curl -X POST http://localhost:8000/api/video_to_ascii?&height=150&invert=false&colored --data-binary @video.mp4`

***

### Audio to ASCII
**description**: given an audio as binary data, processes into ASCII art of the waveform.

**endpoint:** `/api/audio_to_ascii`

**data:** audio

**parameters:**
- **height (not present; positive 8-bit integer)**: defines the height of <ins> half </ins> of the characters of the resulting ASCII art; corresponds to the height of the highest peak in the waveform from silence; if not present, it'll default to 255.

- **invert (not present; present with no associated value; bool)**: defines whether the ASCII arts' luminosity should be inverted: the most luminous spots will use the least dense characters, and vice versa; default: false; present with no associated value: true.

- **uniform (not present; present with no associated value; bool)**: defines whether the ASCII arts should be made all out of the densest character; when paired with **invert**, only uses the least dense character; default: false; present with no associated value: true.

**example**: `curl -X POST http://localhost:8000/api/audio_to_ascii?height=100&uniform --data-binary @audio.mp3`

## Foot Notes

**"not present" value**: just means that the parameter could also be missing, and if so., it'll take on its default value.

**"present with no associated value" value**: like this: `...endpoint?parameter_with_no_associated_value&value_with_associated_value=associated_value`