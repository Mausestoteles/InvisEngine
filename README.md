# InvisEngine

Eine Windows-Arbeitsumgebung, die auf dem Monitor sichtbar ist, für Screen-Capture-Programme (OBS, Discord, Teams, Zoom, Snipping Tool, …) aber nicht.

Basis ist `SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)`. Voraussetzung ist **Windows 10 2004 oder neuer**.
Die API wirkt nur auf Fenster des eigenen Prozesses, deshalb laufen alle Apps der Engine (Terminal, Explorer, Browser) im Engine-Prozess selbst.

## Stack

- Tauri 2 (Rust) als einzelne portable `.exe`, ohne Installer
- Web-Oberfläche in TypeScript + Vite, gerendert von WebView2 (auf Windows 10/11 vorinstalliert)

## Stand: Prototyp (Schritt 1)

- Hauptfenster und Test-Browserfenster (WebView2 mit externer Seite) sind von Capture ausgeschlossen
- Fenster starten versteckt und werden erst nach gesetzter Affinity angezeigt, damit kein Frame durchrutscht
- Kein Taskleisten-Eintrag
- Hotkey `Ctrl+Alt+Space` blendet alle Fenster ein/aus
- Button zum Umschalten des Schutzes, zum Vergleich in OBS

### Test in OBS

1. `invis-engine.exe` starten.
2. In OBS eine **Bildschirmaufnahme** hinzufügen: Die Engine-Fenster dürfen nicht zu sehen sein.
3. **Schutz umschalten** klicken: Jetzt muss das Hauptfenster in OBS erscheinen.
4. Dasselbe mit **Test-Browserfenster öffnen** und einer **Fensteraufnahme** wiederholen.

## Roadmap

1. Prototyp: unsichtbares Fenster + WebView2 ← *aktuell*
2. Terminal (ConPTY + xterm.js)
3. Datei-Explorer
4. Browser (WebView2-Tabs)

## Entwicklung

```sh
npm install
npm run tauri dev      # entwickeln
npm run tauri build    # -> src-tauri/target/release/invis-engine.exe
```

Die CI (GitHub Actions, `windows-latest`) baut die portable exe als Artifact `InvisEngine-portable`.
