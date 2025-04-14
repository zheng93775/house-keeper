const houseData = Vue.reactive({
  currentHouseId: localStorage.getItem("housekeeper_house_current") || "",
  currentHouse: null,
});

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
      keyword: "",
      emptySearchResult: false,
      resultItems: [],
      houseData,
    }),
    methods: {
      handleSearch() {
        this.keyword = this.keyword.trim();
        this.resultItems = [];
        const searchByKeyword = (item, parentNavName) => {
          // console.log('searchByKeyword', this.keyword, item.name, parentNavName);
          const navName = parentNavName + "/" + item.name;
          if (
            (item.name && item.name.includes(this.keyword)) ||
            (item.content && item.content.includes(this.keyword))
          ) {
            // console.log('includes', item.name.includes(this.keyword), item.content.includes(this.keyword));
            this.resultItems.push({
              id: item.id,
              navName,
              content: item.content
                ? item.content.replaceAll(
                    this.keyword,
                    `<span class="text-red-500">${this.keyword}</span>`
                  )
                : "",
            });
          }
          if (item.items) {
            item.items.forEach((childItem) =>
              searchByKeyword(childItem, navName)
            );
          }
        };
        if (this.keyword) {
          searchByKeyword(this.houseData.currentHouse, "");
          this.emptySearchResult = this.resultItems.length == 0;
        } else {
          this.emptySearchResult = true;
        }
        console.log("this.resultItems", this.resultItems);
      },
      handleSelect(item) {
        localStorage.setItem("housekeeper_item_current", item.id);
        location.href = "index.html";
      },
    },
  });
  app.use(vant);
  app.mount("#app");
}
