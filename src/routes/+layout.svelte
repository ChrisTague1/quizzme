<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<header class="topbar">
  <a href="/" class="brand">Quizzme</a>
</header>

<main class="container">
  {@render children()}
</main>
