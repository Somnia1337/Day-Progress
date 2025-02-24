# [Day-Progress](https://github.com/Somnia1337/Day-Progress) v0.4.1

A simple reimplementation of [Day-Progress](https://sindresorhus.com/day-progress) by Sindre Sorhus (originally for macOS), showing the percentage of time remaining in the day (excluding sleep hours).

<table>
  <tr>
    <td><a href="https://sindresorhus.com/day-progress">Day-Progress</a> by Sindre Sorhus</td>
    <td>This Project</td>
  </tr>
  <tr>
    <td align="center"><img src="https://github.com/Somnia1337/Day-Progress/blob/main/day-progress-on-macos_v0.4.0_1.png?raw=true" width="400px"></td>
    <td align="center"><img src="https://github.com/Somnia1337/Day-Progress/blob/main/preview_v0.4.0_1.png?raw=true" width="400px"></td>
  </tr>
</table>

This is a tray application, meaning:

- It does not have a main window or command-line interface (CLI).
- Configuration (your wake and sleep times) could be set in `configs.txt`.

## Configuration

`configs.txt` could be written like this:

```text
07:00
23:00
```

(`v0.4.0` update) If you're a night owl 🐱, you can use a sleep time later than 00:00:

```text
09:00
01:00 / 25:00
```

Note: If the program fails to parse your configuration, it will fallback to a schedule of 08:00 - 00:00.
