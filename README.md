# winresz

A simple window resize utility for Windows.

## How to use it

- Change Resolution by window title
  ```
  winresz --title-contains "Windowed Projector" 1280x720
  ```
- Change Resolution by executable path
  ```
  winresz --path-endswith "notepad.exe" 1280x720
  ```
