const { createApp, reactive } = Vue;
const { Toast } = vant;

const houseData = reactive({
  houses: [],
});

async function fetchHouses() {
  try {
    const response = await fetch("/api/houses", {
      headers: {
        Cookie: `token=${localStorage.getItem("token")}`,
      },
    });
    if (!response.ok) {
      if (response.status === 401) {
        Toast.fail("鉴权失败，请重新登录");
        setTimeout(() => {
          window.location.href = "login.html";
        }, 2000);
        return;
      }
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    houseData.houses = await response.json();
    if (houseData.houses.length === 0) {
      Toast.fail("您还没有房屋，请先创建一个");
      setTimeout(() => {
        window.location.href = "house-create.html";
      }, 2000);
    }
  } catch (error) {
    console.error("Error fetching houses:", error);
    Toast.fail("获取房屋列表失败: " + error);
    setTimeout(() => {
      window.location.href = "login.html";
    }, 2000);
  }
}

fetchHouses();

const app = createApp({
  data: () => ({
    houseData,
  }),
  methods: {
    createNewHouse() {
      window.location.href = "house-create.html";
    },
    selectHouse(houseId) {
      localStorage.setItem("housekeeper_house_current", houseId);
      window.location.href = "index.html";
    },
  },
});

app.use(vant);
app.mount("#app");
