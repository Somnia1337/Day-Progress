# [Day-Progress](https://github.com/Somnia1337/Day-Progress) v0.3.0

A simple reimplementation of [Day-Progress](https://sindresorhus.com/day-progress) by Sindre Sorhus (originally for macOS), showing the percentage of time remaining in the day (excluding sleep hours).

<div align=center>
  <img src="https://github.com/Somnia1337/Day-Progress/blob/main/preview_v0.3.0.png?raw=true" width="450px">
</div>

This is a tray application, meaning:

- It does not have a main window or command-line interface (CLI).
- Configuration (your wake and sleep times) could be set in `configs.txt`, which have fallbacks to 07:00 and 23:00.

`configs.txt` could be written like this:

```text
07:00
23:00
```
