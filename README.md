# Anything to ASCII
Anything to ASCII is a fun and easy-to-use program to transform anything you want into ASCII art: 
be it images, videos, or even audio tracks, this application can turn them  into an ASCII art representation of them.

## Features 🛠️
- Can convert images and videos (and audio tracks, because it's funny). 
- Support for a wide range of media types.
- Support for both single-threaded and multi-threaded computations.
- Resizing of the input media based on user settings.
- Now with better memory locality!
- Faster than ever!
- Easy-to-use API

## Update

#### 29/01/2025
In this update, the application was completely remade, even if some parts are still similar.
The biggest new things are mainly:

- New in-memory structures: now, instead of storing the images in 2-dimensional vectors (`Vec<Vec<_>>`), they are now stored in a custom `FlatMatrix`, which, as the name suggests, stores the data flatly, bettering memory usage and locality, thus improving speed.
- Now the application is available as a library too! (local only for now, will be available on )

#### 30/01/2025
Added an API and relevant docs. Also fixed a pretty major problem with the audio to ascii conversion.

## Examples

**Disclaimer**: depending on your platform and/or application/website that's displaying the GIFs, they may or may not look horrible. If you aren't satisfied with their quality, you're welcome to try the application yourself! 😉

### Image conversion:

![](https://github.com/confused-ace-noises/anything_to_ascii/blob/master/.assets/ascii_image_video.gif?raw=true)

### Video Conversion:

![](https://github.com/confused-ace-noises/anything_to_ascii/blob/master/.assets/ascii_video_video_pizza.gif?raw=true)
