<template>
  <Post v-on:test-emit="testEmit" />
</template>

<script>
import Post from './components/Post.vue';

export default {
  name: 'App',
  components: {
    Post
  },
  methods: {
    testEmit(data) {
      console.log(data);
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
