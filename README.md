## Integration rbatis with actix web


```
https://salvo.rs/zh-hans/ 
```

1. Create mysql table with `blog.sql` in project root
2. Run `cargo run`


3. Create blog

```
curl -X "POST" "http://127.0.0.1:9991" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "title": "test create title",
  "description": "test create description"
}'
```

4. Update user

```
curl -X "PUT" "http://127.0.0.1:9991?id=15" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "title": "test update title",
  "description": "test update description"
}'
```

5. Delete user

```
curl -X "DELETE" "http://127.0.0.1:9991?id=15" \
     -H 'Content-Type: application/json; charset=utf-8'
```


6. Show user

```
curl -X "GET" "http://127.0.0.1:9991?id=15" \
     -H 'Content-Type: application/json; charset=utf-8'
```



