# ChoiceScript Save Manager Injector
A bootstrap app to automatically inject the [ChoiceScriptSavePlugin](https://github.com/ChoicescriptIDE/ChoiceScriptSavePlugin) into the Steam editions of Choice of Games/Hosted Games (since patching app.asar results in license invalidation :/)

## How it works

On the first run, the app renames the original executable and then places itself in its place (essentially automatic installation)
On subsequent runs, when the game is launched through Steam, it launches the app instead, which then in turn, launches the original executable but with the addition of a remote debugging flag.
After the remote debug becomes available, it connects itself to it, waits a second for the game to load, and then injects the slightly modified ChoiceScriptSavePlugin.

## How to use

1. Download the executable from the [Releases](https://github.com/jjayrex/Electron_ChoiceScript_Save_Manager_Injector_Bootstrap/releases/latest) page.
2. Put it in the root game folder (next to the [name of the game].exe).
3. Run the executable manually for the first time.
4. Launch the game normally through Steam on subsequent runs.
