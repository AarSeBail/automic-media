# Automic Media

This repository is a proof of concept for creating generative art by running automata on video frames. It has terrible tooling and I'm not going to fix it since this was just a curiosity. The core library is well written, but outside of that the code is a mess because why bother.

## Examples

There are two example files in the repository. Both require a directory of frames, `in_frames`, as well as the existence of the directory `out_frames`. The `out_frames` may be compiled into a video or gif using tools like ffmpeg.

The examples are full of commented out code, since the current method of designing ant behavior is just to edit the code and recompile.
1. `texture_gif`: This spawns a large number of ants, each with a limited lifespan. The ants move on a toroidal grid, which creates a looping, tiling grid. This is nice for the creation of animated textures.

<p align="center">
  <img width="256" src="https://raw.githubusercontent.com/AarSeBail/automic-media/with-assets/assets/t2.gif">
</p>


2. `video_frames`: This spawns a set of ants for each frame, each with a limited lifespan. This produces a video that neither loops nor tiles.
