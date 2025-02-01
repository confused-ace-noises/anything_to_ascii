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

## Updates

#### 29/01/2025
In this update, the application was completely remade, even if some parts are still similar.
The biggest new things are mainly:

- New in-memory structures: now, instead of storing the images in 2-dimensional vectors (`Vec<Vec<_>>`), they are now stored in a custom `FlatMatrix`, which, as the name suggests, stores the data flatly, bettering memory usage and locality, thus improving speed.
- Now the application is available as a library too! (local only for now, will be available on )

#### 30/01/2025
Added an API and relevant docs (api_docs.md in the root of the repo). Also fixed a pretty major problem with the audio to ascii conversion.

#### 31/01/2025
Big quality of life updates!
- Added logging with different levels (silent, normal and verbose);
- Added a progress bar! Finally!
- Added library docs!
- Now easily available on android by using `anything_to_ascii_android.sh`!

Also fixed a bug where any transparent pixels would get omitted from the printing.

## Examples

**Disclaimer**: depending on your platform and/or application/website that's displaying the GIFs, they may or may not look horrible. If you aren't satisfied with their quality, you're welcome to try the application yourself! 😉

### Image conversion:

![](https://github.com/confused-ace-noises/anything_to_ascii/blob/master/.assets/ascii_image_video.gif?raw=true)

### Video Conversion:

![](https://github.com/confused-ace-noises/anything_to_ascii/blob/master/.assets/ascii_video_video_pizza.gif?raw=true)

***

# Installation guide

## Windows
First of all, install rust. I recommend using [rust's tools](https://www.rust-lang.org/tools/install).

Then install `ffmpeg` and `libavutil`. 

After that, install it:
`cargo install --git https://github.com/confused-ace-noises/anything_to_ascii.git`

All done! You're ready to use `anything_to_ascii`!

## Linux
First of all, install rust. I recommend using [rust's tools](https://www.rust-lang.org/tools/install).

Then install `ffmpeg` and `libavutil`:

##### Debian, Ubuntu, Mint or anything using `apt`
`sudo apt install libavutil-dev ffmpeg`

##### Fedora, CentOS or anything using `dnf`
`sudo dnf install ffmpeg-free-devel clang-devel ffmpeg`

##### Arch, Manjaro or anything using `pacman`:
`sudo pacman -Sy ffmpeg libavcodec libavformat libavutil clang`

##### Alpine
`sudo apk ffmpeg-libavutil ffmpeg`

if you don't find your distribution here, im sure you're experienced enough to install it yourself!

After that, install it:
`cargo install --git https://github.com/confused-ace-noises/anything_to_ascii.git`

All done! You're ready to use `anything_to_ascii`!

## Android
First of all, install `Termux` from your app store of your choice.

Then, run:
`curl https://raw.githubusercontent.com/confused-ace-noises/anything_to_ascii/refs/heads/master/anything_to_ascii_android_install.sh > anything_to_ascii_android_install.sh`

After that:
`chmod +x anything_to_ascii_android_install.sh`

And then, finally:
`bash anything_to_ascii_android_install.sh`

Now, you'll be guided through the installation process.
WARNING: It could take a while, especially if you have a slow internet connection.

## iOS / MacOS
I have tried *desperatly* to cross-compile the application for iOS and/or MacOS, but this application unfortunately cannot be cross-compiled.
If you're available to distribute a natively-compiled version of the application for iOS and/or MacOS, I'd be happy to collaborate to do so.
