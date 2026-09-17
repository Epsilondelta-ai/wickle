# 전체 파일과 공개 API 찾아보기

[목차](README.md) · [아키텍처](02b-architecture.md)

최종 release에는 참조 파일 313개와 Rust 파일 226개가 있다. 아래 모든 Rust 파일을 구현 강의와 연결했다. 이 표의 소스 링크는 교재를 저장소에서 읽을 때 사용할 수 있다. 교재 폴더만 받은 경우 `lab.py snapshot 36 --dest 새폴더`로 같은 경로를 복원한다. 중간 단계는 각 장의 전체 구현 문서가 기준이다.

## 모듈 책임으로 읽는 순서

| 파일 묶음 | 읽을 때 답할 질문 |
| --- | --- |
| serialization/error/profile/resolution | 어떤 데이터가 유효하고 버전은 어디서 고정되는가? |
| context/message/run/views | 저장할 사실, 실행 주체, 공개할 정보가 어떻게 다른가? |
| policy/tool_schema/input_binding | 모델 소유 입력과 Host 권한은 어디서 구분되는가? |
| state와 state 하위 모듈 | 어떤 변경을 원자적으로 검사하고 저장하는가? |
| budget/clock | 외부 호출을 언제 예약하고 어떻게 시간을 재는가? |
| model_protocol/model_execution/model_dispatch/model_routing | 파싱·선택·실행의 책임은 어떻게 다른가? |
| agent와 agent 하위 모듈 | 실행·재개·복구·정리에서 저장 경계는 어디인가? |
| tool_execution와 하위 모듈 | 결과 오류와 외부 효과를 어떻게 분리하는가? |
| hooks/component_runtime | 확장을 어디까지 허용하며 누가 자원을 소유하는가? |
| context_source/context_strategy/context_projection | 원본·활성 자료·모델 표현을 어떻게 분리하는가? |
| skills/artifacts/verification | 지시·큰 원본·검증 근거를 어떻게 보존하는가? |
| adapter crates | 공통 계약을 외부 protocol/저장소로 어떻게 옮기는가? |
| tests와 tests/support | 어떤 실패를 실제로 발생시키며 callback 수를 어떻게 확인하는가? |

## 모든 Rust 파일의 구현 경로

| 최종 파일 | 최초 구현 장 | 마지막 변경 장 | 줄 수 |
| --- | --- | --- | --- |
| [crates/wickle-adapter-runtime/src/lib.rs](../../../crates/wickle-adapter-runtime/src/lib.rs) | [19](19-adapters.md) | [20](20-sources.md) | 14 |
| [crates/wickle-adapter-runtime/src/lifecycle.rs](../../../crates/wickle-adapter-runtime/src/lifecycle.rs) | [19](19-adapters.md) | [24](24-recovery.md) | 267 |
| [crates/wickle-adapter-runtime/src/registry.rs](../../../crates/wickle-adapter-runtime/src/registry.rs) | [19](19-adapters.md) | [20](20-sources.md) | 517 |
| [crates/wickle-adapter-runtime/src/runtime.rs](../../../crates/wickle-adapter-runtime/src/runtime.rs) | [19](19-adapters.md) | [20](20-sources.md) | 671 |
| [crates/wickle-adapter-runtime/tests/agent_components.rs](../../../crates/wickle-adapter-runtime/tests/agent_components.rs) | [19](19-adapters.md) | [20](20-sources.md) | 548 |
| [crates/wickle-adapter-runtime/tests/context_sources.rs](../../../crates/wickle-adapter-runtime/tests/context_sources.rs) | [20](20-sources.md) | [24](24-recovery.md) | 446 |
| [crates/wickle-adapter-runtime/tests/hook_exports.rs](../../../crates/wickle-adapter-runtime/tests/hook_exports.rs) | [19](19-adapters.md) | [19](19-adapters.md) | 309 |
| [crates/wickle-adapter-runtime/tests/runtime_lifecycle.rs](../../../crates/wickle-adapter-runtime/tests/runtime_lifecycle.rs) | [19](19-adapters.md) | [24](24-recovery.md) | 524 |
| [crates/wickle-adapter-runtime/tests/runtime_registry.rs](../../../crates/wickle-adapter-runtime/tests/runtime_registry.rs) | [19](19-adapters.md) | [20](20-sources.md) | 321 |
| [crates/wickle-adapter-runtime/tests/support/context_sources.rs](../../../crates/wickle-adapter-runtime/tests/support/context_sources.rs) | [20](20-sources.md) | [24](24-recovery.md) | 453 |
| [crates/wickle-adapter-runtime/tests/support/mod.rs](../../../crates/wickle-adapter-runtime/tests/support/mod.rs) | [19](19-adapters.md) | [24](24-recovery.md) | 746 |
| [crates/wickle-mcp/src/client.rs](../../../crates/wickle-mcp/src/client.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 415 |
| [crates/wickle-mcp/src/executor.rs](../../../crates/wickle-mcp/src/executor.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 183 |
| [crates/wickle-mcp/src/factory.rs](../../../crates/wickle-mcp/src/factory.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 216 |
| [crates/wickle-mcp/src/lib.rs](../../../crates/wickle-mcp/src/lib.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 15 |
| [crates/wickle-mcp/src/snapshot.rs](../../../crates/wickle-mcp/src/snapshot.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 228 |
| [crates/wickle-mcp/src/transport.rs](../../../crates/wickle-mcp/src/transport.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 151 |
| [crates/wickle-mcp/tests/factory.rs](../../../crates/wickle-mcp/tests/factory.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 259 |
| [crates/wickle-mcp/tests/stdio.rs](../../../crates/wickle-mcp/tests/stdio.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 445 |
| [crates/wickle-mcp/tests/support/binding.rs](../../../crates/wickle-mcp/tests/support/binding.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 260 |
| [crates/wickle-mcp/tests/support/mod.rs](../../../crates/wickle-mcp/tests/support/mod.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 161 |
| [crates/wickle-model-anthropic/src/codec.rs](../../../crates/wickle-model-anthropic/src/codec.rs) | [27](27-anthropic.md) | [28](28-bedrock.md) | 359 |
| [crates/wickle-model-anthropic/src/connection.rs](../../../crates/wickle-model-anthropic/src/connection.rs) | [27](27-anthropic.md) | [27](27-anthropic.md) | 190 |
| [crates/wickle-model-anthropic/src/inspection.rs](../../../crates/wickle-model-anthropic/src/inspection.rs) | [27](27-anthropic.md) | [27](27-anthropic.md) | 143 |
| [crates/wickle-model-anthropic/src/lib.rs](../../../crates/wickle-model-anthropic/src/lib.rs) | [27](27-anthropic.md) | [28](28-bedrock.md) | 20 |
| [crates/wickle-model-anthropic/src/model.rs](../../../crates/wickle-model-anthropic/src/model.rs) | [27](27-anthropic.md) | [27](27-anthropic.md) | 263 |
| [crates/wickle-model-anthropic/src/response.rs](../../../crates/wickle-model-anthropic/src/response.rs) | [27](27-anthropic.md) | [28](28-bedrock.md) | 370 |
| [crates/wickle-model-anthropic/tests/messages.rs](../../../crates/wickle-model-anthropic/tests/messages.rs) | [27](27-anthropic.md) | [34](34-evidence.md) | 569 |
| [crates/wickle-model-anthropic/tests/support/mod.rs](../../../crates/wickle-model-anthropic/tests/support/mod.rs) | [27](27-anthropic.md) | [27](27-anthropic.md) | 98 |
| [crates/wickle-model-azure-openai/src/auth.rs](../../../crates/wickle-model-azure-openai/src/auth.rs) | [26](26-azure.md) | [26](26-azure.md) | 96 |
| [crates/wickle-model-azure-openai/src/connection.rs](../../../crates/wickle-model-azure-openai/src/connection.rs) | [26](26-azure.md) | [26](26-azure.md) | 197 |
| [crates/wickle-model-azure-openai/src/inspection.rs](../../../crates/wickle-model-azure-openai/src/inspection.rs) | [26](26-azure.md) | [26](26-azure.md) | 214 |
| [crates/wickle-model-azure-openai/src/lib.rs](../../../crates/wickle-model-azure-openai/src/lib.rs) | [26](26-azure.md) | [26](26-azure.md) | 15 |
| [crates/wickle-model-azure-openai/src/model.rs](../../../crates/wickle-model-azure-openai/src/model.rs) | [26](26-azure.md) | [26](26-azure.md) | 290 |
| [crates/wickle-model-azure-openai/tests/responses.rs](../../../crates/wickle-model-azure-openai/tests/responses.rs) | [26](26-azure.md) | [34](34-evidence.md) | 561 |
| [crates/wickle-model-azure-openai/tests/support/mod.rs](../../../crates/wickle-model-azure-openai/tests/support/mod.rs) | [26](26-azure.md) | [26](26-azure.md) | 106 |
| [crates/wickle-model-bedrock/src/auth.rs](../../../crates/wickle-model-bedrock/src/auth.rs) | [28](28-bedrock.md) | [28](28-bedrock.md) | 241 |
| [crates/wickle-model-bedrock/src/connection.rs](../../../crates/wickle-model-bedrock/src/connection.rs) | [28](28-bedrock.md) | [28](28-bedrock.md) | 295 |
| [crates/wickle-model-bedrock/src/framing.rs](../../../crates/wickle-model-bedrock/src/framing.rs) | [28](28-bedrock.md) | [28](28-bedrock.md) | 143 |
| [crates/wickle-model-bedrock/src/inspection.rs](../../../crates/wickle-model-bedrock/src/inspection.rs) | [28](28-bedrock.md) | [28](28-bedrock.md) | 246 |
| [crates/wickle-model-bedrock/src/lib.rs](../../../crates/wickle-model-bedrock/src/lib.rs) | [28](28-bedrock.md) | [28](28-bedrock.md) | 20 |
| [crates/wickle-model-bedrock/src/model.rs](../../../crates/wickle-model-bedrock/src/model.rs) | [28](28-bedrock.md) | [28](28-bedrock.md) | 336 |
| [crates/wickle-model-bedrock/tests/bedrock.rs](../../../crates/wickle-model-bedrock/tests/bedrock.rs) | [28](28-bedrock.md) | [34](34-evidence.md) | 696 |
| [crates/wickle-model-bedrock/tests/support/mod.rs](../../../crates/wickle-model-bedrock/tests/support/mod.rs) | [28](28-bedrock.md) | [28](28-bedrock.md) | 127 |
| [crates/wickle-model-gemini/src/codec.rs](../../../crates/wickle-model-gemini/src/codec.rs) | [29](29-gemini.md) | [34](34-evidence.md) | 445 |
| [crates/wickle-model-gemini/src/connection.rs](../../../crates/wickle-model-gemini/src/connection.rs) | [29](29-gemini.md) | [29](29-gemini.md) | 197 |
| [crates/wickle-model-gemini/src/inspection.rs](../../../crates/wickle-model-gemini/src/inspection.rs) | [29](29-gemini.md) | [29](29-gemini.md) | 162 |
| [crates/wickle-model-gemini/src/lib.rs](../../../crates/wickle-model-gemini/src/lib.rs) | [29](29-gemini.md) | [30](30-vertex.md) | 19 |
| [crates/wickle-model-gemini/src/model.rs](../../../crates/wickle-model-gemini/src/model.rs) | [29](29-gemini.md) | [30](30-vertex.md) | 233 |
| [crates/wickle-model-gemini/src/response.rs](../../../crates/wickle-model-gemini/src/response.rs) | [29](29-gemini.md) | [30](30-vertex.md) | 242 |
| [crates/wickle-model-gemini/tests/generate.rs](../../../crates/wickle-model-gemini/tests/generate.rs) | [29](29-gemini.md) | [34](34-evidence.md) | 672 |
| [crates/wickle-model-gemini/tests/support/mod.rs](../../../crates/wickle-model-gemini/tests/support/mod.rs) | [29](29-gemini.md) | [29](29-gemini.md) | 102 |
| [crates/wickle-model-openai/src/connection.rs](../../../crates/wickle-model-openai/src/connection.rs) | [25](25-openai.md) | [25](25-openai.md) | 201 |
| [crates/wickle-model-openai/src/inspection.rs](../../../crates/wickle-model-openai/src/inspection.rs) | [25](25-openai.md) | [25](25-openai.md) | 143 |
| [crates/wickle-model-openai/src/lib.rs](../../../crates/wickle-model-openai/src/lib.rs) | [25](25-openai.md) | [26](26-azure.md) | 20 |
| [crates/wickle-model-openai/src/model.rs](../../../crates/wickle-model-openai/src/model.rs) | [25](25-openai.md) | [26](26-azure.md) | 262 |
| [crates/wickle-model-openai/tests/responses.rs](../../../crates/wickle-model-openai/tests/responses.rs) | [25](25-openai.md) | [34](34-evidence.md) | 615 |
| [crates/wickle-model-openai/tests/support/mod.rs](../../../crates/wickle-model-openai/tests/support/mod.rs) | [25](25-openai.md) | [26](26-azure.md) | 99 |
| [crates/wickle-model-responses/src/codec.rs](../../../crates/wickle-model-responses/src/codec.rs) | [26](26-azure.md) | [31](31-xai.md) | 443 |
| [crates/wickle-model-responses/src/lib.rs](../../../crates/wickle-model-responses/src/lib.rs) | [26](26-azure.md) | [31](31-xai.md) | 13 |
| [crates/wickle-model-responses/src/response.rs](../../../crates/wickle-model-responses/src/response.rs) | [26](26-azure.md) | [31](31-xai.md) | 558 |
| [crates/wickle-model-responses/src/sse.rs](../../../crates/wickle-model-responses/src/sse.rs) | [26](26-azure.md) | [26](26-azure.md) | 159 |
| [crates/wickle-model-router/src/dispatcher.rs](../../../crates/wickle-model-router/src/dispatcher.rs) | [14](14-routing.md) | [14](14-routing.md) | 107 |
| [crates/wickle-model-router/src/lib.rs](../../../crates/wickle-model-router/src/lib.rs) | [12](12-catalog.md) | [14](14-routing.md) | 142 |
| [crates/wickle-model-router/src/routing.rs](../../../crates/wickle-model-router/src/routing.rs) | [14](14-routing.md) | [14](14-routing.md) | 191 |
| [crates/wickle-model-router/tests/catalog.rs](../../../crates/wickle-model-router/tests/catalog.rs) | [12](12-catalog.md) | [12](12-catalog.md) | 884 |
| [crates/wickle-model-router/tests/dispatcher.rs](../../../crates/wickle-model-router/tests/dispatcher.rs) | [14](14-routing.md) | [14](14-routing.md) | 363 |
| [crates/wickle-model-router/tests/routed_execution.rs](../../../crates/wickle-model-router/tests/routed_execution.rs) | [14](14-routing.md) | [23](23-verification.md) | 766 |
| [crates/wickle-model-router/tests/routing.rs](../../../crates/wickle-model-router/tests/routing.rs) | [14](14-routing.md) | [14](14-routing.md) | 748 |
| [crates/wickle-model-router/tests/support/routed.rs](../../../crates/wickle-model-router/tests/support/routed.rs) | [14](14-routing.md) | [21](21-skills.md) | 657 |
| [crates/wickle-model-vertex/src/auth.rs](../../../crates/wickle-model-vertex/src/auth.rs) | [30](30-vertex.md) | [30](30-vertex.md) | 60 |
| [crates/wickle-model-vertex/src/connection.rs](../../../crates/wickle-model-vertex/src/connection.rs) | [30](30-vertex.md) | [30](30-vertex.md) | 274 |
| [crates/wickle-model-vertex/src/inspection.rs](../../../crates/wickle-model-vertex/src/inspection.rs) | [30](30-vertex.md) | [30](30-vertex.md) | 169 |
| [crates/wickle-model-vertex/src/lib.rs](../../../crates/wickle-model-vertex/src/lib.rs) | [30](30-vertex.md) | [30](30-vertex.md) | 14 |
| [crates/wickle-model-vertex/src/model.rs](../../../crates/wickle-model-vertex/src/model.rs) | [30](30-vertex.md) | [30](30-vertex.md) | 249 |
| [crates/wickle-model-vertex/tests/support/mod.rs](../../../crates/wickle-model-vertex/tests/support/mod.rs) | [30](30-vertex.md) | [30](30-vertex.md) | 102 |
| [crates/wickle-model-vertex/tests/vertex.rs](../../../crates/wickle-model-vertex/tests/vertex.rs) | [30](30-vertex.md) | [34](34-evidence.md) | 574 |
| [crates/wickle-model-xai/src/connection.rs](../../../crates/wickle-model-xai/src/connection.rs) | [31](31-xai.md) | [31](31-xai.md) | 184 |
| [crates/wickle-model-xai/src/inspection.rs](../../../crates/wickle-model-xai/src/inspection.rs) | [31](31-xai.md) | [31](31-xai.md) | 143 |
| [crates/wickle-model-xai/src/lib.rs](../../../crates/wickle-model-xai/src/lib.rs) | [31](31-xai.md) | [31](31-xai.md) | 12 |
| [crates/wickle-model-xai/src/model.rs](../../../crates/wickle-model-xai/src/model.rs) | [31](31-xai.md) | [31](31-xai.md) | 232 |
| [crates/wickle-model-xai/tests/responses.rs](../../../crates/wickle-model-xai/tests/responses.rs) | [31](31-xai.md) | [34](34-evidence.md) | 583 |
| [crates/wickle-model-xai/tests/support/mod.rs](../../../crates/wickle-model-xai/tests/support/mod.rs) | [31](31-xai.md) | [31](31-xai.md) | 101 |
| [crates/wickle-state-sqlite/src/lib.rs](../../../crates/wickle-state-sqlite/src/lib.rs) | [13](13-sqlite.md) | [35](35-integration.md) | 627 |
| [crates/wickle-state-sqlite/tests/agent_recovery.rs](../../../crates/wickle-state-sqlite/tests/agent_recovery.rs) | [24](24-recovery.md) | [25](25-openai.md) | 566 |
| [crates/wickle-state-sqlite/tests/host_contract.rs](../../../crates/wickle-state-sqlite/tests/host_contract.rs) | [33](33-events.md) | [33](33-events.md) | 3 |
| [crates/wickle-state-sqlite/tests/state_store.rs](../../../crates/wickle-state-sqlite/tests/state_store.rs) | [13](13-sqlite.md) | [35](35-integration.md) | 1090 |
| [crates/wickle-state-sqlite/tests/support/context_recovery.rs](../../../crates/wickle-state-sqlite/tests/support/context_recovery.rs) | [24](24-recovery.md) | [25](25-openai.md) | 251 |
| [crates/wickle-state-sqlite/tests/support/mod.rs](../../../crates/wickle-state-sqlite/tests/support/mod.rs) | [13](13-sqlite.md) | [21](21-skills.md) | 382 |
| [crates/wickle-state-sqlite/tests/support/recovery_store.rs](../../../crates/wickle-state-sqlite/tests/support/recovery_store.rs) | [24](24-recovery.md) | [24](24-recovery.md) | 151 |
| [crates/wickle-state-sqlite/tests/support/workers.rs](../../../crates/wickle-state-sqlite/tests/support/workers.rs) | [13](13-sqlite.md) | [13](13-sqlite.md) | 163 |
| [crates/wickle/src/agent.rs](../../../crates/wickle/src/agent.rs) | [15](15-agent.md) | [24](24-recovery.md) | 937 |
| [crates/wickle/src/agent/admission.rs](../../../crates/wickle/src/agent/admission.rs) | [15](15-agent.md) | [24](24-recovery.md) | 555 |
| [crates/wickle/src/agent/artifacts.rs](../../../crates/wickle/src/agent/artifacts.rs) | [21](21-skills.md) | [22](22-compaction.md) | 66 |
| [crates/wickle/src/agent/components.rs](../../../crates/wickle/src/agent/components.rs) | [19](19-adapters.md) | [20](20-sources.md) | 384 |
| [crates/wickle/src/agent/driver.rs](../../../crates/wickle/src/agent/driver.rs) | [15](15-agent.md) | [24](24-recovery.md) | 1199 |
| [crates/wickle/src/agent/hooks.rs](../../../crates/wickle/src/agent/hooks.rs) | [18](18-hooks.md) | [19](19-adapters.md) | 135 |
| [crates/wickle/src/agent/persistence.rs](../../../crates/wickle/src/agent/persistence.rs) | [24](24-recovery.md) | [24](24-recovery.md) | 209 |
| [crates/wickle/src/agent/recovery.rs](../../../crates/wickle/src/agent/recovery.rs) | [24](24-recovery.md) | [24](24-recovery.md) | 340 |
| [crates/wickle/src/agent/resume.rs](../../../crates/wickle/src/agent/resume.rs) | [17](17-resume.md) | [24](24-recovery.md) | 1191 |
| [crates/wickle/src/agent/sources.rs](../../../crates/wickle/src/agent/sources.rs) | [20](20-sources.md) | [20](20-sources.md) | 75 |
| [crates/wickle/src/agent/tools.rs](../../../crates/wickle/src/agent/tools.rs) | [16](16-tools.md) | [21](21-skills.md) | 321 |
| [crates/wickle/src/agent/verification.rs](../../../crates/wickle/src/agent/verification.rs) | [23](23-verification.md) | [23](23-verification.md) | 773 |
| [crates/wickle/src/artifacts.rs](../../../crates/wickle/src/artifacts.rs) | [21](21-skills.md) | [22](22-compaction.md) | 559 |
| [crates/wickle/src/budget.rs](../../../crates/wickle/src/budget.rs) | [07](07-budget.md) | [24](24-recovery.md) | 505 |
| [crates/wickle/src/clock.rs](../../../crates/wickle/src/clock.rs) | [07](07-budget.md) | [07](07-budget.md) | 94 |
| [crates/wickle/src/component_runtime.rs](../../../crates/wickle/src/component_runtime.rs) | [19](19-adapters.md) | [20](20-sources.md) | 990 |
| [crates/wickle/src/context.rs](../../../crates/wickle/src/context.rs) | [04](04-contracts.md) | [04](04-contracts.md) | 111 |
| [crates/wickle/src/context_projection.rs](../../../crates/wickle/src/context_projection.rs) | [10](10-context.md) | [22](22-compaction.md) | 1158 |
| [crates/wickle/src/context_source.rs](../../../crates/wickle/src/context_source.rs) | [20](20-sources.md) | [20](20-sources.md) | 294 |
| [crates/wickle/src/context_source/records.rs](../../../crates/wickle/src/context_source/records.rs) | [20](20-sources.md) | [20](20-sources.md) | 508 |
| [crates/wickle/src/context_source/runtime.rs](../../../crates/wickle/src/context_source/runtime.rs) | [20](20-sources.md) | [20](20-sources.md) | 537 |
| [crates/wickle/src/context_strategy.rs](../../../crates/wickle/src/context_strategy.rs) | [22](22-compaction.md) | [22](22-compaction.md) | 297 |
| [crates/wickle/src/context_strategy/compression.rs](../../../crates/wickle/src/context_strategy/compression.rs) | [22](22-compaction.md) | [22](22-compaction.md) | 214 |
| [crates/wickle/src/context_strategy/engine.rs](../../../crates/wickle/src/context_strategy/engine.rs) | [22](22-compaction.md) | [22](22-compaction.md) | 410 |
| [crates/wickle/src/context_strategy/model_compactor.rs](../../../crates/wickle/src/context_strategy/model_compactor.rs) | [22](22-compaction.md) | [23](23-verification.md) | 131 |
| [crates/wickle/src/context_strategy/operations.rs](../../../crates/wickle/src/context_strategy/operations.rs) | [22](22-compaction.md) | [22](22-compaction.md) | 265 |
| [crates/wickle/src/context_strategy/records.rs](../../../crates/wickle/src/context_strategy/records.rs) | [22](22-compaction.md) | [22](22-compaction.md) | 438 |
| [crates/wickle/src/context_strategy/runtime.rs](../../../crates/wickle/src/context_strategy/runtime.rs) | [22](22-compaction.md) | [22](22-compaction.md) | 221 |
| [crates/wickle/src/error.rs](../../../crates/wickle/src/error.rs) | [04](04-contracts.md) | [24](24-recovery.md) | 173 |
| [crates/wickle/src/future.rs](../../../crates/wickle/src/future.rs) | [23](23-verification.md) | [23](23-verification.md) | 14 |
| [crates/wickle/src/hooks.rs](../../../crates/wickle/src/hooks.rs) | [18](18-hooks.md) | [19](19-adapters.md) | 696 |
| [crates/wickle/src/hooks/records.rs](../../../crates/wickle/src/hooks/records.rs) | [18](18-hooks.md) | [21](21-skills.md) | 418 |
| [crates/wickle/src/hooks/runtime.rs](../../../crates/wickle/src/hooks/runtime.rs) | [18](18-hooks.md) | [21](21-skills.md) | 762 |
| [crates/wickle/src/input_binding.rs](../../../crates/wickle/src/input_binding.rs) | [11](11-binding.md) | [19](19-adapters.md) | 1555 |
| [crates/wickle/src/lib.rs](../../../crates/wickle/src/lib.rs) | [03](03-workspace.md) | [24](24-recovery.md) | 189 |
| [crates/wickle/src/message.rs](../../../crates/wickle/src/message.rs) | [04](04-contracts.md) | [21](21-skills.md) | 357 |
| [crates/wickle/src/model.rs](../../../crates/wickle/src/model.rs) | [04](04-contracts.md) | [24](24-recovery.md) | 298 |
| [crates/wickle/src/model_catalog.rs](../../../crates/wickle/src/model_catalog.rs) | [12](12-catalog.md) | [12](12-catalog.md) | 551 |
| [crates/wickle/src/model_dispatch.rs](../../../crates/wickle/src/model_dispatch.rs) | [14](14-routing.md) | [14](14-routing.md) | 170 |
| [crates/wickle/src/model_execution.rs](../../../crates/wickle/src/model_execution.rs) | [08](08-model.md) | [22](22-compaction.md) | 531 |
| [crates/wickle/src/model_execution/routed.rs](../../../crates/wickle/src/model_execution/routed.rs) | [14](14-routing.md) | [24](24-recovery.md) | 633 |
| [crates/wickle/src/model_protocol.rs](../../../crates/wickle/src/model_protocol.rs) | [08](08-model.md) | [14](14-routing.md) | 913 |
| [crates/wickle/src/model_routing.rs](../../../crates/wickle/src/model_routing.rs) | [14](14-routing.md) | [14](14-routing.md) | 416 |
| [crates/wickle/src/policy.rs](../../../crates/wickle/src/policy.rs) | [05](05-policy.md) | [24](24-recovery.md) | 472 |
| [crates/wickle/src/profile.rs](../../../crates/wickle/src/profile.rs) | [04](04-contracts.md) | [04](04-contracts.md) | 470 |
| [crates/wickle/src/recovery.rs](../../../crates/wickle/src/recovery.rs) | [24](24-recovery.md) | [24](24-recovery.md) | 50 |
| [crates/wickle/src/resolution.rs](../../../crates/wickle/src/resolution.rs) | [04](04-contracts.md) | [20](20-sources.md) | 711 |
| [crates/wickle/src/run.rs](../../../crates/wickle/src/run.rs) | [04](04-contracts.md) | [24](24-recovery.md) | 1099 |
| [crates/wickle/src/serialization.rs](../../../crates/wickle/src/serialization.rs) | [04](04-contracts.md) | [22](22-compaction.md) | 222 |
| [crates/wickle/src/skills.rs](../../../crates/wickle/src/skills.rs) | [21](21-skills.md) | [21](21-skills.md) | 417 |
| [crates/wickle/src/skills/records.rs](../../../crates/wickle/src/skills/records.rs) | [21](21-skills.md) | [21](21-skills.md) | 111 |
| [crates/wickle/src/skills/runtime.rs](../../../crates/wickle/src/skills/runtime.rs) | [21](21-skills.md) | [21](21-skills.md) | 564 |
| [crates/wickle/src/state.rs](../../../crates/wickle/src/state.rs) | [06](06-state.md) | [24](24-recovery.md) | 2180 |
| [crates/wickle/src/state/checkpoint.rs](../../../crates/wickle/src/state/checkpoint.rs) | [13](13-sqlite.md) | [24](24-recovery.md) | 659 |
| [crates/wickle/src/state/context_state.rs](../../../crates/wickle/src/state/context_state.rs) | [22](22-compaction.md) | [22](22-compaction.md) | 357 |
| [crates/wickle/src/state/hook_state.rs](../../../crates/wickle/src/state/hook_state.rs) | [18](18-hooks.md) | [22](22-compaction.md) | 296 |
| [crates/wickle/src/state/reconciliation_state.rs](../../../crates/wickle/src/state/reconciliation_state.rs) | [24](24-recovery.md) | [24](24-recovery.md) | 100 |
| [crates/wickle/src/state/recovery_state.rs](../../../crates/wickle/src/state/recovery_state.rs) | [24](24-recovery.md) | [24](24-recovery.md) | 225 |
| [crates/wickle/src/state/skill_state.rs](../../../crates/wickle/src/state/skill_state.rs) | [21](21-skills.md) | [21](21-skills.md) | 95 |
| [crates/wickle/src/state/source_state.rs](../../../crates/wickle/src/state/source_state.rs) | [20](20-sources.md) | [20](20-sources.md) | 205 |
| [crates/wickle/src/state/verification_state.rs](../../../crates/wickle/src/state/verification_state.rs) | [23](23-verification.md) | [23](23-verification.md) | 495 |
| [crates/wickle/src/tool_execution.rs](../../../crates/wickle/src/tool_execution.rs) | [16](16-tools.md) | [24](24-recovery.md) | 598 |
| [crates/wickle/src/tool_execution/reconciliation.rs](../../../crates/wickle/src/tool_execution/reconciliation.rs) | [24](24-recovery.md) | [24](24-recovery.md) | 398 |
| [crates/wickle/src/tool_execution/resume.rs](../../../crates/wickle/src/tool_execution/resume.rs) | [17](17-resume.md) | [24](24-recovery.md) | 327 |
| [crates/wickle/src/tool_execution/round.rs](../../../crates/wickle/src/tool_execution/round.rs) | [16](16-tools.md) | [24](24-recovery.md) | 1022 |
| [crates/wickle/src/tool_schema.rs](../../../crates/wickle/src/tool_schema.rs) | [09](09-schema.md) | [11](11-binding.md) | 719 |
| [crates/wickle/src/verification.rs](../../../crates/wickle/src/verification.rs) | [23](23-verification.md) | [23](23-verification.md) | 484 |
| [crates/wickle/src/views.rs](../../../crates/wickle/src/views.rs) | [05](05-policy.md) | [24](24-recovery.md) | 161 |
| [crates/wickle/tests/agent_hooks.rs](../../../crates/wickle/tests/agent_hooks.rs) | [18](18-hooks.md) | [18](18-hooks.md) | 830 |
| [crates/wickle/tests/agent_recovery.rs](../../../crates/wickle/tests/agent_recovery.rs) | [24](24-recovery.md) | [24](24-recovery.md) | 409 |
| [crates/wickle/tests/agent_resume.rs](../../../crates/wickle/tests/agent_resume.rs) | [17](17-resume.md) | [24](24-recovery.md) | 1322 |
| [crates/wickle/tests/agent_runtime.rs](../../../crates/wickle/tests/agent_runtime.rs) | [15](15-agent.md) | [35](35-integration.md) | 1087 |
| [crates/wickle/tests/agent_tool_loop.rs](../../../crates/wickle/tests/agent_tool_loop.rs) | [16](16-tools.md) | [23](23-verification.md) | 2092 |
| [crates/wickle/tests/artifacts.rs](../../../crates/wickle/tests/artifacts.rs) | [21](21-skills.md) | [21](21-skills.md) | 339 |
| [crates/wickle/tests/budget.rs](../../../crates/wickle/tests/budget.rs) | [07](07-budget.md) | [16](16-tools.md) | 730 |
| [crates/wickle/tests/context_projection.rs](../../../crates/wickle/tests/context_projection.rs) | [10](10-context.md) | [21](21-skills.md) | 1428 |
| [crates/wickle/tests/context_sources.rs](../../../crates/wickle/tests/context_sources.rs) | [20](20-sources.md) | [20](20-sources.md) | 609 |
| [crates/wickle/tests/contracts.rs](../../../crates/wickle/tests/contracts.rs) | [04](04-contracts.md) | [24](24-recovery.md) | 1194 |
| [crates/wickle/tests/input_binding.rs](../../../crates/wickle/tests/input_binding.rs) | [11](11-binding.md) | [16](16-tools.md) | 1348 |
| [crates/wickle/tests/model_execution.rs](../../../crates/wickle/tests/model_execution.rs) | [08](08-model.md) | [12](12-catalog.md) | 806 |
| [crates/wickle/tests/model_protocol.rs](../../../crates/wickle/tests/model_protocol.rs) | [08](08-model.md) | [12](12-catalog.md) | 571 |
| [crates/wickle/tests/policy.rs](../../../crates/wickle/tests/policy.rs) | [05](05-policy.md) | [24](24-recovery.md) | 922 |
| [crates/wickle/tests/skills.rs](../../../crates/wickle/tests/skills.rs) | [21](21-skills.md) | [22](22-compaction.md) | 897 |
| [crates/wickle/tests/state.rs](../../../crates/wickle/tests/state.rs) | [06](06-state.md) | [24](24-recovery.md) | 1004 |
| [crates/wickle/tests/state_checkpoint.rs](../../../crates/wickle/tests/state_checkpoint.rs) | [13](13-sqlite.md) | [17](17-resume.md) | 450 |
| [crates/wickle/tests/state_hooks.rs](../../../crates/wickle/tests/state_hooks.rs) | [18](18-hooks.md) | [18](18-hooks.md) | 187 |
| [crates/wickle/tests/state_resume_invariants.rs](../../../crates/wickle/tests/state_resume_invariants.rs) | [17](17-resume.md) | [17](17-resume.md) | 446 |
| [crates/wickle/tests/state_sources.rs](../../../crates/wickle/tests/state_sources.rs) | [20](20-sources.md) | [20](20-sources.md) | 280 |
| [crates/wickle/tests/support/agent.rs](../../../crates/wickle/tests/support/agent.rs) | [15](15-agent.md) | [24](24-recovery.md) | 815 |
| [crates/wickle/tests/support/agent_hooks.rs](../../../crates/wickle/tests/support/agent_hooks.rs) | [18](18-hooks.md) | [18](18-hooks.md) | 602 |
| [crates/wickle/tests/support/agent_resume.rs](../../../crates/wickle/tests/support/agent_resume.rs) | [17](17-resume.md) | [24](24-recovery.md) | 879 |
| [crates/wickle/tests/support/context_sources.rs](../../../crates/wickle/tests/support/context_sources.rs) | [20](20-sources.md) | [20](20-sources.md) | 713 |
| [crates/wickle/tests/support/mod.rs](../../../crates/wickle/tests/support/mod.rs) | [07](07-budget.md) | [24](24-recovery.md) | 230 |
| [crates/wickle/tests/support/tool_execution.rs](../../../crates/wickle/tests/support/tool_execution.rs) | [16](16-tools.md) | [24](24-recovery.md) | 708 |
| [crates/wickle/tests/tool_execution.rs](../../../crates/wickle/tests/tool_execution.rs) | [16](16-tools.md) | [24](24-recovery.md) | 1240 |
| [crates/wickle/tests/tool_schema.rs](../../../crates/wickle/tests/tool_schema.rs) | [09](09-schema.md) | [12](12-catalog.md) | 634 |
| [crates/wickle/tests/verification.rs](../../../crates/wickle/tests/verification.rs) | [23](23-verification.md) | [23](23-verification.md) | 931 |
| [tests/host_contract/delivery.rs](../../../tests/host_contract/delivery.rs) | [33](33-events.md) | [33](33-events.md) | 570 |
| [tests/support/adapter_consumer.rs](../../../tests/support/adapter_consumer.rs) | [19](19-adapters.md) | [23](23-verification.md) | 878 |
| [tests/support/agent_consumer.rs](../../../tests/support/agent_consumer.rs) | [15](15-agent.md) | [23](23-verification.md) | 398 |
| [tests/support/anthropic_consumer.rs](../../../tests/support/anthropic_consumer.rs) | [27](27-anthropic.md) | [27](27-anthropic.md) | 177 |
| [tests/support/azure_consumer.rs](../../../tests/support/azure_consumer.rs) | [26](26-azure.md) | [26](26-azure.md) | 167 |
| [tests/support/bedrock_consumer.rs](../../../tests/support/bedrock_consumer.rs) | [28](28-bedrock.md) | [28](28-bedrock.md) | 174 |
| [tests/support/budget_consumer.rs](../../../tests/support/budget_consumer.rs) | [07](07-budget.md) | [24](24-recovery.md) | 271 |
| [tests/support/catalog_consumer.rs](../../../tests/support/catalog_consumer.rs) | [12](12-catalog.md) | [12](12-catalog.md) | 179 |
| [tests/support/compaction_consumer.rs](../../../tests/support/compaction_consumer.rs) | [22](22-compaction.md) | [23](23-verification.md) | 450 |
| [tests/support/consumer.rs](../../../tests/support/consumer.rs) | [03](03-workspace.md) | [04](04-contracts.md) | 105 |
| [tests/support/context_consumer.rs](../../../tests/support/context_consumer.rs) | [10](10-context.md) | [24](24-recovery.md) | 414 |
| [tests/support/event_consumer.rs](../../../tests/support/event_consumer.rs) | [33](33-events.md) | [33](33-events.md) | 600 |
| [tests/support/gather_consumer.rs](../../../tests/support/gather_consumer.rs) | [35](35-integration.md) | [35](35-integration.md) | 641 |
| [tests/support/gemini_consumer.rs](../../../tests/support/gemini_consumer.rs) | [29](29-gemini.md) | [29](29-gemini.md) | 167 |
| [tests/support/hooks_consumer.rs](../../../tests/support/hooks_consumer.rs) | [18](18-hooks.md) | [23](23-verification.md) | 688 |
| [tests/support/input_binding_consumer.rs](../../../tests/support/input_binding_consumer.rs) | [11](11-binding.md) | [24](24-recovery.md) | 422 |
| [tests/support/lifecycle_consumer.rs](../../../tests/support/lifecycle_consumer.rs) | [35](35-integration.md) | [35](35-integration.md) | 396 |
| [tests/support/mcp_consumer.rs](../../../tests/support/mcp_consumer.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 162 |
| [tests/support/mcp_fixture.rs](../../../tests/support/mcp_fixture.rs) | [32](32-mcp.md) | [32](32-mcp.md) | 72 |
| [tests/support/model_adapters_consumer.rs](../../../tests/support/model_adapters_consumer.rs) | [31](31-xai.md) | [31](31-xai.md) | 245 |
| [tests/support/model_consumer.rs](../../../tests/support/model_consumer.rs) | [08](08-model.md) | [12](12-catalog.md) | 250 |
| [tests/support/model_http.rs](../../../tests/support/model_http.rs) | [26](26-azure.md) | [26](26-azure.md) | 173 |
| [tests/support/openai_consumer.rs](../../../tests/support/openai_consumer.rs) | [25](25-openai.md) | [25](25-openai.md) | 170 |
| [tests/support/policy_consumer.rs](../../../tests/support/policy_consumer.rs) | [05](05-policy.md) | [05](05-policy.md) | 132 |
| [tests/support/recovery_consumer.rs](../../../tests/support/recovery_consumer.rs) | [24](24-recovery.md) | [27](27-anthropic.md) | 116 |
| [tests/support/report_process_consumer.rs](../../../tests/support/report_process_consumer.rs) | [35](35-integration.md) | [36](36-release.md) | 408 |
| [tests/support/resume_consumer.rs](../../../tests/support/resume_consumer.rs) | [17](17-resume.md) | [23](23-verification.md) | 716 |
| [tests/support/routing_consumer.rs](../../../tests/support/routing_consumer.rs) | [14](14-routing.md) | [24](24-recovery.md) | 495 |
| [tests/support/skills_consumer.rs](../../../tests/support/skills_consumer.rs) | [21](21-skills.md) | [23](23-verification.md) | 250 |
| [tests/support/source_consumer.rs](../../../tests/support/source_consumer.rs) | [20](20-sources.md) | [23](23-verification.md) | 567 |
| [tests/support/sqlite_consumer.rs](../../../tests/support/sqlite_consumer.rs) | [13](13-sqlite.md) | [24](24-recovery.md) | 294 |
| [tests/support/state_consumer.rs](../../../tests/support/state_consumer.rs) | [06](06-state.md) | [24](24-recovery.md) | 208 |
| [tests/support/tool_loop_consumer.rs](../../../tests/support/tool_loop_consumer.rs) | [16](16-tools.md) | [23](23-verification.md) | 530 |
| [tests/support/tool_schema_consumer.rs](../../../tests/support/tool_schema_consumer.rs) | [09](09-schema.md) | [12](12-catalog.md) | 176 |
| [tests/support/verification_consumer.rs](../../../tests/support/verification_consumer.rs) | [23](23-verification.md) | [23](23-verification.md) | 355 |
| [tests/support/version_matrix_consumer.rs](../../../tests/support/version_matrix_consumer.rs) | [34](34-evidence.md) | [34](34-evidence.md) | 246 |
| [tests/support/vertex_consumer.rs](../../../tests/support/vertex_consumer.rs) | [30](30-vertex.md) | [30](30-vertex.md) | 163 |
| [tests/support/xai_consumer.rs](../../../tests/support/xai_consumer.rs) | [31](31-xai.md) | [31](31-xai.md) | 174 |

## 정확한 API reference 만들기

```sh
cargo doc --workspace --no-deps --locked
```

복원한 실습 폴더에서 실행한다. `target/doc/wickle/index.html`과 각 crate의 index를 브라우저로 열면 release 소스의 public signature·field·doc comment를 모두 읽을 수 있다. 이것이 생략 없는 API reference이며, 강의는 이를 사용하는 순서와 이유를 설명한다. private module 경로를 외부 API로 사용하지 않는다.

API 하나를 찾으면 정의 → impl → 호출자 → 해당 기능 테스트 → 독립 consumer 순서로 읽는다. 검색은 예를 들어 `rg 'impl .*StateStore|fn commit' crates`처럼 한다. `rg`가 없다면 편집기의 전체 검색으로 같은 이름을 찾는다.

## Rust 이외의 파일

Cargo.toml은 crate/dependency/lint 계약, Cargo.lock은 재현할 dependency graph, rust-toolchain.toml은 개발 compiler, `.github/workflows/ci.yml`은 검사 환경이다. `scripts/check-package.py`는 배포 파일만 사용하는 소비자 검증을 수행한다. 문서와 LICENSE도 정답 patch에 포함된다. 이들은 최종 `course.json`의 `files` 목록에서 모두 확인할 수 있다.
