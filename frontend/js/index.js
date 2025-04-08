const houseData = Vue.reactive({
  currentHouseId: localStorage.getItem("housekeeper_house_current") || "",
  currentItemId: localStorage.getItem("housekeeper_item_current") || "",
  currentHouse: null,
});

const navItems = Vue.computed(() => {
  const result = [];
  if (!houseData.currentHouse) return result;
  const findItem = (items) => {
    if (!items) return false;
    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      if (item.id == houseData.currentItemId) {
        result.push(item);
        return true;
      }
      if (findItem(item.items)) {
        result.splice(0, 0, item);
        return true;
      }
    }
    return false;
  };
  findItem(houseData.currentHouse.items);
  result.splice(0, 0, houseData.currentHouse);
  return result;
});

const currentItem = Vue.computed(() => {
  const size = navItems.value.length;
  return size ? navItems.value[size - 1] : null;
});

function saveCurrentHouse() {
  // 调用后端接口保存当前房屋数据
  fetch(`/api/houses/${houseData.currentHouseId}/detail`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(houseData.currentHouse),
  })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return response.json();
    })
    .then((data) => {
      houseData.currentHouse.version = data.version;
    })
    .catch((error) => {
      console.error("Error saving current house:", error);
    });
}

function addSubItem(name) {
  if (!currentItem.value.items) currentItem.value.items = [];
  const newItem = {
    id: String(Math.random()).substring(2),
    name,
    content: "",
    items: [],
  };
  currentItem.value.items.push(newItem);
  setTimeout(saveCurrentHouse);
  return newItem;
}

function selectItem(itemId) {
  houseData.currentItemId = itemId;
  localStorage.setItem("housekeeper_item_current", itemId);
}

function deleteCurrentItem() {
  const navSize = navItems.value.length;
  if (navSize > 1) {
    const parentItem = navItems.value[navSize - 2];
    if (parentItem && parentItem.items && parentItem.items.length) {
      parentItem.items = parentItem.items.filter(
        (item) => item.id != houseData.currentItemId
      );
      selectItem(parentItem.id);
      setTimeout(saveCurrentHouse);
    }
  } else {
    // 调用后端接口删除当前房屋
    fetch(`/api/houses/${houseData.currentHouseId}`, {
      method: "DELETE",
      headers: {},
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        localStorage.removeItem(
          "housekeeper_house_" + houseData.currentHouseId
        );
        location.href = "my-houses.html";
      })
      .catch((error) => {
        console.error("Error deleting current house:", error);
      });
  }
}

async function fetchHouseDetail() {
  try {
    const response = await fetch(
      "/api/houses/" + houseData.currentHouseId + "/detail",
      {
        headers: {},
      }
    );
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
    houseData.currentHouse = await response.json();
  } catch (error) {
    console.error("Error fetching houses:", error);
    Toast.fail("获取房屋列表失败: " + error);
    setTimeout(() => {
      window.location.href = "login.html";
    }, 2000);
  }
}

if (!houseData.currentHouseId) {
  location.href = "./house-create.html";
} else {
  fetchHouseDetail();
  const app = Vue.createApp({
    data: () => ({
      addModalVisible: false,
      areaName: "",
      houseData,
      currentItem,
      navItems,
      // 定义菜单选项
      menuOptions: [
        { text: "创建新的房屋", value: "house-create.html" },
        { text: "我的房屋列表", value: "my-houses.html" },
      ],
    }),
    methods: {
      showAddModal() {
        this.addModalVisible = true;
        this.areaName = "";
      },
      handleAdd() {
        addSubItem(this.areaName);
      },
      handleSave() {
        setTimeout(saveCurrentHouse);
      },
      handleSearch() {
        location.href = "search.html";
      },
      handleSelect(item) {
        selectItem(item.id);
      },
      handleDelete() {
        vant.Dialog.confirm({
          title: "警告",
          message: "确认删除吗？",
        }).then(() => {
          deleteCurrentItem();
        });
      },
      // 处理菜单选择事件
      onMenuChange(value) {
        location.href = value;
      },
    },
  });
  app.use(vant);
  app.mount("#app");
}
