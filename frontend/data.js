const houseData = Vue.reactive({
  currentHouseId: localStorage.getItem("housekeeper_house_current") || "",
  houses: {},
  currentItemId: localStorage.getItem("housekeeper_item_current") || "",
});

// 从接口获取房屋数据
async function fetchHouses() {
  try {
    const response = await fetch("/api/houses", {
      headers: {
        Cookie: `token=${localStorage.getItem("token")}`,
      },
    });
    if (!response.ok) {
      if (response.status === 401) {
        // 401 状态码通常表示未授权，即用户未登录
        window.location.href = "login.html";
        return;
      }
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const houses = await response.json();
    houses.forEach((house) => {
      houseData.houses[house.id] = house;
    });
    // 设置当前房屋 ID
    if (
      houseData.currentHouseId &&
      houseData.houses[houseData.currentHouseId]
    ) {
      // 当前房屋 ID 存在且对应的房屋数据也存在
    } else if (Object.keys(houseData.houses).length > 0) {
      // 如果当前房屋 ID 不存在，选择第一个房屋
      const firstHouseId = Object.keys(houseData.houses)[0];
      houseData.currentHouseId = firstHouseId;
      localStorage.setItem("housekeeper_house_current", firstHouseId);
    }
  } catch (error) {
    console.error("Error fetching houses:", error);
  }
}

// 页面加载时获取房屋数据
fetchHouses();

function createHouse(name) {
  const currentHouseId = String(Math.random()).substring(2);
  houseData.currentHouseId = currentHouseId;
  houseData.currentItemId = "";
  houseData.houses[currentHouseId] = { id: "", name, items: [] };

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
      let housesStr = localStorage.getItem("housekeeper_houses");
      if (housesStr) {
        housesStr += "," + currentHouseId;
      } else {
        housesStr = currentHouseId;
      }
      localStorage.setItem("housekeeper_houses", housesStr);
      localStorage.setItem("housekeeper_house_current", currentHouseId);
      localStorage.setItem("housekeeper_item_current", "");
      saveCurrentHouse();
    })
    .catch((error) => {
      console.error("Error creating house:", error);
    });
}

function saveCurrentHouse() {
  const house = houseData.houses[houseData.currentHouseId];
  // 调用后端接口保存当前房屋数据
  fetch(`/api/houses/${houseData.currentHouseId}/detail`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      Cookie: `token=${localStorage.getItem("token")}`,
    },
    body: JSON.stringify({
      data: house,
      version: house.version || "",
    }),
  })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return response.json();
    })
    .then((data) => {
      house.version = data.version;
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

function selectHouse(houseId) {
  houseData.currentHouseId = houseId;
  localStorage.setItem("housekeeper_house_current", houseId);
}

function deleteCurrent() {
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
      headers: {
        Cookie: `token=${localStorage.getItem("token")}`,
      },
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        localStorage.removeItem(
          "housekeeper_house_" + houseData.currentHouseId
        );
        delete houseData.houses[houseData.currentHouseId];
        let housesStr = localStorage.getItem("housekeeper_houses");
        if (housesStr) {
          const houseIds = housesStr
            .split(",")
            .filter((id) => id != houseData.currentHouseId);
          localStorage.setItem("housekeeper_houses", houseIds.join(","));
          if (houseIds.length) selectHouse(houseIds[0]);
          else selectHouse("");
        }
      })
      .catch((error) => {
        console.error("Error deleting current house:", error);
      });
  }
}

const currentHouse = Vue.computed(() => {
  return houseData.houses[houseData.currentHouseId];
});

const navItems = Vue.computed(() => {
  const result = [];
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
  findItem(currentHouse.value.items);
  result.splice(0, 0, currentHouse.value);
  return result;
});

const currentItem = Vue.computed(() => {
  const size = navItems.value.length;
  return size ? navItems.value[size - 1] : null;
});
