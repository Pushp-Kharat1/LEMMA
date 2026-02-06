# mm-boink

Budget-Optimized Inference with Neural Knowledge (BOINK) is the resource management and self-regulation layer for the LEMMA mathematical engine.

## Overview

The purpose of this crate is to improve search efficiency by imposing a credit-based scarcity model on rule applications. It prevents the search engine from exploring mathematically irrelevant paths and ensures that high-complexity problems are allocated more computational resources than trivial ones.

## Core Components

### 1. Credit Bank (bank.rs)
The Bank manages the persistent state of LEMMA's computational currency. 
- Tracks total credits earned through efficient problem solving.
- Handles credit withdrawals for rule applications.
- Supports trading large credit balances for permanent engine upgrades (e.g., increased search depth or extra retry attempts).
- Persists state via JSON for continuity between sessions.

### 2. Budget Allocation (budget.rs)
The Budget module determines the "cost of living" for a specific problem.
- Classifies problems into difficulty levels: Easy, Medium, Hard, and Olympiad.
- Allocates credits based on detected mathematical domains (e.g., Calculus requires a larger budget than basic Algebra).
- Implements penalty logic where repeated failures on a specific problem type reduce the budget for future attempts of that type.

### 3. Guardrails (guardrail.rs)
The Guardrail system acts as a structural filter before the search begins.
- Recursively analyzes the Abstract Syntax Tree (AST) of a mathematical expression.
- Identifies specific domains (Trigonometry, Number Theory, Calculus, etc.).
- Filters the global rule set to include only those relevant to the detected domains, preventing the application of irrelevant transformations (e.g., preventing Number Theory rules from being tested on Calculus problems).

### 4. Supervisor (supervisor.rs)
The Supervisor implements the feedback loop that drives engine evolution.
- Monitors problem-solving runs.
- Rewards the Bank when a solution is found under-budget.
- Records overspends and calculates the resulting budget penalties for the next run.
- Manages the interaction between the Budget and the Bank.

### 5. Patterns (patterns.rs)
A direct pattern-matching layer designed to bypass the expensive MCTS search for common identities.
- Currently implements basic integration patterns (Power rule, Exponential, Trigonometric).
- Reduces credit consumption by providing instant "shortcuts" for well-known mathematical forms.

## Integrity and Limitations

### Accuracy
The system relies entirely on the accuracy of the mathematical metadata assigned to rules. If a rule is miscategorized in the `mm-rules` crate, the BOINK guardrails may incorrectly filter it out.

### State of Development
The current implementation is in the early integration phase. While the core logic for bank persistence and domain filtering is functional, the cost-per-rule estimation is currently using heuristic averages rather than dynamically learned weights.

### Performance
The overhead of AST analysis in the guardrail is negligible compared to the search space reduction it provides. By filtering out irrelevant rules, it significantly reduces the branching factor in the MCTS tree.
