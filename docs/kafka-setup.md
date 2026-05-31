# Local Kafka Environment Setup Guide

## 概述

本文档描述如何在本机使用 Docker 部署 Kafka 环境，用于 `charge-mgt-gateway` 开发和测试。

## 前置要求

### 1.1 系统要求

- macOS / Linux / Windows (with WSL2)
- Docker 20.10+
- Docker Compose 2.0+

### 1.2 验证安装

```bash
# 检查 Docker 版本
docker --version

# 检查 Docker Compose 版本
docker compose version
```

### 1.3 Docker 镜像加速配置（可选）

如果拉取镜像速度慢，可配置国内镜像源：

1. 打开 Docker Desktop → Settings → Docker Engine
2. 添加以下配置：

```json
{
  "registry-mirrors": [
    "https://docker.mirrors.ustc.edu.cn",
    "https://hub-mirror.c.163.com"
  ]
}
```

3. Apply & Restart

---

## Docker Compose 配置

### 2.1 配置文件

项目根目录已有 `docker-compose.yml`，使用 **Apache Kafka** 镜像：

```yaml
services:
  kafka:
    image: apache/kafka:3.8.0
    container_name: kafka
    ports:
      - "9092:9092"
    environment:
      KAFKA_NODE_ID: 1
      KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT
      KAFKA_LISTENERS: PLAINTEXT://:9092,CONTROLLER://:9093
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_CONTROLLER_LISTENER_NAMES: CONTROLLER
      KAFKA_CONTROLLER_QUORUM_VOTERS: 1@kafka:9093
      KAFKA_PROCESS_ROLES: controller,broker
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
      KAFKA_AUTO_CREATE_TOPICS_ENABLE: "true"
      KAFKA_GROUP_INITIAL_REBALANCE_DELAY_MS: 0
    networks:
      - kafka-network

  kafka-ui:
    image: provectuslabs/kafka-ui:latest
    container_name: kafka-ui
    ports:
      - "8080:8080"
    environment:
      KAFKA_CLUSTERS_0_NAME: local
      KAFKA_CLUSTERS_0_BOOTSTRAPSERVERS: kafka:9092
    depends_on:
      - kafka
    networks:
      - kafka-network

networks:
  kafka-network:
    driver: bridge
```

### 2.2 配置说明

| 镜像 | 版本 | 说明 |
|------|------|------|
| `apache/kafka` | 3.8.0 | Apache 官方 Kafka 镜像，使用 KRaft 模式（无需 Zookeeper） |
| `provectuslabs/kafka-ui` | latest | Web 管理界面（可选） |

| 端口 | 用途 |
|------|------|
| 9092 | 宿主机访问 Kafka broker |
| 8080 | Kafka UI Web 界面 |

### 2.3 为什么选择 Apache Kafka

| 对比项 | Apache Kafka | 其他发行版 |
|--------|-------------|-----------|
| 官方支持 | ✅ Apache 官方 | ❌ 第三方维护 |
| KRaft 模式 | ✅ 支持 | 部分支持 |
| 镜像可用性 | ✅ 国内可拉取 | 部分不可用 |
| 维护状态 | 活跃 | 不一致 |

---

## 启动与停止

### 3.1 启动服务

```bash
# 在项目根目录执行
docker compose up -d

# 等待服务启动
docker compose ps
```

**预期输出**：
```
NAME       IMAGE                           STATUS
kafka      apache/kafka:3.8.0             running
kafka-ui   provectuslabs/kafka-ui:latest  running
```

### 3.2 停止服务

```bash
docker compose down

# 如果需要删除数据卷（清空所有数据）
docker compose down -v
```

### 3.3 重启服务

```bash
docker compose restart
```

### 3.4 查看日志

```bash
# 查看所有服务日志
docker compose logs

# 查看 Kafka 日志
docker compose logs -f kafka

# 查看 Kafka UI 日志
docker compose logs -f kafka-ui
```

---

## 验证部署

### 4.1 Kafka UI 验证

打开浏览器访问：http://localhost:8080

检查：
- [ ] Kafka 集群状态为 `Alive`
- [ ] Broker 数量为 1
- [ ] 可以查看 Topics 列表

### 4.2 命令行验证

```bash
# 进入 Kafka 容器
docker exec -it kafka /bin/bash

# 列出所有 topic（Kafka 启动后需要等待几秒）
/opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --list

# 创建测试 topic
/opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic test --partitions 1 --replication-factor 1

# 验证 topic 创建成功
/opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --describe --topic test
```

### 4.3 生产者/消费者测试

```bash
# 在 Kafka 容器内执行

# 打开生产者
/opt/kafka/bin/kafka-console-producer.sh --bootstrap-server localhost:9092 --topic test

# 新开一个终端，打开消费者
/opt/kafka/bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic test --from-beginning

# 在生产者端输入消息，消费者端应该能收到
```

---

## Gateway 配置

### 5.1 Gateway 配置

编辑 `crates/charge-mgt-gateway/config/default.yaml`：

```yaml
gateway:
  id: "gateway-01"
  host: "127.0.0.1"

device:
  listen_addr: "0.0.0.0"
  listen_port: 9000

cloud:
  api_url: "https://cloud.example.com"
  api_key: "test_key"

kafka:
  brokers: "localhost:9092"
  topic_prefix: "charge_mgt"
```

### 5.2 Kafka Topic 预期

当 Gateway 运行时，会自动创建以下 Topic：

| Topic | 用途 |
|-------|------|
| `charge_mgt.Alphas` | Alphas 厂商充电桩消息 |
| `charge_mgt.Star` | Star 厂商充电桩消息 |
| `charge_mgt.unknown` | 未知厂商充电桩消息 |

---

## 常见问题

### Q1: 镜像拉取失败 "not found"

**原因**：无法连接 Docker Hub 或镜像不存在

**解决**：
1. 配置国内镜像加速器（见 1.3 节）
2. 或检查网络代理设置
3. 尝试使用其他可用镜像

### Q2: 端口被占用

```bash
# 检查端口占用
lsof -i :9092
lsof -i :8080

# 杀死占用进程或修改 docker-compose.yml 端口映射
```

### Q3: Kafka UI 无法连接 Kafka

**检查**：
```bash
# 确认 Kafka 健康状态
docker compose ps

# 查看 Kafka 日志
docker compose logs kafka
```

### Q4: 消息无法发送

**检查**：
1. Kafka 是否正常运行 (`docker compose ps`)
2. Gateway 是否连接的是 `localhost:9092`
3. 查看 Gateway 日志中的 `[KAFKA]` 标识

### Q5: Kafka 启动失败

**原因**：KRaft 模式配置问题

**解决**：
1. 确保 `KAFKA_PROCESS_ROLES` 和 `KAFKA_CONTROLLER_QUORUM_VOTERS` 配置正确
2. 参考 [Apache Kafka Docker 文档](https://hub.docker.com/r/apache/kafka)

---

## 生产级部署注意

当前配置仅适用于**本地开发测试**，生产环境需要：

| 项目 | 开发配置 | 生产配置 |
|------|----------|----------|
| Broker 数量 | 1 | 3+ |
| 副本因子 | 1 | 3 |
| 安全认证 | 无 | SASL/SSL |
| 分区数 | 1 | 根据负载调整 |
| 持久化 | 临时卷 | 持久化存储 |
| 模式 | KRaft | KRaft 或 Zookeeper |

---

## 参考链接

- [Apache Kafka Docker 镜像](https://hub.docker.com/r/apache/kafka)
- [Apache Kafka 官方文档](https://kafka.apache.org/documentation/)
- [Kafka KRaft 模式文档](https://kafka.apache.org/documentation/#kraft)
- [Kafka UI 项目](https://github.com/provectus/kafka-ui)
- [rdkafka Rust 客户端](https://github.com/fede1024/rust-rdkafka)