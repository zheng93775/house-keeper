const app = Vue.createApp({
  data() {
    return {
      form: {
        username: "",
        password: "",
      },
      loading: false,
    };
  },
  methods: {
    async handleLogin() {
      this.loading = true;
      try {
        const response = await fetch("/api/login", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(this.form),
        });

        if (!response.ok) {
          const error = await response.json();
          throw new Error(error.message || "登录失败");
        }

        const data = await response.json();
        vant.Toast.success(`欢迎回来，${data.username}`);
        setTimeout(() => {
          window.location.href = "index.html";
        }, 1500);
      } catch (error) {
        vant.Dialog.alert({
          title: "登录错误",
          message: error.message,
        });
      } finally {
        this.loading = false;
      }
    },
  },
});

app.use(vant);
app.mount("#app");
