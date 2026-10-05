# FocusIsland Privacy Policy

**Effective date: October 5, 2026**

FocusIsland is an offline-first desktop productivity application. This policy describes the data handled by the FocusIsland application. It does not cover GitHub, SignPath, or other third-party websites and services; those services have their own privacy policies.

## Data handled by the application

FocusIsland does not require an account and does not collect, upload, sell, or share your task content or usage data. The application contains no analytics or telemetry and makes no network requests as part of normal use.

The information you enter or create (tasks, notes, reminders, appearance preferences, timer state, and focus-session records) is stored locally in a SQLite database on your device:

- Windows: `%APPDATA%\com.focusisland.desktop\focusisland.sqlite3`
- macOS: `~/Library/Application Support/com.focusisland.desktop/focusisland.sqlite3`

The database is not encrypted by FocusIsland. Access to it is governed by your operating system account and device security. Anyone who can access your account or these files may be able to read them. FocusIsland does not receive a copy. Uninstalling the application may leave this data behind; to erase it, quit FocusIsland and remove the database and its containing app-data folder yourself. Back up the database first if you want to keep your information.

## Operating-system features

If enabled, FocusIsland asks the operating system to launch the application when you sign in. This changes a local startup setting and can be disabled in Customize. Reminders and focus-completion alerts are delivered through the operating system's notification service. Your operating system may apply its own notification settings and privacy practices. Notification sounds are optional and off by default.

FocusIsland does not connect to a calendar, email account, or cloud service. Reminders are local to this device. The app does not request location, contacts, microphone, or camera access.

## Your choices

You can choose what to enter, disable notifications, turn off launch-at-login, and remove your local data. Restrict access to your Windows/macOS account and keep your device protected if your notes or tasks are sensitive.

## Changes and contact

Material changes to this policy will be recorded in the project repository. For privacy questions or to report a concern, open an issue at [github.com/vishvajeet2012/FocusIsland/issues](https://github.com/vishvajeet2012/FocusIsland/issues). Do not include private task or note contents in a public issue.

This policy applies to FocusIsland itself. GitHub's handling of information when you visit the project, download releases, or submit an issue is governed by [GitHub's Privacy Statement](https://docs.github.com/en/site-policy/privacy-policies/github-privacy-statement).
