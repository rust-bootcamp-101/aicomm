.PHONY: chat_server
chat_server:
	@cargo run --bin chat-server

.PHONY: notify_server
notify_server:
	@cargo run --bin notify-server

.PHONY: upgrades
upgrades:
	@echo "查看更新依赖信息"
	@cargo upgrades

.PHONY: upgrade
upgrade:
	@echo "更新依赖"
	@cargo upgrade
