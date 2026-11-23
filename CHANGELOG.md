# Changelog

The releases of TeachPad, newest first. Each release lists its changes under Added, Changed, Deprecated, Removed, Fixed and Security.

## [0.1.0] - 2026-11-22

The first release. The app plays a sample lesson, and a stand-in tutor answers in the chat.

The installers are unsigned, so each system asks before it opens one. The [Install section of the README](https://github.com/cppalliance/teachpad#install) shows the way past each prompt.

### Added

- **Page commands.** A host fills the textbook page through 21 commands: text and footnotes, bar charts, tree diagrams and timelines, the boxes (tip, key idea, remember, tailored for you, hint and milestone), progress and success marks, and a Continue button.
- **Quizzes.** Pop quizzes with one right option or several, short answers, short essays and Try It code boxes. The quiz rules and the answer key stay in Rust, and the host marks the written answers.
- **Chat.** A tutor chat beside the page, with markdown messages, typing dots, suggestion chips and a composer.
- **Desktop app.** One Tauri window on an in-process loopback server. A browser on the same address shows the same lesson.
- **Sample lessons.** `straw-poll.json`, which the app plays by default, and `contact-sheet.json`, which draws every element of the design once.
- **Installers.** A setup program for Windows, a disk image each for Apple silicon and Intel Macs, and a Debian package and an AppImage for Linux. `cargo installer` builds them.
