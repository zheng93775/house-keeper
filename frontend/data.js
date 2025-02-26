const houseData = Vue.reactive({
  currentHouseId: localStorage.getItem("housekeeper_house_current") || "",
  houses: {},
  currentItemId: localStorage.getItem("housekeeper_item_current") || "",
});

if (localStorage.getItem("housekeeper_houses")) {
  const housesStr = localStorage.getItem("housekeeper_houses");
  const houseIds = housesStr.split(",");
  for (let i = 0; i < houseIds.length; i++) {
    const houseId = houseIds[i];
    const houseStr = localStorage.getItem("housekeeper_house_" + houseId);
    if (houseStr) {
      houseData.houses[houseId] = JSON.parse(houseStr);
    }
  }
}

function createHouse(name) {
  const currentHouseId = String(Math.random()).substring(2);
  houseData.currentHouseId = currentHouseId;
  houseData.currentItemId = "";
  houseData.houses[currentHouseId] = { id: "", name, items: [] };
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
}

function saveCurrentHouse() {
  const house = houseData.houses[houseData.currentHouseId];
  localStorage.setItem(
    "housekeeper_house_" + houseData.currentHouseId,
    JSON.stringify(house)
  );
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
    localStorage.removeItem("housekeeper_house_" + houseData.currentHouseId);
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
