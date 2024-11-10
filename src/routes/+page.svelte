<script lang="ts">
  import {WebviewWindow} from "@tauri-apps/api/webviewWindow"

  import {WINDOWS} from "$const/windows"

  const handleSettingsClick = async () => {
    const settingsWindow = new WebviewWindow(WINDOWS.SETTINGS.label, {url: WINDOWS.SETTINGS.url})

    settingsWindow.once('tauri://error', (e) => {
      console.error(e)
      throw new Error(`${WINDOWS.SETTINGS} window failed to open!`)
    })
  }
</script>

<main class="container bg-gray-950">
  <h1>Welcome to Tauri + Svelte</h1>

  <button onclick={handleSettingsClick} type="button">Go to settings</button>
</main>
