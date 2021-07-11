<template>
  <div class="post">
    <div class="content">
      <form @submit="onSave" class="form" @submit.prevent>
        <h3>{{title}}</h3>
        <div class="form-group">
          <label for="" class="label-control">Title</label>
          <input type="text" class="form-control" v-model="post.title" />
        </div>
        <div class="form-group">
          <label for="" class="label-control">Body</label>
          <textarea type="text" class="form-control" v-model="post.body" rows="5" />
        </div>
        <div class="form-buttons">
          <button type="submit" class="btn">Submit</button>
        </div>
      </form>

      <div class="list-post">
        <h1 @click="testEmit">List posts</h1>
        <div class="cards">
          <div class="card" v-for="item in posts" :key="item.id">
            <div class="title">
              <div class="text">{{item.title}}</div>
              <i class="time">{{new Date(item.create_time * 1000).toLocaleString()}}</i>
            </div>
            <div class="body">
              {{item.body}}
            </div>
            <div class="actions">
              <button class="edit btn" v-on:click="onGetInfo(item.id)">Edit</button>
              <button class="delete btn" v-on:click="onDelete(item.id)">Delete</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

function onSave() {
  if(this.formType === this.formTypes.add) {
    axios.post('http://localhost:9999/api/v1/posts', this.post, {headers: {'Content-Type': 'application/json'},}).then(() => {
        this.getPosts();
        this.resetForm();
      })
      .catch(console.error);
  } else {
    axios.put(`http://localhost:9999/api/v1/posts/${this.post.id}`, this.post).then(() => {
        this.formType = this.formTypes.add;
        this.title = this.titles.addPost;
        this.getPosts();
        this.resetForm();
      })
      .catch(console.error);
  }
  
}

function resetForm() {
  this.post = {
    id: null,
    title:'',
    body: ''
  };
}

function getPosts() {
  axios.get(`http://localhost:9999/api/v1/posts`)
    .then(response => {
      this.posts = response.data;
    })
    .catch(e => {
      console.log(e);
    })
}


function onGetInfo(id) {

  axios.get(`http://localhost:9999/api/v1/posts/${id}`).then((res) => {
    this.formType = this.formTypes.update;
    this.title = this.titles.updatePost;
    this.post = {
      id: res.data.id,
      title: res.data.title,
      body: res.data.body,
    }
    console.log(res.data)
  })
  .catch(console.error);
}

function onDelete(id) {
  axios.delete(`http://localhost:9999/api/v1/posts/${id}`).then((res) => {
    console.log(res.data)
    this.getPosts();
  })
  .catch(console.error);
}

export default {
  name: "Post",
  created() {
    this.getPosts();
  },
  data(){
    return {
      formTypes: {
        add: 'add',
        update: 'update'
      },
      titles: {
        addPost: 'Add new post',
        updatePost: 'Update post'
      },
      post: {
        id: null,
        title:'',
        body: '',
      },
      posts: [],
      formType: 'add',
      title: 'Add new post',
    }
  },
  methods: {
    onSave,
    onDelete,
    onGetInfo,
    getPosts,
    resetForm,
    testEmit() {
      this.$emit('test-emit', {data: 'this is data'});
    }
  },
  
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1 {
  text-align: center;
}
.content {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: space-between;
  align-items: baseline;
}

.list-post {
  width: 50%;
}

.form {
  width: 50%;
  justify-content: center;
  display: flex;
  flex-direction: column;
  padding-bottom: 10px;
  border-bottom: 1px solid #eee;
}

.form-group {
  padding: 0 5px;
  margin: 10px 0px;
  display: flex;
  gap: 10px;
}
.label-control {
  width: 50px;
}
.form-control {
  padding: 5px 3px;
  border-radius: 5px;
  width: 100%;
}

.form-buttons {
  padding: 0 5px;
  display: flex;
  justify-content: flex-end;
  gap: 5px;
}

.btn {
  padding: 5px 10px;
  background: rgb(126, 202, 111);
  color: #fff;
  border: none;
  outline: none;
  cursor: pointer;
  border-radius: 5px;
}

.cards {
  display: flex;
  gap: 12px;
  flex-direction: column;
}

.card {
  padding: 10px;
  border-radius: 10px;
  border: 1px solid rgb(185, 180, 180);
  text-align: left;
  position: relative;
}

.title {
  margin-bottom: 5px;
}

.title .text {
  color: rgb(24, 23, 23);
  font-weight: 600;
  font-size: 15px;
}

.title .time {
  font-size: 10px;
}

.card .body {
  font-size: 14px;
}

.card .actions {
  position: absolute;
  top: 5px;
  right: 5px;
  display: flex;
  gap: 5px;
}

.btn {
  border: none;
  border-radius: 5px;
  outline: none;
  cursor: pointer;
}

.card .actions button.edit {
  color: #fff;
  background: rgb(65, 169, 238);
  padding: 3px;

}
.card .actions button.delete {
  color: #fff;
  background: rgb(238, 85, 47);
  padding: 3px;
  
}
</style>
