<script lang="ts">
  let configDir = $state()

  $inspect(configDir)
</script>

<h2>This is settings page!</h2>

<input type="file" name="configDir" bind:files={configDir} webkitdirectory>
