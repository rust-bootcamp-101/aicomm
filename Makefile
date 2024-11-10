DOCKER=podman
PWD=$(shell pwd)

.PHONY: build-docker
build-docker:
	$(DOCKER) build -t chat-server:latest --build-arg APP_NAME=chat-server --env OPENAI_API_KEY=$(OPENAI_API_KEY) --build-arg APP_PORT=6688 .
	$(DOCKER) build -t chat-server:latest --build-arg APP_NAME=notify-server --build-arg APP_PORT=6687 .
	$(DOCKER) build -t chat-server:latest --build-arg APP_NAME=bot --env OPENAI_API_KEY=$(OPENAI_API_KEY) --build-arg APP_PORT=6686 .
	$(DOCKER) build -t chat-server:latest --build-arg APP_NAME=analytics-server --build-arg APP_PORT=6690 .

.PHONY: run-docker
run-docker: kill-dockers
	$(DOCKER) container prune -f
	# 运行容器时挂载本地的配置文件到容器里面
	$(DOCKER) run --name chat -d -p 6688:6688 --mount type=bind,source=$(PWD)/fixtures/chat.yml,target=/app/chat.yml,readonly localhost/chat-server:latest

.PHONY: kill-dockers
kill-dockers:
	@$(DOCKER) kill $(shell $(DOCKER) ps -aq) || true
	@$(DOCKER) container prune -f || true
