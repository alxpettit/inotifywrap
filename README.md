# InotifyWrap

A utility like `inotifywait`, but instead of waiting, it restarts a program it's wrapping.

This was made to wrap the Godot game engine as its used to execute a scene, so that the engine will restart if you `close_write` the `.so` file(s).

Usage example: 

```
inotifywrap [inotifywrap args] godot [godot args]
```

Anything beginning with hyphens (e.g., `-t`, `--this`) is assumed to be `inotifywrap` arguments, and the first argument without these hyphens is read as the command to wrap, instead of a `inotifywrap` flag.
