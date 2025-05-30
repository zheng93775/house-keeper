const houseData = Vue.reactive({
  currentHouseId: localStorage.getItem("housekeeper_house_current") || "",
  currentItemId: localStorage.getItem("housekeeper_item_current") || "",
  currentHouse: null,
});

function findItemChain(targetItemId) {
  const result = [];
  if (!houseData.currentHouse) return result;
  const findItem = (items) => {
    if (!items) return false;
    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      if (item.id == targetItemId) {
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
}

const navItems = Vue.computed(() => {
  return findItemChain(houseData.currentItemId);
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
        if (response.status === 401) {
          vant.Toast.fail("请先登录后，再进行操作");
          setTimeout(() => {
            window.location.href = "login.html";
          }, 2000);
        } else if (response.status === 409) {
          vant.Toast.fail("已经有其他人修改了数据，请刷新页面");
        }
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
    images: [],
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
        window.location.href = "login.html";
        return;
      }
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    houseData.currentHouse = await response.json();
  } catch (error) {
    console.error("Error fetching houses:", error);
    vant.Toast.fail("获取房屋列表失败: " + error);
    setTimeout(() => {
      window.location.href = "login.html";
    }, 2000);
  }
}

// 处理图片上传
async function handleFileChange(event) {
  const files = event.target.files;
  if (files.length > 0) {
    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      // 验证文件类型是否为图片
      if (!file.type.startsWith("image/")) {
        console.error("请选择图片类型的文件");
        continue;
      }

      const compressedImage = await compressImage(file);
      // 将压缩后的图片转换为 Blob 对象
      const blob = await (await fetch(compressedImage)).blob();
      const formData = new FormData();
      formData.append("image", blob, `image_${Date.now()}.webp`);

      try {
        // 调用服务端的图片上传接口
        const response = await fetch("/api/images", {
          method: "POST",
          body: formData,
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        const imageUrl = `/api/images/${data.file_name}`;

        if (!currentItem.value.images) {
          currentItem.value.images = [];
        }
        currentItem.value.images.push(imageUrl);
      } catch (error) {
        console.error("Error uploading image:", error);
      }
    }
    setTimeout(saveCurrentHouse);
    // 清空文件输入框
    event.target.value = "";
  }
}

// 压缩图片
async function compressImage(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = (event) => {
      const img = new Image();
      img.src = event.target.result;
      img.onload = () => {
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");
        const maxWidth = 800;
        const maxHeight = 600;
        let width = img.width;
        let height = img.height;
        if (width > maxWidth) {
          height = height * (maxWidth / width);
          width = maxWidth;
        }
        if (height > maxHeight) {
          width = width * (maxHeight / height);
          height = maxHeight;
        }
        canvas.width = width;
        canvas.height = height;
        ctx.drawImage(img, 0, 0, width, height);
        canvas.toBlob(
          (blob) => {
            const reader = new FileReader();
            reader.readAsDataURL(blob);
            reader.onload = (event) => {
              resolve(event.target.result);
            };
            reader.onerror = reject;
          },
          "image/webp",
          0.8
        );
      };
      img.onerror = reject;
    };
    reader.onerror = reject;
  });
}

// 删除图片
function deleteImage(index) {
  if (currentItem.value.images) {
    currentItem.value.images.splice(index, 1);
    setTimeout(saveCurrentHouse);
  }
}

function generateTreeData(item) {
  const result = {
    text: item.name,
    value: item.id || item.version,
  };
  if (item.items && item.items.length > 0) {
    result.children = item.items
      .map((child) => generateTreeData(child))
      .filter((child) => child.value != currentItem.value.id);
  }
  return result;
}

if (!houseData.currentHouseId) {
  location.href = "./my-houses.html";
} else {
  fetchHouseDetail();
  const app = Vue.createApp({
    data: () => ({
      addModalVisible: false,
      areaName: "",
      renameModalVisible: false,
      newName: "",
      houseData,
      currentItem,
      navItems,
      // 定义菜单选项
      menuOptions: [
        { text: "创建新的房屋", value: "house-create.html" },
        { text: "我的房屋列表", value: "my-houses.html" },
        { text: "保存", value: "save" },
        { text: "重命名", value: "rename" },
        { text: "添加区域", value: "addArea" },
        { text: "移动", value: "move" },
        { text: "删除", value: "delete" },
      ],
      // 引用文件输入框
      fileInput: null,
      moveModalVisible: false, // 移动对话框显示状态
      selectItemId: null, // 目标选择项
      treeData: [], // 树状选择数据
    }),
    methods: {
      handleAdd() {
        addSubItem(this.areaName);
      },
      handleRename() {
        currentItem.value.name = this.newName;
        setTimeout(saveCurrentHouse);
      },
      // 处理移动选项
      handleMove() {
        if (navItems.value.length <= 1) return;
        this.moveModalVisible = true;
        this.treeData = [generateTreeData(houseData.currentHouse)];
      },
      // 处理移动对话框确认事件
      handleMoveConfirm() {
        // console.log("this.selectItemId", this.selectItemId);
        if (this.selectItemId) {
          const navSize = navItems.value.length;
          if (navSize > 1) {
            const current = currentItem.value;
            // 从原父项中移除当前项
            const parentItem = navItems.value[navSize - 2];
            // console.log("parentItem", parentItem);
            if (parentItem && parentItem.items) {
              parentItem.items = parentItem.items.filter(
                (item) => item.id !== current.id
              );
            }
            // 添加到目标项
            const targetItemChain = findItemChain(this.selectItemId);
            if (targetItemChain.length > 0) {
              const targetParentItem =
                targetItemChain[targetItemChain.length - 1];
              if (!targetParentItem.items) {
                targetParentItem.items = [];
              }
              targetParentItem.items.push(current);
            }
            setTimeout(saveCurrentHouse);
          }
        }
        this.moveModalVisible = false;
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
      // 处理菜单选择事件，添加对“移动”选项的处理
      onMenuChange(value) {
        if (value.endsWith(".html")) {
          location.href = value;
          return;
        }
        switch (value) {
          case "save":
            setTimeout(saveCurrentHouse);
            break;
          case "addArea":
            this.addModalVisible = true;
            this.areaName = "";
            break;
          case "delete":
            this.handleDelete();
            break;
          case "rename":
            this.renameModalVisible = true;
            this.newName = currentItem.value.name;
            break;
          case "move":
            this.handleMove();
            break;
        }
      },
      // 处理图片上传按钮点击事件
      handleUpload() {
        this.$refs.fileInput.click();
      },
      // 处理文件选择事件
      handleFileChange,
      // 删除图片
      deleteImage,
    },
    mounted() {
      this.fileInput = this.$refs.fileInput;
    },
  });
  app.use(vant);
  app.mount("#app");
}
