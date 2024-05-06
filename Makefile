# .envファイルのパスを指定する(必要に応じて変更)
ENV_FILE ?= .env

# この変数にMakefileが実行される際のシェルを指定する
SHELL := /bin/bash

# .envファイルから環境変数を読み込む
include $(ENV_FILE)
export $(shell sed 's/=.*//' $(ENV_FILE))

prepare: envload
	cargo sqlx prepare
# デフォルトのターゲット
run: envload
	RUST_BACKTRACE=1 cargo run

# .envファイルから環境変数を読み込む
envload:
	@echo "Loading environment variables from $(ENV_FILE)"
	$(eval export $(shell sed 's/=.*//' $(ENV_FILE)))
	@echo "DATABASE_URL=$(DATABASE_URL)"

migrate: envload
	sqlx migrate run

.PHONY: envload
