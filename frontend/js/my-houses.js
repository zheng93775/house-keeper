const { createApp, reactive } = Vue;
const { Toast, Dialog } = vant;

const houseData = reactive({
  currentUserId: "",
  houses: [],
  memberModalVisible: false,
  currentHouse: {},
  newMemberUsername: "",
});

async function fetchHouses() {
  try {
    const response = await fetch("/api/houses", {
      headers: {},
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
    const resp = await response.json();
    houseData.currentUserId = resp.user_id;
    houseData.houses = resp.houses;
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

async function saveHouseMembers() {
  try {
    const houseId = houseData.currentHouse.id;
    const usernames = houseData.currentHouse.members.map(
      (member) => member.username
    );
    const response = await fetch(`/api/houses/${houseId}/members`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ usernames }),
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

    const resp = await response.json();
    Toast.success(resp.message);
    houseData.memberModalVisible = false;
  } catch (error) {
    console.error("Error saving house members:", error);
    Toast.fail("保存房屋成员失败: " + error);
  }
}

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
    // 成员管理方法
    manageMembers(house) {
      console.log("manageMembers", house);
      this.houseData.memberModalVisible = true;
      this.houseData.currentHouse = house;
      this.houseData.newMemberUsername = "";
    },
    addMember() {
      if (this.houseData.newMemberUsername) {
        this.houseData.currentHouse.members =
          this.houseData.currentHouse.members || [];
        this.houseData.currentHouse.members.push({
          username: this.houseData.newMemberUsername,
        });
        this.houseData.newMemberUsername = "";
      }
    },
    deleteMember(index) {
      this.houseData.currentHouse.members.splice(index, 1);
    },
    saveMembers() {
      this.addMember();
      saveHouseMembers();
    },
  },
});

app.use(vant);
app.mount("#app");
