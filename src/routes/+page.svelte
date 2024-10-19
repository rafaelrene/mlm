<script lang="ts">
  import {invoke} from "@tauri-apps/api/core";

  const test = async () => {
    const config = await invoke("test", {st: "ST IS THIS..."})

    debugger
  }
</script>

<main class="container bg-gray-950">
  <h1>Welcome to Tauri + Svelte</h1>

  <button onclick={test}>GET!</button>
</main>
