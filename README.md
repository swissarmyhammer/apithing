# API Thing

A standardized API approach based on context and parameter traits.

[![CI](https://github.com/swissarmyhammer/apithing/workflows/CI/badge.svg)](https://github.com/swissarmyhammer/apithing/actions)
[![Crates.io](https://img.shields.io/crates/v/apithing.svg)](https://crates.io/crates/apithing)
[![Documentation](https://docs.rs/apithing/badge.svg)](https://docs.rs/apithing)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/swissarmyhammer/apithing#license)

## Overview

ApiThing provides a trait-based framework for building consistent, type-safe APIs using shared contexts and parameter objects. The framework enables you to define operations that are composable, testable, and maintainable while enforcing consistent patterns across different API families.

### Key Features

- **Type-Safe Operations**: Define operations with compile-time validated inputs and outputs
- **Shared Context**: Reuse contexts (database connections, caches, etc.) across multiple operations
- **Composable Design**: Build complex workflows from simple, focused operations
- **Error Handling**: Rich, domain-specific error types with comprehensive error propagation
- **Multiple API Families**: Support different operation families that share the same context
- **Executor Pattern**: Optional ergonomic API for managing stateful operation execution

## Installation

```
cargo add apithing
```

## Quick Start

Read (./examples/basic_usage.rs)

## Core Architecture

ApiThing is built around several key concepts:

```mermaid
graph TB
    Context["Context (C)<br/>• Shared state<br/>• Resources<br/>• Connections"]
    Operation["ApiOperation&lt;C,P&gt;<br/>fn execute()<br/>→ Output<br/>→ Error"]
    Parameters["Parameters (P)<br/>• Input params<br/>• Validation<br/>• Type safety"]

    Execute["Execute&lt;C,P&gt;<br/>execute_on()<br/>(ergonomic API)"]

    ApiExecutor["ApiExecutor&lt;C&gt;<br/>• Stateful<br/>• Context mgmt<br/>• Multi-ops"]

    Context --> Operation
    Parameters --> Operation
    Operation --> Execute
    Execute --> ApiExecutor
```

### Core Concepts

- **Operations**: Implement `ApiOperation<C, P>` trait with `execute()` method
- **Context**: Shared state across operations (database connections, caches, etc.)
- **Parameters**: Type-safe input parameters that define operation behavior
- **Output**: Strongly-typed results returned by operations
- **Error**: Domain-specific error types for comprehensive error handling

## Examples

The repository includes several comprehensive examples:

- **[basic_usage.rs](examples/basic_usage.rs)**: Direct operation execution patterns
- **[executor_pattern.rs](examples/executor_pattern.rs)**: Stateful executor usage
- **[advanced_patterns.rs](examples/advanced_patterns.rs)**: Complex workflows and patterns

Run examples with:

```bash
cargo run --example basic_usage
cargo run --example executor_pattern
cargo run --example advanced_patterns
```
