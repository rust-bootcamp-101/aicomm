swiftide-pgvector
作用：
swiftide 是Rust下一个RAG库(RAG: Retrieval Augmented Generation 检索增强生成, 是一种结合了搜寻检索和生成能力的自然语言处理架构。透过这个架构，模型可以从外部知识库搜寻相关信息，然后使用这些信息来生成回应或完成特定的NLP任务。 大白话就是可以从外部加载一些相关知识的文本当做prompt一并提交给大模型， 给大模型提供上下文， 让大模型更好的回答问题)，工具链(类似于Python下的[LangChain](https://python.langchain.com/docs/introduction/), 因为是Rust, 所以提供了类型的支持, 但不一定快， 因为LLM的应用性能瓶颈在token的Output的速度)，但没有包含pgvector，所以，这个crate就是要让swiftide支持pgvector

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
