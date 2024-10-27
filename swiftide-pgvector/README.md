swiftide-pgvector
作用：
swiftide 是Rust下一个RAG库，工具链，但没有包含pgvector，所以，这个crate就是要让swiftide支持pgvector

注意：pgvector 需要额外安装
https://github.com/pgvector/pgvector

create extension vector 有权限问题，需要授权给账号(或超级管理员)
https://stackoverflow.com/questions/16527806/cannot-create-extension-without-superuser-role

In the worst case you can create a new database superuser for PostgreSQL:

```bash
$ createuser --superuser <user_name>
```

or alter an existing database user's role:

```bash
psql -h 127.0.0.1 -p 5432 -d postgres

postgres# ALTER ROLE <user_name> SUPERUSER;
```
