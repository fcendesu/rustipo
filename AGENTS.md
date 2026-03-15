# Agent Guide

This file defines how contributors and AI coding agents should work in this repository.

## Working style

- Make focused, incremental changes.
- Prefer clarity over cleverness.
- Keep behavior changes and refactors separate when practical.
- Avoid touching unrelated code while implementing a task.

## Commit messages

Use Conventional Commits:
`<type>(<scope>): <subject>`

Examples:

- `feat(cli): add build command`
- `fix(core): handle invalid input`
- `chore(repo): simplify docs`

## Commit size

Prefer small, single-intent commits.

Rules:

- 1 commit = 1 logical change
- keep commits reviewable and easy to revert
- keep commits runnable/testable when possible
- avoid large mixed commits and noisy micro-commits

## File structure and maintainability

Keep the codebase focused and maintainable.

Rules:

- 1 file = 1 clear responsibility
- separate parsing, business logic, and IO/rendering concerns
- target ~150-300 lines per file; split files beyond ~350 lines
- target ~40-60 lines per function; extract helpers when needed
- avoid adding unrelated logic to large files
- keep behavior changes and pure refactors separate when practical

## Coding guidelines

- Prefer explicit and readable code.
- Keep public APIs simple.
- Use actionable error messages with useful context.
- Add comments only when intent is not obvious.
- Avoid unnecessary dependencies.

## Architecture expectations

- Preserve clear module boundaries.
- Keep models, domain logic, and side effects loosely coupled.
- Favor deterministic behavior where outputs are generated from inputs.
- Choose the simplest design that satisfies requirements.

## Testing and verification

Before considering work done, ensure:

- code compiles
- happy path works
- at least one failure path is handled
- behavior is covered by checks appropriate to the change

Prioritize tests for:

- parsing and validation
- route/path generation
- error handling
- regressions in changed modules

## Scope control

- Implement only what the task requires.
- Do not introduce new features outside requested scope.
- If requirements are ambiguous, follow existing patterns in the codebase.
