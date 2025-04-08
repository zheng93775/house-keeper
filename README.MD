# 项目介绍

本项目旨在开发一个用于记录房屋所有区域和物品清单的备忘录，是一个Web系统，只提供给少量用户使用。

## 服务端

使用Rust语言较新的版本开发，部署在内存非常紧缺的服务器上，所以代码实现中应尽量不占用太多内存。

### 数据存储

数据打算存储在服务器的某个目录下，从系统环境变量中获取访问数据存储目录的路径，目录下有以下文件

user.json --- 存储用户数据，json数组
house.json --- 存储房屋列表数据，json数据
house --- 房屋目录
house/{house-id}.json --- 某个房屋的数据，json对象
images --- 图片目录
images/xxxxxxxyyyyyyyyy.jpg --- 图片

#### user.json 数据示例

其中密码是明文存储，此系统不需要关注安全问题，只面向少数人使用

```
[
  { id: 'x9hgnd', username: 'zhengjh', password: 'smithDay', token: 'xxxxxxxxxxxxxxxxx' },
  { id: 'hgjerg', username: 'delivery', password: 'tianzhen', token: 'xxxxxxxxxxxxxxxxx' },
]
```

#### house.json 数据示例

```
[
  {
    id: 'jjd8ghd',
    name: '我的小屋',
    creator: 'x9hgnd',
    members: [
      { userId: 'hgjerg', username: 'delivery' }
    ]
  }
]
```

#### house/{house-id}.json 数据示例

```
{
  version: 'xjdfhnnnd',
  data: [
    {
      name: "客厅",
      content: "沙发，茶几",
      items: [
        {
          name: "电视柜",
          content: "电视机、吹风机",
          items: []
        }
      ]
    }
  ]
}
```

### 数据备份

每天凌晨3点执行数据备份，备份目录由环境变量指定。进行备份前，先对比和以前的备份是否有差异，比如备份user.json，先找到最近的一个备份，比如是user.20241102.json，如果当前user.json有变更，那么备份至user.{current_date}.json；如果没有变更，则不进行备份。需要备份的数据文件是所有的json文件，images不需要备份。

### 接口

#### 登录

POST 用户名、密码。如果用户名密码能在user.json中匹配成功，则生成token记录到user.json中，在http响应中通过Set-Cookie写入时间长达1个月的Cookie，HttpOnly。后续用户访问和修改房屋数据时，需要校验Cookie的合法性，传的Cookie需要在user.json中能查询到相应的用户信息。

#### 创建新的房屋

传参房屋名称，house.json文件中添加一条新的记录，同时根据房屋ID，在house/{house-id}.json中创建一个空的数据文件

#### 删除某个房屋

传参房屋ID，从Cookie中能找到当前的登录用户，首先校验该房屋的创建人就是当前登录用户，才允许进行删除。删除将把house.json里对应的记录移除，并且删除house/{house-id}.json对应的文件。

#### 设置房屋的成员列表

传参房屋ID、成员用户名列表，从Cookie中能找到当前的登录用户，首先校验该房屋的创建人就是当前登录用户，才允许操作。根据用户名列表找到相应的用户，修改members字段。

#### 查询我的房屋列表

无需传参，从Cookie中能找到对应的用户数据，根据用户ID在house.json中找到创建人是该用户，或者成员是该用户的房屋。返回房屋列表。

#### 查询某个房屋的详细数据

传参房屋ID，从Cookie中能找到对应的用户数据，判断当前登录用户是房屋的创建人或成员，允许查询房屋的详细数据，将 house/{house-id}.json 里存的数据返回

#### 修改某个房屋的详细数据

传参房屋ID和详细数据，从Cookie中能找到对应的用户数据，判断当前登录用户是房屋的创建人或成员。修改时version必须要能匹配得上，生成一个新的version，house/{house-id}.json 写入新的数据和新version，返回version给前端。下一次修改必须要带上最新的version进行修改，这个方式是为了避免多个用户同时修改一个房屋详细，修改被覆盖的问题，通过version限制被别人修改过的文件就不能脏覆盖。提示用户数据已被修改，应刷新页面后再重新修改。

#### 上传图片

校验登录用户，将图片上传至images目录，文件名应改成随机的文件名，将文件名返回给前端

#### 下载图片

校验登录用户，根据文件名从images目录下载图片

### 静态目录

根据环境变量获取静态目录的路径，比如 /data/static 。该目录放着所有前端的文件，包括html、js、css、图片等静态资源。服务启动后，将提供web服务功能，比如请求/index.html，则返回/data/static/index.html文件的内容；如果访问/js/jquery.js，则返回/data/static/js/jquery.js。

## 前端

前端页面待定