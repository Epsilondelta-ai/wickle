# Wickle

[English](README.md) | **한국어** | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**엡실론델타가 만드는 확장 가능한 에이전트 엔진.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="쳇바퀴를 달리는 귀여운 고슴도치, Wickle 마스코트" width="420" />
</p>

Wickle은 엡실론델타가 Rust로 개발하는 에이전트 엔진입니다. 모델의 판단과 도구 호출을 반복하는 실행 루프를 애플리케이션에서 사용하는 라이브러리로 제공하는 것을 목표로 합니다.

Agent Profile로 에이전트의 동작을 구성하고, 어댑터를 통해 다양한 모델과 도구를 연결하는 구조로 설계하고 있습니다.

**현재 상태:** 개발 중입니다. 현재 crate에는 빌드·테스트 기반이 있으며, 에이전트 실행 기능은 아직 구현되지 않았습니다.
