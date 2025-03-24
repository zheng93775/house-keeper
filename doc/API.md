# 服务端接口文档

## 登录
### 接口地址
`POST /login`

### 请求参数
| 参数名 | 类型 | 描述 |
| ---- | ---- | ---- |
| username | String | 用户名 |
| password | String | 密码 |

### 请求示例
```json
{
    "username": "zhengjh",
    "password": "smithDay"
}
```

### 响应结果
- 成功 ：
  - 状态码： 200
  - 响应头：设置 Set-Cookie ，包含一个有效期为 1 个月的 token ，且 HttpOnly
  - 响应体：
```json
{
    "username": "zhengjh"
}
 ```

- 失败 ：
  - 状态码：根据具体错误情况而定
  - 错误信息：根据具体错误类型返回相应的错误提示
  - 
### 逻辑说明
如果用户名和密码能在 user.json 中匹配成功，则生成 token 记录到 user.json 中，并在 HTTP 响应中通过 Set-Cookie 写入 token 。后续用户访问和修改房屋数据时，需要校验 Cookie 的合法性。

## 创建新的房屋
### 接口地址
POST /houses

### 请求参数 参数名 类型 描述 name

String

房屋名称
### 请求示例
```json
{
    "name": "我的小屋"
}
 ```

### 响应结果
- 成功 ：
  - 状态码： 201
  - 响应体：
```json
{
    "id": "jjd8ghd",
    "message": "House created successfully"
}
 ```

- 失败 ：
  - 状态码：根据具体错误情况而定
  - 错误信息：根据具体错误类型返回相应的错误提示
### 逻辑说明
在 house.json 文件中添加一条新的记录，同时根据房屋 ID，在 house/{house-id}.json 中创建一个空的数据文件。

## 删除某个房屋
### 接口地址
DELETE /houses/{house-id}

### 请求参数 参数名 类型 描述 house-id

String

房屋 ID
### 请求示例
```plaintext
DELETE /houses/jjd8ghd
 ```

### 响应结果
- 成功 ：
  - 状态码： 200
  - 响应体：根据具体情况返回成功信息
- 失败 ：
  - 状态码：根据具体错误情况而定
  - 错误信息：根据具体错误类型返回相应的错误提示
### 逻辑说明
从 Cookie 中获取当前登录用户，首先校验该房屋的创建人是否为当前登录用户，若为创建人则允许删除。删除操作将把 house.json 里对应的记录移除，并且删除 house/{house-id}.json 对应的文件。

## 设置房屋的成员列表
### 接口地址
PUT /houses/{house-id}/members

### 请求参数 参数名 类型 描述 house-id

String

房屋 ID usernames

Array[String]

成员用户名列表
### 请求示例
```json
{
    "usernames": ["delivery"]
}
 ```

### 响应结果
- 成功 ：
  - 状态码： 200
  - 响应体：根据具体情况返回成功信息
- 失败 ：
  - 状态码：根据具体错误情况而定
  - 错误信息：根据具体错误类型返回相应的错误提示
### 逻辑说明
从 Cookie 中获取当前登录用户，首先校验该房屋的创建人是否为当前登录用户，若为创建人则允许操作。根据用户名列表找到相应的用户，修改 members 字段。

## 查询我的房屋列表
### 接口地址
GET /houses/mine

### 请求参数
无

### 请求示例
```plaintext
GET /houses/mine
 ```

### 响应结果
- 成功 ：
  - 状态码： 200
  - 响应体：房屋列表
```json
[
    {
        "id": "jjd8ghd",
        "name": "我的小屋",
        "creator": "x9hgnd",
        "members": [
            {
                "userId": "hgjerg",
                "username": "delivery"
            }
        ]
    }
]
 ```

- 失败 ：
  - 状态码：根据具体错误情况而定
  - 错误信息：根据具体错误类型返回相应的错误提示
### 逻辑说明
从 Cookie 中获取对应的用户数据，根据用户 ID 在 house.json 中找到创建人是该用户，或者成员是该用户的房屋，返回房屋列表。

## 查询某个房屋的详细数据
### 接口地址
GET /houses/{house-id}/detail

### 请求参数 参数名 类型 描述 house-id

String

房屋 ID
### 请求示例
```plaintext
GET /houses/jjd8ghd/detail
 ```

### 响应结果
- 成功 ：
  - 状态码： 200
  - 响应体：房屋详细数据
```json
{
    "version": "xjdfhnnnd",
    "data": [
        {
            "name": "客厅",
            "content": "沙发，茶几",
            "children": [
                {
                    "name": "电视柜",
                    "content": "电视机、吹风机",
                    "children": []
                }
            ]
        }
    ]
}
```

- 失败 ：
  - 状态码：根据具体错误情况而定
  - 错误信息：根据具体错误类型返回相应的错误提示
### 逻辑说明
从 Cookie 中获取对应的用户数据，判断当前登录用户是否为房屋的创建人或成员，若是则允许查询房屋的详细数据，将 house/{house-id}.json 里存的数据返回。

## 修改某个房屋的详细数据
### 接口地址
PUT /houses/{house-id}/detail

### 请求参数 参数名 类型 描述 house-id

String

房屋 ID detail

Object

详细数据 version

String

版本号
### 请求示例
```json
{
    "detail": {
        "version": "xjdfhnnnd",
        "data": [
            {
                "name": "客厅",
                "content": "沙发，茶几，新物品",
                "children": [
                    {
                        "name": "电视柜",
                        "content": "电视机、吹风机",
                        "children": []
                    }
                ]
            }
        ]
    },
    "version": "xjdfhnnnd"
}
 ```
```

### 响应结果
- 成功 ：
  - 状态码： 200
  - 响应体：新的版本号
```json
{
    "version": "new-version"
}
 ```

- 失败 ：
  - 状态码：根据具体错误情况而定
  - 错误信息：根据具体错误类型返回相应的错误提示
### 逻辑说明
从 Cookie 中获取对应的用户数据，判断当前登录用户是否为房屋的创建人或成员。修改时 version 必须要能匹配得上，生成一个新的 version ，将新的数据和新 version 写入 house/{house-id}.json ，返回新的 version 给前端。下一次修改必须要带上最新的 version 进行修改，以避免多个用户同时修改一个房屋详细数据时被覆盖的问题。

## 上传图片
### 接口地址
POST /images

### 请求参数 参数名 类型 描述 file

Binary

图片文件
### 请求示例
使用表单上传图片文件

### 响应结果
- 成功 ：
  - 状态码： 200
  - 响应体：图片文件名
```json
{
    "filename": "xxxxxxxyyyyyyyyy.jpg"
}
 ```

- 失败 ：
  - 状态码：根据具体错误情况而定
  - 错误信息：根据具体错误类型返回相应的错误提示
### 逻辑说明
校验登录用户，将图片上传至 images 目录，文件名改成随机的文件名，将文件名返回给前端。