function createHouse(name) {
  fetch("/api/houses", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Cookie: `token=${localStorage.getItem("token")}`,
    },
    body: JSON.stringify({ name }),
  })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return response.json();
    })
    .then((data) => {
      localStorage.setItem("housekeeper_house_current", data.id);
      location.href = "index.html";
    })
    .catch((error) => {
      console.error("Error creating house:", error);
    });
}

const app = Vue.createApp({
  data: () => {
    return {
      name: "",
    };
  },
  methods: {
    handleCreateHouse() {
      createHouse(this.name);
    },
  },
});
app.use(vant);
app.mount("#app");
